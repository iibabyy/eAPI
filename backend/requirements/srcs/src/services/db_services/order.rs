use actix_web::HttpResponse;
use sqlx::{self, Pool, Postgres};

use crate::models::order::{CreateOrderModel, Order};

pub async fn get_order(
	order_id: i32,
	db: &Pool<Postgres>,
) -> Result<Order, HttpResponse> {
	// let query = 
		// QueryBuilder::select(
	// 		"table",
	// 		"
	// 		order_id,
	// 		user_id,
	// 		product_id,
	// 		order_details_id
	// 		"
	// 	)
	// 	.where_("order_id = $1")
	// 	.to_query();

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
		order_id,
	)
	.fetch_optional(db).await {
		Ok(Some(order)) => Ok(order),
		Ok(None) => Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string())),
	}
}

pub async fn create_order(
	order: CreateOrderModel,
	db: &Pool<Postgres>,
) -> Result<Order, HttpResponse> {

	let order = match sqlx::query_as!(
		Order,
		r#"
		INSERT INTO orders (
			user_id,
			product_id
		)
		VALUES (
			$1,
			$2
		)
		RETURNING
			order_id, user_id, product_id, order_details_id
		"#,
		order.user_id,
		order.product_id,
	)
	.fetch_one(db).await {
		Ok(order) => order,
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	Ok(order)
}
