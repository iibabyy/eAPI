mod routes;
mod utils;
mod impls;
mod models;
mod database;

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use deadpool_redis::Runtime;
use sqlx::postgres::PgPoolOptions;
use utils::{app_state::AppState, constant::{self}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // creating db connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&constant::DATABASE_URL.clone())
        .await
        .expect("Failed to create pool");

    // creating redis connection pool
    let redis_pool = deadpool_redis::Config::from_url(constant::REDIS_ADDR.clone())
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // creating 
    let redis_store = RedisSessionStore::new(constant::REDIS_ADDR.clone())
        .await
		.expect("failed to connect to redis");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new( AppState { db: db_pool.clone(), redis: redis_pool.clone() } ))
        .wrap(Logger::new("%a %r %s"))
        .wrap(SessionMiddleware::new( redis_store.clone(), Key::generate() ))
        .configure(routes::config)
        // .default_service(web::to(|| HttpResponse::Ok()))
        // .wrap(Cors::default().allowed_origin("http://frontend").allowed_origin("http://localhost").allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]).allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE]).max_age(3600),)
    })
    .bind(("localhost", constant::LISTEN.clone()))
    .expect("")
    .run()
    .await
}