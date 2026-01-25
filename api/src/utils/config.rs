use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    // pub redis_url: String,
    pub secret_key: String,
    pub access_token_max_seconds: i64,
    pub refresh_token_max_seconds: i64,
}

impl Config {
    pub fn init() -> Self {
        let database_url = database_url();
        // let redis_url = redis_url();
        let port = port();
        let secret_key = secret_key();
        let access_token_max_seconds = access_token_max_age_in_seconds();
        let refresh_token_max_seconds = refresh_token_max_age_in_seconds();

        Self {
            port,
            database_url,
            // redis_url,
            secret_key,
            access_token_max_seconds,
            refresh_token_max_seconds
        }
    }
}

fn secret_key() -> String {
    env::var("SECRET_KEY")
        .expect("DATABASE_URL need to be set")
        .to_string()
}

fn access_token_max_age_in_seconds() -> i64 {
    let minutes = env::var("ACCESS_TOKEN_MAX_AGE_IN_MINUTES")
        .unwrap_or("15".to_string())
        .parse::<i64>()
        .expect("ACCESS_TOKEN_MAX_AGE_IN_MINUTES: invalid value");

    minutes * 60
}

fn refresh_token_max_age_in_seconds() -> i64 {
    let days = env::var("REFRESH_TOKEN_MAX_AGE_IN_DAYS")
        .unwrap_or("30".to_string())
        .parse::<i64>()
        .expect("REFRESH_TOKEN_MAX_AGE_IN_SECONDS: invalid value");

    days * 24 * 60 * 60
}

fn port() -> u16 {
    env::var("LISTEN")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("LISTEN: invalid port")
}

// fn redis_url() -> String  {
// 	let redis_host = env::var("REDIS_HOST").unwrap_or("localhost".to_string());
// 	let redis_port = env::var("REDIS_PORT").unwrap_or("6379".to_string()).parse::<u16>().expect("REDIS_PORT: invalid value");

// 	format!(
// 		"redis://{}:{}",
// 		redis_host,
// 		redis_port,
// 	)
// }

fn database_url() -> String {
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER need to be set");
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("POSTGRES_PORT")
        .unwrap_or("5432".to_string())
        .parse::<u16>()
        .expect("POSTGRES_PORT: invalid value");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB need to be set");

    format!("postgres://{user}:{password}@{host}:{port}/{db}")
}
