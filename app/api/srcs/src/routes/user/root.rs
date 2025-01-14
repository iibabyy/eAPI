use actix_web::{delete, error::InternalError, get, post, web::{self, Json}, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{impls::user::{add_sold_to_user, create_user, delete_user, get_all_users, get_user}, models::user::*, utils::app_state::AppState};


/* --- -------------- */
/* --- [ ROUTES ] --- */
/* --- -------------- */

#[get("/")]
async fn get_by_id(
    request: Json<UserIdModel>,
    data: web::Data<AppState>
) -> HttpResponse {
    let user = match get_user(request.user_id, &data.db).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body("user not found".to_string()),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    HttpResponse::Ok().json(user)
}


#[post("/")]
async fn register(
    body: web::Json<CreateUserModel>, 
    data: web::Data<AppState>
) -> HttpResponse {

    let user = match create_user(&body.username, &body.email, &body.password, &data.db).await {
        Ok(user) => user,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    HttpResponse::Ok().json(user)
}

#[delete("/")]
async fn delete(
    body: web::Json<UserIdModel>, 
    data: web::Data<AppState>
) -> HttpResponse {
    match delete_user(body.user_id, &data.db).await {
        Ok(_) => HttpResponse::Ok().body(format!("user deleted.")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[get("/all")]
async fn get_all(
    data: web::Data<AppState>
) -> HttpResponse {

    match get_all_users(&data.db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    }

}

#[post("/sold")]
async fn add_sold(
    body: Json<AddSoldModel>,
    data: web::Data<AppState>
) -> HttpResponse {
    if body.sold_to_add < 1 { return HttpResponse::BadRequest().body("invalid parameters: sold: not positive") }

    match add_sold_to_user(body.user_id, body.sold_to_add, &data.db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur DB: {e}"))
    }

}
