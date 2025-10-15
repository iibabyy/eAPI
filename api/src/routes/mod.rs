mod auth;
mod orders;
mod products;
mod root;
mod user;

use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(root::root);

    config.service(
        web::scope("/api")
            .configure(user::config)
            .configure(auth::config)
            .configure(products::config)
            .configure(orders::config),
    );
}
