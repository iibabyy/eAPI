use actix_web::{get, HttpResponse};


#[get("/")]
async fn root() -> HttpResponse {
	HttpResponse::Ok().body(format!(
		"Hi, this is my app !"
	))
}