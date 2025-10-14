mod database;
mod dtos;
mod error;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use colored::Colorize;
use database::{init::init_db, psql::DBClient};
use sqlx::postgres::PgPoolOptions;
use utils::{config::Config, AppState};

#[cfg(test)]
const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenvy::dotenv().ok();
    env_logger::init();

    let config = Config::init();

    init_db(&config.database_url).await?;

    // creating db connection pool
    let db_client = DBClient::new(
        PgPoolOptions::new()
            .max_connections(25)
            .connect(&config.database_url)
            .await?,
    );

    // // creating redis connection pool
    // let redis_pool = deadpool_redis::Config::from_url(&config.redis_url)
    //     .create_pool(Some(Runtime::Tokio1))?;

    let port = config.port;

    eprintln!(
        "{}{}",
        "Server listening on port 0.0.0.0:".bright_black(),
        port.to_string().bright_black(),
    );

    HttpServer::new(move || {
        let app_data = web::Data::new(AppState {
            db_client: db_client.clone(),
            // redis: redis_pool.clone(),
            env: config.clone(),
        });

        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .app_data(app_data)
            .configure(routes::config)
            .wrap(Logger::new("%a %r %s"))
            .wrap(cors)
        // .wrap(SessionMiddleware::new( redis_store.clone(), Key::generate() ))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
