use lazy_static::lazy_static;
use std::env;

lazy_static!{
	pub static ref LISTEN:			u16		= set_listen();
	pub static ref DB_NAME: 		String	= set_db_name();
	pub static ref DB_USER:			String	= set_db_user();
	pub static ref DB_PASSWD:		String	= set_db_password();
	pub static ref DB_ADDR:			String	= set_addr();
	pub static ref DB_PORT:			u16		= set_port();
	pub static ref DATABASE_URL:	String	= set_database_url();
	pub static ref HASH_SECRET:	String	= set_hash_secret();
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

fn set_port() -> u16 {
	let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT: invalid environment variable");

	match port.parse::<u16>() {
		Ok(port) => return port,
		Err(err) => {
			eprintln!("POSTGRES_PORT: invalid port: {port}: {err}");
			panic!()
		}
	}
}

fn set_addr() -> String {
	dotenv::dotenv().ok();
	env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS: invalid environment variable")
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

fn set_hash_secret() -> String {
	dotenv::dotenv().ok();
	env::var("HASH_SECRET").expect("HASH_SECRET: invalid environment variable")
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