use actix_web::{delete, get, put, web::{self, Data, Path, Query}, HttpResponse};
use uuid::Uuid;
use validator::Validate;
use crate::{
    database::{OrderExtractor, ProductExtractor, UserExtractor}, dtos::{orders::{OrderDto, OrderListResponseDto}, products::{FilterProductDto, FilterProductListResponseDto, ProductDto, ProductListResponseDto}, users::*, *}, error::{ErrorMessage, HttpError}, extractors::auth::{Authenticated, RequireAuth}, utils::{status::Status, AppState}
};


pub(super) fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/users")
            .service(get_me)
			.service(get_by_id)
			.service(get_all)
			.service(delete)
			// .service(add_sold)

            .configure(orders::config)
            .configure(products::config)
		);
}


/* --- -------------- */
/* --- [ ROUTES ] --- */
/* --- -------------- */

#[get("/{user_id}", wrap = "RequireAuth")]
async fn get_by_id(
    id: web::Path<Uuid>,
    data: web::Data<AppState>
) -> Result <HttpResponse, HttpError> {
    let user = data
        .db_client
        .get_user(&id.into_inner())
        .await
        .map_err(|err| HttpError::server_error(ErrorMessage::ServerError))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::UserNoLongerExist))?;

    let filtered_user = FilterForeignUserDto::filter_user(&user);

    Ok(HttpResponse::Ok().json(
        ForeignUserResponseDto {
            status: Status::Success,
            data: filtered_user,
        }
    ))
}

#[get("/me", wrap = "RequireAuth")]
async fn get_me(
    user: Authenticated,
) -> Result<HttpResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user);

    let response_data = UserResponseDto {
        status: Status::Success,
        data: filtered_user,
    };

    Ok(HttpResponse::Ok().json(response_data))
}

#[delete("/me", wrap = "RequireAuth")]
async fn delete(
    user: Authenticated,
    data: web::Data<AppState>
) -> Result<HttpResponse, HttpError> {
    data.db_client
        .delete_user(&user.id)
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?;

    Ok(
        HttpResponse::NoContent().finish()
    )
}

#[get("", wrap = "RequireAuth")]
async fn get_all(
    data: web::Data<AppState>,
    query: Query<RequestQueryDto>
) -> Result<HttpResponse, HttpError> {

    query.validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let users: Vec<FilterForeignUserDto> = data
        .db_client
        .get_all_users(page as u32, limit)
        .await
        .map_err(|_err| HttpError::server_error(ErrorMessage::ServerError.to_string()))?
        .iter()
        .map(|user| FilterForeignUserDto::filter_user(user))
        .collect();

    Ok(HttpResponse::Ok().json(
        UserListResponseDto {
            status: Status::Success,
            results: users.len(),
            data: users,
        })
    )

}

mod products {
    use super::*;

    pub(super) fn config(config: &mut web::ServiceConfig) {
        config
            .service(get_my_products)
            .service(get_user_products);
    }

    #[get("/me/products", wrap = "RequireAuth")]
    async fn get_my_products(
        user: Authenticated,
        query: Query<RequestQueryDto>,
        data: Data<AppState>,
    ) -> Result<HttpResponse, HttpError> {
    
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
    
        let products: Vec<ProductDto> = data.db_client
            .get_products_by_user(
                &user.id,
                page as u32,
                limit
            )
            .await
            .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?
            .iter()
            .map(|product| ProductDto::from(product))
            .collect();
    
        Ok(
            HttpResponse::Ok().json( ProductListResponseDto {
                status: Status::Success,
                results: products.len(),
                data: products,
            })
        )
    }
    
    #[get("/{user_id}/products", wrap = "RequireAuth")]
    async fn get_user_products(
        user_id: Path<Uuid>,
        query: Query<RequestQueryDto>,
        data: Data<AppState>,
    ) -> Result<HttpResponse, HttpError> {
    
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
    
        let user = data.db_client
            .get_user(&user_id)
            .await
            .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?
            .ok_or_else(|| HttpError::not_found(ErrorMessage::UserNoLongerExist))?;  // check if user exists
    
        let products: Vec<FilterProductDto> = data.db_client
            .get_products_by_user(
                &user.id,
                page as u32,
                limit
            )
            .await
            .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?
            .iter()
            .map(|product| FilterProductDto::filter(product))
            .collect();
    
        Ok(
            HttpResponse::Ok().json( FilterProductListResponseDto {
                status: Status::Success,
                results: products.len(),
                data: products,
            })
        )
    }    
}

