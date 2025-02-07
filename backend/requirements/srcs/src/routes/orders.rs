use actix_web::{delete, get, post, web::{self, Json}, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{database::{transaction::{DBTransaction, ITransaction}, OrderExtractor, ProductExtractor}, dtos::orders::{CreateOrderDto, FilterOrderDto, FilterOrderResponseDto, OrderDto, OrderResponseDto}, error::{ErrorMessage, HttpError}, extractors::auth::{Authenticated, RequireAuth}, models::{Order, Product, User}, utils::{status::Status, AppState}};

pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/orders")
			.service(create)
			.service(get_by_id)
			.service(delete)
			.service(validate)
		);
}


/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/{order_id}", wrap = "RequireAuth")]
async fn get_by_id(
	user: Authenticated,
	order_id: web::Path<Uuid>,
	data: web::Data<AppState>
) -> Result<HttpResponse, HttpError> {

	let order = data.db_client
		.get_order(&order_id.into_inner())
		.await
		.map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?
		.ok_or_else(|| HttpError::not_found(ErrorMessage::OrderNoLongerExist))?;

	if order.user_id != user.id {
		//	not found, to not indicate if an invalid order id belong to a real user's order or an inexistent order
		return HttpError::not_found(ErrorMessage::OrderNoLongerExist).into()
	}

	Ok(
		HttpResponse::Ok().json( OrderResponseDto {
			status: Status::Success,
			data: OrderDto::from(&order),
		})
	)
}

fn check_order(
    user: &User,
    product: &Product,
    order: &Order,
) -> Result<(), HttpError> {

    if product.user_id == user.id {
        // if user want to buy his own product
        return HttpError::bad_request(ErrorMessage::AutoBuying).into()
    } else if product.number_in_stock < order.products_number {
        return HttpError::conflict(ErrorMessage::ProductOutOfStock).into()
    }

    let total_cost = product.price_in_cents * order.products_number as i64;

    if user.sold_in_cents < total_cost {
        return HttpError::payment_required(ErrorMessage::SoldTooLow).into()
    }

    Ok(())
}

// TODO!: Add tests for this endpoint
#[post("/{order_id}/validate", wrap = "RequireAuth")]
async fn validate(
	user: Authenticated,
	order_id: web::Path<Uuid>,
	data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let mut tx = DBTransaction::begin(data.db_client.pool())
        .await
        .map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?;

    let order: Order = data.db_client
        .get_order_if_belong_to_user(&user.id, &order_id).await
        .map_err(|err| HttpError::from(err))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::OrderNoLongerExist))?;

    let product: Product = data.db_client
        .get_product(&order.product_id).await
        .map_err(|err| HttpError::from(err))?
        .ok_or_else(|| HttpError::not_found(ErrorMessage::ProductNoLongerExist))?;

    check_order(&user, &product, &order)?;

    let total_cost = product.price_in_cents * order.products_number as i64;

    tx
        .lock_user(&user.id).await
            .map_err(|err| HttpError::from(err))?
        .decrease_user_sold(&user.id, total_cost).await
            .map_err(|err| HttpError::from(err))?
        .lock_product(&product.id).await
            .map_err(|err| HttpError::from(err))?
        .decrease_product_stock(&product.id, order.products_number).await
            .map_err(|err| HttpError::from(err))?;

    tx.commit().await.map_err(|err| HttpError::from(err))?;

    Ok( HttpResponse::NoContent().finish() )
    
}

#[post("", wrap = "RequireAuth")]
async fn create(
	user: Authenticated,
	infos: web::Json<CreateOrderDto>,
	data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
	infos.validate()
		.map_err(|err| HttpError::bad_request(err.to_string()))?;

	let order = data.db_client
		.save_order(
			&user.id,
			&infos.product_id,
			infos.order_details_id.as_ref(),
			infos.products_number,
		)
		.await
		.map_err(|err| HttpError::from(err))?;

	Ok(
		HttpResponse::Ok().json( OrderResponseDto {
			status: Status::Success,
			data: OrderDto::from(&order),
		})
	)
}

