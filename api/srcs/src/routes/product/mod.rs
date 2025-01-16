use actix_web::web;

pub mod root;


#[allow(dead_code)]
pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/product")
			.service(root::create)
			.service(root::get)
		);
}
