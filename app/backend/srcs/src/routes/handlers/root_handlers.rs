use actix_web::get;

use crate::utils::api_response::ApiResponse;


#[get("/")]
async fn home() -> ApiResponse {
    ApiResponse::new(200, String::from("Welcome Home !"))
}
