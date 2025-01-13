use actix_web::{get, post, web::{self, Json}, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{utils::app_state::AppState};


/* ------------------ */
/* --- [ MODELS ] --- */
/* ------------------ */

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoPasswordUser {
    pub id: i32,
	pub username: String,
    pub email: String,
    pub sold: i32,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct AddSoldBody {
    pub id: i32,
    pub sold: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserOrdersData {
    pub id: i32,
    pub product_id: i32,
}

/* --- -------------- */
/* --- [ ROUTES ] --- */
/* --- -------------- */

#[get("/get_by_id")]
async fn get_by_id(
    request: HttpRequest,
    data: web::Data<AppState>
) -> HttpResponse {
    let id = match request.query_string().parse::<i32>() {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().body(format!("invalid query parameters: {err}"))
    };

    if id < 1 { return HttpResponse::BadRequest().body("invalid query parameters") }

    match sqlx::query_as!(
        NoPasswordUser,
        r#"
        SELECT
            id,
            username,
            email,
            sold
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
async fn create(
    body: web::Json<CreateUserBody>, 
    data: web::Data<AppState>
) -> HttpResponse {

    match sqlx::query_as!(
        NoPasswordUser,
        r#"
        INSERT INTO users (
            username,
            email,
            password,
            sold
        )
        VALUES (
            $1,
            $2,
            $3,
            0
        )
        RETURNING
            id, username, email, sold
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

#[post("/delete")]
async fn delete(
    body: web::Json<i32>, 
    data: web::Data<AppState>
) -> HttpResponse {

    match sqlx::query_as!(
        NoPasswordUser,
        r#"
        DELETE FROM
            users
        WHERE
            id = $1
        RETURNING
            id, username, email, sold
        "#,
        body.into_inner(),
    )
    .fetch_one(&data.db)
    .await {
        Ok(user) => HttpResponse::Ok().body(format!("user {} deleted.", user.username)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {e}"))
    }

}


#[get("/get_all")]
async fn get_all(
    data: web::Data<AppState>
) -> HttpResponse {

    match sqlx::query_as!(
        NoPasswordUser,
        r#"
        SELECT
            id,
            username,
            email,
            sold
        FROM
            "users"
        ORDER BY username DESC
        "#,
    )
    .fetch_all(&data.db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    }

}

#[post("/add_sold")]
async fn add_sold(
    body: Json<AddSoldBody>,
    data: web::Data<AppState>
) -> HttpResponse {
    let id = body.id;
    let sold = body.sold;

    if id < 1 { return HttpResponse::BadRequest().body("invalid parameters: id") }
    if sold < 1 { return HttpResponse::BadRequest().body("invalid parameters: sold: not positive") }

    match sqlx::query_as!(
        NoPasswordUser,
        r#"
        UPDATE
            "users"
        SET
            sold = sold + $2
        WHERE
            id = $1
        RETURNING
            id, username, email, sold
        "#,
        id,
        sold,
    )
    .fetch_one(&data.db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    }

}

#[get("/get_order_by_id")]
async fn get_order_by_id(
    request: HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {

    let id = match request.query_string().parse::<i32>() {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().body(format!("{err}")),
    };

    match sqlx::query_as!(
        UserOrdersData,
        r#"
        SELECT
            id,
            product_id
        FROM
            orders
        WHERE
            user_id = $1
        "#,
        id,
    )
    .fetch_all(&data.db).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    }

}
