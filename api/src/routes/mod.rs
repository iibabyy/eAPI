pub mod auth;
pub mod orders;
pub mod products;
pub mod user;

use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .configure(user::config)
            .configure(auth::config)
            .configure(products::config)
            .configure(orders::config),
    );
}
