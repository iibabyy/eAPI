use actix_web::{get, post, web::{self, Json, Query}, HttpResponse};
use sqlx::query_as;

use crate::utils::app_state::AppState;


/* ------------------- */
/* --- [ STRUCTS ] --- */
/* ------------------- */

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Product {
    pub id: i32,
	pub name: String,
    pub price: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateProductBody {
	pub name: String,
    pub price: i32,
}


/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/get_by_id")]
async fn products_get_by_id(
	query: Query<i32>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match query_as!(
		Product,
		r#"
		SELECT
			id,
			name,
			price
		FROM
			"products"
		WHERE
			id = $1
		"#,
		query.into_inner()
	)
	.fetch_optional(&data.db).await {
		Ok(Some(product)) => HttpResponse::Ok().json(product),
		Ok(None) => HttpResponse::NotFound().body("Product not found"),
		Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
	}
}

#[post("/create")]
async fn product_create(
	body: Json<CreateProductBody>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match sqlx::query_as!(
		Product,
		r#"
		INSERT INTO "products" (
			name,
			price
		)
		VALUES (
			$1,
			$2
		)
		RETURNING
			id, name, price
		"#,
		body.name,
		body.price,
	)
	.fetch_one(&data.db).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
	}
}
