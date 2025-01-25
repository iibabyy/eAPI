use actix_web::cookie::Key;
use lazy_static::lazy_static;
use std::env;

lazy_static!{
	pub static ref LISTEN:			u16		= set_listen();
	pub static ref DB_NAME: 		String	= set_db_name();
	pub static ref DB_USER:			String	= set_db_user();
	pub static ref DB_PASSWD:		String	= set_db_password();
	pub static ref DB_ADDR:			String	= set_db_host();
	pub static ref DB_PORT:			u16		= set_db_port();
	pub static ref DATABASE_URL:	String	= set_database_url();
	pub static ref SECRET_KEY:		Key		= Key::generate();
	pub static ref REDIS_HOST:		String	= set_redis_host();
	pub static ref REDIS_PORT:		u16		= set_redis_port();
	pub static ref REDIS_ADDR:		String	= set_redis_addr();
}

fn set_listen() -> u16 {
	let port = env::var("LISTEN").expect("LISTEN: invalid environment variable");

	match port.parse::<u16>() {
		Ok(port) => return port,
		Err(err) => {
			eprintln!("LISTEN: invalid port: {port}: {err}");
			panic!()
		}
	}
}

fn set_db_port() -> u16 {
	let port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());

	port.parse::<u16>().expect(&format!("POSTGRES_PORT: invalid port: {port}"))
}

fn set_db_host() -> String {
	dotenv::dotenv().ok();
	env::var("POSTGRES_HOST").unwrap_or("localhost".to_string())
}

fn set_db_user() -> String {
	dotenv::dotenv().ok();
	env::var("POSTGRES_USER").expect("POSTGRES_USER: invalid environment variable")
}

fn set_db_password() -> String {
	dotenv::dotenv().ok();
	env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD: invalid environment variable")
}

fn set_db_name() -> String {
	dotenv::dotenv().ok();
	env::var("POSTGRES_DB").expect("POSTGRES_DB: invalid environment variable")
}

fn set_redis_addr() -> String  {
	format!(
		"redis://{}:{}",
		REDIS_HOST.clone(),
		REDIS_PORT.clone(),
	)
}

fn set_redis_port() -> u16 {
	let port = env::var("REDIS_PORT").unwrap_or("6379".to_string());

	port.parse::<u16>().expect(&format!("POSTGRES_PORT: invalid port: {port}"))
}

fn set_redis_host() -> String {
	dotenv::dotenv().ok();
	env::var("REDIS_HOST").unwrap_or("localhost".to_string())
}

fn set_database_url() -> String {

	format!(
		"postgres://{}:{}@{}:{}/{}",
		DB_USER.clone(),
		DB_PASSWD.clone(),
		DB_ADDR.clone(),
		DB_PORT.clone(),
		DB_NAME.clone(),
	)
}