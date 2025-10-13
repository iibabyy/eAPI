use actix_web::cookie::Key;
use dotenvy_macro::dotenv;
use lazy_static::lazy_static;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
	pub port: u16,
	pub database_url: String,
	// pub redis_url: String,
	pub secret_key: String,
	pub jwt_max_seconds: i64,
}

impl Config {
	pub fn init() -> Self {
		let database_url = database_url();
		// let redis_url = redis_url();
		let port = port();
		let secret_key = secret_key();
		let jwt_max_age = jwt_max_age();

		Self {
			port,
			database_url,
			// redis_url,
			secret_key,
			jwt_max_seconds: jwt_max_age,
		}
	}

}

fn secret_key() -> String {
	dotenv!("SECRET_KEY").to_string()
}

fn jwt_max_age() -> i64 {
	let result_in_seconds = env::var("JWT_MAX_AGE").unwrap_or("300".to_string()).parse::<i64>().expect(&format!("JWT_MAX_AGE: invalid value"));

	result_in_seconds
}

fn port() -> u16 {
	env::var("LISTEN").unwrap_or("8000".to_string()).parse::<u16>().expect(&format!("LISTEN: invalid port"))
}

fn redis_url() -> String  {
	let redis_host = env::var("REDIS_HOST").unwrap_or("localhost".to_string());
	let redis_port = env::var("REDIS_PORT").unwrap_or("6379".to_string()).parse::<u16>().expect("REDIS_PORT: invalid value");

	format!(
		"redis://{}:{}",
		redis_host,
		redis_port,
	)
}

fn database_url() -> String {
	dotenv!("DATABASE_URL").to_string()
}