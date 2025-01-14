use actix_web::{delete, get, post, web::{self, Json, Query}, HttpResponse};

use crate::{impls::product::{create_product, delete_product, get_product}, models::product::*, utils::app_state::AppState};


/* ------------------- */
/* --- [ STRUCTS ] --- */
/* ------------------- */


/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/")]
async fn get(
	body: Query<ProductIdModel>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match get_product(body.product_id, &data.db).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => err,
	}
}

#[post("/")]
async fn create(
	body: Json<CreateProductModel>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match create_product(
			&body.name,
			&body.price,
			&body.user_id,
			body.description.as_ref(),
			&data.db
		).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => err,
	}
}

#[delete("/")]
async fn delete(
	body: Query<ProductIdModel>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match delete_product(
			body.product_id,
			&data.db
		).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => err,
	}
}
