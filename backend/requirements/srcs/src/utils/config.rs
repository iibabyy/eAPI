use actix_web::cookie::Key;
use dotenvy_macro::dotenv;
use lazy_static::lazy_static;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
	pub database_url: String,
	pub redis_url: String,
	pub secret_key: String,
	pub port: u16,
	pub jwt_max_seconds: i64,
}

impl Config {
	pub fn init() -> Self {
		let database_url = database_url();
		let redis_url = redis_url();
		let port = port();
		let secret_key = secret_key();
		let jwt_max_age = jwt_max_age();

		Self {
			database_url,
			redis_url,
			port,
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
	env::var("LISTEN").unwrap_or("80".to_string()).parse::<u16>().expect(&format!("LISTEN: invalid value"))
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
	let db_user = dotenv!("POSTGRES_USER");
	let db_password = dotenv!("POSTGRES_PASSWORD");
	let db_host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
	let db_port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string()).parse::<u16>().expect("POSTGRES_PORT: invalid value");
	let db_name = dotenv!("POSTGRES_DB");

	format!(
		"postgres://{}:{}@{}:{}/{}",
		db_user,
		db_password,
		db_host,
		db_port,
		db_name,
	)
}