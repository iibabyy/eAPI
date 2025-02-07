use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::{Order, Product, User};

use super::{OrderExtractor, ProductExtractor, UserExtractor};


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
		user_id: &Uuid,
	) -> Result<Option<User>, sqlx::Error> {
		let user: Option<User> = sqlx::query_as!(
			User,
			r#"
			SELECT id, name, email, password, sold_in_cents, created_at, updated_at
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
			SELECT id, name, email, password, sold_in_cents, created_at, updated_at
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
			SELECT id, name, email, password, sold_in_cents, created_at, updated_at
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
			SELECT id, name, email, password, sold_in_cents, created_at, updated_at
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
			SELECT id, name, email, password, sold_in_cents, created_at, updated_at
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
			RETURNING id, name, email, password, sold_in_cents, updated_at, created_at
			"#,
			name.into(),
			email.into(),
			password.into(),
		)
		.fetch_one(&self.pool)
		.await?;

		Ok(user)
	}

	async fn delete_user(
		&self,
		user_id: &Uuid,
	) -> Result<(), sqlx::Error> {
		let result = sqlx::query!(
			r#"
			DELETE FROM users
			WHERE id = $1
			"#,
			user_id,
		)
		.execute(self.pool())
		.await?;

		Ok(())
	}

}


#[async_trait]
impl ProductExtractor for DBClient {
	async fn get_product(
		&self,
		product_id: &Uuid,
	) -> Result<Option<Product>, sqlx::Error> {
		let product: Option<Product> = sqlx::query_as!(
			Product,
			r#"
			SELECT id, name, user_id, description, price_in_cents, number_in_stock, created_at, updated_at
			FROM products
			WHERE id = $1
			"#,
			product_id,
		)
		.fetch_optional(&self.pool)
		.await?;

		Ok(product)
	}

	async fn get_products_by_user(
		&self,
		user_id: &Uuid,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;
	
		let products: Vec<Product> = sqlx::query_as!(
			Product,
			r#"
			SELECT id, name, user_id, description, price_in_cents, number_in_stock, created_at, updated_at
			FROM products
			WHERE user_id = $1
			LIMIT $2
			OFFSET $3
			"#,
			user_id,
			limit as i64,
			offset as i64,
		)
		.fetch_all(&self.pool)
		.await?;

		Ok(products)
	}

	async fn get_products_by_name(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;
	
		let products: Vec<Product> = sqlx::query_as!(
				Product,
				r#"
				SELECT id, name, user_id, description, price_in_cents, number_in_stock, created_at, updated_at
				FROM products
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

		Ok(products)
	}

	async fn get_all_products(
		&self,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;

		let products: Vec<Product> = sqlx::query_as!(
				Product,
				r#"
				SELECT id, name, user_id, description, price_in_cents, number_in_stock, created_at, updated_at
				FROM products
				LIMIT $1
				OFFSET $2
				"#,
				limit as i64,
				offset as i64,
			)
			.fetch_all(&self.pool)
			.await?;

		Ok(products)
	}

	async fn get_all_products_starting_by(
		&self,
		name: String,
		page: u32,
		limit: usize,
	) -> Result<Vec<Product>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;

		let products: Vec<Product> = sqlx::query_as!(
				Product,
				r#"
				SELECT id, name, user_id, description, price_in_cents, number_in_stock, created_at, updated_at
				FROM products
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

		Ok(products)
	}

	async fn save_product<T: Into<String> + Send>(
		&self,
		name: T,
		user_id: &Uuid,
		description: Option<&String>,
		price_in_cents: i64,
		number_in_stock: i32,
	) -> Result<Product, sqlx::Error> {
		let product = sqlx::query_as!(
				Product,
				r#"
				INSERT INTO products ( name, user_id, description, price_in_cents, number_in_stock )
				VALUES ( $1, $2, $3, $4, $5 )
				RETURNING id, name, user_id, description, price_in_cents, number_in_stock, updated_at, created_at
				"#,
				name.into(),
				user_id,
				description,
				price_in_cents,
				number_in_stock,
			)
			.fetch_one(&self.pool)
			.await?;

		Ok(product)
	}

	async fn delete_product(
		&self,
		user_id: &Uuid,
	) -> Result<(), sqlx::Error> {
		let product = sqlx::query!(
			r#"
			DELETE FROM products
			WHERE id = $1
			"#,
			user_id,
		)
		.execute(&self.pool)
		.await?;

		Ok(())
	}

}

#[async_trait]
impl OrderExtractor for DBClient {
	async fn get_order(
		&self,
		order_id: &Uuid,
	) -> Result<Option<Order>, sqlx::Error> {
		let order = sqlx::query_as!(
				Order,
				r#"
				SELECT id, user_id, product_id, order_details_id, created_at, updated_at, products_number
				FROM orders
				WHERE id = $1
				"#,
				order_id,
			)
			.fetch_optional(self.pool())
			.await?;
		
		Ok(order)
	}

