use actix_web::web;

pub mod root;


pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/product")
			.service(root::create)
			.service(root::get)
		);
}
