use actix_web::{get, post, web::{self, Json}, HttpResponse};
use crate::{impls::{order::{create_order, get_order}, order_details::{create_order_details, get_order_details}}, models::order::*, utils::app_state::AppState};

/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/{order_id}")]
async fn get_by_id(
	id: web::Path<i32>,
	data: web::Data<AppState>
) -> HttpResponse {

	let order= match get_order(id.into_inner(), &data.db).await {
		Ok(order) => order,
		Err(err) => return err,
	};

	HttpResponse::Ok().json(order)
}


#[post("/")]
async fn create(
	body: web::Json<CreateOrderModel>,
	data: web::Data<AppState>
) -> HttpResponse {

	let order= match create_order(body.into_inner(), &data.db).await {
		Ok(order) => order,
		Err(err) => return err,
	};

	HttpResponse::Ok().json(order)
}


#[get("/details/{details_id}")]
async fn get_details(
	id: web::Path<i32>,
	data: web::Data<AppState>
) -> HttpResponse {

	let order_details = match get_order_details(id.into_inner(), &data.db).await {
		Ok(order_details) => order_details,
		Err(err) => return err,
	};

	HttpResponse::Ok().json(order_details)
}


#[post("/details")]
async fn create_details(
	body: Json<CreateOrderDetailsModel>,
	data: web::Data<AppState>
) -> HttpResponse {

	let order_details = match create_order_details(body.into_inner(), &data.db).await {
		Ok(order_details) => order_details,
		Err(err) => return err,
	};

	HttpResponse::Ok().json(order_details)
}