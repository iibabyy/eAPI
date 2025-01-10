/* ------------------- */
/* --- [ STRUCTS ] --- */
/* ------------------- */

use actix_web::{get, post, web::{self, Json}, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::utils::app_state::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrderBody {
    pub user_id: i32,
    pub product_id: i32,
}


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
			id,
			user_id,
			product_id
		FROM
			orders
		WHERE
			id = $1
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

	match sqlx::query_as!(
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
			id, user_id, product_id
		"#,
		body.user_id,
		body.product_id,
	)
	.fetch_one(&data.db).await {
		Ok(order) => HttpResponse::Ok().json(order),
		Err(err) => HttpResponse::InternalServerError().body(format!("{err}")),
	}

}