#[delete("/{order_id}", wrap = "RequireAuth")]
async fn delete(
	user: Authenticated,
	order_id: web::Path<Uuid>,
	data: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {

	let order = data.db_client
		.get_order(&order_id)
		.await
		.map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?
		.ok_or_else(|| HttpError::not_found(ErrorMessage::OrderNoLongerExist))?;

	if order.user_id != user.id {
		return HttpError::not_found(ErrorMessage::OrderNoLongerExist).into()
	}

	let result = data.db_client
		.delete_order(&order_id)
		.await
		.map_err(|_| HttpError::server_error(ErrorMessage::ServerError))?;

	Ok(HttpResponse::NoContent().finish())
}

mod tests {
	use core::panic;

use actix_web::{cookie::CookieBuilder, http::{self, header::{self, HeaderName, HeaderValue}}, test, App};
	use sqlx::{Pool, Postgres};
	
	use crate::{
		database::psql::DBClient,
		dtos::orders::*,
		error::*,
		utils::{*, test_utils::*},
	};

	use super::*;


    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_order_with_valid_id(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/orders/{}", data.order_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;

        let order_response: OrderResponseDto = serde_json::from_slice(&body)
			.expect("Failed to deserialize orders response from JSON");

        assert_eq!(order_response.status, Status::Success);

		let initial_order = db_client.get_order(&data.order_id).await.unwrap().unwrap();
        assert_eq!(order_response.data, OrderDto::from(&initial_order));
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn get_order_from_other_user(pool: Pool<Postgres>) {
        let (data, data2, _) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::get()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/orders/{}", data2.order_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        let body = test::read_body(resp).await;

        let order_response: serde_json::Value = serde_json::from_slice(&body)
			.expect("Failed to deserialize orders response from JSON");

		let expected_message = ErrorMessage::OrderNoLongerExist.to_string();
		let actual_message = order_response["message"].clone();

		assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn post_order(pool: Pool<Postgres>) {
        let (data, data2, _) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::post()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri("/orders")
			.set_json( CreateOrderDto {
				product_id: data2.product_id,
				order_details_id: None,
				products_number: 1,
			})
            .to_request();

        let resp = test::call_service(&app, req).await;

        // panic!("{:#?}", test::read_body(resp).await);
        assert_eq!(resp.status(), http::StatusCode::OK);

		let body = test::read_body(resp).await;
		let response = serde_json::from_slice::<OrderResponseDto>(&body)
			.expect("Failed to deserialize Json");
		
		assert_eq!(response.status, Status::Success);
		assert_eq!(response.data.user_id, data.user_id);
		assert_eq!(response.data.product_id, data2.product_id);
		assert_eq!(response.data.order_details_id, None);
		assert_eq!(response.data.products_number, 1);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn post_order_with_invalid_product_number(pool: Pool<Postgres>) {
        let (data, _, data3) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::post()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri("/orders")
			.set_json( CreateOrderDto {
				product_id: data3.product_id,
				order_details_id: None,
				products_number: 0,
			})
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

		let body = test::read_body(resp).await;
		let response = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize Json");

		let actual_message = response["message"].clone();
		let expected_message = "products_number: Product number can only be more than 1";

		assert_eq!(actual_message, expected_message);
	}

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_valid_order(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::delete()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/orders/{}", &data.order_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

		let result = db_client.get_order(&data.order_id)
			.await
			.expect("Failed to get order by id");

		match result {
			Some(_) => panic!("User found, but no one expected"),
			None => (), // deleted user not found, Ok
		}
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_invalid_order(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::delete()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/orders/{}", Uuid::new_v4()))
            .to_request();

        let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

		let body = test::read_body(resp).await;

		let response = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize response body");

		let actual_message = response["message"].clone();
		let expected_message = ErrorMessage::OrderNoLongerExist.to_string();

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_order_from_other_user(pool: Pool<Postgres>) {
        let (data, _, data3) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data3.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::delete()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/orders/{}", Uuid::new_v4()))
            .to_request();

        let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

		let body = test::read_body(resp).await;

		let response = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize response body");

		let actual_message = response["message"].clone();
		let expected_message = ErrorMessage::OrderNoLongerExist.to_string();

        assert_eq!(actual_message, expected_message);
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn delete_same_order_twice(pool: Pool<Postgres>) {
        let (data, _, _) = init_test_orders(&pool).await;
        let db_client = DBClient::new(pool.clone());
        let config = test_config();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    db_client: db_client.clone(),
                }))
                .configure(super::config)
        )
        .await;

        let token = token::create_token(&data.user_id.to_string(), config.secret_key.as_bytes(), 60).unwrap();

        let req = test::TestRequest::delete()
            .insert_header(
                (http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
            )
            .uri(&format!("/orders/{}", &data.order_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

		let result = db_client.get_order(&data.order_id)
			.await
			.expect("Failed to get order by id");

		match result {
			Some(_) => panic!("User found, but no one expected"),
			None => (), // deleted user not found, Ok
		}

		// second request (same)

		let req = test::TestRequest::delete()
		.insert_header(
			(http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap())
		)
		.uri(&format!("/orders/{}", &data.order_id))
		.to_request();

		let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

		let body = test::read_body(resp).await;

		let response = serde_json::from_slice::<serde_json::Value>(&body)
			.expect("Failed to deserialize response body");

		let actual_message = response["message"].clone();
		let expected_message = ErrorMessage::OrderNoLongerExist.to_string();

        assert_eq!(actual_message, expected_message);
    }

}