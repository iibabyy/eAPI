use std::ops::DerefMut;

use bcrypt::DEFAULT_COST;
use sqlx::{pool::maybe::MaybePoolConnection, PgConnection, Pool, Postgres, Transaction};
use sqlx_core::database::Database;
use uuid::Uuid;

use crate::utils::password;

pub trait ITransaction: Sized {
	type Error;

	async fn lock_user(
		self,
		user_id: &Uuid,
	) -> Result<Self, Self::Error>;
	
	async fn decrease_user_sold(
		self,
		user_id: &Uuid,
		to_decrease: i64,
	) -> Result<Self, Self::Error>;

	async fn increase_user_sold(
		self,
		user_id: &Uuid,
		to_increase: i64,
	) -> Result<Self, Self::Error>;

	async fn save_user_token_id(
		self,
		user_id: &Uuid,
		new_token: &Uuid,
	) -> Result<Self, Self::Error>;

	async fn lock_product(
		self,
		product_id: &Uuid,
	) -> Result<Self, Self::Error>;
	
	async fn decrease_product_stock(
		self,
		product_id: &Uuid,
		to_decrease: i32,
	) -> Result<Self, Self::Error>;

	async fn increase_product_stock(
		self,
		product_id: &Uuid,
		to_increase: i32,
	) -> Result<Self, Self::Error>;

}

#[derive(Debug)]
pub struct DBTransaction<'c> {
	tx: Transaction<'c, sqlx::Postgres>,
}

impl<'c> DBTransaction<'c> {
	pub async fn begin(pool: &Pool<Postgres>) -> Result<Self, sqlx::Error> {
		let tx = pool.begin().await?;

		Ok(
			Self { tx }
		)
	}
	
	pub async fn commit(mut self) -> Result<(), sqlx::Error> {
		self.tx.commit().await
	}
		
}

// TODO!: write tests for those functions
impl ITransaction for DBTransaction<'_> {
	type Error = sqlx::Error;

	async fn lock_user(
		mut self,
		user_id: &Uuid,
	) -> Result<Self, Self::Error> {
		let _ = sqlx::query(
			r#"
			SELECT
			FROM users
			WHERE id = $1
			FOR UPDATE
			"#
		)
		.bind(user_id)
		.execute(self.deref_mut())
		.await?;

		Ok(self)
	}

	async fn decrease_user_sold(
		mut self,
		user_id: &Uuid,
		to_decrease: i64,
	) -> Result<Self, Self::Error> {
			let _ = sqlx::query(
				r#"
				UPDATE users
				SET sold_in_cents = sold_in_cents - $1
				WHERE id = $2
				"#
			)
			.bind(to_decrease)
			.bind(user_id)
			.execute(self.deref_mut())
			.await?;
		
			Ok(self)
	}

	async fn increase_user_sold(
		mut self,
		user_id: &Uuid,
		to_increase: i64,
	) -> Result<Self, Self::Error> {
			let _ = sqlx::query(
				r#"
				UPDATE users
				SET sold_in_cents = sold_in_cents + $1
				WHERE id = $2
				"#
			)
			.bind(to_increase)
			.bind(user_id)
			.execute(self.deref_mut())
			.await?;
		
			Ok(self)
	}

	async fn lock_product(
		mut self,
		product_id: &Uuid,
	) -> Result<Self, Self::Error> {

			let _ = sqlx::query(
				r#"
				SELECT
				FROM products
				WHERE id = $1
				FOR UPDATE
				"#
			)
			.bind(product_id)
			.execute(self.deref_mut())
			.await?;
	
			Ok(self)
	}

	async fn decrease_product_stock(
		mut self,
		product_id: &Uuid,
		to_decrease: i32,
	) -> Result<Self, Self::Error> {
		
			sqlx::query(
				r#"
				UPDATE products
				SET number_in_stock = number_in_stock - $1
				WHERE id = $2
				"#
			)
			.bind(to_decrease)
			.bind(product_id)
			.execute(self.deref_mut())
			.await?;
	
			Ok(self)
	}

	async fn increase_product_stock(
		mut self,
		product_id: &Uuid,
		to_increase: i32,
	) -> Result<Self, Self::Error> {
		
			sqlx::query(
				r#"
				UPDATE products
				SET number_in_stock = number_in_stock + $1
				WHERE id = $2
				"#
			)
			.bind(to_increase)
			.bind(product_id)
			.execute(self.deref_mut())
			.await?;
	
			Ok(self)
	}

	async fn save_user_token_id(
		mut self,
		new_token_id: &Uuid,
		user_id: &Uuid,
	) -> Result<Self, Self::Error> {
		let hashed_id = bcrypt::hash(new_token_id.to_string(), 4)
			.map_err(|_| sqlx::Error::WorkerCrashed)?;	// no the real error


		sqlx::query(
			r#"
			UPDATE users
			SET last_token_id = $1
			WHERE id = $2
			"#
		)
		.bind(hashed_id)
		.bind(user_id)
		.execute(self.deref_mut())
		.await?;

		Ok(self)
	}
}

impl<'a> std::ops::DerefMut for DBTransaction<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tx.deref_mut()
    }
}

impl<'a> std::ops::Deref for DBTransaction<'a> {
	type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        self.tx.deref()
    }

}

#[cfg(test)]
mod tests {
	use super::*;
	// TODO!: make tests !
}

