use actix_web::web;
use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
	config
	.service(web::scope("/user")
	.service(handlers::user_handler::add_friend)
	.service(handlers::user_handler::greet)
	.service(handlers::user_handler::root)
	)

	.service(handlers::root_handlers::home);
}