	async fn get_all_orders(
		&self,
		page: u32,
		limit: usize,
	) -> Result<Vec<Order>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;

		let orders = sqlx::query_as!(
				Order,
				r#"
				SELECT id, user_id, product_id, order_details_id, created_at, updated_at, products_number
				FROM orders
				"#
			)
			.fetch_all(self.pool())
			.await?;

		Ok(orders)
	}

	async fn save_order(
		&self,
		user_id: &Uuid,
		product_id: &Uuid,
		order_details_id: Option<&Uuid>,
		products_number: i32,
	) -> Result<Order, sqlx::Error> {
		
		let order = sqlx::query_as!(
				Order,
				r#"
				INSERT INTO orders( user_id, product_id, order_details_id, products_number )
				VALUES ( $1, $2, $3, $4 )
				RETURNING id, user_id, product_id, order_details_id, created_at, updated_at, products_number
				"#,
				user_id,
				product_id,
				order_details_id,
				products_number,
			)
			.fetch_one(self.pool())
			.await?;

		Ok(order)
	}

	async fn delete_order(
		&self,
		order_id: &Uuid,
	) -> Result<(), sqlx::Error> {

		let result = sqlx::query!(
			r#"
			DELETE FROM orders
			WHERE id = $1
			"#,
			order_id,
		)
		.execute(self.pool())
		.await?;

		Ok(())
	}

	async fn get_orders_by_user(
		&self,
		user_id: &Uuid,
		page: u32,
		limit: usize,
	) -> Result<Vec<Order>, sqlx::Error> {
		let offset = (page - 1) * limit as u32;

		let orders = sqlx::query_as!(
				Order,
				r#"
				SELECT id, user_id, product_id, order_details_id, created_at, updated_at, products_number
				FROM orders
				WHERE user_id = $1
				"#,
				user_id,
			)
			.fetch_all(self.pool())
			.await?;

		Ok(orders)
	}

}

