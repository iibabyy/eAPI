use chrono::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub sold_in_cents: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Product {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price_in_cents: i64,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Order {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub order_details_id: Option<uuid::Uuid>,
    pub products_number: i32,
    // others fields ?

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
