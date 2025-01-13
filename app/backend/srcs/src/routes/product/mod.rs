use actix_web::{get, post, web::{self, Json, Query}, HttpRequest, HttpResponse};
use sqlx::query_as;

use crate::utils::app_state::AppState;


/* ------------------- */
/* --- [ STRUCTS ] --- */
/* ------------------- */

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Product {
    pub id: i32,
	pub name: String,
	pub description: Option<String>,
    pub price: i32,
    pub user_id: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateProductBody {
	pub name: String,
    pub price: i32,
	pub description: Option<String>,
    pub user_id: i32,
}


/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/get_by_id")]
async fn get_by_id(
	request: HttpRequest,
	data: web::Data<AppState>,
) -> HttpResponse {

	let id = match request.query_string().parse::<i32>() {
		Ok(id) => id,
		Err(err) => return HttpResponse::BadRequest().body(format!("{err}")),
	};

	match query_as!(
		Product,
		r#"
		SELECT
			id,
			name,
			price,
			description,
			user_id
		FROM
			"products"
		WHERE
			id = $1
		"#,
		id,
	)
	.fetch_one(&data.db).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
	}
}



#[post("/create")]
async fn create(
	body: Json<CreateProductBody>,
	data: web::Data<AppState>,
) -> HttpResponse {

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
			id, name, price, user_id, description
		"#,
		body.name,
		body.price,
		body.user_id,
		body.description,
	)
	.fetch_one(&data.db).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
	}
}
