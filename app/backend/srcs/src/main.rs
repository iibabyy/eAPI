mod routes;
mod utils;

use std::ops::Deref;

use actix_web::{middleware::Logger, web, App, HttpServer};
use postgres::{Client, NoTls};
use utils::database::MyDatabase;

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

    let mut db = MyDatabase::init();

    db.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS user (
            id          SERIAL PRIMARY KEY UNIQUE,
            username    VARCHAR UNIQUE NOT NULL,
            password    UNIQUE NOT NULL,
            email       VARCHAR UNIQUE NOT NULL,
            )
        ",
    ).unwrap();

    // HttpServer::new(move || {
    //     App::new()
    //     .app_data(web::Data::new( db ))
    //     .wrap(Logger::new("%a %r %s"))
    //     .configure(routes::home_routes::config)
    // })
    // .bind((addr, port))?
    // .run()
    // .await

    Ok(())
}
