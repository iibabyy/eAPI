#[derive(Clone)]
pub struct AppState {
	pub db: sqlx::PgPool,
	pub redis: deadpool_redis::Pool,
}
