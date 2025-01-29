pub mod config;
pub mod password;
pub mod token;
pub mod test_utils;

use config::Config;

use crate::database::db::DBClient;

#[derive(Clone)]
pub struct AppState {
	pub db_client: DBClient,
	// pub redis: deadpool_redis::Pool,
	pub env: Config,
}
