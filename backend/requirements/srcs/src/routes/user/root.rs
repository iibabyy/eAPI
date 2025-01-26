use actix_session::Session;
use actix_web::{delete, get, post, put, web::{self, Json, Path, Query}, HttpResponse};
use crate::{models::user::*, services::{self, db_services}, utils::app_state::AppState};


/* --- -------------- */
/* --- [ ROUTES ] --- */
/* --- -------------- */

#[post("/login")]
async fn login (
    infos: Json<LoginUserModel>,
    session: Session,
    data: web::Data<AppState>
) -> HttpResponse {
    let user = match services::users::login::try_to_login(infos.into_inner(), &session, &data.db).await {
        Ok(user) => user,
        Err(err) => {
            return err
        },
    };

    HttpResponse::Ok().json(user)
}

#[get("/{user_id}")]
async fn get_by_id(
    id: web::Path<i32>,
    data: web::Data<AppState>
) -> HttpResponse {
    let user = match db_services::users::get_user(id.into_inner(), &data.db).await {
        Ok(user) => user,
        Err(err) => return err,
    };

    HttpResponse::Ok().json(user)
}


#[post("/")]
async fn register(
    body: web::Json<CreateUserModel>, 
    data: web::Data<AppState>
) -> HttpResponse {

    eprintln!("received: {:?}", &body.0);

    let user = match db_services::users::create_user(&body.username, &body.email, &body.password, &data.db).await {
        Ok(user) => user,
        Err(err) => return err,
    };

    HttpResponse::Ok().json(user)
}

#[delete("/{user_id}")]
async fn delete(
    id: web::Path<i32>,
    data: web::Data<AppState>
) -> HttpResponse {
    match db_services::users::delete_user(id.into_inner(), &data.db).await {
        Ok(_) => HttpResponse::Ok().body(format!("user deleted.")),
        Err(err) => err,
    }
}


#[get("/")]
async fn get_all(
    data: web::Data<AppState>
) -> HttpResponse {

    eprintln!("received");
    match db_services::users::get_all_users(&data.db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => err,
    }

}

#[put("/{user_id}/sold")]
async fn add_sold(
    id: Path<i32>,
    infos: Query<AddSoldModel>,
    data: web::Data<AppState>
) -> HttpResponse {

    match db_services::users::add_sold_to_user(id.into_inner(), infos.sold_to_add, &data.db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err,
    }

}
