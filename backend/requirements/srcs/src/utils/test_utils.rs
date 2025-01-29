use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::database::{db::DBClient, UserExtractor};

use super::config::Config;

pub struct TestUser {
	name: &'static str,
	email: &'static str,
	password: &'static str,
}

pub fn test_config() -> Config {
	Config {
		database_url: "".to_string(),
		redis_url: "".to_string(),
		secret_key: "my-test-secret".to_string(),
		port: 8000,
		jwt_max_age: 2,
	}
}

pub async fn init_test_users(pool: &Pool<Postgres>) -> (Uuid, Uuid, Uuid) {
	let db_client = DBClient::new(pool.clone());

	let users = vec![
		TestUser {
            name: "John Doe",
            email: "johndoe@gmail.com",
            password: "password1234",
        },
        TestUser {
            name: "Nico Smith",
            email: "nicosmith@gmail.com",
            password: "123justgetit",
        },
        TestUser {
            name: "Michelle Like",
            email: "michellelike@gmail.com",
            password: "mostsecurepass",
        },
	];

	let mut users_id = vec![];

	for user_data in users {
		let user = db_client
			.save_user(user_data.name, user_data.email, user_data.password)
			.await
			.unwrap();
		users_id.push(user.id);
	}

	(
		users_id[0],
		users_id[1],
		users_id[2],
	)

}
