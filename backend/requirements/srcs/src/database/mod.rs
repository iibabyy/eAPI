use async_trait::async_trait;
use sqlx::Postgres;

pub mod db;

#[async_trait]
pub trait UserExtractor {
	async fn get_user(
		&self,
		user_id: Uuid,
	) -> Result<Option<User>, sqlx::Error>;

	async fn get_user_by_email(
		&self,
		email: String,
	) -> Result<Option<User>, sqlx::Error>;

	async fn get_users_by_name(
		&self,
		name: String,
	) -> Result<Vec<User>, sqlx::Error>;

	async fn get_all_users(
		&self,
		page: u32,
		limit: usize,
	) -> Result<Vec<User>, sqlx::Error>;

	async fn save_user<T: Into<String> + Send>(
		&self,
		name: T,
		email: T,
		password: T,
	) -> Result<User, sqlx::Error>;
}