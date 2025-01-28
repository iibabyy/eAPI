use actix_web::{post, web::{self, Json}, HttpResponse};

use crate::{dtos::user::LoginUserDto, error::HttpError, extractors::auth::RequireAuth, utils::AppState};



pub(super) fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/auth")
			.service(login)
		);
}


#[post("/login", wrap = "RequireAuth")]
async fn login (
    infos: Json<LoginUserDto>,
    data: web::Data<AppState>
) -> Result <HttpResponse, HttpError> {

    Ok(HttpResponse::NotImplemented().finish())
}
