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
			.service(user::create)
			.service(user::get_by_id)
			.service(user::get_all)
			.service(user::delete)
			.service(user::add_sold)
		);
}

#[allow(dead_code)]
pub fn products_config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/product")
			.service(product::create)
			.service(product::get_by_id)
		);
}

#[allow(dead_code)]
pub fn order_config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/order")
			.service(order::get_by_id)
			.service(order::create)
			.service(order::create_details)
		);
}

