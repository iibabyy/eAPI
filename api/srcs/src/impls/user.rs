use actix_session::Session;
use actix_web::{http::header, web, HttpResponse};
use bcrypt::DEFAULT_COST;
use deadpool_redis::redis::AsyncCommands;
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::models::user::{LoginUserModel, NoPasswordUser, PasswordUser};

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

pub fn increase_session_counter(session: &mut Session) -> Result<(), actix_web::Error> {

	if let Some(count) = session.get::<i32>("counter")? {
		session.insert("counter", count + 1)?;
	} else {
		session.insert("counter", 1)?;
	}

	Ok(())
}

pub async fn try_to_login(
	infos: LoginUserModel,
	session: &Session,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, HttpResponse> {

	let user = match sqlx::query_as!(
		PasswordUser,
		r#"
		SELECT
			user_id,
			password
		FROM
			users
		WHERE
			email = $1
		"#,
		infos.email
	)
	.fetch_optional(db).await {
		Ok(Some(user)) => user,
		Ok(None) => return Err(HttpResponse::NotFound().body("email or password incorrect")),
		Err(err) => {
			// sleep 1 ? to counter brutforce
			return Err(HttpResponse::InternalServerError().body(err.to_string()))
		},
	};

	let valid_password = match bcrypt::verify(infos.password, &user.password) {
		Ok(is_valid) => is_valid,
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	if valid_password == false {
		return Err(HttpResponse::NotFound().body("email or password incorrect"))
	}

	let user = get_user_from_db(user.user_id, db).await?;

	insert_into_session(session, "user", &user).await?;

	Ok(user)
}

pub async fn get_user_from_db(
	id: i32,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, HttpResponse> {
	
	let user = match sqlx::query_as!(
		NoPasswordUser,
		r#"
		SELECT
			user_id,
			username,
			email,
			sold
		FROM
			users
		WHERE
			user_id = $1
		"#,
		id,
	)
	.fetch_one(db).await {
		Ok(user) => user,
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	Ok(user)
}

pub async fn get_user_from_redis(
	id: i32,
	redis: &deadpool_redis::Pool,
) -> Result<NoPasswordUser, HttpResponse> {

	let mut connection = match redis.get().await {
		Ok(conn) => conn,
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	let key = format!("user:{}", id);

	match connection.get::<_, String>(&key).await {
		Ok(json) => {
			let user: NoPasswordUser = match serde_json::from_str(&json) {
				Ok(user) => user,
				Err(err) => return Err(HttpResponse::InternalServerError().body(format!("failed to deserialize user: {}", err))),
			};

			Ok(user)
		},
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string()))
	}
}

pub async fn create_user(
	username: &String,
	email: &String,
	password: &String,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, HttpResponse> {
	let password = match bcrypt::hash(password, DEFAULT_COST) {
		Ok(hash) => hash,
		Err(err) => return Err(HttpResponse::InternalServerError().body(format!("failed to hash password: {}", err.to_string()))),
	};

	let user = match sqlx::query_as!(
		NoPasswordUser,
		r#"
		INSERT INTO users (
			username,
			email,
			password
		)
		VALUES (
			$1,
			$2,
			$3
		)
		RETURNING
			user_id, username, email, sold
		"#,
		username,
		email,
		password,
	)
	.fetch_one(db).await {
		Ok(user) => user,
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	};

	Ok(user)
}

pub async fn delete_user(
	user_id: i32,
	db: &Pool<Postgres>,
) -> Result<(), HttpResponse> {
	match sqlx::query!(
		r#"
		DELETE FROM
			users
		WHERE
			user_id = $1
		"#,
		user_id,
	)
	.execute(db).await {
		Ok(_) => Ok(()),
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	}
}

pub async fn get_all_users(
	db: &Pool<Postgres>,
) -> Result<Vec<NoPasswordUser>, HttpResponse> {
	match sqlx::query_as!(
		NoPasswordUser,
		r#"
		SELECT
			user_id,
			username,
			email,
			sold
		FROM
			users
		ORDER BY username ASC
		LIMIT 100
		"#
	)
	.fetch_all(db).await {
		Ok(users) => Ok(users),
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
	}
}

pub async fn add_sold_to_user(
	user_id: i32,
	sold_to_add: i32,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, HttpResponse> {

	if sold_to_add < 1 { return Err(HttpResponse::BadRequest().body("sold to add can not be negative neither null")) }

	let user = match sqlx::query_as!(
        NoPasswordUser,
        r#"
        UPDATE
            "users"
        SET
            sold = sold + $2
        WHERE
            user_id = $1
        RETURNING
            user_id, username, email, sold
        "#,
        user_id,
        sold_to_add,
    )
    .fetch_one(db).await {
        Ok(user) => user,
		Err(sqlx::Error::RowNotFound) => return Err(HttpResponse::NotFound().body(sqlx::Error::RowNotFound.to_string())),
		Err(err) => return Err(HttpResponse::InternalServerError().body(err.to_string())),
    };

	Ok(user)
}
