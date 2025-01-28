use actix_session::Session;
use actix_web::{delete, get, post, put, web::{self, Json, Path, Query}, HttpMessage, HttpRequest, HttpResponse};
use futures_util::TryFutureExt;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use crate::{database::UserExtractor, dtos::{user::{FilterForeignUserDto, FilterUserDto, ForeignUserResponseDto, RegisterUserDto, RequestQueryDto, UserData, UserListResponseDto, UserResponseDto}, Status}, error::{ErrorMessage, HttpError}, extractors::auth::Authenticated, utils::AppState};
use crate::extractors::auth::RequireAuth;


pub(super) fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/users")
			.service(get_by_id)
			.service(get_all)
			// .service(delete)
			// .service(add_sold)
		);
}


/* --- -------------- */
/* --- [ ROUTES ] --- */
/* --- -------------- */

#[get("/{user_id}", wrap = "RequireAuth")]
async fn get_by_id(
    id: web::Path<Uuid>,
    data: web::Data<AppState>
) -> Result <HttpResponse, HttpError> {
    let user = data
        .db_client
        .get_user(id.into_inner())
        .await
        .map_err(|err| HttpError::server_error(ErrorMessage::ServerError))?;

    if user.is_none() {
        return Err(HttpError::not_found(ErrorMessage::UserNoLongerExist))
    }

    let filtered_user = FilterForeignUserDto::filter_user(&user.unwrap());

    Ok(HttpResponse::Ok().json(
        ForeignUserResponseDto {
            status: Status::Success,
            data: filtered_user,
        }
    ))
}

#[get("/me", wrap = "RequireAuth")]
async fn get_me(
    user: Authenticated,
) -> Result<HttpResponse, HttpError> {
    let filtered_user = FilterUserDto::filter_user(&user);

    let response_data = UserResponseDto {
        status: Status::Success,
        data: UserData {
            user: filtered_user,
        },
    };

    Ok(HttpResponse::Ok().json(response_data))
}

// #[delete("/{user_id}", wrap = "RequireAuth")]
// async fn delete(
//     id: web::Path<i32>,
//     data: web::Data<AppState>
// ) -> HttpResponse {
//     match db_services::users::delete_user(id.into_inner(), &data.db_client).await {
//         Ok(_) => HttpResponse::Ok().body(format!("user deleted.")),
//         Err(err) => err,
//     }
// }


#[get("/", wrap = "RequireAuth")]
async fn get_all(
    data: web::Data<AppState>,
    query: Query<RequestQueryDto>
) -> Result<HttpResponse, HttpError> {

    query
        .validate()
        .map_err(|err| HttpError::bad_request(err.to_string()))?;

    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let users: Vec<FilterForeignUserDto> = data
        .db_client
        .get_all_users(page as u32, limit)
        .await
        .map_err(|err| HttpError::server_error(ErrorMessage::ServerError.to_string()))?
        .iter()
        .map(|user| FilterForeignUserDto::filter_user(user))
        .collect();

    Ok(HttpResponse::Ok().json(
        UserListResponseDto {
            status: Status::Success,
            results: users.len(),
            users,
        })
    )

}

// #[put("/{user_id}/sold", wrap = "RequireAuth")]
// async fn add_sold(
//     id: Path<i32>,
//     infos: Query<AddSoldModel>,
//     data: web::Data<AppState>
// ) -> HttpResponse {

//     match db_services::users::add_sold_to_user(id.into_inner(), infos.sold_to_add, &data.db_client).await {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(err) => err,
//     }

// }
