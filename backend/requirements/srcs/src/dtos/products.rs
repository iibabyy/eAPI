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

    #[validate(range(min = 1, max = 9999, message = "Invalid number in stock"))]
    pub number_in_stock: i32,

	#[validate(range(min = 0, message = "Prices can not be negative"))]
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
    pub number_in_stock: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for ProductDto {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl ProductDto {
    pub fn from(product: &Product) -> Self {
        ProductDto {
            id: product.id,
            user_id: product.user_id,
            name: product.name.to_owned(),
            description: product.description.to_owned(),
            price_in_cents: product.price_in_cents,
            number_in_stock: product.number_in_stock,

            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterProductDto {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub number_in_stock: i32,
    pub price_in_cents: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterProductDto {
    pub fn filter(product: &Product) -> Self {
        FilterProductDto {
            id: product.id,
            user_id: product.user_id,
            name: product.name.to_owned(),
            description: product.description.to_owned(),
            price_in_cents: product.price_in_cents,
            number_in_stock: product.number_in_stock,

            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponseDto {
    pub status: Status,
    pub data: ProductDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterProductResponseDto {
    pub status: Status,
    pub data: FilterProductDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponseDto {
    pub status: Status,
    pub data: Vec<ProductDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterProductListResponseDto {
    pub status: Status,
    pub data: Vec<FilterProductDto>,
    pub results: usize,
}
