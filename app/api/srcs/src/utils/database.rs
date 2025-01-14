// use super::constant;

// pub struct MyDatabase {
// 	pub client: Client,
// }

// impl MyDatabase {
// 	pub fn init() -> Self {
// 		let database_url = constant::DATABASE_URL.clone();
// 		let db = Self {
// 			client: Client::connect(&database_url, NoTls).expect("failed to connect to database"),
// 		};

// 		db
// 	}
	
// 	pub fn batch_execute(&mut self, query: &str) -> Result<(), postgres::Error> {
// 			self.client.batch_execute(query)
// 		}
// }