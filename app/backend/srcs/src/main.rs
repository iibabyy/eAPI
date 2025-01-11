mod routes;
mod utils;
mod database;

use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use utils::app_state::AppState;
// use utils::database::MyDatabase;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let port = utils::constant::LISTEN.clone();
    let database_url = utils::constant::DATABASE_URL.clone();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new( AppState { db: pool.clone() } ))
        .wrap(Logger::new("%a %r %s"))
        .configure(routes::config)
        .wrap(
            Cors::default()
                .allowed_origin("http://frontend")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
                .max_age(3600),
        )
    })
    .bind(("localhost", port))?
    .run()
    .await
}