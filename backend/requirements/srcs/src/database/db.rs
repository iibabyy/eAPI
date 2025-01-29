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


#[cfg(test)]
mod test {
	 use futures_util::TryFutureExt;

use crate::utils::test_utils::init_test_users;

use super::*;

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_user_by_id(pool: Pool<Postgres>) {
		let (id_1, _, _) = init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let user = db_client
			.get_user(id_1)
			.await
			.unwrap_or_else(|err| panic!("Failed to get user by id: {}", err))
			.expect("User not found");

		assert_eq!(user.id, id_1);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_user_by_nonexistent_id(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let nonexistant_id = Uuid::new_v4();

		let result = db_client
			.get_user(nonexistant_id)
			.await
			.unwrap_or_else(|err| panic!("Failed to get user by id: {}", err));

		assert!(result.is_none(), "Expected user to be None");
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_user_by_email(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let email_to_find = "madamou@gmail.com";

		let user = db_client
			.get_user_by_email(email_to_find.to_string())
			.await
			.unwrap_or_else(|err| panic!("Failed to get user by email: {}", err))
			.expect("User not found");

		assert_eq!(user.email, email_to_find);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_user_by_noneistent_email(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let email_to_find = "nonexistant@gmail.com";

		let result = db_client
			.get_user_by_email(email_to_find.to_string())
			.await
			.unwrap_or_else(|err| panic!("Failed to get user by email: {}", err));

		assert!(result.is_none(), "Expected user to be None");
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_users_by_name(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let name_to_find = "Idrissa Baby";

		let users = db_client
			.get_users_by_name(name_to_find.to_string(), 1, 10)
			.await
			.unwrap_or_else(|err| panic!("Failed to get users by name: {}", err));

		assert!(users.len() == 2, "Expected to found 2 users");

		for user in users {
			assert_eq!(user.name, name_to_find);
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_users_by_nonexistent_name(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let name_to_find = "Whoever Iam";

		let users = db_client
			.get_users_by_name(name_to_find.to_string(), 1, 10)
			.await
			.unwrap_or_else(|err| panic!("Failed to get users by name: {}", err));

		assert_eq!(users.len(), 0);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_users(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let users = db_client
			.get_all_users(1, 10)
			.await
			.unwrap_or_else(|err| panic!("Failed to get all users: {}", err));

		assert_eq!(users.len(), 4);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_user(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let name = "Mohammed Dembele";
		let email = "mdembele@gmail.com";
		let password = "somestrongpassword";

		db_client.save_user(name, email, password).await.unwrap();

		let user = db_client
			.get_user_by_email(email.to_string())
			.await
			.unwrap_or_else(|err| panic!("Failed to get users by name: {err}"))
			.expect("User not found");

		assert_eq!(name, user.name);
		assert_eq!(email, user.email);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_user_but_email_is_taken(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let name = "Imhad Thari";
		let email = "ithari@gmail.com";
		let password = "mostsecurepass";

		let result = db_client.save_user(name, email, password).await;

		match result {
			Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
				// Ok !
			},
			_ => {
				panic!("Expected unique constraint violation error");
			}
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_user_but_name_too_long(pool: Pool<Postgres>) {
		init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let too_long_name = "a".repeat(200);
		let email = "exemple@gamil.com";
		let password = "password";

		let result = db_client
			.save_user(too_long_name.as_str(), email, password)
			.await;
		
		assert!(result.is_err(), "Expected save to fail");
	}

}
