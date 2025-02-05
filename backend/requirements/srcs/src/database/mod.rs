use async_trait::async_trait;
use sqlx::Postgres;
use uuid::Uuid;

use crate::models::{Order, Product, User};

pub mod psql;

#[async_trait]
pub trait UserExtractor {
	async fn get_user(
		&self,
		user_id: &Uuid,
	) -> Result<Option<User>, sqlx::Error>;

	async fn get_user_by_email(
		&self,
		email: String,
	) -> Result<Option<User>, sqlx::Error>;

	async fn get_users_by_name(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<User>, sqlx::Error>;

	async fn get_all_users(
		&self,
		page: u32,
		limit: usize,
	) -> Result<Vec<User>, sqlx::Error>;
	async fn get_all_users_starting_by(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<User>, sqlx::Error>;

	async fn save_user<T: Into<String> + Send>(
		&self,
		name: T,
		email: T,
		password: T,
	) -> Result<User, sqlx::Error>;

	async fn delete_user(
		&self,
		user_id: &Uuid,
	) -> Result<(), sqlx::Error>;
}

#[async_trait]
pub trait ProductExtractor {
	async fn get_product(
		&self,
		product_id: &Uuid,
	) -> Result<Option<Product>, sqlx::Error>;

	async fn get_products_by_name(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error>;

	async fn get_all_products(
		&self,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error>;

	async fn get_all_products_starting_by(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error>;

	async fn save_product<T: Into<String> + Send>(
		&self,
		name: T,
		user_id: &Uuid,
		description: Option<&String>,
		price_in_cents: i64,
	) -> Result<Product, sqlx::Error>;

	async fn delete_product(
		&self,
		product_id: &Uuid,
	) -> Result<(), sqlx::Error>;

	async fn get_products_by_user(
		&self,
		user_id: &Uuid,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error>;
}

#[async_trait]
pub trait OrderExtractor {
	async fn get_order(
		&self,
		order_id: &Uuid,
	) -> Result<Option<Order>, sqlx::Error>;

	async fn get_all_orders(
		&self,
		page: u32,
		limit: usize,
	) -> Result<Vec<Order>, sqlx::Error>;

	async fn save_order(
		&self,
		user_id: &Uuid,
		product_id: &Uuid,
		order_details_id: Option<&Uuid>,
		products_number: i32,
	) -> Result<Order, sqlx::Error>;

	async fn delete_order(
		&self,
		order: &Uuid,
	) -> Result<(), sqlx::Error>;

	async fn get_orders_by_user(
		&self,
		user_id: &Uuid,
		page: u32,
		limit: usize,
	) -> Result<Vec<Order>, sqlx::Error>;
}
