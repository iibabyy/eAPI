use std::{ops::Deref, rc::Rc, task::{Context, Poll}};

use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, error::{ErrorInternalServerError, ErrorUnauthorized}, http, web, FromRequest, HttpMessage, HttpRequest};
use futures_util::{future::{ready, LocalBoxFuture, Ready}, FutureExt};
use uuid::Uuid;

use crate::{database::UserExtractor, error::{ErrorMessage, ErrorResponse, HttpError}, models::User, utils::{self, token::extract_token_from, AppState}};

// LocalBoxFuture<'static, Result<ServiceResponse<actix_web::body::BoxBody>, actix_web::Error>>

pub struct Authenticated(User);

impl FromRequest for Authenticated {
	type Error = actix_web::Error;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(
		req: &actix_web::HttpRequest,
		_payload: &mut actix_web::dev::Payload
	) -> Self::Future {
		let value = req.extensions().get::<User>().cloned();

		let result = match value {
			Some(user) => Ok(Authenticated(user)),
			None => Err(ErrorInternalServerError(ErrorResponse {
				status: "fail".to_string(),
				message: ErrorMessage::UserNotFound.to_string(),
			}))
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

pub struct  AuthMiddleware<S> {
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
		let token = match extract_token_from(req.request()) {
			Ok(token) => token,
			Err(err) => return Box::pin(ready(Err(ErrorUnauthorized(err)))),
		};

		let app_state = req.app_data::<web::Data<AppState>>().unwrap();

		let jwt_claims = match utils::token::decode_token(
			&token,
			app_state.env.secret_key.as_bytes()
		) {
			Ok(claims) => claims,
			Err(e) => {
				return Box::pin(ready(Err(ErrorUnauthorized(ErrorResponse {
					status: "fail".to_string(),
					message: e.message,
				}))))
			}
		};

		let user_id = jwt_claims.sub;
		let jwt_id = jwt_claims.jti;

		let cloned_app_state = app_state.clone();
		let cloned_service = Rc::clone(&self.service);

		async move {
			let user = cloned_app_state
				.db_client
				.get_user(&user_id)
				.await
				.map_err(|e| ErrorInternalServerError(HttpError::server_error(e.to_string())))?
				.ok_or_else(|| ErrorUnauthorized(ErrorResponse {
					status: "fail".to_string(),
					message: ErrorMessage::UserNotFound.to_string(),
				}))?;

			// check if it was the last active token
			if user.last_token_id.is_none() {
				return Err(
					ErrorUnauthorized(ErrorResponse {
						status: "fail".to_string(),
						message: ErrorMessage::InvalidToken.to_string(),
					})
				)
			}

			let last_token_id = user.last_token_id.as_ref().unwrap();

			let is_last_token = bcrypt::verify(jwt_id, last_token_id)
				.map_err(|_| ErrorInternalServerError(ErrorResponse {
					status: "fail".to_string(),
					message: ErrorMessage::ServerError.to_string(),
				}))?;

			if is_last_token == false {
				return Err(
					ErrorUnauthorized(ErrorResponse {
						status: "fail".to_string(),
						message: ErrorMessage::InvalidToken.to_string(),
					})
				)
			}

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
	use actix_web::{cookie::Cookie, get, http::{self, header::{self, HeaderName, HeaderValue}}, test, web::Header, App, HttpResponse};
	use sqlx::{Pool, Postgres};

	use crate::{database::psql::DBClient, utils::{password, test_utils::{self, init_test_users}, token}};

	use super::*;

	#[get("/", wrap = "RequireAuth")]
	async fn handler_with_requireauth() -> HttpResponse {
		HttpResponse::Ok().finish()
	}


	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn auth_middelware_valid_token(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
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

		let token = token::create_token(&user.id, config.secret_key.as_bytes(), 60, &Uuid::new_v4()).unwrap();
		
		let request = test::TestRequest::default()
			.insert_header(
				(HeaderName::from(http::header::AUTHORIZATION), HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
			)
			.to_request();

		let response = test::call_service(&app, request).await;

		assert_eq!(response.status(), http::StatusCode::OK);
	}


	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn auth_middelware_missing_token(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);
        let config = test_utils::test_config();
		// let redis_pool = deadpool_redis::Config::from_url(&config.redis_url).create_pool(Some(Runtime::Tokio1)).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    db_client,
                    env: config.clone(),
					// redis: redis_pool,
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

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn auth_middelware_invalid_token(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);
		let config = test_utils::test_config();
		// let redis_pool = deadpool_redis::Config::from_url(&config.redis_url).create_pool(Some(Runtime::Tokio1)).unwrap();


		let app = test::init_service(
			App::new()
				.app_data(web::Data::new(AppState {
					db_client,
					env: config,
					// redis: redis_pool,
				}))
				.service(handler_with_requireauth),
		)
		.await;


		let request = test::TestRequest::default()
			.uri("/")
			.insert_header(
				(HeaderName::from(http::header::AUTHORIZATION), HeaderValue::from_static("Bearer invalid-token"))
			)
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