pub mod orders {
    use super::*;

    pub(super) fn config(config: &mut web::ServiceConfig) {
        config
            .service(get_my_orders);
    }

    #[get("/me/orders", wrap = "RequireAuth")]
    async fn get_my_orders(
        user: Authenticated,
        query: Query<RequestQueryDto>,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpError> {
        query.validate()
            .map_err(|err| HttpError::bad_request(err.to_string()))?;
    
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);
    
        let orders: Vec<OrderDto> = data.db_client
            .get_orders_by_user(&user.id, page as u32, limit)
            .await
            .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?
            .iter()
            .map(|order| OrderDto::from(order))
            .collect();
    
        Ok(HttpResponse::Ok().json(
            OrderListResponseDto {
                status: Status::Success,
                results: orders.len(),
                data: orders,
            })
        )
    }
}

// #[put("/{user_id}/sold", wrap = "RequireAuth")]
// async fn add_sold(
//     id: Path<i32>,
//     infos: Query<AddSoldModel>,
//     data: web::Data<AppState>
// ) -> HttpResponse {

//     match db_services::users::add_sold_to_user(id.into_inner(), infos.sold_to_add, &data.db_client).await {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(err) => err,
//     }

// }



#[cfg(test)]
mod tests {
    use actix_web::{cookie::CookieBuilder, http::{self, header::{self, HeaderName, HeaderValue}}, test, App};
    use sqlx::{Pool, Postgres};

    use crate::{
        database::psql::DBClient,
        error::{ErrorMessage, ErrorResponse},
        utils::{
            password,
            test_utils::{init_test_orders, init_test_products, init_test_users, test_config},
            token,
        },
    };

