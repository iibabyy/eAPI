use actix_web::{web::{self, Json}, HttpResponse};
use sqlx::Error;

use crate::{models::order::{CreateOrderBody, Order}, utils::app_state::AppState, HttpResult};


pub async fn create_order(
	order: Json<CreateOrderBody>,
	data: web::Data<AppState>,
) -> Result<Order, Error> {

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
	.fetch_one(&data.db).await {
		Ok(order) => order,
		Err(err) => return Err(err),
	};

	Ok(order)
}
