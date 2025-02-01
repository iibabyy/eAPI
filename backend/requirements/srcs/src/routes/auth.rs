use actix_web::{cookie::{time::Duration, CookieBuilder}, get, post, web::{self, Json}, HttpRequest, HttpResponse};
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::json;
use validator::Validate;

use crate::{database::UserExtractor, dtos::*, error::{ErrorMessage, HttpError}, extractors::auth::RequireAuth, utils::{constants, password, status::Status, token::{self, extract_token_from, TokenClaims}, AppState}};


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
		data.env.jwt_max_seconds,
	)
	.map_err(|_| HttpError::server_error(ErrorMessage::HashingError))?;

	let refresh_token = token::create_token(
		&user.id.to_string(),
		data.env.secret_key.as_bytes(),
		data.env.jwt_max_seconds,
	)
	.map_err(|_| HttpError::server_error(ErrorMessage::HashingError))?;

	let filtered_user = FilterUserDto::filter_user(&user);

	let cookie = CookieBuilder::new(constants::REFRESH_TOKEN.to_string(), refresh_token)
		.path("/")
		.max_age(Duration::minutes(data.env.jwt_max_seconds))
		.http_only(true)
		// .same_site(SameSite::Strict)
		.finish();

    Ok(
		 HttpResponse::Ok()
		 	.cookie(cookie)
			.json(UserLoginResponseDto {
				status: Status::Success,
				token,
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

#[get("/logout")]
async fn logout() -> HttpResponse {
	let cookie = CookieBuilder::new(constants::REFRESH_TOKEN.clone(), "")
		.path("/")
		.max_age(Duration::seconds(-1))
		.http_only(true)
		.finish();

	HttpResponse::Ok()
		.cookie(cookie)
		.json(json!({"status": Status::Success}))
}

#[post("/refresh")]
async fn refresh(
	request: HttpRequest,
	data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {

	// verify deprecated token
	let deprecated_token = match extract_token_from(&request) {
		Ok(token) => token,
		Err(err) => return Err(HttpError::unauthorized(err.to_string())),
	};

	// find refresh-token
	let refresh_token = request
		.cookie(&constants::REFRESH_TOKEN)
		.ok_or_else(|| HttpError::unauthorized(ErrorMessage::RefreshTokenNotProvided))?;

	let user_id = token::decode_token(refresh_token.value(), data.env.secret_key.as_bytes())?;

	let new_token = token::create_token(&user_id, data.env.secret_key.as_bytes(), data.env.jwt_max_seconds)
		.map_err(|_| HttpError::server_error(ErrorMessage::HashingError))?;

	Ok(
		HttpResponse::Ok().json(TokenResponseDto {
			status: Status::Success,
			token: new_token,
		})
	)
}

#[cfg(test)]
mod tests {
	use actix_web::{http::{self, header::HeaderName}, test, App};
	use sqlx::{Pool, Postgres};

	use crate::{database::db::DBClient, error::ErrorResponse, utils::test_utils::test_config};
	use super::*;


	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn register_valid_user(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let name = "Ayoub Arab".to_string();
		let email = "ayarab@gmail.com".to_string();
		let password = "password".to_string();

		let request = test::TestRequest::post()
			.uri("/auth/register")
			.set_json(RegisterUserDto {
				name: name.clone(),
				email: email.clone(),
				password: password.clone(),
				password_confirm: password.clone(),
			})
			.to_request();

		let response = test::call_service(&app, request).await;

		assert_eq!(response.status(), http::StatusCode::CREATED);

		let body = test::read_body(response).await;
		let response_data = serde_json::from_slice::<UserResponseDto>(&body)
			.expect("Failed to deserialize user response from JSON");

		let user = &response_data.data.user;

		assert_eq!(user.name, name);
		assert_eq!(user.email, email);

	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn register_duplicate_email(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();
		
		let name = "Idrissa 1".to_string();
		let email = "ibaby@gmail.com".to_string();
		let password = "password".to_string();

		db_client
			.save_user(&name, &email, &password)
			.await
			.unwrap();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/register")
			.set_json(RegisterUserDto {
				name: name.clone(),
				email: email.clone(),
				password: password.clone(),
				password_confirm: password.clone(),
			})
			.to_request();

		let response = test::call_service(&app, request).await;

		assert_eq!(response.status(), http::StatusCode::CONFLICT);

		let body = test::read_body(response).await;
		let expected_message = "An User with this email already exists".to_string();

		let mut body_json = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize Json");
		
		let actual_message = body_json["message"].as_str().unwrap();

		assert_eq!(expected_message, actual_message);
	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn login_valid_credentials(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		let name = "Ayoub Arab".to_string();
		let email = "ayarab@gmail.com".to_string();
		let password = "pawword".to_string();

		let hashed_password = password::hash(&password).unwrap();

		db_client
		.save_user(&name, &email, &hashed_password)
		.await
		.unwrap();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/login")
			.set_json(LoginUserDto {
				email: email.clone(),
				password: password.clone(),
			})
			.to_request();

		let response = test::call_service(&app, request).await;

		assert_eq!(response.status(), http::StatusCode::OK);

		let body = test::read_body(response).await;
		let response_data = serde_json::from_slice::<UserResponseDto>(&body)
			.expect("Failed to deserialize Json");
		
		let user = response_data.data.user;

		assert_eq!(email, user.email);
		assert_eq!(name, user.name);

	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn login_valid_credentials_receive_cookie(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		let name = "Ayoub Arab".to_string();
		let email = "ayarab@gmail.com".to_string();
		let password = "pawword".to_string();

		let hashed_password = password::hash(&password).unwrap();

		db_client
		.save_user(&name, &email, &hashed_password)
		.await
		.unwrap();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/login")
			.set_json(LoginUserDto {
				email: email.clone(),
				password: password.clone(),
			})
			.to_request();

		let response = test::call_service(&app, request).await;
		
		let cookie = response
			.response()
			.cookies()
			.find(|header| header.name() == "token");

		assert!(cookie.is_some());

	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn login_with_nonexistent_user(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/login")
			.set_json(LoginUserDto {
				email: "nonexistent@gmail.com".to_string(),
				password: "password".to_string(),
			})
			.to_request();

		let response = test::call_service(&app, request).await;
		
		assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);

		let body = test::read_body(response).await;
		let body = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize json");

		let expected_message = "Email or password is wrong";
		let actual_mesage = body["message"].as_str().unwrap();

		assert_eq!(actual_mesage, expected_message)
	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn login_with_wrong_email(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		db_client
			.save_user("Ayoub Arab", "ayarab@gmail.com", "password");

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/login")
			.set_json(LoginUserDto {
				email: "nonexistent@gmail.com".to_string(),
				password: "password".to_string(),
			})
			.to_request();

		let response = test::call_service(&app, request).await;
		
		assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);

		let body = test::read_body(response).await;
		let body = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize json");

		let expected_message = "Email or password is wrong";
		let actual_mesage = body["message"].as_str().unwrap();

		assert_eq!(actual_mesage, expected_message)
	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn login_with_wrong_password(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		db_client
			.save_user("Ayoub Arab", "ayarab@gmail.com", "password");

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/login")
			.set_json(LoginUserDto {
				email: "ayarab@gmail.com".to_string(),
				password: "wrongpassword".to_string(),
			})
			.to_request();

		let response = test::call_service(&app, request).await;
		
		assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);

		let body = test::read_body(response).await;
		let body = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize json");

		let expected_message = "Email or password is wrong";
		let actual_mesage = body["message"].as_str().unwrap();

		assert_eq!(actual_mesage, expected_message)
	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn login_with_no_data(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/login")
			.to_request();

		let response = test::call_service(&app, request).await;
		
		assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

		let body = test::read_body(response).await;
		let actual_message = String::from_utf8_lossy(&body);

		let expected_message = "Content type error";

		assert_eq!(actual_message, expected_message);
	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
	async fn login_with_empty_json(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_config();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					env: config,
					db_client,
				}))
				.configure(super::config)
		)
		.await;

		let request = test::TestRequest::post()
			.uri("/auth/login")
			.set_json(json!({}))
			.to_request();

		let response = test::call_service(&app, request).await;
		
		assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

		let body = test::read_body(response).await;
		let actual_message = String::from_utf8_lossy(&body);

		let expected_message = "Json deserialize error: missing field";

		assert!(actual_message.contains(expected_message));
	}

	#[sqlx::test(migrator="crate::MIGRATOR")]
    async fn logout_with_valid_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let config = test_config();
        let hashed_password = password::hash("password123").unwrap();
        let user = db_client
            .save_user("John", "john@example.com", &hashed_password)
            .await
            .unwrap();

        let token =
            token::create_token(&user.id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/logout")
			.cookie(CookieBuilder::new("token", token).finish())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let status_field_value = body_json["status"].as_str().unwrap();

        assert_eq!(status_field_value, "success");
    }

    #[sqlx::test(migrator="crate::MIGRATOR")]
    async fn logout_with_invalid_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/logout")
			.cookie(CookieBuilder::new("token", "invalid_token").finish())
            .to_request();

        let result = test::try_call_service(&app, req).await.err();

        match result {
            Some(err) => {
                let expected_status = http::StatusCode::UNAUTHORIZED;
                let actual_status = err.as_response_error().status_code();

                assert_eq!(actual_status, expected_status);

                let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
                    .expect("Failed to deserialize JSON string");
                let expected_message = ErrorMessage::InvalidToken.to_string();
                assert_eq!(err_response.message, expected_message);
            }
            None => {
                panic!("Service call succeeded, but an error was expected.");
            }
        }
    }

    #[sqlx::test(migrator="crate::MIGRATOR")]
    async fn logout_with_misssing_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/logout")
            .to_request();

        let result = test::try_call_service(&app, req).await.err();

        match result {
            Some(err) => {
                let expected_status = http::StatusCode::UNAUTHORIZED;
                let actual_status = err.as_response_error().status_code();

                assert_eq!(actual_status, expected_status);

                let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
                    .expect("Failed to deserialize JSON string");
                let expected_message = ErrorMessage::TokenNotProvided.to_string();
                assert_eq!(err_response.message, expected_message);
            }
            None => {
                panic!("Service call succeeded, but an error was expected.");
            }
        }
    }

    #[sqlx::test(migrator="crate::MIGRATOR")]
    async fn logout_with_expired_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let user_id = uuid::Uuid::new_v4();
        let expired_token =
            token::create_token(&user_id.to_string(), config.secret_key.as_bytes(), -60).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/logout")
			.cookie(CookieBuilder::new("token", expired_token).finish())
            .to_request();

        let result = test::try_call_service(&app, req).await.err();

        match result {
            Some(err) => {
                let expected_status = http::StatusCode::UNAUTHORIZED;
                let actual_status = err.as_response_error().status_code();

                assert_eq!(actual_status, expected_status);

                let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
                    .expect("Failed to deserialize JSON string");
                let expected_message = ErrorMessage::InvalidToken.to_string();
                assert_eq!(err_response.message, expected_message);
            }
            None => {
                panic!("Service call succeeded, but an error was expected.");
            }
        }
    }
}
