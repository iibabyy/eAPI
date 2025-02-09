pub mod config;
pub mod password;
pub mod token;
pub mod test_utils;
pub mod constants;
pub mod status;

use config::Config;

use crate::database::psql::DBClient;

#[derive(Clone)]
pub struct AppState {
	pub db_client: DBClient,
	// pub redis: deadpool_redis::Pool,
	pub env: Config,
}
