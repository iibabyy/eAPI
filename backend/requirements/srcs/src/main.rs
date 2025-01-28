#![allow(unused)]

mod routes;
mod utils;
mod services;
mod dtos;
mod models;
mod database;
mod error;

use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, http, middleware::Logger, web, App, HttpServer};
use deadpool_redis::Runtime;
use sqlx::postgres::PgPoolOptions;
use utils::{AppState, config::{self, Config}};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    let config = Config::init();

    // creating db connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    // creating redis connection pool
    let redis_pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(Runtime::Tokio1))?;

    let app_state = AppState {
        db: db_pool.clone(),
        redis: redis_pool.clone(),
        env: config.clone()
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .app_data(web::Data::new( app_state ))
            .wrap(Logger::new("%a %r %s"))
            .configure(routes::config)
            .wrap(cors)
            // .wrap(SessionMiddleware::new( redis_store.clone(), Key::generate() ))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}