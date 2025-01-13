use actix_web::HttpResponse;
use sqlx::query_as;

use crate::models::order::CreateOrderBody;

type HttpResult<T> = Result<T, HttpResponse>;

trait UserData {
	fn id(&self) -> i32;
	fn email(&self) -> String;
	fn username(&self) -> String;
	fn sold(&self) -> i32;
}
