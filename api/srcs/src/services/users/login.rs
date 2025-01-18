use actix_session::Session;
use actix_web::HttpResponse;
use sqlx::Postgres;

use crate::{models::user::*, services::{self, db_services}};


pub async fn try_to_login(
	infos: LoginUserModel,
	session: &Session,
	db: &sqlx::Pool<Postgres>,
) -> Result<NoPasswordUser, HttpResponse> {

	match services::sessions::increase_session_counter(session) {
		Ok(()) => (),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	let user = services::db_services::users::get_password_user(infos.email, db).await?;

	let valid_password = match bcrypt::verify(infos.password, &user.password) {
		Ok(is_valid) => is_valid,
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	if valid_password == false {
		return Err(HttpResponse::NotFound().body("email or password incorrect"))
	}

	let user = db_services::users::get_user(user.user_id, db).await?;

	services::sessions::insert_into_session(session, "user", &user).await?;

	Ok(user)
}
