
/* ------------------ */
/* --- [ MODELS ] --- */
/* ------------------ */

use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Debug)]
pub struct PasswordUser {
    pub user_id: i32,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NoPasswordUser {
    pub user_id: i32,
	pub username: String,
    pub email: String,
    pub sold: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserModel {
	pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUserModel {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddSoldModel {
    pub sold_to_add: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserOrdersData {
    pub order_id: i32,
    pub product_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserIdModel {
    pub user_id: i32,
}
