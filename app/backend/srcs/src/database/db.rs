
use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Socket};
use crate::{user::User, utils};

use super::PostgresResult;



pub struct UserDb {
	client: Client,
	connection: Connection<Socket, NoTlsStream>,
}

/*------[ UserDb Impl ]------*/
impl UserDb {
	pub async fn init() -> PostgresResult<Self> {
		let connection_str = format!(
			"host={} user={} password={} dbname={}",
			utils::constant::DB_ADDR.clone(),
			utils::constant::DB_USER.clone(),
			utils::constant::DB_PASSWD.clone(),
			utils::constant::DB_NAME.clone(),
		);

		let (client, connection) = tokio_postgres::connect(&connection_str, NoTls).await?;

		Ok( Self {
			client,
			connection,
		})
	}

	pub async fn create_users_table(&self) -> PostgresResult<()> {

		self.batch_execute("
				CREATE TABLE IF NOT EXISTS users (
					id SERIAL PRIMARY KEY,
					name VARCHAR NOT NULL,
					email VARCHAR NOT NULL UNIQUE
				)
			"
		).await?;

		Ok(())
	}

	pub async fn insert_user(&self, name: &str, email: &str) -> PostgresResult<()> {
		self.execute(
			"INSERT INTO users (name, email) VALUES ($1, $2)",
			&[&name, &email],
		).await?;

		Ok(())
	}

	pub async fn clients(&self) -> PostgresResult<Vec<User>> {
		let rows = self.query(
				"SELECT id, name, email FROM users",
				&[],
			).await?;

		let clients = rows
			.iter()
			.map(|row|
				User {
					id: row.get("id"),
					username: row.get("username"),
					email: row.get("email"),
				}
			)
			.collect();

		Ok(clients)
	}


}




/*------[ DELEGATE METHODS ]------*/
impl UserDb {
	pub async fn batch_execute(&self, query: &str) -> PostgresResult<()> {
		self.client.batch_execute(query).await
	}

	pub async fn execute<T>(
		&self,
		statement: &T,
		params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
	) -> Result<u64, tokio_postgres::Error>
	where
		T: ?Sized + tokio_postgres::ToStatement, {
		
		self.client.execute(statement, params).await
	}

	pub async fn query<T>(
		&self,
		statement: &T,
		params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
	) -> PostgresResult<Vec<tokio_postgres::Row>>
	where
		T: ?Sized + tokio_postgres::ToStatement, {
		self.client.query(statement, params).await
	}
}
