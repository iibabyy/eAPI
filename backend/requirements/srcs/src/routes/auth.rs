use actix_web::{cookie::{time::Duration, CookieBuilder, SameSite}, post, web::{self, Json}, HttpRequest, HttpResponse};
use serde_json::json;
use validator::Validate;

use crate::{database::UserExtractor, dtos::{user::{FilterUserDto, LoginUserDto, RegisterUserDto, UserData, UserLoginResponseDto, UserResponseDto}, Status}, error::{ErrorMessage, HttpError}, extractors::auth::RequireAuth, utils::{password, token, AppState}};


pub(super) fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/auth")
			.service(login)
			.service(register)
			.service(logout)
		);
}


const TOKEN_MAX_AGE_IN_SECONDS: i64 = 5 * 60;

#[post("/login")]
async fn login (
    infos: Json<LoginUserDto>,
	data: web::Data<AppState>
) -> Result <HttpResponse, HttpError> {
	let infos = infos.into_inner();

	infos.validate()
		.map_err(|err| HttpError::bad_request(err.to_string()))?;

	// searching user

	let result = data
		.db_client
		.get_user_by_email(infos.email)
		.await
		.map_err(|err| HttpError::server_error(err.to_string()))?;

	if result.is_none() { return Err(HttpError::unauthorized(ErrorMessage::WrongCredentials)) }
	let user = result.unwrap();
	
	// check passwords

	let password_matches = match password::compare(&infos.password, &user.password) {
		Ok(result) => result,
		Err(ErrorMessage::HashingError) => return Err(HttpError::server_error(ErrorMessage::HashingError)),
		Err(err) => return Err(HttpError::unauthorized(err)),
	};

	if password_matches == false { return Err(HttpError::unauthorized(ErrorMessage::WrongCredentials)) }

	// building response

	let token = token::create_token(
		&user.id.to_string(),
		data.env.secret_key.as_bytes(),
		TOKEN_MAX_AGE_IN_SECONDS,
	)
	.map_err(|_| HttpError::server_error(ErrorMessage::HashingError))?;
	
	let filtered_user = FilterUserDto::filter_user(&user);

	let cookie = CookieBuilder::new("token", token.to_string())
		.path("/")
		.max_age(Duration::minutes(TOKEN_MAX_AGE_IN_SECONDS))
		.http_only(true)
		// .same_site(SameSite::Strict)
		.finish();

    Ok(
		 HttpResponse::Ok()
		 	.cookie(cookie)
			.json(UserResponseDto {
				status: Status::Success,
				data: UserData { user: filtered_user },
			})
	)
}

#[post("/register")]
async fn register(
    infos: Json<RegisterUserDto>,
	data: web::Data<AppState>
) -> Result<HttpResponse, HttpError> {
	let infos = infos.into_inner();

	infos.validate()
		.map_err(|err| HttpError::bad_request(err.to_string()))?;

	let hashed_password = match password::hash(infos.password) {
		Ok(hash) => hash,
		Err(ErrorMessage::HashingError) => return Err(HttpError::server_error(ErrorMessage::HashingError)),
		Err(err) => return Err(HttpError::server_error(err)),
	};

	let result = data.db_client
		.save_user(
			infos.name,
			infos.email,
			hashed_password
		)
		.await;

	match result {
		Ok(user) => Ok(HttpResponse::Created().json(UserResponseDto {
			status: Status::Success,
			data: UserData {
				user: FilterUserDto::filter_user(&user),
			}
		})),

		Err(sqlx::Error::Database(db_err)) => {
			if db_err.is_unique_violation()		{ Err(HttpError::unique_constraint_voilation(ErrorMessage::EmailExist)) }
			else	{ Err(HttpError::server_error(db_err.to_string())) }
		},

		Err(err) => Err(HttpError::server_error(err.to_string())),
	}
}

#[post("/logout", wrap = "RequireAuth")]
async fn logout() -> HttpResponse {
	let cookie = CookieBuilder::new("token", "")
		.path("/")
		.max_age(Duration::seconds(-1))
		.http_only(true)
		.finish();

	HttpResponse::Ok()
		.cookie(cookie)
		.json(json!({"status": "succes"}))
}


// #[cfg(test)]
// mod tests {
// 	use crate::database::db::DBClient;

// use super::*;

// }
