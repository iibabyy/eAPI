use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
use crate::{models::Product, utils::status::{validate_password, Status}};


#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    pub user_id: Uuid,
    pub description: Option<String>,

	#[validate(range(min = 0))]
	pub price_in_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductDto {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price_in_cents: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductData {
    pub product: ProductDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponseDto {
    pub status: Status,
    pub data: ProductData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForeignProductResponseDto {
    pub status: Status,
    pub data: ProductDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponseDto {
    pub status: Status,
    pub products: Vec<ProductDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub status: Status,
    pub data: ProductData,
    pub token: String,
}
