use actix_web::{get, web::{Path, Query}};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{utils::api_response::ApiResponse, ActixResult};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    username: String,
    friend: String,
}

#[get("/")]
async fn root() -> ApiResponse {
    ApiResponse::new(
        200,
        String::from("Hmm, who are you ?")
    )
}


// #[get("/new")]
// async fn root() -> ApiResponse {
//     ApiResponse::new(
//         200,
//         String::from("Hmm, who are you ?")
//     )
// }

#[get("/{username}")]
async fn greet(user: Path<String>) -> ApiResponse {
    ApiResponse::new(
        200,
        format!("Hello {user} !")
    )
}

#[get("/query")]
async fn add_friend(user: Query<User>) -> ActixResult<ApiResponse> {
    Ok(ApiResponse::new(
        200,
        format!("Hi {} ! I will ask {} if he wants to be your friend !", user.username, user.friend)
    ))
}
