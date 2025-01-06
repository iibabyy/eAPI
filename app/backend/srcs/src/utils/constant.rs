use lazy_static::lazy_static;
use std::env;

lazy_static!{
	pub static ref ADDRESS: String = set_addr();
	pub static ref PORT: u16 = set_port();
	pub static ref DATABASE_URL: String = set_database_url();
}

fn set_port() -> u16 {
	let port = env::var("LISTEN").expect("LISTEN: invalid environment variable");

	match port.parse::<u16>() {
		Ok(port) => return port,
		Err(err) => {
			eprintln!("LISTEN: invalid port: {port}: {err}");
			panic!()
		}
	}
}

fn set_addr() -> String {
	dotenv::dotenv().ok();
	env::var("ADDRESS").expect("ADDRESS: invalid environment variable")
}

fn set_database_url() -> String {
	dotenv::dotenv().ok();
	env::var("DATABASE_URL").expect("DATABASE_URL: invalid environment variable")
}