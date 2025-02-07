use std::ops::DerefMut;

use sqlx::{pool::maybe::MaybePoolConnection, PgConnection, Pool, Postgres, Transaction};
use sqlx_core::database::Database;
use uuid::Uuid;

pub trait ITransaction {
	type Error;

	async fn lock_user<'a>(
		&'a mut self,
		user_id: &Uuid,
	) -> Result<&'a mut Self, Self::Error>;
	
	async fn decrease_user_sold<'a>(
		&'a mut self,
		user_id: &Uuid,
		to_decrease: i64,
	) -> Result<&'a mut Self, Self::Error>;
	
	async fn increase_user_sold<'a>(
		&'a mut self,
		user_id: &Uuid,
		to_increase: i64,
	) -> Result<&'a mut Self, Self::Error>;

	async fn lock_product<'a>(
		&'a mut self,
		product_id: &Uuid,
	) -> Result<&'a mut Self, Self::Error>;
	
	async fn decrease_product_stock<'a>(
		&'a mut self,
		product_id: &Uuid,
		to_decrease: i32,
	) -> Result<&'a mut Self, Self::Error>;
	
	async fn increase_product_stock<'a>(
		&'a mut self,
		product_id: &Uuid,
		to_increase: i32,
	) -> Result<&'a mut Self, Self::Error>;

}

#[derive(Debug)]
pub struct DBTransaction<'c> {
	tx: Transaction<'c, sqlx::Postgres>,
}

impl<'c> DBTransaction<'c> {
	pub async fn begin(pool: &Pool<Postgres>) -> Result<Self, sqlx::Error> {
		let tx =
			Transaction::begin(
				MaybePoolConnection::PoolConnection(pool.acquire().await?)
			)
			.await?;

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

	async fn lock_user<'a>(
			&'a mut self,
			user_id: &Uuid,
		) -> Result<&'a mut Self, Self::Error> {
			let _ = sqlx::query!(
				r#"
				SELECT
				FROM users
				WHERE id = $1
				FOR UPDATE
				"#,
				user_id,
			)
			.execute(self.deref_mut())
			.await?;
	
			Ok(self)
	}

	async fn decrease_user_sold<'a>(
		&'a mut self,
		user_id: &Uuid,
		to_decrease: i64,
	) -> Result<&'a mut Self, Self::Error> {
			let _ = sqlx::query!(
				r#"
				UPDATE users
				SET sold_in_cents = sold_in_cents - $1
				WHERE id = $2
				"#,
				to_decrease,
				user_id,
			)
			.execute(self.deref_mut())
			.await?;
		
			Ok(self)
	}

	async fn increase_user_sold<'a>(
		&'a mut self,
		user_id: &Uuid,
		to_increase: i64,
	) -> Result<&'a mut Self, Self::Error> {
			let _ = sqlx::query!(
				r#"
				UPDATE users
				SET sold_in_cents = sold_in_cents + $1
				WHERE id = $2
				"#,
				to_increase,
				user_id,
			)
			.execute(self.deref_mut())
			.await?;
		
			Ok(self)
	}

	async fn lock_product<'a>(
		&'a mut self,
		product_id: &Uuid,
	) -> Result<&'a mut Self, Self::Error> {

			let _ = sqlx::query!(
				r#"
				SELECT
				FROM products
				WHERE id = $1
				FOR UPDATE
				"#,
				product_id,
			)
			.execute(self.deref_mut())
			.await?;
	
			Ok(self)
	}

	async fn decrease_product_stock<'a>(
		&'a mut self,
		product_id: &Uuid,
		to_decrease: i32,
	) -> Result<&'a mut Self, Self::Error> {
		
			sqlx::query!(
				r#"
				UPDATE products
				SET number_in_stock = number_in_stock - $1
				WHERE id = $2
				"#,
				to_decrease,
				product_id,
			)
			.execute(self.deref_mut())
			.await?;
	
			Ok(self)
	}

	async fn increase_product_stock<'a>(
		&'a mut self,
		product_id: &Uuid,
		to_increase: i32,
	) -> Result<&'a mut Self, Self::Error> {
		
			sqlx::query!(
				r#"
				UPDATE products
				SET number_in_stock = number_in_stock + $1
				WHERE id = $2
				"#,
				to_increase,
				product_id,
			)
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

