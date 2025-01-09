use actix_web::{get, web::{self, Path, Query}, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{user::{LoginInput, User}, utils::{api_response::ApiResponse, app_state::AppState}, ActixResult};


#[get("/")]
async fn root() -> ApiResponse {
    ApiResponse::new(
        200,
        String::from("Hmm, who are you ?")
    )
}

#[get("get/")]
async fn get_user(input: Query<LoginInput>, data: web::Data<AppState>) -> HttpResponse {
    let user = match sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password
        FROM
            "users"
        WHERE
            email = $1
        "#,
        input.email,
    )
    .fetch_optional(&data.db)
    .await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().body("username or password incorrect"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    };

    match input.password == user.password {
        true => return HttpResponse::Ok().json(user),
        false => return HttpResponse::NotFound().body("username or password incorrect"),
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