#[cfg(test)]
mod user_tests {
	use futures_util::TryFutureExt;
	use crate::utils::test_utils::init_test_users;
	use super::*;

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_user_by_id(pool: Pool<Postgres>) {
		let (id_1, _, _) = init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let user = db_client
			.get_user(&id_1)
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
			.get_user(&nonexistant_id)
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

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn delete_user(pool: Pool<Postgres>) {
		let (user_id, _, _) = init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		db_client.delete_user(&user_id).await.expect("Failed to delete user");

		let result = db_client.get_user(&user_id).await.unwrap();

		match result {
			Some(_) => panic!("User found, but no one expected"),
			None => (),
		}
	}


	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn delete_invalid_user(pool: Pool<Postgres>) {
		let (_, _, _) = init_test_users(&pool).await;
		let db_client = DBClient::new(pool);

		let result = db_client
			.delete_user(&Uuid::new_v4())
			.await
			.map_err(|err| panic!("Unexpected Error while deleting unexistent user: {err}"));

		// no errors returned if user not found, expected behavior
	}

}

#[cfg(test)]
mod products_tests {
	use futures_util::TryFutureExt;
	use crate::utils::test_utils::init_test_products;
	use super::*;

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_product_by_id(pool: Pool<Postgres>) {
		let (product_data, _, _) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let product = db_client
			.get_product(&product_data.product_id)
			.await
			.unwrap_or_else(|err| panic!("Failed to get product by id: {}", err))
			.expect("product not found");

		assert_eq!(product.id, product_data.product_id);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_product_by_nonexistent_id(pool: Pool<Postgres>) {
		init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let nonexistant_id = Uuid::new_v4();

		let result = db_client
			.get_product(&nonexistant_id)
			.await
			.unwrap_or_else(|err| panic!("Failed to get product by id: {}", err));

		assert!(result.is_none(), "Expected product to be None");
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_products_by_user(pool: Pool<Postgres>) {
		let (_, _, data) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let products = db_client
			.get_products_by_user(&data.user_id, 1, 5)
			.await
			.unwrap_or_else(|err| panic!("Failed to get products by user: {}", err));

		assert_eq!(products.len(), 1);

		let product = products.iter().nth(0).unwrap();
		assert_eq!(product.user_id, data.user_id);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_products_by_nonexistent_user_id(pool: Pool<Postgres>) {
		let (_, _, _) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let nonexistent_user_id = Uuid::new_v4();

		let result = db_client
			.get_products_by_user(&nonexistent_user_id, 1, 5)
			.await
			.unwrap_or_else(|err| panic!("Failed to get products by user id: {}", err));

		assert_eq!(result.len(), 0);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_products_by_name(pool: Pool<Postgres>) {
		init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let name_to_find = "shoes";

		let products = db_client
			.get_products_by_name(name_to_find.to_string(), 1, 10)
			.await
			.unwrap_or_else(|err| panic!("Failed to get products by name: {}", err));

		assert_eq!(products.len(), 1);
		assert_eq!(products.iter().nth(0).unwrap().name, name_to_find);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_products_by_nonexistent_name(pool: Pool<Postgres>) {
		init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let name_to_find = "Unkwown product";

		let products = db_client
			.get_products_by_name(name_to_find.to_string(), 1, 5)
			.await
			.unwrap_or_else(|err| panic!("Failed to get products by name: {}", err));

		assert_eq!(products.len(), 0);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_products(pool: Pool<Postgres>) {
		init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let products = db_client
			.get_all_products(1, 10)
			.await
			.unwrap_or_else(|err| panic!("Failed to get all products: {}", err));

		assert_eq!(products.len(), 3);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_product(pool: Pool<Postgres>) {
		let (data, _, _) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let name = "Car";
		let user_id = data.user_id;
		let description = Some("A beautiful car".to_string());
		let price_in_cents = 1200;

		db_client.save_product(name, &user_id, description.as_ref(), price_in_cents, 1).await.unwrap();

		let products = db_client
			.get_products_by_name(name.to_string(), 1, 5)
			.await
			.unwrap_or_else(|err| panic!("Failed to get products by name: {err}"));

		assert_eq!(products.len(), 1);

		let product = products.iter().nth(0).unwrap();

		assert_eq!(product.name, name);
		assert_eq!(product.user_id, user_id);
		assert_eq!(product.description, description);
		assert_eq!(product.price_in_cents, price_in_cents);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_product_with_invalid_user_id(pool: Pool<Postgres>) {
		let (_, _, _) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let name = "Car";
		let user_id = Uuid::new_v4();
		let description = Some("A beautiful car".to_string());
		let price_in_cents = 1200;

		let result = db_client.save_product(name, &user_id, description.as_ref(), price_in_cents, 1).await;

		match result {
			Err(sqlx::Error::Database(db_err)) => {
				if db_err.is_foreign_key_violation() {
					// Ok
					return ;
				} else {
					panic!("Foreign key violation expected, found: {}", db_err.message())
				}
			},
			Err(err) => panic!("Database error expected, found: {err}"),
			Ok(_) => panic!("Call succeded, but a Database error was expected"),
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_product_but_name_too_long(pool: Pool<Postgres>) {
		let (data, _, _) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let too_long_name = "a".repeat(200);
		let user_id = data.user_id;
		let description = None;
		let price_in_cents = 12;

		let result = db_client
			.save_product(too_long_name.as_str(), &user_id, description, price_in_cents, 1)
			.await;

		assert!(result.is_err(), "Expected save to fail");
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn delete_product(pool: Pool<Postgres>) {
		let (data, _, _) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		db_client.delete_product(&data.product_id).await.expect("Failed to delete product");

		let result = db_client.get_product(&data.product_id).await.unwrap();

		match result {
			Some(_) => panic!("Product found, but no one expected"),
			None => (),
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn delete_invalid_product(pool: Pool<Postgres>) {
		let (_, _, _) = init_test_products(&pool).await;
		let db_client = DBClient::new(pool);

		let result = db_client
			.delete_product(&Uuid::new_v4())
			.await
			.map_err(|err| panic!("Unexpected error while deleting unexistent product: {err}"));

		// no errors returned if product not found, expected behavior
	}

}

mod orders_test {
	use futures_util::TryFutureExt;
	use crate::utils::test_utils::{init_test_orders};
	use super::*;

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_order_by_id(pool: Pool<Postgres>) {
		let (order_data, _, _) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let order = db_client
			.get_order(&order_data.order_id)
			.await
			.unwrap_or_else(|err| panic!("Failed to get order by id: {}", err))
			.expect("order not found");

		assert_eq!(order.id, order_data.order_id);
		assert_eq!(order.product_id, order_data.product_id);
		assert_eq!(order.user_id, order_data.user_id);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_order_by_nonexistent_id(pool: Pool<Postgres>) {
		init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let nonexistant_id = Uuid::new_v4();

		let result = db_client
			.get_order(&nonexistant_id)
			.await
			.unwrap_or_else(|err| panic!("Failed to get order by id: {}", err));

		assert!(result.is_none(), "Expected order to be None");
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_orders_by_user(pool: Pool<Postgres>) {
		let (_, _, data) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let orders = db_client
			.get_orders_by_user(&data.user_id, 1, 5)
			.await
			.unwrap_or_else(|err| panic!("Failed to get orders by user: {}", err));

		assert_eq!(orders.len(), 1);

		let order = orders.iter().nth(0).unwrap();
		assert_eq!(order.user_id, data.user_id);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_orders_by_nonexistent_user_id(pool: Pool<Postgres>) {
		let (_, _, _) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let nonexistent_user_id = Uuid::new_v4();

		let result = db_client
			.get_orders_by_user(&nonexistent_user_id, 1, 5)
			.await
			.unwrap_or_else(|err| panic!("Failed to get orders by user id: {}", err));

		assert_eq!(result.len(), 0);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn get_orders(pool: Pool<Postgres>) {
		init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let orders = db_client
			.get_all_orders(1, 10)
			.await
			.unwrap_or_else(|err| panic!("Failed to get all orders: {}", err));

		assert_eq!(orders.len(), 3);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_order(pool: Pool<Postgres>) {
		let (data, data2, data3) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let test_user = db_client.save_user(
			"test_user",
			"test_user@gmail.com",
			"test",
		).await.unwrap();

		let user_id = &test_user.id;
		let product_id = &data3.product_id;
		let order_details_id = None;

		db_client.save_order(
			user_id,
			product_id,
			order_details_id,
			2,
		).await.unwrap();

		let orders = db_client
			.get_orders_by_user(user_id, 1, 5)
			.await
			.unwrap_or_else(|err| panic!("Failed to get orders by user_id: {err}"));

		assert_eq!(orders.len(), 1);

		let order = orders.iter().nth(0).unwrap();

		assert_eq!(order.user_id, user_id.clone());
		assert_eq!(order.product_id, product_id.clone());
		assert_eq!(order.order_details_id, order_details_id.copied());
		assert_eq!(order.products_number, 2);
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_order_with_invalid_user_id(pool: Pool<Postgres>) {
		let (_, _, data) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let user_id = Uuid::new_v4();
		let product_id = &data.product_id;
		let order_details_id = None;

		let result =  db_client.save_order(
			&user_id,
			product_id,
			order_details_id,
			1,
		).await;

		match result {
			Err(sqlx::Error::Database(db_err)) => {
				if db_err.is_foreign_key_violation() {
					// Ok
					return ;
				} else {
					panic!("Foreign key violation expected, found: {}", db_err.message())
				}
			},
			Err(err) => panic!("Database error expected, found: {err}"),
			Ok(_) => panic!("Call succeded, but a Database error was expected"),
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_order_with_invalid_product_id(pool: Pool<Postgres>) {
		let (_, _, data) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let user_id = &data.user_id;
		let product_id = Uuid::new_v4();
		let order_details_id = None;

		let result =  db_client.save_order(
			user_id,
			&product_id,
			order_details_id,
			1,
		).await;

		match result {
			Err(sqlx::Error::Database(db_err)) => {
				if db_err.is_foreign_key_violation() {
					// Ok
					return ;
				} else {
					panic!("Foreign key violation expected (found: {})", db_err.message())
				}
			},
			Err(err) => panic!("Database error expected, found: {err}"),
			Ok(_) => panic!("Call succeded, but a Database error was expected"),
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_order_with_invalid_order_details_id(pool: Pool<Postgres>) {
		let (_, _, data) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let test_user = db_client.save_user(
			"test_user",
			"test_user@gmail.com",
			"test",
		).await.unwrap();

		let user_id = &test_user.id;
		let product_id = &data.product_id;
		let order_details_id = Some(Uuid::new_v4());

		let result = db_client.save_order(
			user_id,
			product_id,
			order_details_id.as_ref(),
			1,
		).await;

		match result {
			Err(sqlx::Error::Database(db_err)) => {
				if db_err.is_foreign_key_violation() {
					// Ok
					return ;
				} else {
					panic!("Foreign key violation expected (found: {})", db_err.message())
				}
			},
			Err(err) => panic!("Database error expected, found: {err}"),
			Ok(_) => panic!("Call succeded, but a Database error was expected"),
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn save_order_but_products_number_too_low(pool: Pool<Postgres>) {
		let (data, _, data3) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let result = db_client.save_order(
			&data.user_id,
			&data3.product_id,
			None,
			0,
		).await.err();

		match result {
			None => panic!("Call succeded, but an error was expected"),
			Some(sqlx::Error::Database(db_err)) => {
				if db_err.is_check_violation() {
					// Ok
				} else {
					// other error
					panic!("Failed to save order: {}", db_err.message())
				}
			},

			// other error
			Some(err) => panic!("Failed to save order: {err}"),
		}

	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn delete_order(pool: Pool<Postgres>) {
		let (data, _, _) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		db_client.delete_order(&data.order_id).await.expect("Failed to delete order");

		let result = db_client.get_order(&data.order_id).await.unwrap();

		match result {
			Some(_) => panic!("order found, but no one expected"),
			None => (),	// not found, ok
		}
	}

	#[sqlx::test(migrator = "crate::MIGRATOR")]
	async fn delete_invalid_order(pool: Pool<Postgres>) {
		let (_, _, _) = init_test_orders(&pool).await;
		let db_client = DBClient::new(pool);

		let result = db_client
			.delete_order(&Uuid::new_v4())
			.await
			.map_err(|err| panic!("Unexpected error while deleting unexistent product: {err}"));

		// no errors returned if order not found, expected behavior
	}

}