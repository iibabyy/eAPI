use actix_web::HttpResponse;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::{Pool, Postgres};

use crate::models::user::NoPasswordUser;

pub async fn get_user(
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
		Err(err) => return Err(HttpResponse::from_error(err))
	};

	Ok(user)
}

pub async fn create_user(
	username: &String,
	email: &String,
	password: &String,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, HttpResponse> {
	let password = match hash(password, DEFAULT_COST) {
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
		Err(err) => return Err(HttpResponse::from_error(err)),
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
		Err(err) => Err(HttpResponse::from_error(err)),
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
		Err(err) => Err(HttpResponse::from_error(err)),
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
        Err(err) => return Err(HttpResponse::from_error(err)),
    };

	Ok(user)
}
