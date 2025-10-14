use chrono::prelude::*;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::utils;

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub last_token_id: Option<String>,
    pub sold_in_cents: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub number_in_stock: i32,
    pub price_in_cents: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub order_details_id: Option<Uuid>,
    pub products_number: i32,
    // others fields ?

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
