use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub order_id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub order_details_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderIdModel {
    pub order_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrderModel {
    pub user_id: i32,
    pub product_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderDetails {
    pub order_details_id: i32,
    pub delivery_address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrderDetailsModel {
	pub order_id: i32,
    pub delivery_address: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct OrderDetailsIdModel {
	pub order_id: i32,
    pub delivery_address: String,
}

