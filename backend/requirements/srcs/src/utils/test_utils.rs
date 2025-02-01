use std::str::FromStr;

use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::database::{db::DBClient, ProductExtractor, UserExtractor};

use super::config::Config;

pub struct TestUser {
	name: &'static str,
	email: &'static str,
	password: &'static str,
}

#[derive(PartialEq, Eq)]
pub struct TestProducts {
	name: &'static str,
	user_id: Uuid,
	description: Option<String>,
	price: i32,
}

#[derive(Clone, Copy)]
pub struct TestProductsData {
	pub product_id: Uuid,
	pub user_id: Uuid,
}

pub fn test_config() -> Config {
	Config {
		database_url: "".to_string(),
		redis_url: "".to_string(),
		secret_key: "my-test-secret".to_string(),
		port: 8000,
		jwt_max_seconds: 60 * 2,
	}
}

pub async fn init_test_users(pool: &Pool<Postgres>) -> (Uuid, Uuid, Uuid) {
	let db_client = DBClient::new(pool.clone());


	let users = vec![
		TestUser {
            name: "Idrissa Baby",
            email: "ibaby@gmail.com",
            password: "password1234",
        },
        TestUser {
            name: "Moussa Adamou",
            email: "madamou@gmail.com",
            password: "123justgetit",
        },
        TestUser {
            name: "Imhad Thari",
            email: "ithari@gmail.com",
            password: "mostsecurepass",
        },
        TestUser {
            name: "Idrissa Baby",
            email: "duplicate@gmail.com",
            password: "mypassword",
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

pub async fn init_test_products(pool: &Pool<Postgres>) -> (TestProductsData, TestProductsData, TestProductsData) {
	let db_client = DBClient::new(pool.clone());
	let (user_1, user_2, user_3) = init_test_users(pool).await;

	let products = vec![
		TestProducts {
            name: "shoes",
			description: None,
			user_id: user_1,
			price: 35,
        },
        TestProducts {
            name: "jacket",
			description: Some("A black jacket".to_string()),
			user_id: user_2,
			price: 50,
        },
        TestProducts {
            name: "hat",
			description: Some("A tall hat".to_string()),
			user_id: user_3,
			price: 15,
        },
	];

	let mut products_id = vec![];

	for product in products {
		let product = db_client
			.save_product(
				product.name,
				&product.user_id,
				product.description,
				product.price
			)
			.await
			.unwrap();

		products_id.push(TestProductsData {
			product_id: product.id,
			user_id: product.user_id
		});
	}

	(
		products_id[0],
		products_id[1],
		products_id[2],
	)

}

pub async fn assert_user_infos(id: impl ToString, name: impl ToString, email: impl ToString, db_client: &DBClient) {
	let user = db_client
		.get_user(Uuid::from_str(&id.to_string()).expect("Invalid id"))
		.await
		.expect("Failed to get user")
		.expect("User not found");

	assert_eq!(user.name, name.to_string());
	assert_eq!(user.email, email.to_string());
}
