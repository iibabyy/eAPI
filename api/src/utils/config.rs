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
    env::var("SECRET_KEY")
        .expect("DATABASE_URL need to be set")
        .to_string()
}

fn jwt_max_age() -> i64 {
    env::var("JWT_MAX_AGE")
        .unwrap_or("300".to_string())
        .parse::<i64>()
        .expect("JWT_MAX_AGE: invalid value")
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
