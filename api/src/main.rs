#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_wrap)]

mod database;
mod docs;
mod dtos;
mod error;
mod middleware;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Result, middleware::Logger, rt::signal::unix::{SignalKind, signal}, web};
use database::{init::init_database, psql::DBClient};
use docs::ApiDoc;
use sqlx::postgres::PgPoolOptions;
use utils::{config::Config, AppState};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }

    dotenvy::dotenv().ok();
    env_logger::init();

    let config = Config::init();

    init_database(&config.database_url).await?;

    let db_client = DBClient::new(
        PgPoolOptions::new()
            .max_connections(25)
            .connect(&config.database_url)
            .await?,
    );

    let port = config.port;

    let server = HttpServer::new(move || {
        let app_data = web::Data::new(AppState {
            db_client: db_client.clone(),
            env: config.clone(),
        });

        App::new()
            .app_data(app_data)
            .configure(routes::config)
            .service(swagger_ui_service())
            .service(web::resource("/").route(web::get().to(redirect_to_docs)))
            .service(web::resource("/docs").route(web::get().to(redirect_to_docs)))
            .wrap(Logger::new("%a %r %s"))
            .wrap(cors_wrapper())
    });

    let mut hangup    = signal(SignalKind::hangup())?;
    let mut interrupt = signal(SignalKind::interrupt())?;
    let mut terminate = signal(SignalKind::terminate())?;

    server
        .shutdown_signal(async move { hangup.recv().await; })
        .shutdown_signal(async move { interrupt.recv().await; })
        .shutdown_signal(async move { terminate.recv().await; })
        .bind(("0.0.0.0", port))?
        .run()
        .await?;

    Ok(())
}

fn swagger_ui_service() -> utoipa_swagger_ui::SwaggerUi {
    SwaggerUi::new("/docs/{_:.*}")
    .url("/docs/openapi.json", ApiDoc::openapi())
    .config(
        utoipa_swagger_ui::Config::from("/docs/openapi.json")
            .filter(true)
            .default_models_expand_depth(10), // .default_model_expand_depth(10),
    )
}

fn cors_wrapper() -> Cors {
    Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin()
}

async fn redirect_to_docs() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Found()
        .append_header(("Location", "/docs/"))
        .finish()
    )
}
