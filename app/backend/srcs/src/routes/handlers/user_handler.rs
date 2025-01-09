use actix_web::{get, web::{self, Path, Query}, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{user::User, utils::{api_response::ApiResponse, app_state::AppState}, ActixResult};


#[get("/")]
async fn root() -> ApiResponse {
    ApiResponse::new(
        200,
        String::from("Hmm, who are you ?")
    )
}

#[get("get/{email}")]
async fn get_user(email: Path<String>, data: web::Data<AppState>) -> HttpResponse {
    match sqlx::query_as!(
        User,
        r#"SELECT id, username, email FROM "users" WHERE email = $1"#,
        email.into_inner()
    )
    .fetch_optional(&data.db)
    .await {
        Ok(Some(user)) => { HttpResponse::Ok().json(user) },
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    }
}

#[get("/add/{username}/{email}/{password}")]
async fn add_user(
    infos: web::Path<(String, String, String)>,
    data: web::Data<AppState>
) -> HttpResponse {
    let infos = infos.into_inner();
    let username = infos.0;
    let email = infos.1;
    let password = infos.2;

    match sqlx::query!(
        r#"
        INSERT INTO users (
            username,
            email,
            password
        )
        VALUES (
        $1,
        $2,
        $3
        )"#,
        username,
        email,
        password,
    )
    .execute(&data.db)
    .await {
        Ok(_) => HttpResponse::Ok().body("User added successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {e}"))
    }
}
