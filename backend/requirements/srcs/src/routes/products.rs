// use actix_web::{get, web::{self, Query}, HttpResponse};
// use uuid::Uuid;
// use validator::Validate;
// use crate::{
//     dtos::{*, products::*},
//     database::ProductExtractor,
//     extractors::auth::{RequireAuth, Authenticated},
//     error::{ErrorMessage, HttpError},
//     utils::{status::Status, AppState}
// };


// pub(super) fn config(config: &mut web::ServiceConfig) {
// 	config
// 		.service(web::scope("/products")
//             .service(get_me)
// 			.service(get_by_id)
// 			.service(get_all)
// 			// .service(delete)
// 			// .service(add_sold)
// 		);
// }


// /* --- -------------- */
// /* --- [ ROUTES ] --- */
// /* --- -------------- */

// #[get("/{product_id}", wrap = "RequireAuth")]
// async fn get_by_id(
//     id: web::Path<Uuid>,
//     data: web::Data<AppState>
// ) -> Result <HttpResponse, HttpError> {
//     let product = data
//         .db_client
//         .get_product(id.into_inner())
//         .await
//         .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?;

//     if product.is_none() {
//         return Err(HttpError::not_found(ErrorMessage::ProductNoLongerExist))
//     }

//     let filtered_product = FilterForeignproductDto::filter_product(&product.unwrap());

//     Ok(HttpResponse::Ok().json(
//         ForeignproductResponseDto {
//             status: Status::Success,
//             data: filtered_product,
//         }
//     ))
// }

// #[get("/me", wrap = "RequireAuth")]
// async fn get_me(
//     product: Authenticated,
// ) -> Result<HttpResponse, HttpError> {
//     let filtered_product = FilterproductDto::filter_product(&product);

//     let response_data = productResponseDto {
//         status: Status::Success,
//         data: productData {
//             product: filtered_product,
//         },
//     };

//     Ok(HttpResponse::Ok().json(response_data))
// }

// // #[delete("/{product_id}", wrap = "RequireAuth")]
// // async fn delete(
// //     id: web::Path<i32>,
// //     data: web::Data<AppState>
// // ) -> HttpResponse {
// //     match db_services::products::delete_product(id.into_inner(), &data.db_client).await {
// //         Ok(_) => HttpResponse::Ok().body(format!("product deleted.")),
// //         Err(err) => err,
// //     }
// // }


// #[get("/", wrap = "RequireAuth")]
// async fn get_all(
//     data: web::Data<AppState>,
//     query: Query<RequestQueryDto>
// ) -> Result<HttpResponse, HttpError> {

//     query.validate()
//         .map_err(|err| HttpError::bad_request(err.to_string()))?;

//     let page = query.page.unwrap_or(1);
//     let limit = query.limit.unwrap_or(10);

//     let products: Vec<FilterForeignproductDto> = data
//         .db_client
//         .get_all_products(page as u32, limit)
//         .await
//         .map_err(|_err| HttpError::server_error(ErrorMessage::ServerError.to_string()))?
//         .iter()
//         .map(|product| FilterForeignproductDto::filter_product(product))
//         .collect();

//     Ok(HttpResponse::Ok().json(
//         productListResponseDto {
//             status: Status::Success,
//             results: products.len(),
//             products,
//         })
//     )

// }

// // #[put("/{product_id}/sold", wrap = "RequireAuth")]
// // async fn add_sold(
// //     id: Path<i32>,
// //     infos: Query<AddSoldModel>,
// //     data: web::Data<AppState>
// // ) -> HttpResponse {

// //     match db_services::products::add_sold_to_product(id.into_inner(), infos.sold_to_add, &data.db_client).await {
// //         Ok(product) => HttpResponse::Ok().json(product),
// //         Err(err) => err,
// //     }

// // }



// #[cfg(test)]
// mod tests {
//     use actix_web::{cookie::CookieBuilder, http::{self, header::{self, HeaderName, HeaderValue}}, test, App};
//     use sqlx::{Pool, Postgres};

//     use crate::{
//         database::db::DBClient,
//         error::{ErrorMessage, ErrorResponse},
//         utils::{
//             password,
//             test_utils::{test_config, init_test_products},
//             token,
//         },
//     };

