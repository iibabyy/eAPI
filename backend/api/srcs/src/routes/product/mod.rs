use actix_web::web;

pub mod root;


pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/products")
			.service(root::create)
			.service(root::get)
		);
}
