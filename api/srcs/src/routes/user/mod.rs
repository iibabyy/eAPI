use actix_web::web;

pub mod root;

pub(super) fn config(config: &mut web::ServiceConfig) {
	config
		.service(web::scope("/user")
			.service(root::register)
			.service(root::login)
			.service(root::get_by_id)
			.service(root::get_all)
			.service(root::delete)
			.service(root::add_sold)
		);
}
