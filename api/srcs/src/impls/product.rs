use actix_web::HttpResponse;
use sqlx::{Pool, Postgres};

use crate::models::product::Product;

pub async fn get_product(
	product_id: i32,
	db: &Pool<Postgres>,
) -> Result<Product, HttpResponse> {
	match sqlx::query_as!(
		Product,
		r#"
		SELECT
			product_id,
			name,
			price,
			description,
			user_id
		FROM
			"products"
		WHERE
			product_id = $1
		"#,
		product_id,
	)
	.fetch_one(db).await {
		Ok(product) => Ok(product),
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	}
}

pub async fn create_product(
	name: &String,
	price: &i32,
	user_id: &i32,
	description: Option<&String>,
	db: &Pool<Postgres>,
) -> Result<Product, HttpResponse> {
	match sqlx::query_as!(
		Product,
		r#"
		INSERT INTO "products" (
			name,
			price,
			user_id,
			description
		)
		VALUES (
			$1,
			$2,
			$3,
			$4
		)
		RETURNING
			product_id, name, price, user_id, description
		"#,
		name,
		price,
		user_id,
		description,
	)
	.fetch_one(db).await {
		Ok(product) => Ok(product),
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	}
}

pub async fn delete_product(
	product_id: i32,
	db: &Pool<Postgres>,
) -> Result<(), HttpResponse> {
	match sqlx::query!(
		r#"
		DELETE FROM
			products
		WHERE
			product_id = $1
		"#,
		product_id,
	)
	.execute(db).await {
		Ok(_) => Ok(()),
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	}
}
