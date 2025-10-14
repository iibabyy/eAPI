use crate::{
    database::ProductExtractor,
    dtos::{products::*, *},
    error::{ErrorMessage, HttpError},
    extractors::auth::{Authenticated, RequireAuth},
    utils::{status::Status, AppState},
};
use actix_web::{
    delete, get, post,
    web::{self, Json, Path, Query},
    HttpResponse,
};
use uuid::Uuid;
use validator::Validate;

pub(super) fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/products")
            .service(get_by_id)
            .service(get_all)
            .service(delete)
            .service(create),
    );
}

/* ------------ ---------- ------------ */
/* ------------ [ ROUTES ] ------------ */
/* ------------ ---------- ------------ */

#[get("/{product_id}", wrap = "RequireAuth")]
async fn get_by_id(
    id: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let product = data
        .db_client
        .get_product(&id.into_inner())
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::ProductNoLongerExist))?;

    let product = FilterProductDto::filter(&product);

    Ok(HttpResponse::Ok().json(FilterProductResponseDto {
        status: Status::Success,
        data: product,
    }))
}

#[delete("/{product_id}", wrap = "RequireAuth")]
async fn delete(
    user: Authenticated,
    product_id: Path<Uuid>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let product = data
        .db_client
        .get_product(&product_id.into_inner())
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?;

    if product.is_none() {
        return HttpError::not_found(ErrorMessage::ProductNoLongerExist).into();
    }

    let product = product.unwrap();
    if product.user_id != user.id {
        return HttpError::unauthorized(ErrorMessage::PermissionDenied).into();
    }

    data.db_client
        .delete_product(&product.id)
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("", wrap = "RequireAuth")]
async fn get_all(
    data: web::Data<AppState>,
    query: Query<RequestQueryDto>,
) -> Result<HttpResponse, HttpError> {
    query
        .validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let products: Vec<FilterProductDto> = data
        .db_client
        .get_all_products(page as u32, limit)
        .await
        .map_err(|_err| HttpError::server_error(ErrorMessage::ServerError.to_string()))?
        .iter()
        .map(FilterProductDto::filter)
        .collect();

    Ok(HttpResponse::Ok().json(FilterProductListResponseDto {
        status: Status::Success,
        results: products.len(),
        data: products,
    }))
}

#[post("", wrap = "RequireAuth")]
async fn create(
    product: Json<CreateProductDto>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    product
        .validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let product = data
        .db_client
        .save_product(
            &product.name,
            &product.user_id,
            product.description.as_ref(),
            product.price_in_cents,
            product.number_in_stock,
        )
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?;

    Ok(HttpResponse::Ok().json(ProductResponseDto {
        status: Status::Success,
        data: ProductDto::from(&product),
    }))
}

// #[put("/{product_id}/sold", wrap = "RequireAuth")]
// async fn add_sold(
//     id: Path<i32>,
//     infos: Query<AddSoldModel>,
//     data: web::Data<AppState>
// ) -> HttpResponse {

//     match db_services::products::add_sold_to_product(id.into_inner(), infos.sold_to_add, &data.db_client).await {
//         Ok(product) => HttpResponse::Ok().json(product),
//         Err(err) => err,
//     }

// }

#[cfg(test)]
mod tests {
    use actix_web::{http, test, App};
    use sqlx::{Pool, Postgres};

    use crate::{
        database::{psql::DBClient, UserModifier},
        error::ErrorMessage,
        utils::{
            test_utils::{init_test_products, test_config},
            token,
        },
    };