//     use super::*;

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn get_me_with_valid_token(pool: Pool<Postgres>) {
//         let (product_id, _, _) = init_test_products(&pool).await;
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let token =
//             token::create_token(&product_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config),
//         )
//         .await;

//         let req = test::TestRequest::get()
//             .insert_header(
//                 (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
//             )
//             .uri("/products/me")
//             .to_request();

//         let resp = test::call_service(&app, req).await;

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let body = test::read_body(resp).await;

//         let product_response: productResponseDto =
//             serde_json::from_slice(&body).expect("Failed to deserialize product response from JSON");
//         let product = product_response.data.product;

//         assert_eq!(product_id.to_string(), product.id);
//     }

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn get_me_with_invalid_token(pool: Pool<Postgres>) {
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config),
//         )
//         .await;

//         let req = test::TestRequest::get()
//             .insert_header(
//                 (HeaderName::from(http::header::AUTHORIZATION), HeaderValue::from_str("Bearer invalid-token").unwrap())
//             )
//             .uri("/products/me")
//             .to_request();

//         let result = test::try_call_service(&app, req).await.err();

//         match result {
//             Some(err) => {
//                 let expected_status = http::StatusCode::UNAUTHORIZED;
//                 let actual_status = err.as_response_error().status_code();

//                 assert_eq!(actual_status, expected_status);

//                 let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
//                     .expect("Failed to deserialize JSON string");
//                 let expected_message = ErrorMessage::InvalidToken.to_string();
//                 assert_eq!(err_response.message, expected_message);
//             }
//             None => {
//                 panic!("Service call succeeded, but an error was expected.");
//             }
//         }
//     }

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn get_me_with_missing_token(pool: Pool<Postgres>) {
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config),
//         )
//         .await;

//         let req = test::TestRequest::get().uri("/products/me").to_request();

//         let result = test::try_call_service(&app, req).await.err();

//         match result {
//             Some(err) => {
//                 let expected_status = http::StatusCode::UNAUTHORIZED;
//                 let actual_status = err.as_response_error().status_code();

//                 assert_eq!(actual_status, expected_status);

//                 let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
//                     .expect("Failed to deserialize JSON string");
//                 let expected_message = ErrorMessage::TokenNotProvided.to_string();
//                 assert_eq!(err_response.message, expected_message);
//             }
//             None => {
//                 panic!("Service call succeeded, but an error was expected.");
//             }
//         }
//     }

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn get_me_with_expired_token(pool: Pool<Postgres>) {
//         let (product_id, _, _) = init_test_products(&pool).await;
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let expired_token =
//             token::create_token(&product_id.to_string(), config.secret_key.as_bytes(), -60).unwrap();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config),
//         )
//         .await;

//         let req = test::TestRequest::get()
//             .insert_header(
//                 (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {expired_token}")).unwrap())
//             )
//             .uri("/products/me")
//             .to_request();

//         let result = test::try_call_service(&app, req).await.err();

//         match result {
//             Some(err) => {
//                 let expected_status = http::StatusCode::UNAUTHORIZED;
//                 let actual_status = err.as_response_error().status_code();

//                 assert_eq!(actual_status, expected_status);

//                 let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
//                     .expect("Failed to deserialize JSON string");
//                 let expected_message = ErrorMessage::InvalidToken.to_string();
//                 assert_eq!(err_response.message, expected_message);
//             }
//             None => {
//                 panic!("Service call succeeded, but an error was expected.");
//             }
//         }
//     }

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn all_products_with_page_one_and_limit_two_query_parameters(pool: Pool<Postgres>) {
//         let (_, _, _) = init_test_products(&pool).await;
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let hashed_password = password::hash("password123").unwrap();
//         let product = db_client
//             .save_product("Vivian", "vivian@example.com", &hashed_password)
//             .await
//             .unwrap();

//         let token =
//             token::create_token(&product.id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config)
//         )
//         .await;

//         let req = test::TestRequest::get()
//             .insert_header(
//                 (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
//             )
//             .uri("/products/?page=1&limit=2")
//             .to_request();

//         let resp = test::call_service(&app, req).await;

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let body = test::read_body(resp).await;

//         let product_list_response: productListResponseDto =
//             serde_json::from_slice(&body).expect("Failed to deserialize products response from JSON");

//         assert_eq!(product_list_response.products.len(), 2);
//     }

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn all_products_with_valid_token(pool: Pool<Postgres>) {
//         let (product_id, _, _) = init_test_products(&pool).await;
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let token =
//             token::create_token(&product_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config) 
//         )
//         .await;

//         let req = test::TestRequest::get()
//             .insert_header(
//                 (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
//             )
//             .uri("/products/")
//             .to_request();

//         let result = test::try_call_service(&app, req).await. unwrap();

//         assert_eq!(result.status(), http::StatusCode::OK);

//         let body = test::read_body(result).await;
//         let body = serde_json::from_slice::<productListResponseDto>(&body)
//             .expect("Failed to deserialize json response");
        
//         assert_eq!(body.results, 4);
//         assert_eq!(body.status.to_string(), "success");
//     }

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn all_products_with_invalid_token(pool: Pool<Postgres>) {
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config)
//         )
//         .await;

//         let req = test::TestRequest::get()
//             .insert_header(
//                 (http::header::AUTHORIZATION, http::header::HeaderValue::from_static("Bearer invalid-token"))
//             )
//             .uri("/products/")
//             .to_request();

//         let result = test::try_call_service(&app, req).await.err();

//         match result {
//             Some(err) => {
//                 let expected_status = http::StatusCode::UNAUTHORIZED;
//                 let actual_status = err.as_response_error().status_code();

//                 assert_eq!(actual_status, expected_status);

//                 let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
//                     .expect("Failed to deserialize JSON string");
//                 let expected_message = ErrorMessage::InvalidToken.to_string();
//                 assert_eq!(err_response.message, expected_message);
//             }
//             None => {
//                 panic!("Service call succeeded, but an error was expected.");
//             }
//         }
//     }

//     #[sqlx::test(migrator = "crate::MIGRATOR")]
//     async fn all_products_with_missing_token(pool: Pool<Postgres>) {
//         let db_client = DBClient::new(pool.clone());
//         let config = test_config();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(AppState {
//                     env: config.clone(),
//                     db_client,
//                 }))
//                 .configure(super::config)
//         )
//         .await;

//         let req = test::TestRequest::get().uri("/products/").to_request();

//         let result = test::try_call_service(&app, req).await.err();

//         match result {
//             Some(err) => {
//                 let expected_status = http::StatusCode::UNAUTHORIZED;
//                 let actual_status = err.as_response_error().status_code();

//                 assert_eq!(actual_status, expected_status);

//                 let err_response: ErrorResponse = serde_json::from_str(&err.to_string())
//                     .expect("Failed to deserialize JSON string");
//                 let expected_message = ErrorMessage::TokenNotProvided.to_string();
//                 assert_eq!(err_response.message, expected_message);
//             }
//             None => {
//                 panic!("Service call succeeded, but an error was expected.");
//             }
//         }
//     }

// }
