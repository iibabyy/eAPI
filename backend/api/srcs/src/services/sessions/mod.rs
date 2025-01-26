use actix_session::Session;
use actix_web::HttpResponse;
use serde::Serialize;


pub async fn insert_into_session<T: Serialize>(
	session: &Session,
	key: impl Into<String>,
	value: &T,
) -> Result<(), HttpResponse> {
	match session.insert(key, value) {
		Ok(_) => Ok(()),
		Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
	}
}

pub fn increase_session_counter(session: &Session) -> Result<(), actix_web::Error> {

	if let Some(count) = session.get::<i32>("counter")? {
		session.insert("counter", count + 1)?;
	} else {
		session.insert("counter", 1)?;
	}

	Ok(())
}
