use actix_web::web;

pub mod root;


pub fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/order")
			.service(root::get_by_id)
			.service(root::create)
			.service(root::create_details)
		);
}

