pub mod config;
pub mod constants;
pub mod models;
pub mod password;
pub mod status;
pub mod test_utils;
pub mod token;

use config::Config;

use crate::database::psql::DBClient;

#[derive(Clone)]
pub struct AppState {
    pub db_client: DBClient,
    // pub redis: deadpool_redis::Pool,
    pub env: Config,
}