    use super::*;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_by_id_with_valid_token_and_id(pool: Pool<Postgres>) {
        let (user_id, _, _) = init_test_users(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let token =
            token::create_token(&user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let initial_user = db_client.get_user(&user_id).await.expect("Failed to get user by id").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config),
        )
        .await;

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/users/{}", user_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let user_response: ForeignUserResponseDto =
            serde_json::from_slice(&body).expect("Failed to deserialize user response from JSON");
        let responded_user = user_response.data;

        assert_eq!(responded_user, FilterForeignUserDto::filter_user(&initial_user));
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_by_id_with_invalid_id(pool: Pool<Postgres>) {
        let (user_id, _, _) = init_test_users(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let token =
            token::create_token(&user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let initial_user = db_client.get_user(&user_id).await.expect("Failed to get user by id").unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config),
        )
        .await;

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/users/{}", Uuid::new_v4()))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        let body = test::read_body(resp).await;

        let user_response: ErrorResponse =
            serde_json::from_slice(&body).expect("Failed to deserialize user response from JSON");

        let expected_message = ErrorMessage::UserNoLongerExist.to_string();
        let actual_message = user_response.message;

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_by_id_with_invalid_token(pool: Pool<Postgres>) {
        let (user_id, _, _) = init_test_users(&pool).await;
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

        let req = test::TestRequest::get()
            .insert_header(
                (HeaderName::from(http::header::AUTHORIZATION), HeaderValue::from_str("Bearer invalid-token").unwrap())
            )
            .uri(&format!("/users/{}", user_id))
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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_by_id_with_missing_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let (user_id, _, _) = init_test_users(&pool).await;
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

        let req = test::TestRequest::get()
            .uri(&format!("/users/{}", user_id))
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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_by_id_with_expired_token(pool: Pool<Postgres>) {
        let (user_id, _, _) = init_test_users(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

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

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {expired_token}")).unwrap())
            )
            .uri(&format!("/users/{}", user_id))
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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_me_with_valid_token(pool: Pool<Postgres>) {
        let (user_id, _, _) = init_test_users(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let token =
            token::create_token(&user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config),
        )
        .await;

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri("/users/me")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let user_response: UserResponseDto =
            serde_json::from_slice(&body).expect("Failed to deserialize user response from JSON");
        let user = user_response.data;

        assert_eq!(user_id.to_string(), user.id);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_me_with_invalid_token(pool: Pool<Postgres>) {
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

        let req = test::TestRequest::get()
            .insert_header(
                (HeaderName::from(http::header::AUTHORIZATION), HeaderValue::from_str("Bearer invalid-token").unwrap())
            )
            .uri("/users/me")
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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_me_with_missing_token(pool: Pool<Postgres>) {
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

        let req = test::TestRequest::get().uri("/users/me").to_request();

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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_me_with_expired_token(pool: Pool<Postgres>) {
        let (user_id, _, _) = init_test_users(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

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

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {expired_token}")).unwrap())
            )
            .uri("/users/me")
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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn all_users_with_page_one_and_limit_two_query_parameters(pool: Pool<Postgres>) {
        let (_, _, _) = init_test_users(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let hashed_password = password::hash("password123").unwrap();
        let user = db_client
            .save_user("Vivian", "vivian@example.com", &hashed_password)
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
                .configure(super::config)
        )
        .await;

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri("/users?page=1&limit=2")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let user_list_response: UserListResponseDto =
            serde_json::from_slice(&body).expect("Failed to deserialize users response from JSON");

        assert_eq!(user_list_response.data.len(), 2);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn all_users_with_valid_token(pool: Pool<Postgres>) {
        let (user_id, _, _) = init_test_users(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let token =
            token::create_token(&user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config) 
        )
        .await;

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri("/users")
            .to_request();

        let result = test::try_call_service(&app, req).await. unwrap();

        assert_eq!(result.status(), http::StatusCode::OK);

        let body = test::read_body(result).await;
        let body = serde_json::from_slice::<UserListResponseDto>(&body)
            .expect("Failed to deserialize json response");
        
        assert_eq!(body.results, 4);
        assert_eq!(body.status.to_string(), "success");
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn all_users_with_invalid_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config)
        )
        .await;

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_static("Bearer invalid-token"))
            )
            .uri("/users")
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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn all_users_with_missing_token(pool: Pool<Postgres>) {
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client,
                }))
                .configure(super::config)
        )
        .await;

        let req = test::TestRequest::get().uri("/users").to_request();

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


    #[cfg(test)]
    mod products {
        use super::*;

        #[sqlx::test(migrator = "crate::MIGRATOR")]
        async fn get_user_product_with_valid_id(pool: Pool<Postgres>) {
            let (data, _, _) = init_test_products(&pool).await;
            let db_client = DBClient::new(pool.clone());
            let config = test_config();
    
            let token = token::create_token(
                    &data.user_id.to_string(),
                    config.secret_key.as_bytes(),
                    60
                ) .unwrap();
    
            let initial_user = db_client.get_user(&data.user_id).await.expect("Failed to get user by id").unwrap();
    
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(AppState {
                        env: config.clone(),
                        db_client,
                    }))
                    .configure(super::config),
            )
            .await;
    
            let req = test::TestRequest::get()
                .insert_header(
                    (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
                )
                .uri(&format!("/users/{}/products", data.user_id))
                .to_request();
    
            let resp = test::call_service(&app, req).await;
    
            assert_eq!(resp.status(), http::StatusCode::OK);
    
            let body = test::read_body(resp).await;
    
            let response: ProductListResponseDto =
                serde_json::from_slice(&body).expect("Failed to deserialize user response from JSON");
            let products = response.data;
    
            assert_eq!(products.len(), 1);
            assert_eq!(products[0].id, data.product_id);
        }

        #[sqlx::test(migrator = "crate::MIGRATOR")]
        async fn get_user_product_with_invalid_id(pool: Pool<Postgres>) {
            let (data, _, _) = init_test_products(&pool).await;
            let db_client = DBClient::new(pool.clone());
            let config = test_config();
    
            let token = token::create_token(
                    &data.user_id.to_string(),
                    config.secret_key.as_bytes(),
                    60
                ) .unwrap();
    
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(AppState {
                        env: config.clone(),
                        db_client,
                    }))
                    .configure(super::config),
            )
            .await;
    
            let req = test::TestRequest::get()
                .insert_header(
                    (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
                )
                .uri(&format!("/users/{}/products", Uuid::new_v4()))    // invalid id
                .to_request();
    
            let resp = test::call_service(&app, req).await;
    
            assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
        }
    
        #[sqlx::test(migrator = "crate::MIGRATOR")]
        #[should_panic]
        async fn get_user_product_with_invalid_token(pool: Pool<Postgres>) {
            let db_client = DBClient::new(pool.clone());
            let config = test_config();
    
            let token = token::create_token(
                    &Uuid::new_v4().to_string(),
                    config.secret_key.as_bytes(),
                    60
                ) .unwrap();
    
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(AppState {
                        env: config.clone(),
                        db_client,
                    }))
                    .configure(super::config),
            )
            .await;
    
            let req = test::TestRequest::get()
                .insert_header(
                    (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
                )
                .uri(&format!("/users/{}/products", Uuid::new_v4()))    // invalid id
                .to_request();
    
            let resp = test::call_service(&app, req).await; // should panic
    
        }
    
        #[sqlx::test(migrator = "crate::MIGRATOR")]
        async fn get_my_product_with_valid_id(pool: Pool<Postgres>) {
            let (data, _, _) = init_test_products(&pool).await;
            let db_client = DBClient::new(pool.clone());
            let config = test_config();
    
            let token = token::create_token(
                    &data.user_id.to_string(),
                    config.secret_key.as_bytes(),
                    60
                ) .unwrap();
    
    
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(AppState {
                        env: config.clone(),
                        db_client,
                    }))
                    .configure(super::config),
            )
            .await;
    
