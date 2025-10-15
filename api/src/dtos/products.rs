use crate::{utils::models::Product, utils::status::Status};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductDto {
    #[validate(length(min = 1, message = "Name is required"))]
    #[schema(example = "Smartphone")]
    pub name: String,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,

    #[schema(example = "A high-quality smartphone")]
    pub description: Option<String>,

    #[validate(range(min = 1, max = 9999, message = "Invalid number in stock"))]
    #[schema(example = 10)]
    pub number_in_stock: i32,

    #[validate(range(min = 0, message = "Prices can not be negative"))]
    #[schema(example = 25000)]
    pub price_in_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilterProductDto {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
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

            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductResponseDto {
    pub status: Status,
    pub data: ProductDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterProductResponseDto {
    pub status: Status,
    pub data: FilterProductDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductListResponseDto {
    pub status: Status,
    pub data: Vec<ProductDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterProductListResponseDto {
    pub status: Status,
    pub data: Vec<FilterProductDto>,
    pub results: usize,
}
