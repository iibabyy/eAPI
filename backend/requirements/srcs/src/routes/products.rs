use actix_web::{delete, get, post, web::{self, Json}, HttpResponse};

use crate::{models::product::*, services::db_services, utils::app_state::AppState};

use actix_web::web;


pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/products")
			.service(root::create)
			.service(root::get)
		);
}



#[get("/{product_id}")]
async fn get(
	id: web::Path<i32>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match db_services::product::get_product(id.into_inner(), &data.db).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => err,
	}
}

#[post("/")]
async fn create(
	body: Json<CreateProductModel>,
	data: web::Data<AppState>,
) -> HttpResponse {

	match db_services::product::create_product(
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

	match db_services::product::delete_product(
			id.into_inner(),
			&data.db
		).await {
		Ok(product) => HttpResponse::Ok().json(product),
		Err(err) => err,
	}
}
