mod user;
mod product;
mod root;
mod order;

use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(root::root);
	
	config
		.configure(user::config)
		.configure(product::config)
		.configure(order::config);
}
