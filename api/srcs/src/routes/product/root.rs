use actix_web::{delete, get, post, web::{self, Json}, HttpResponse};

use crate::{impls::product::{create_product, delete_product, get_product}, models::product::*, utils::app_state::AppState};


/* ------------------- */
/* --- [ STRUCTS ] --- */
/* ------------------- */


/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/{product_id}")]
async fn get(
	id: web::Path<i32>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match get_product(id.into_inner(), &data.db).await {
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

#[delete("/{product_id}")]
async fn delete(
	id: web::Path<i32>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match delete_product(
			id.into_inner(),
			&data.db
		).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => err,
	}
}