    use super::*;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_all_products(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let saved_product = db_client
            .save_product(
                "computer",
                &data.user_id,
                Some("A super computer".to_string()).as_ref(),
                350 * 100,
                1,
            )
            .await
            .unwrap();

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::get()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri("/products?page=1&limit=10")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let product_list_response: FilterProductListResponseDto = serde_json::from_slice(&body)
            .expect("Failed to deserialize products response from JSON");

        assert_eq!(product_list_response.results, 4);
        assert!(product_list_response
            .data
            .iter()
            .any(|product| product.id == data.product_id));
        assert!(product_list_response
            .data
            .iter()
            .any(|product| product.id == saved_product.id));
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_all_products_with_limit(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let product = db_client
            .save_product(
                "computer",
                &data.user_id,
                Some("A super computer".to_string()).as_ref(),
                350 * 100,
                1,
            )
            .await
            .unwrap();

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::get()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri("/products?page=1&limit=1") // limit to 1 result
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let product_list_response: FilterProductListResponseDto = serde_json::from_slice(&body)
            .expect("Failed to deserialize products response from JSON");

        assert_eq!(product_list_response.results, 1);
        assert_eq!(product_list_response.data[0].id, product.id);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    #[should_panic]
    async fn get_all_products_with_invalid_token(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let _ = db_client
            .save_product(
                "computer",
                &data.user_id,
                Some("A super computer".to_string()).as_ref(),
                350 * 100,
                1,
            )
            .await
            .unwrap();

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token =
            token::create_token(&Uuid::new_v4(), config.secret_key.as_bytes(), 60, &token_id)
                .unwrap();

        let req = test::TestRequest::get()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri("/products")
            .to_request();

        // should panick
        let _ = test::call_service(&app, req).await;
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_product_with_valid_id(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let product = db_client
            .save_product(
                "computer",
                &data.user_id,
                Some("A super computer".to_string()).as_ref(),
                350 * 100,
                1,
            )
            .await
            .unwrap();

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::get()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri(&format!("/products/{}", product.id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let product_response: FilterProductResponseDto = serde_json::from_slice(&body)
            .expect("Failed to deserialize products response from JSON");

        assert_eq!(product_response.status, Status::Success);
        assert_eq!(product_response.data, FilterProductDto::filter(&product));
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_product_with_invalid_id(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::get()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri(&format!("/products/{}", Uuid::new_v4()))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        let body = test::read_body(resp).await;
        let body =
            serde_json::from_slice::<serde_json::Value>(&body).expect("Failed to deserialize Json");

        let actual_message = body["message"].clone();
        let expected_message = ErrorMessage::ProductNoLongerExist.to_string();

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn post_product(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::post()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri("/products")
            .set_json(CreateProductDto {
                name: "Smartphone".to_string(),
                user_id: data.user_id,
                description: Some("A black smartphone".to_string()),
                price_in_cents: 250 * 100,
                number_in_stock: 1,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response = serde_json::from_slice::<ProductResponseDto>(&body)
            .expect("Failed to deserialize Json");

        assert_eq!(response.status, Status::Success);
        assert_eq!(response.data.user_id, data.user_id);
        assert_eq!(response.data.name, "Smartphone");
        assert_eq!(
            response.data.description,
            Some("A black smartphone".to_string())
        );
        assert_eq!(response.data.price_in_cents, 250 * 100);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn post_product_but_name_too_long(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::post()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri("/products")
            .set_json(CreateProductDto {
                name: "a".repeat(101), // max is 100 characters
                user_id: data.user_id,
                description: None,
                price_in_cents: 250 * 100,
                number_in_stock: 1,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let body = test::read_body(resp).await;

        let response = serde_json::from_slice::<serde_json::Value>(&body)
            .expect("Failed to deserialize response body");

        let actual_message = response["message"].clone();
        let expected_message = ErrorMessage::ServerError.to_string();

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn post_product_but_description_too_long(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::post()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri("/products")
            .set_json(CreateProductDto {
                name: "test".to_string(),
                user_id: data.user_id,
                description: Some("a".repeat(1001)), // max is 1000 characters
                price_in_cents: 250 * 100,
                number_in_stock: 1,
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let body = test::read_body(resp).await;

        let response = serde_json::from_slice::<serde_json::Value>(&body)
            .expect("Failed to deserialize response body");

        let actual_message = response["message"].clone();
        let expected_message = ErrorMessage::ServerError.to_string();

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_valid_product(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::delete()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri(&format!("/products/{}", &data.product_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

        let result = db_client
            .get_product(&data.product_id)
            .await
            .expect("Failed to get product by id");

        match result {
            Some(_) => panic!("User found, but no one expected"),
            None => (), // deleted user not found, Ok
        }
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_invalid_product(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::delete()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri(&format!("/products/{}", Uuid::new_v4()))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        let body = test::read_body(resp).await;

        let response = serde_json::from_slice::<serde_json::Value>(&body)
            .expect("Failed to deserialize response body");

        let actual_message = response["message"].clone();
        let expected_message = ErrorMessage::ProductNoLongerExist.to_string();

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_same_product_twice(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_products(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config),
        )
        .await;

        let token_id = Uuid::new_v4();
        db_client
            .modify_user_last_token_id(Some(&token_id), &data.user_id)
            .await
            .unwrap();

        let token = token::create_token(&data.user_id, config.secret_key.as_bytes(), 60, &token_id)
            .unwrap();

        let req = test::TestRequest::delete()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri(&format!("/products/{}", &data.product_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

        let result = db_client
            .get_product(&data.product_id)
            .await
            .expect("Failed to get product by id");

        match result {
            Some(_) => panic!("User found, but no one expected"),
            None => (), // deleted user not found, Ok
        }

        // second request (same)

        let req = test::TestRequest::delete()
            .insert_header((
                http::header::AUTHORIZATION,
                http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            ))
            .uri(&format!("/products/{}", &data.product_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        let body = test::read_body(resp).await;

        let response = serde_json::from_slice::<serde_json::Value>(&body)
            .expect("Failed to deserialize response body");

        let actual_message = response["message"].clone();
        let expected_message = ErrorMessage::ProductNoLongerExist.to_string();

        assert_eq!(actual_message, expected_message);
    }
}
