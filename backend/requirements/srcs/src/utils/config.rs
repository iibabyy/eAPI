use actix_web::cookie::Key;
use lazy_static::lazy_static;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
	pub database_url: String,
	pub redis_url: String,
	pub secret_key: String,
	pub port: u16,
	pub jwt_max_age: i64,
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
			jwt_max_age,
		}
	}

}

fn secret_key() -> String {
	env::var("SECRET_KEY").expect("SECRET_KEY must be set")
}

fn jwt_max_age() -> i64 {
	env::var("JWT_MAX_AGE").unwrap_or("5".to_string()).parse::<i64>().expect(&format!("JWT_MAX_AGE: invalid value"))
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
	let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
	let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
	let db_host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
	let db_port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string()).parse::<u16>().expect("POSTGRES_PORT: invalid value");
	let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

	format!(
		"postgres://{}:{}@{}:{}/{}",
		db_user,
		db_password,
		db_host,
		db_port,
		db_name,
	)
}