/* ------------------- */
/* --- [ STRUCTS ] --- */
/* ------------------- */

use actix_web::{get, post, web::{self, Json}, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{HttpResult, impls::order::create_order, models::order::*, utils::app_state::AppState};

/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/get_by_id")]
async fn get_by_id(
	request: HttpRequest,
	data: web::Data<AppState>
) -> HttpResponse {
	let id = match request.query_string().parse::<i32>() {
		Ok(id) => id,
		Err(err) => return HttpResponse::BadRequest().body(format!("invalid query parameter: {err}")),
	};

	if id < 1 { return HttpResponse::BadRequest().body("invalid query parameter") }

	match sqlx::query_as!(
		Order,
		r#"
		SELECT
			order_id,
			user_id,
			product_id,
			order_details_id
		FROM
			orders
		WHERE
			order_id = $1
		"#,
		id,
	)
	.fetch_one(&data.db).await {
		Ok(order) => HttpResponse::Ok().json(order),
		Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
	}

}

#[post("/create")]
async fn create(
	body: Json<CreateOrderBody>,
	data: web::Data<AppState>
) -> HttpResponse {
	match create_order(body, data).await {
		Ok(order) => HttpResponse::Ok().json(order),
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
	}
}

#[post("/create_details")]
async fn create_details(
	body: Json<CreateDetailsBody>,
	data: web::Data<AppState>
) -> HttpResponse {

	let details = match sqlx::query_as!(
		OrderDetails,
		r#"
		INSERT INTO order_details (
			delivery_address
		)
		VALUES (
			$1
		)
		RETURNING
			order_details_id, delivery_address
		"#,
		body.delivery_address,
	)
	.fetch_one(&data.db).await {
		Ok(details) => details,
		Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
	};

	match sqlx::query!(
		r#"
		UPDATE
			orders
		SET
			order_details_id = $1
		WHERE
			order_id = $2
		"#,
		details.order_details_id,
		body.order_id,
	)
	.execute(&data.db).await {
		Ok(_) => HttpResponse::Ok().json(details),
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
	}


}