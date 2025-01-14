/* ------------------- */
/* --- [ STRUCTS ] --- */
/* ------------------- */

use actix_web::{get, post, web::{self, Json, Query}, HttpResponse};
use crate::{impls::order::{create_order, create_order_details, get_order}, models::order::*, utils::app_state::AppState};

/* ------------------ */
/* --- [ ROUTES ] --- */
/* ------------------ */

#[get("/")]
async fn get_by_id(
	infos: Query<OrderIdModel>,
	data: web::Data<AppState>
) -> HttpResponse {

	match get_order(infos.order_id, &data.db).await {
		Ok(order) => HttpResponse::Ok().json(order),
		Err(err) => err,
	}

}

#[post("/")]
async fn create(
	body: Json<CreateOrderModel>,
	data: web::Data<AppState>
) -> HttpResponse {
	match create_order(body, &data.db).await {
		Ok(order) => HttpResponse::Ok().json(order),
		Err(err) => err,
	}
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