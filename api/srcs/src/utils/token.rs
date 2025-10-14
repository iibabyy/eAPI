use actix_web::{http, HttpRequest};
use bcrypt::DEFAULT_COST;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ErrorMessage, ErrorResponse, HttpError};

use super::password;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
	// subject
	pub sub: Uuid,

	// created at
	pub iat: usize,

	// expire {exp} seconds(?) after iat
	pub exp: usize,

	// JWT id
	pub jti: String,
}

pub fn create_token(
	user_id: &Uuid,
	secret: &[u8],
	expires_in_seconds: i64,
	token_id: &Uuid,
) -> Result<String, jsonwebtoken::errors::Error> {
	let now = Utc::now();
	let iat = now.timestamp() as usize;
	let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
	let jti = token_id.to_string();

	let claims = TokenClaims {
		sub: *user_id,
		iat,
		exp,
		jti,
	};

	jsonwebtoken::encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(secret),
	)

}

pub fn decode_token(token: impl Into<String>, secret: &[u8]) -> Result<TokenClaims, HttpError> {
	let decoded = jsonwebtoken::decode::<TokenClaims>(
		&token.into(),
		&DecodingKey::from_secret(secret),
		&Validation::new(jsonwebtoken::Algorithm::HS256)
	);

	match decoded {
		Ok(token) => {
			
			// if token.claims.exp < Utc::now().timestamp() as usize { panic!("Expired !") }
			Ok(token.claims)
		},
		Err(err) => {
			HttpError::new(ErrorMessage::InvalidToken, 401).into()
		},
	}
}

fn jwt_failed(message: impl ToString) -> ErrorResponse {
	ErrorResponse {
		status: "fail".to_string(),
		message: message.to_string(),
	}
}

pub fn extract_token_from(request: &HttpRequest) -> Result<String, ErrorResponse> {
	let value = request.headers()
		.get(http::header::AUTHORIZATION);

	if value.is_none() { return Err(jwt_failed(ErrorMessage::TokenNotProvided)) }

	let value = match value.unwrap().to_str() {
		Ok(value) => value,
		Err(err) => return Err(jwt_failed(ErrorMessage::InvalidToken)),
	};

	let (token_type, token_value) = match value.split_once(' ') {
		Some(result) => result,
		None => return Err(jwt_failed(ErrorMessage::InvalidToken)),
	};

	if token_type != "Bearer" { return Err(jwt_failed(ErrorMessage::InvalidToken)) }

	return Ok(token_value.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_and_decode_token() {
		let user_id = Uuid::new_v4();
		let secret = b"my-super-secret-key";

		let token = create_token(&user_id, secret, 60, &Uuid::new_v4()).unwrap();
		let decoded_id = decode_token(&token, secret).unwrap().sub;

		assert_eq!(decoded_id, user_id);
	}

	#[test]
	fn decode_invalid_token() {
		let secret = b"my-super-secret-key";
		let invalid_token = "invalid-token";

		let result = decode_token(invalid_token, secret);

		assert!(result.is_err());

		let error = result.unwrap_err();
		assert_eq!(error.status, 401);
		assert_eq!(error.message, ErrorMessage::InvalidToken.to_string())
	}

	#[test]
	fn decode_expired_token() {
		let secret = b"my-super-secret-key";
		let expired_token = create_token(&Uuid::new_v4(), secret, -60, &Uuid::new_v4()).unwrap();

		let result = decode_token(expired_token, secret);

		assert!(result.is_err());

		let error = result.unwrap_err();
		assert_eq!(error.status, 401);
		assert_eq!(error.message, ErrorMessage::InvalidToken.to_string())
	}


}
