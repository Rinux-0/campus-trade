use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Item {
    pub item_id: String,
    pub item_name: String,
    pub category: String,
    pub price: i32,
    pub status: i32,
    pub seller_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Order {
    pub order_id: String,
    pub item_id: String,
    pub buyer_id: String,
    pub order_date: NaiveDate,
}
