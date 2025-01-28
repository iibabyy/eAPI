pub mod config;
pub mod password;
pub mod token;

#[derive(Clone)]
pub struct AppState {
	pub db: sqlx::PgPool,
	pub redis: deadpool_redis::Pool,
	pub env: Config,
}
