use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use bcrypt::BcryptError;
use serde::{Deserialize, Serialize};

use crate::utils::status::Status;


#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
	pub status: String,
	pub message: String,
}

impl fmt::Display for ErrorResponse {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).unwrap())
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub status: Status,
	pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    PasswordTooLong(usize),
    PasswordTooShort(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    UserNotFound,
    ProductNoLongerExist,
    ProductOutOfStock,
    ProductNotFound,
    NotEnoughProducts(i32),
    OrderNoLongerExist,
    OrderNotFound,
    TokenNotProvided,
    SoldTooLow,
    RefreshTokenNotProvided,
    PermissionDenied,
    AutoBuying,
}

impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".to_string(),
            ErrorMessage::WrongCredentials => "Email or password is wrong".to_string(),
            ErrorMessage::EmailExist => "An User with this email already exists".to_string(),
            ErrorMessage::UserNoLongerExist => "User no longer exists".to_string(),
            ErrorMessage::ProductNoLongerExist => "Product no longer exists".to_string(),
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::PasswordTooLong(max_length) => format!("Password must not be more than {} characters", max_length),
            ErrorMessage::PasswordTooShort(max_length) => format!("Password must not be less than {} characters", max_length),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => "You are not logged in, please provide token".to_string(),
            ErrorMessage::RefreshTokenNotProvided => "Refresh token not found, please log in".to_string(),
            ErrorMessage::PermissionDenied => "You are not allowed to perform this action".to_string(),
            ErrorMessage::ProductNotFound => "Product not found".to_string(),
            ErrorMessage::UserNotFound => "User not found".to_string(),
            ErrorMessage::OrderNotFound => "Order not found".to_string(),
            ErrorMessage::OrderNoLongerExist => "Order no longer exists".to_string(),
            ErrorMessage::NotEnoughProducts(stock) if stock > &0 => format!("Only {stock} products remaining"),
            ErrorMessage::NotEnoughProducts(_) => "0 products remaining".to_string(),
            ErrorMessage::SoldTooLow => "Sold too low".to_string(),
            ErrorMessage::AutoBuying => "Impossible to buy your own article".to_string(),
            ErrorMessage::ProductOutOfStock => "Product out of stock".to_string(),
        }
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl From<BcryptError> for ErrorMessage {
    fn from(value: BcryptError) -> Self {
        return ErrorMessage::HashingError
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
	pub message: String,
	pub status: u16,
}

impl<T> Into<Result<T, HttpError>> for HttpError {
    fn into(self) -> Result<T, HttpError> {
        Err(self)
    }
}

impl From<sqlx::Error> for HttpError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) => {
                let message = db_err.message();

                match message {
                    "auto-buying" => HttpError::bad_request(ErrorMessage::AutoBuying),
                    _ => {
                        eprintln!("Warning: unknown database error: {} -> convert it to server error", message);
                        HttpError::server_error(ErrorMessage::ServerError)
                    },
                }
            },

            _ => HttpError::server_error(ErrorMessage::ServerError),
        }
    }
}

impl HttpError {
	pub fn new(message: impl Into<String>, status: u16) -> Self {
		HttpError {
			message: message.into(),
			status,
		}
	}

	pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 500,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400,
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 409,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 401,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 404,
        }
    }

    pub fn payment_required(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 402,
        }
    }

	pub fn into_http_response(self) -> HttpResponse {
		match self.status {
			400 => HttpResponse::BadRequest().json(Response {
                status: Status::Failure,
                message: self.message.into(),
            }),

			401 => HttpResponse::Unauthorized().json(Response {
                status: Status::Failure,
                message: self.message.into(),
            }),

			402 => HttpResponse::PaymentRequired().json(Response {
                status: Status::Error,
                message: self.message.into(),
            }),
            
			404 => HttpResponse::NotFound().json(Response {
                status: Status::Failure,
                message: self.message.into(),
            }),
            
			409 => HttpResponse::Conflict().json(Response {
                status: Status::Failure,
                message: self.message.into(),
            }),
           
		    500 => HttpResponse::InternalServerError().json(Response {
                status: Status::Error,
                message: self.message.into(),
            }),

			_ => {
                eprintln!(
                    "Warning: Missing pattern match. Converted status code {} to 500.",
                    self.status
                );

                HttpResponse::InternalServerError().json(Response {
                    status: Status::Error,
                    message: ErrorMessage::ServerError.to_string(),
                })
            }
		}
	}

}

impl fmt::Display for HttpError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,
			"HttpError: message: {}, status {}",
			self.message,
			self.status
		)
	}
}


impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
		self.clone().into_http_response()
	}
}
