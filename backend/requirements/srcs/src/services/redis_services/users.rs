use actix_web::HttpResponse;
use deadpool_redis::redis::AsyncCommands;

use crate::models::User;


pub async fn get_user_from_redis(
	id: i32,
	redis: &deadpool_redis::Pool,
) -> Result<Option<NoPasswordUser>, HttpResponse> {

	let mut connection: deadpool_redis::Connection = match redis.get().await {
		Ok(conn) => conn,
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	let key = format!("user:{}", id);

	match connection.get::<_, Option<String>>(&key).await {
		Ok(Some(json)) => {
			let user: NoPasswordUser = match serde_json::from_str(&json) {
				Ok(user) => user,
				Err(err) => return Err(HttpResponse::InternalServerError().body(format!("failed to deserialize user: {}", err))),
			};

			Ok(Some(user))
		},
		Ok(None) => return Ok(None),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string()))
	}
}
