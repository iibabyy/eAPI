use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};

use crate::MIGRATOR;

pub async fn init_database(db_url: &str) -> Result<(), sqlx::Error> {
    if !Postgres::database_exists(db_url).await.unwrap_or(false) {
        Postgres::create_database(db_url).await?;
    }

    let pool = PgPool::connect(db_url).await?;

    MIGRATOR.run(&pool).await?;

    Ok(())
}
