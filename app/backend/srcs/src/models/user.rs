
/* ------------------ */
/* --- [ MODELS ] --- */
/* ------------------ */

use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoPasswordUser {
    pub user_id: i32,
	pub username: String,
    pub email: String,
    pub sold: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserBody {
	pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUserBody {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddSoldBody {
    pub user_id: i32,
    pub sold: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserOrdersData {
    pub order_id: i32,
    pub product_id: i32,
}
