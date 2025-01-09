use actix_web::{
    get,
    web::{self, Path, Query},
    HttpResponse,
};

use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{
    user::User,
    utils::{api_response::ApiResponse, app_state::AppState},
    ActixResult,
};

#[get("/")]
async fn root() -> ApiResponse {
    ApiResponse::new(200, String::from("Hmm, who are you ?"))
}

// #[get("user/{id}")]
// async fn get_user(data: web::Data<AppState>) -> ApiResponse {
//     let client = match data.db.get().await {
//         Ok(client) => client,
//         Err(_) => return HttpResponse::InternalServerError().finish(),
//     };
//     return ApiResponse::new(200, String::from("Try to compile :/"));
// }

#[get("/add")]
async fn add_user(data: web::Data<AppState>) -> ApiResponse {
    ApiResponse::new(200, String::from("Hmm, who are you ?"))
}
