use std::{ops::Deref, rc::Rc, task::{Context, Poll}};

use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, error::{ErrorInternalServerError, ErrorUnauthorized}, web, FromRequest, HttpMessage};
use futures_util::{future::{ready, LocalBoxFuture, Ready}, FutureExt};
use uuid::Uuid;

use crate::{database::UserExtractor, error::{ErrorMessage, ErrorResponse, HttpError}, models::User, utils::{self, AppState}};

pub struct Authenticated(User);

impl FromRequest for Authenticated {
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(
		req: &actix_web::HttpRequest,
		payload: &mut actix_web::dev::Payload
	) -> Self::Future {
		let value = req.extensions().get::<User>().cloned();

		let result = match value {
			Some(user) => Ok(Authenticated(user)),
			None => Err(ErrorInternalServerError(HttpError::server_error(
				"Authentication Error"
			)))
		};

		ready(result)
	}
}

impl Deref for Authenticated {
	type Target = User;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub struct AuthMiddleware<S> {
	service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
	S: Service<
			ServiceRequest,
			Response = ServiceResponse<actix_web::body::BoxBody>,
			Error = actix_web::Error,
		> + 'static,
{
	type Response = ServiceResponse<actix_web::body::BoxBody>;
	type Error = actix_web::Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

	fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(ctx)
	}

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let token = req
			.cookie("token")
			.map(|c| c.value().to_string());

		if token.is_none() {
			let json_error = ErrorResponse {
				status: "fail".to_string(),
				message: ErrorMessage::TokenNotProvided.to_string(),
			};

			return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
		}
	
		let app_state = req.app_data::<web::Data<AppState>>().unwrap();

		let user_id = match utils::token::decode_token(
			&token.unwrap(), 
			app_state.env.secret_key.as_bytes()
		) {
			Ok(id) => id,
			Err(e) => {
				return Box::pin(ready(Err(ErrorUnauthorized(ErrorResponse {
					status: "fail".to_string(),
					message: e.message,
				}))))
			}
		};

		let cloned_app_state = app_state.clone();
		let cloned_service = Rc::clone(&self.service);

		async move {
			let user_id = Uuid::parse_str(&user_id).unwrap();
			let result = cloned_app_state
				.db_client
				.get_user(user_id)
				.await
				.map_err(|e| ErrorInternalServerError(HttpError::server_error(e.to_string())))?;

			let user = match result {
				Some(user) => user,
				None => return Err(ErrorUnauthorized(ErrorResponse {
					status: "fail".to_string(),
					message: ErrorMessage::UserNoLongerExist.to_string(),
				}))
			};

			// store user information for next middlewares/endpoint handlers
			req.extensions_mut().insert::<User>(user);

			let res = cloned_service.call(req).await?;
			Ok(res)
		}
		.boxed_local()
	}
}


pub struct RequireAuth;

impl<S> Transform<S, ServiceRequest> for RequireAuth
where
	S: Service<
			ServiceRequest,
			Response = ServiceResponse<actix_web::body::BoxBody>,
			Error = actix_web::Error,
		> + 'static,
{
	type Response = ServiceResponse<actix_web::body::BoxBody>;
	type Error = actix_web::Error;
	type Transform = AuthMiddleware<S>;
	type InitError = ();
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(AuthMiddleware {
			service: Rc::new(service),
		}))
	}
}

#[cfg(test)]
mod tests {
	use actix_web::{cookie::Cookie, dev::{AppConfig, ServiceFactory}, get, http, test, App, HttpResponse};
	use deadpool_redis::Runtime;
use sqlx::{Pool, Postgres};

	use crate::{database::db::DBClient, utils::{self, password, test_utils, token}};

	use super::*;

	#[get("/", wrap = "RequireAuth")]
	async fn handler_with_requireauth() -> HttpResponse {
		HttpResponse::Ok().into()
	}


	#[sqlx::test]
	async fn auth_middelware_valid_token(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
        let config = test_utils::test_config();
		// let redis_pool = deadpool_redis::Config::from_url(&config.redis_url).create_pool(Some(Runtime::Tokio1)).unwrap();
	
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    db_client: db_client.clone(),
                    env: config.clone(),
					// redis: redis_pool,
                }))
                .service(handler_with_requireauth),
        )
        .await;

		let hashed_password = password::hash("password123").unwrap();
		let user = db_client
			.save_user("John", "john@example.com", &hashed_password)
			.await
			.unwrap();

		let token = token::create_token(&user.id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

		let request = test::TestRequest::default()
			.cookie(Cookie::new("token", token))
			.to_request();

		let response = test::call_service(&app, request).await;

		assert_eq!(response.status(), http::StatusCode::OK);
	}


	#[sqlx::test]
	async fn auth_middelware_missing_token(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
        let config = test_utils::test_config();
		let redis_pool = deadpool_redis::Config::from_url(&config.redis_url).create_pool(Some(Runtime::Tokio1)).unwrap();
	
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    db_client,
                    env: config.clone(),
					redis: redis_pool,
                }))
                .service(handler_with_requireauth),
        )
        .await;

		let request = test::TestRequest::default().to_request();
		let result = test::try_call_service(&app, request).await.err();

		match result {
			Some(err) => {
				let expected_status = http::StatusCode::UNAUTHORIZED;
				let actual_status = err.as_response_error().status_code();

				assert_eq!(actual_status, expected_status);

				let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
					.expect("failed to deserialize json string");
				let expected_message = ErrorMessage::TokenNotProvided.to_string();

				assert_eq!(err_response.message, expected_message);
			},
			None => {
				panic!("Service call succeeded, but an error was expected");
			}
		}
	}

	async fn auth_middelware_invalid_token(pool: Pool<Postgres>) {
		let db_client = DBClient::new(pool);
		let config = test_utils::test_config();
		let redis_pool = deadpool_redis::Config::from_url(&config.redis_url).create_pool(Some(Runtime::Tokio1)).unwrap();

		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					db_client,
					env: config,
					redis: redis_pool,
				}))
				.service(handler_with_requireauth),
		)
		.await;


		let token = "invalid-token".to_string();

		let request = test::TestRequest::default()
			.cookie(Cookie::new("token", token))
			.to_request();

		let result = test::try_call_service(&app, request).await.err();
	
		match result {
			Some(err) => {
				let expected_status = http::StatusCode::UNAUTHORIZED;
				let actual_status = err.as_response_error().status_code();

				assert_eq!(actual_status, expected_status);

				let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
					.expect("failed to deserialize json string");
				let expected_message = ErrorMessage::InvalidToken.to_string();

				assert_eq!(err_response.message, expected_message);
			},
			None => {
				panic!("Service call succeeded, but an error was expected");
			}
		}

	}

}
