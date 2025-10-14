use std::path::Path;

use sqlx::{migrate::{MigrateDatabase, Migrator}, PgPool, Postgres};

use crate::{utils::config::Config};

pub async fn init_db(db_url: &str) -> Result<(), sqlx::Error> {
	if !Postgres::database_exists(db_url).await.unwrap_or(false) {
		Postgres::create_database(db_url).await?;
	}
	
	let pool = PgPool::connect(db_url).await?;

	let migrator = Migrator::new(Path::new("/migrations")).await.unwrap();
	migrator.run(&pool).await?;

	Ok(())
}