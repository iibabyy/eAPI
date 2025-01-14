use actix_web::web::{self, Json};
use sqlx::{self, Pool, Postgres};

use crate::{models::order::{CreateOrderDetailsModel, CreateOrderModel, Order, OrderDetails, OrderDetailsIdModel}, utils::app_state::AppState};

pub async fn get_order(
	order_id: i32,
	db: &Pool<Postgres>,
) -> Result<Order, sqlx::Error> {
	sqlx::query_as!(
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
	.fetch_one(db).await
}

pub async fn create_order(
	order: Json<CreateOrderModel>,
	db: &Pool<Postgres>,
) -> Result<Order, sqlx::Error> {

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
		Err(err) => return Err(err),
	};

	Ok(order)
}

async fn new_order_details(
	infos: CreateOrderDetailsModel,
	db: &Pool<Postgres>,
) -> Result<OrderDetails, sqlx::Error> {
	sqlx::query_as!(
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
	.fetch_one(db).await
}

pub async fn create_order_details(
	infos: CreateOrderDetailsModel,
	db: &Pool<Postgres>,
) -> Result<OrderDetails, sqlx::Error> {
	let order = get_order(infos.order_id, db).await?;
	
	let order_details = 
	if order.order_details_id.is_some() {

		// modify details infos if one exists
		sqlx::query_as!(
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
		.fetch_one(db).await?
	} else {

		// create order_details in database
		let details = sqlx::query_as!(
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
		.fetch_one(db).await?;

		// assign it to the order
		sqlx::query!(
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
		.execute(db).await?;

		details
	};

	Ok(order_details)

}