use sea_orm::DatabaseConnection;

pub struct AppState {
	pub db: DatabaseConnection,
}

impl AppState {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: db.clone() }
	}
}