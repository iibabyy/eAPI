mod user;
mod products;
mod root;

use actix_web::web;

#[allow(dead_code)]
pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(root::root);
	
	config
		.configure(user_config)
		.configure(products_config);
}


#[allow(dead_code)]
pub fn user_config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/user")
			.service(user::user_create)
			.service(user::user_get_by_id)
		);
}

#[allow(dead_code)]
pub fn products_config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/user")
			.service(products::product_create)
			.service(products::products_get_by_id)
		);
}

