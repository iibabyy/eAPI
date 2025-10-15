use actix_web::{
    cookie::{time::Duration, CookieBuilder},
    post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::{
        transaction::{DBTransaction, ITransaction},
        UserExtractor, UserModifier, UserUtils,
    },
    dtos::users::*,
    error::{ErrorMessage, HttpError},
    utils::{
        constants, password,
        status::Status,
        token::{self, extract_token_from},
        AppState,
    },
};

pub(super) fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(login)
            .service(register)
            .service(logout)
            .service(refresh),
    );
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginUserDto,
    responses(
        (status = 200, description = "Login successful", body = LoginResponseDto),
        (status = 400, description = "Invalid request data", body = Response),
        (status = 401, description = "Invalid credentials", body = Response)
    ),
    tag = "Authentication"
)]
#[post("/login")]
async fn login(
    infos: Json<LoginUserDto>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    infos
        .validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    // searching user
    let infos = infos.into_inner();

    let user = data
        .db_client
        .get_user_by_email(infos.email)
        .await
        .map_err(|err| HttpError::server_error(err.to_string()))?
        .ok_or_else(|| HttpError::unauthorized(ErrorMessage::WrongCredentials))?;

    // check passwords
    let password_matches = match password::compare(&infos.password, &user.password) {
        Ok(result) => result,
        Err(ErrorMessage::HashingError) => {
            return HttpError::server_error(ErrorMessage::HashingError).into()
        }
        Err(err) => return HttpError::unauthorized(err).into(),
    };

    if !password_matches {
        return HttpError::unauthorized(ErrorMessage::WrongCredentials).into();
    }

    // building response
    let token_id = Uuid::new_v4();

    let token = token::create_token(
        &user.id,
        data.env.secret_key.as_bytes(),
        data.env.jwt_max_seconds,
        &token_id,
    )
    .map_err(|_| HttpError::server_error(ErrorMessage::HashingError))?;

    let refresh_token = token::create_token(
        &user.id,
        data.env.secret_key.as_bytes(),
        60 * 10, // 10mn	// TODO: change this for prod
        &Uuid::nil(),
    )
    .map_err(|_| HttpError::server_error(ErrorMessage::HashingError))?;

    data.db_client
        .modify_user_last_token_id(Some(&token_id), &user.id)
        .await
        .map_err(HttpError::from)?;

    let filtered_user = FilterUserDto::filter_user(&user);

    let cookie = CookieBuilder::new(constants::REFRESH_TOKEN.to_string(), refresh_token)
        .path("/")
        .max_age(Duration::seconds(data.env.jwt_max_seconds))
        .http_only(true)
        // .same_site(SameSite::Strict)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(LoginResponseDto {
        status: Status::Success,
        data: filtered_user,
        token,
    }))
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterUserDto,
    responses(
        (status = 201, description = "User registered successfully", body = UserResponseDto),
        (status = 400, description = "Invalid request data", body = Response),
        (status = 409, description = "Email already exists", body = Response)
    ),
    tag = "Authentication"
)]
#[post("/register")]
async fn register(
    infos: Json<RegisterUserDto>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let infos = infos.into_inner();

    infos
        .validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let hashed_password = match password::hash(infos.password) {
        Ok(hash) => hash,
        Err(ErrorMessage::HashingError) => {
            return HttpError::server_error(ErrorMessage::HashingError).into()
        }
        Err(err) => return HttpError::server_error(err).into(),
    };

    let result = data
        .db_client
        .save_user(infos.name, infos.email, hashed_password)
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Created().json(UserResponseDto {
            status: Status::Success,
            data: FilterUserDto::filter_user(&user),
        })),

        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                HttpError::conflict(ErrorMessage::EmailExist).into()
            } else {
                HttpError::server_error(db_err.to_string()).into()
            }
        }

        Err(err) => HttpError::server_error(err.to_string()).into(),
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 200, description = "Logout successful", body = Response)
    ),
    tag = "Authentication"
)]
#[post("/logout")]
async fn logout() -> HttpResponse {
    let cookie = CookieBuilder::new(constants::REFRESH_TOKEN.clone(), "")
        .path("/")
        .max_age(Duration::seconds(0))
        .http_only(true)
        .finish();

    // TODO!: set the last_token_id of the user (db) to NULL

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": Status::Success}))
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    responses(
        (status = 200, description = "Token refreshed successfully", body = Response),
        (status = 401, description = "Invalid or expired refresh token", body = Response)
    ),
    tag = "Authentication"
)]
#[post("/refresh")]
async fn refresh(
    request: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    // find refresh-token
    let refresh_token = request
        .cookie(&constants::REFRESH_TOKEN)
        .ok_or_else(|| HttpError::unauthorized(ErrorMessage::RefreshTokenNotProvided))?;

    let refresh_token_claims =
        token::decode_token(refresh_token.value(), data.env.secret_key.as_bytes())?;
    let refresh_user_id = refresh_token_claims.sub;

    // verify deprecated token

    let deprecated_token = match extract_token_from(&request) {
        Ok(token) => token,
        Err(err) => return HttpError::unauthorized(err.message).into(),
    };

    let deprecated_claims = token::decode_token(deprecated_token, data.env.secret_key.as_bytes())?;
    let deprecated_user_id = deprecated_claims.sub;

    if deprecated_user_id != refresh_user_id {
        return HttpError::unauthorized(ErrorMessage::InvalidToken).into();
    }

    // check if it was the last active token
    let is_last_token = data
        .db_client
        .check_is_last_token(&deprecated_claims.jti, &refresh_user_id)
        .await
        .map_err(HttpError::from)?;

    if !is_last_token {
        return HttpError::unauthorized(ErrorMessage::InvalidToken).into();
    }

    // create the new token
    let new_token_id = Uuid::new_v4();

    let new_token = token::create_token(
        &refresh_user_id,
        data.env.secret_key.as_bytes(),
        data.env.jwt_max_seconds,
        &new_token_id,
    )
    .map_err(|_| HttpError::server_error(ErrorMessage::HashingError))?;

    // set the new token id as the user's last active token
    DBTransaction::begin(data.db_client.pool())
        .await
        .map_err(HttpError::from)?
        // .lock_user(&new_token_id).await
        // 	.map_err(HttpError::from)?
        .save_user_token_id(&new_token_id, &refresh_user_id)
        .await
        .map_err(HttpError::from)?
        .commit()
        .await
        .map_err(HttpError::from)?;

    Ok(HttpResponse::Ok().json(json!({
        "status": Status::Success.to_string(),
        "token": new_token,
    })))
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::{self},
        test, App,
    };
    use sqlx::{Pool, Postgres};

    use super::*;
    use crate::{
        database::psql::DBClient,
        utils::{constants::REFRESH_TOKEN, test_utils::test_config},
    };

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn register_valid_user(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool);
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config,
                    db_client,
                }))
                .service(super::register),
        )
        .await;

        let name = "Ayoub Arab".to_string();
        let email = "ayarab@gmail.com".to_string();
        let password = "password".to_string();

        let request = test::TestRequest::post()
            .uri("/register")
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

        let user = &response_data.data;

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn register_duplicate_email(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool);
        let config = test_config();

        let name = "Idrissa 1".to_string();
        let email = "ibaby@gmail.com".to_string();
        let password = "password".to_string();

        db_client.save_user(&name, &email, &password).await.unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config,
                    db_client,
                }))
                .service(super::register),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/register")
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

        let body_json =
            serde_json::from_slice::<serde_json::Value>(&body).expect("Failed to deserialize Json");

        let actual_message = body_json["message"].as_str().unwrap();

        assert_eq!(expected_message, actual_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
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
                .service(super::login),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(LoginUserDto {
                email: email.clone(),
                password: password.clone(),
            })
            .to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::OK);

        let body = test::read_body(response).await;
        let response_data =
            serde_json::from_slice::<LoginResponseDto>(&body).expect("Failed to deserialize Json");

        let user = response_data.data;

        assert_eq!(email, user.email);
        assert_eq!(name, user.name);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn login_valid_credentials_receive_cookie_and_token(pool: Pool<Postgres>) {
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
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .service(super::login),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(LoginUserDto {
                email: email.clone(),
                password: password.clone(),
            })
            .to_request();

        let response = test::call_service(&app, request).await;

        // find the refresh token and get the subject (user id)
        let refresh_token = response
            .response()
            .cookies()
            .find(|header| header.name() == REFRESH_TOKEN.to_string())
            .expect("refresh-token cookie not found");

        let refresh_token_subject =
            token::decode_token(refresh_token.value(), config.secret_key.as_bytes())
                .unwrap()
                .sub;

        // deserialize response and get the authentication token
        let authentication_token =
            serde_json::from_slice::<LoginResponseDto>(&test::read_body(response).await)
                .unwrap()
                .token;

        // get the user id from auth token
        let authentication_token_subject =
            token::decode_token(authentication_token, config.secret_key.as_bytes())
                .unwrap()
                .sub;

        assert_eq!(refresh_token_subject, authentication_token_subject);

        let token_subject = db_client
            .get_user(&authentication_token_subject)
            .await
            .expect("Failed to get user")
            .expect("User not found");

        assert_eq!(token_subject.name, name);
        assert_eq!(token_subject.email, email);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn login_with_nonexistent_user(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool);
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config,
                    db_client,
                }))
                .service(super::login),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(LoginUserDto {
                email: "nonexistent@gmail.com".to_string(),
                password: "password".to_string(),
            })
            .to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);

        let body = test::read_body(response).await;
        let body =
            serde_json::from_slice::<serde_json::Value>(&body).expect("Failed to deserialize json");

        let expected_message = "Email or password is wrong";
        let actual_mesage = body["message"].as_str().unwrap();

        assert_eq!(actual_mesage, expected_message)
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn login_with_wrong_email(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool);
        let config = test_config();

        let _ = db_client.save_user("Ayoub Arab", "ayarab@gmail.com", "password");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config,
                    db_client,
                }))
                .service(super::login),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(LoginUserDto {
                email: "nonexistent@gmail.com".to_string(),
                password: "password".to_string(),
            })
            .to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);

        let body = test::read_body(response).await;
        let body =
            serde_json::from_slice::<serde_json::Value>(&body).expect("Failed to deserialize json");

        let expected_message = "Email or password is wrong";
        let actual_mesage = body["message"].as_str().unwrap();

        assert_eq!(actual_mesage, expected_message)
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn login_with_wrong_password(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool);
        let config = test_config();

        let _ = db_client.save_user("Ayoub Arab", "ayarab@gmail.com", "password");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config,
                    db_client,
                }))
                .service(super::login),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(LoginUserDto {
                email: "ayarab@gmail.com".to_string(),
                password: "wrongpassword".to_string(),
            })
            .to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);

        let body = test::read_body(response).await;
        let body =
            serde_json::from_slice::<serde_json::Value>(&body).expect("Failed to deserialize json");

        let expected_message = "Email or password is wrong";
        let actual_mesage = body["message"].as_str().unwrap();

        assert_eq!(actual_mesage, expected_message)
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn login_with_no_data(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool);
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config,
                    db_client,
                }))
                .service(super::login),
        )
        .await;

        let request = test::TestRequest::post().uri("/login").to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

        let body = test::read_body(response).await;
        let actual_message = String::from_utf8_lossy(&body);

        let expected_message = "Content type error";

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn login_with_empty_json(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool);
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config,
                    db_client,
                }))
                .service(super::login),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(json!({}))
            .to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

        let body = test::read_body(response).await;
        let actual_message = String::from_utf8_lossy(&body);

        let expected_message = "Json deserialize error: missing field";

        assert!(actual_message.contains(expected_message));
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn logout_clear_refresh_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .service(super::logout),
        )
        .await;

        let req = test::TestRequest::post().uri("/logout").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let new_refresh_token = resp
            .response()
            .cookies()
            .find(|cookie| cookie.name().to_string() == REFRESH_TOKEN.to_string())
            .expect("Refresh cookie not cleared");

        assert!(new_refresh_token.value().is_empty());
        assert_eq!(new_refresh_token.max_age().unwrap(), Duration::seconds(0));
    }
}
