mod user;
mod product;
mod root;
mod order;

use actix_web::web;

#[allow(dead_code)]
pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(root::root);
	
	config
		.configure(user_config)
		.configure(products_config)
		.configure(order_config);
}

#[allow(dead_code)]
pub fn user_config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/user")
			.service(user::root::register)
			.service(user::root::get_by_id)
			.service(user::root::get_all)
			.service(user::root::delete)
			.service(user::root::add_sold)
		);
}

#[allow(dead_code)]
pub fn products_config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/product")
			.service(product::root::create)
			.service(product::root::get)
		);
}

#[allow(dead_code)]
pub fn order_config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/order")
			.service(order::root::get_by_id)
			.service(order::root::create)
			.service(order::root::create_details)
		);
}

