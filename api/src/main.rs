mod database;
mod docs;
mod dtos;
mod error;
mod middleware;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
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

        let redirect_to_docs = async || -> Result<HttpResponse> {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/docs/"))
                .finish())
        };

        App::new()
            .app_data(app_data)
            .configure(routes::config)
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/docs/openapi.json", ApiDoc::openapi())
                    .config(
                        utoipa_swagger_ui::Config::from("/docs/openapi.json")
                            .filter(true)
                            .default_models_expand_depth(10), // .default_model_expand_depth(10),
                    ),
            )
            .service(web::resource("/").route(web::get().to(redirect_to_docs)))
            .service(web::resource("/docs").route(web::get().to(redirect_to_docs)))
            .wrap(Logger::new("%a %r %s"))
            .wrap(cors)
        // .wrap(SessionMiddleware::new( redis_store.clone(), Key::generate() ))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
