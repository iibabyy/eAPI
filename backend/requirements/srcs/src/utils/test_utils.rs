use std::str::FromStr;

use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::database::{psql::DBClient, OrderExtractor, ProductExtractor, UserExtractor};

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

pub async fn assert_user_infos(id: impl ToString, name: impl ToString, email: impl ToString, db_client: &DBClient) {
	let user = db_client
		.get_user(&Uuid::from_str(&id.to_string()).expect("Invalid id"))
		.await
		.expect("Failed to get user")
		.expect("User not found");

	assert_eq!(user.name, name.to_string());
	assert_eq!(user.email, email.to_string());
}


#[derive(PartialEq, Eq)]
pub struct TestProduct {
	name: &'static str,
	user_id: Uuid,
	description: Option<String>,
	price_in_cents: i64,
}

#[derive(Clone, Copy)]
pub struct TestProductData {
	pub product_id: Uuid,
	pub user_id: Uuid,
}


pub async fn init_test_products(pool: &Pool<Postgres>) -> (TestProductData, TestProductData, TestProductData) {
	let db_client = DBClient::new(pool.clone());
	let (user_1, user_2, user_3) = init_test_users(pool).await;

	let products = vec![
		TestProduct {
            name: "shoes",
			description: None,
			user_id: user_1,
			price_in_cents: 35,
        },
        TestProduct {
            name: "jacket",
			description: Some("A black jacket".to_string()),
			user_id: user_2,
			price_in_cents: 50,
        },
        TestProduct {
            name: "hat",
			description: Some("A tall hat".to_string()),
			user_id: user_3,
			price_in_cents: 15,
        },
	];

	let mut products_id = vec![];

	for product in products {
		let product = db_client
			.save_product(
				product.name,
				&product.user_id,
				product.description.as_ref(),
				product.price_in_cents
			)
			.await
			.unwrap();

		products_id.push(TestProductData {
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


#[derive(PartialEq, Eq)]
pub struct TestOrder {
	user_id: Uuid,
	product_id: Uuid,
	order_details_id: Option<Uuid>,
	products_number: i32,
}

#[derive(Clone, Copy)]
pub struct TestOrderData {
	pub order_id: Uuid,
	pub order_details_id: Option<Uuid>,
	pub product_id: Uuid,
	pub user_id: Uuid,
}

pub async fn init_test_orders(pool: &Pool<Postgres>) -> (TestOrderData, TestOrderData, TestOrderData) {
	let db_client = DBClient::new(pool.clone());
	let (product_1, product_2, product_3) = init_test_products(pool).await;

	let orders = vec![
		TestOrder {
			user_id: product_1.user_id,
			product_id: product_2.product_id,
			order_details_id: None,
			products_number: 1,
        },
        TestOrder {
			user_id: product_2.user_id,
			product_id: product_3.product_id,
			order_details_id: None,
			products_number: 1,
        },
        TestOrder {
			user_id: product_3.user_id,
			product_id: product_1.product_id,
			order_details_id: None,
			products_number: 2,
        },
	];

	let mut orders_data = vec![];

	for order in orders {
		let order = db_client
			.save_order(
				&order.user_id,
				&order.product_id,
				order.order_details_id.as_ref(),
				order.products_number,
			)
			.await
			.unwrap();

		orders_data.push(TestOrderData {
			order_id: order.id,
			user_id: order.user_id,
			product_id: order.product_id,
			order_details_id: order.order_details_id,
		});
	}

	(
		orders_data[0],
		orders_data[1],
		orders_data[2],
	)

}
