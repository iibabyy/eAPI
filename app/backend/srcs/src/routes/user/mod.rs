use actix_web::{get, post, web::{self, Query}, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::utils::app_state::AppState;

/* --- --------------- */
/* --- [ STRUCTS ] --- */
/* --- --------------- */

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoPasswordUser {
    pub id: i32,
	pub username: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserBody {
	pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUserBody {
    pub email: String,
    pub password: String,
}


/* --- -------------- */
/* --- [ ROUTES ] --- */
/* --- -------------- */

#[get("/get_by_id")]
async fn user_get_by_id(
    request: HttpRequest,
    data: web::Data<AppState>
) -> HttpResponse {
    let id = match request.query_string().parse::<i32>() {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().body(format!("invalid query parameters: {err}"))
    };

    if id < 1 { return HttpResponse::BadRequest().body("invalid query parameters: expected id") }

    match sqlx::query_as!(
        NoPasswordUser,
        r#"
        SELECT
            id,
            username,
            email
        FROM
            "users"
        WHERE
            id = $1
        "#,
        id,
    )
    .fetch_optional(&data.db).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("username or password incorrect"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    }
}


#[post("/create")]
async fn user_create(
    body: web::Json<CreateUserBody>,
    data: web::Data<AppState>
) -> HttpResponse {



    match sqlx::query_as!(
        NoPasswordUser,
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
        )
        RETURNING
            id, username, email
        "#,
        body.username,
        body.email,
        body.password,
    )
    .fetch_one(&data.db)
    .await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {e}"))
    }
}
