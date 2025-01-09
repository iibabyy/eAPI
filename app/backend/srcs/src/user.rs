use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
	pub username: String,
    pub email: String,
}
