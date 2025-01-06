mod routes;
mod utils;

use std::ops::Deref;

use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

type ActixResult<T> = Result<T, actix_web::Error>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let port = utils::constant::PORT.deref().clone();
    let addr = utils::constant::ADDRESS.deref().clone();
    let database_url = utils::constant::DATABASE_URL.deref().clone();

    let db: DatabaseConnection = Database::connect(database_url).await.expect("failed to connect to database");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new( AppState { db: db.clone() } ))
        .wrap(Logger::new("%a %r %s"))
        .configure(routes::home_routes::config)
    })
    .bind((addr, port))?
    .run()
    .await
}
