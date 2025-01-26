
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Product {
    pub product_id: i32,
	pub name: String,
	pub description: Option<String>,
    pub price: i32,
    pub user_id: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateProductModel {
	pub name: String,
    pub price: i32,
	pub description: Option<String>,
    pub user_id: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ProductIdModel {
    pub product_id: i32,
}
