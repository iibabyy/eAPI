use actix_session::Session;
use actix_web::{delete, get, post, put, web::{self, Json, Path, Query}, HttpResponse};
use crate::{impls::user::{add_sold_to_user, create_user, delete_user, get_all_users, get_user_from_db, try_to_login}, models::user::*, utils::app_state::AppState};


/* --- -------------- */
/* --- [ ROUTES ] --- */
/* --- -------------- */

#[get("/")]
async fn login (
    infos: Json<LoginUserModel>,
    session: Session,
    data: web::Data<AppState>
) -> HttpResponse {
    let user = match try_to_login(infos.into_inner(), &session, &data.db).await {
        Ok(user) => user,
        Err(err) => return err,
    };

    HttpResponse::Ok().json(user)
}

#[get("/{user_id}")]
async fn get_by_id(
    id: web::Path<i32>,
    data: web::Data<AppState>
) -> HttpResponse {
    let user = match get_user_from_db(id.into_inner(), &data.db).await {
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

    let user = match create_user(&body.username, &body.email, &body.password, &data.db).await {
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
    match delete_user(id.into_inner(), &data.db).await {
        Ok(_) => HttpResponse::Ok().body(format!("user deleted.")),
        Err(err) => err,
    }
}


#[get("/")]
async fn get_all(
    data: web::Data<AppState>
) -> HttpResponse {

    match get_all_users(&data.db).await {
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

    match add_sold_to_user(id.into_inner(), infos.sold_to_add, &data.db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err,
    }

}
