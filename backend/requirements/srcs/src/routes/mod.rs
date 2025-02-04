mod root;
mod user;
mod auth;
mod products;
mod orders;

use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(root::root);
		
		config
		.service(web::scope("/api")
			.configure(user::config)
			.configure(auth::config)
			// .configure(product::config)
			// .configure(order::config)
		);
}
