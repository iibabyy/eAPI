use actix_web::{web::{self, Json}, HttpResponse};
use sqlx::{Pool, Postgres};

use crate::{models::user::{NoPasswordUser, UserIdModel}, utils::app_state::AppState};

pub async fn get_user(
	id: i32,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, sqlx::Error> {
	
	let user = sqlx::query_as!(
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
	.fetch_one(db).await?;

	Ok(user)
}

pub async fn create_user(
	username: &String,
	email: &String,
	password: &String,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, sqlx::Error> {
	let user = sqlx::query_as!(
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
	.fetch_one(db).await?;

	Ok(user)
}

pub async fn delete_user(
	user_id: i32,
	db: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
	sqlx::query!(
		r#"
		DELETE FROM
			users
		WHERE
			user_id = $1
		"#,
		user_id,
	)
	.execute(db).await?;

	Ok(())
}

pub async fn get_all_users(
	db: &Pool<Postgres>,
) -> Result<Vec<NoPasswordUser>, sqlx::Error> {
	sqlx::query_as!(
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
	.fetch_all(db).await
}

pub async fn add_sold_to_user(
	user_id: i32,
	sold_to_add: i32,
	db: &Pool<Postgres>,
) -> Result<NoPasswordUser, sqlx::Error> {

	if sold_to_add < 1 { return Err(sqlx::Error::WorkerCrashed) }	// wrong error type

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
        Err(err) => return Err(err),
    };

	Ok(user)
}
