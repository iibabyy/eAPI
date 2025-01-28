#![allow(unused)]

mod routes;
mod utils;
mod extractors;
mod dtos;
mod models;
mod database;
mod error;

use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, http, middleware::Logger, web, App, HttpServer};
use database::db::DBClient;
use deadpool_redis::Runtime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use utils::{AppState, config::{self, Config}};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Start !");
    // if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    // }

    dotenv().ok();
    env_logger::init();
    let config = Config::init();

    eprintln!("Config done !");

    // creating db connection pool
    let db_client = DBClient::new(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await?
    );
    eprintln!("Postgres pool done !");

    // creating redis connection pool
    let redis_pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(Runtime::Tokio1))?;
    eprintln!("Redis pool done !");

    let port = config.port;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .wrap(Logger::new("%a %r %s"))
            .wrap(cors)
            .configure(routes::config)
            .app_data(web::Data::new( AppState {
                db_client: db_client.clone(),
                redis: redis_pool.clone(),
                env: config.clone(),
            }))
            // .wrap(SessionMiddleware::new( redis_store.clone(), Key::generate() ))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}