use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::User;

use super::UserExtractor;


#[derive(Debug, Clone)]
pub struct DBClient {
	pool: Pool<Postgres>,
}

impl DBClient {
	pub fn new(pool: Pool<Postgres>) -> Self {
		DBClient {
			pool
		}
	}
	
	pub fn pool(&self) -> &Pool<Postgres> {
		&self.pool
	}
}

#[async_trait]
impl UserExtractor for DBClient {
	async fn get_user(
		&self,
		user_id: Uuid,
	) -> Result<Option<User>, sqlx::Error> {
		let user: Option<User> = sqlx::query_as!(
			User,
			r#"
			SELECT id, name, email, password, sold, created_at, updated_at
			FROM users
			WHERE id = $1
			"#,
			user_id,
		)
		.fetch_optional(&self.pool)
		.await?;

		Ok(user)
	}

	async fn get_user_by_email(
		&self,
		email: String,
	) -> Result<Option<User>, sqlx::Error> {
		let user: Option<User> = sqlx::query_as!(
			User,
			r#"
			SELECT id, name, email, password, sold, created_at, updated_at
			FROM users
			WHERE email = $1
			"#,
			email,
		)
		.fetch_optional(&self.pool)
		.await?;

		Ok(user)
	}

	async fn get_users_by_name(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<User>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;
	
		let users: Vec<User> = sqlx::query_as!(
			User,
			r#"
			SELECT id, name, email, password, sold, created_at, updated_at
			FROM users
			WHERE name = $1
			LIMIT $2
			OFFSET $3
			"#,
			name,
			limit as i64,
			offset as i64,
		)
		.fetch_all(&self.pool)
		.await?;

		Ok(users)
	}

	async fn get_all_users(
		&self,
		page: u32,
		limit: usize,
	) -> Result<Vec<User>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;

		let users: Vec<User> = sqlx::query_as!(
			User,
			r#"
			SELECT id, name, email, password, sold, created_at, updated_at
			FROM users
			LIMIT $1
			OFFSET $2
			"#,
			limit as i64,
			offset as i64,
		)
		.fetch_all(&self.pool)
		.await?;

		Ok(users)
	}

	async fn get_all_users_starting_by(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<User>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;

		let users: Vec<User> = sqlx::query_as!(
			User,
			r#"
			SELECT id, name, email, password, sold, created_at, updated_at
			FROM users
			WHERE starts_with(name, $1)
			LIMIT $2
			OFFSET $3
			"#,
			name,
			limit as i64,
			offset as i64,
		)
		.fetch_all(&self.pool)
		.await?;

		Ok(users)
	}

	async fn save_user<T: Into<String> + Send>(
		&self,
		name: T,
		email: T,
		password: T,
	) -> Result<User, sqlx::Error> {
		let user = sqlx::query_as!(
			User,
			r#"
			INSERT INTO users ( name, email, password )
			VALUES ( $1, $2, $3 )
			RETURNING id, name, email, password, sold, updated_at, created_at
			"#,
			name.into(),
			email.into(),
			password.into(),
		)
		.fetch_one(&self.pool)
		.await?;

		Ok(user)
	}

}