            let req = test::TestRequest::get()
                .insert_header(
                    (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
                )
                .uri("/users/me/products")
                .to_request();
    
            let resp = test::call_service(&app, req).await;
    
            assert_eq!(resp.status(), http::StatusCode::OK);
    
            let body = test::read_body(resp).await;
    
            let response: ProductListResponseDto =
                serde_json::from_slice(&body).expect("Failed to deserialize user response from JSON");
            let products = response.data;
    
            assert_eq!(products.len(), 1);
            assert_eq!(products[0].id, data.product_id);
        }
    
    }
    
    #[cfg(test)]
    mod orders {
        use super::*;

        
        #[sqlx::test(migrator = "crate::MIGRATOR")]
        async fn get_my_orders_with_valid_token(pool: Pool<Postgres>) {
            let (data, _, data3) = init_test_orders(&pool).await;
            let db_client = DBClient::new(pool.clone());
            let config = test_config();
    
            let token = token::create_token(
                    &data.user_id.to_string(),
                    config.secret_key.as_bytes(),
                    60
                ) .unwrap();
    
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(AppState {
                        env: config.clone(),
                        db_client: db_client.clone(),
                    }))
                    .configure(super::config),
            )
            .await;
    
            db_client
                .save_order(
                    &data.user_id,
                    &data3.product_id,
                    None,
                    1
                ).await
                .map_err(|err| panic!("Failed to save order: {err}"));

            let req = test::TestRequest::get()
                .insert_header(
                    (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
                )
                .uri("/users/me/orders")
                .to_request();
    
            let resp = test::call_service(&app, req).await;
    
            assert_eq!(resp.status(), http::StatusCode::OK);
    
            let body = test::read_body(resp).await;
    
            let response: OrderListResponseDto = serde_json::from_slice(&body)
                .expect("Failed to deserialize user response from JSON");

            let orders = response.data;
    
            assert!(orders.iter().all(|order| order.user_id == data.user_id));
            assert!(orders.iter().any(|order| order.product_id == data3.product_id));
            assert_eq!(orders.len(), 2);
        }

        #[sqlx::test(migrator = "crate::MIGRATOR")]
        #[should_panic]
        async fn get_my_orders_with_invalid_id(pool: Pool<Postgres>) {
            let (data, _, data3) = init_test_orders(&pool).await;
            let db_client = DBClient::new(pool.clone());
            let config = test_config();
    
            let token = token::create_token(
                    &Uuid::new_v4().to_string(),
                    config.secret_key.as_bytes(),
                    60
                ) .unwrap();
    
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(AppState {
                        env: config.clone(),
                        db_client: db_client.clone(),
                    }))
                    .configure(super::config),
            )
            .await;
    
            let req = test::TestRequest::get()
                .insert_header(
                    (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
                )
                .uri("/users/me/orders")
                .to_request();
    
            // should panic
            let resp = test::call_service(&app, req).await;
        }

    }
}
