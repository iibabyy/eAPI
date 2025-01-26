use actix_web::HttpResponse;
use sqlx::{Pool, Postgres};

use crate::models::order::*;

use super::order;


pub async fn get_order_details(
	id: i32,
	db: &Pool<Postgres>,
) -> Result<OrderDetails, HttpResponse> {
	match sqlx::query_as!(
		OrderDetails,
		r#"
		SELECT
			order_details_id,
			delivery_address
		FROM
			order_details
		WHERE
			order_details_id = $1
		"#,
		id,
	)
	.fetch_optional(db).await {
		Ok(Some(order)) => Ok(order),
		Ok(None) => Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
	}
}

pub async fn create_order_details(
	infos: CreateOrderDetailsModel,
	db: &Pool<Postgres>,
) -> Result<OrderDetails, HttpResponse> {
	let order = order::get_order(infos.order_id, db).await?;
	
	let order_details = 
	if order.order_details_id.is_some() {

		// modify details infos if one exists
		match sqlx::query_as!(
			OrderDetails,
			r#"
			UPDATE
				order_details
			SET
				delivery_address = $1
			WHERE
				order_details_id = $2
			RETURNING
				order_details_id, delivery_address
			"#,
			infos.delivery_address,
			order.order_details_id.unwrap(),
		)
		.fetch_one(db).await {
			Ok(details) => details,
			Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
		}
	} else {

		// create order_details in database
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
			infos.delivery_address,
		)
		.fetch_one(db).await {
			Ok(details) => details,
			Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
		};

		// assign it to the order
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
			order.order_id,
		)
		.execute(db).await {
			Ok(_) => (),
			Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
		};

		details
	};

	Ok(order_details)

}