use crate::{utils::models::Order, utils::status::Status};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub product_id: Uuid,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub order_details_id: Option<Uuid>,

    #[validate(range(min = 1, message = "Product number can only be more than 1"))]
    #[schema(example = 2)]
    pub products_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderDto {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub order_details_id: Option<Uuid>,
    pub products_number: i32,
    pub product_id: uuid::Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OrderDto {
    pub fn from(order: &Order) -> Self {
        OrderDto {
            id: order.id,
            user_id: order.user_id,
            product_id: order.product_id,
            products_number: order.products_number,
            order_details_id: order.order_details_id,

            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilterOrderDto {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub product_id: uuid::Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FilterOrderDto {
    #[allow(dead_code)]
    pub fn filter(order: &Order) -> Self {
        FilterOrderDto {
            id: order.id,
            user_id: order.user_id,
            product_id: order.product_id,

            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderResponseDto {
    pub status: Status,
    pub data: OrderDto,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterOrderResponseDto {
    pub status: Status,
    pub data: FilterOrderDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderListResponseDto {
    pub status: Status,
    pub data: Vec<OrderDto>,
    pub results: usize,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterOrderListResponseDto {
    pub status: Status,
    pub data: Vec<FilterOrderDto>,
    pub results: usize,
}
