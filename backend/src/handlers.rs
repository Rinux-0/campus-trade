use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::AppState;
use crate::db;
use crate::models::{Item, Order, User};

#[derive(Deserialize, IntoParams)]
pub struct ItemPath {
    pub item_id: String,
}

#[derive(Deserialize, IntoParams)]
pub struct UserPath {
    pub user_id: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateItem {
    pub item_id: String,
    pub item_name: String,
    pub category: String,
    pub price: i32,
    pub seller_id: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdatePrice {
    pub price: i32,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct BuyItem {
    pub item_id: String,
    pub buyer_id: String,
    pub order_date: chrono::NaiveDate,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateUser {
    pub user_name: String,
    pub phone: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateUser {
    pub user_name: Option<String>,
    pub phone: Option<String>,
}

#[utoipa::path(get, path = "/api/users", tag = "users", responses((status = 200, body = [User])))]
pub async fn users(State(state): State<AppState>) -> Json<Vec<User>> {
    let rows = sqlx::query_as::<_, User>("SELECT * FROM \"user\" ORDER BY user_id")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    Json(rows)
}

#[utoipa::path(post, path = "/api/users", tag = "users", responses((status = 200, body = User)))]
pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUser>,
) -> Json<User> {
    // find smallest available user_id by checking for gaps
    let ids: Vec<String> = sqlx::query_scalar(
        "SELECT user_id FROM \"user\" WHERE user_id ~ '^u[0-9]+$' ORDER BY user_id"
    )
    .fetch_all(&state.pool).await.unwrap_or_default();

    let mut next_num = 1;
    for id in &ids {
        let num: i32 = id[1..].parse().unwrap_or(0);
        if num == next_num {
            next_num += 1;
        } else {
            break;
        }
    }
    let next = format!("u{:03}", next_num);

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO \"user\" (user_id, user_name, phone) VALUES ($1,$2,$3) RETURNING *"
    )
    .bind(&next)
    .bind(&body.user_name)
    .bind(&body.phone)
    .fetch_one(&state.pool)
    .await
    .expect("Create user failed");
    Json(user)
}

#[utoipa::path(put, path = "/api/users/{user_id}", tag = "users", responses((status = 200, body = User)))]
pub async fn update_user(
    State(state): State<AppState>,
    Path(path): Path<UserPath>,
    Json(body): Json<UpdateUser>,
) -> Json<User> {
    let current = sqlx::query_as::<_, User>("SELECT * FROM \"user\" WHERE user_id = $1")
        .bind(&path.user_id)
        .fetch_one(&state.pool)
        .await
        .expect("User not found");

    let user = sqlx::query_as::<_, User>(
        "UPDATE \"user\" SET user_name = $1, phone = $2 WHERE user_id = $3 RETURNING *"
    )
    .bind(body.user_name.unwrap_or(current.user_name))
    .bind(body.phone.unwrap_or(current.phone))
    .bind(&path.user_id)
    .fetch_one(&state.pool)
    .await
    .expect("Update user failed");
    Json(user)
}

#[utoipa::path(delete, path = "/api/users/{user_id}", tag = "users", responses((status = 200, body = User)))]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(path): Path<UserPath>,
) -> Result<Json<User>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let uid = &path.user_id;
    let mut tx = state.pool.begin().await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string() }))
    ))?;

    // cascade delete in FK-safe order
    sqlx::query("DELETE FROM orders WHERE buyer_id = $1").bind(uid).execute(&mut *tx).await.ok();
    sqlx::query("DELETE FROM orders WHERE item_id IN (SELECT item_id FROM item WHERE seller_id = $1)").bind(uid).execute(&mut *tx).await.ok();
    sqlx::query("DELETE FROM item WHERE seller_id = $1").bind(uid).execute(&mut *tx).await.ok();

    let user = sqlx::query_as::<_, User>("DELETE FROM \"user\" WHERE user_id = $1 RETURNING *")
        .bind(uid).fetch_one(&mut *tx).await.map_err(|e| (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("delete failed: {}", e) }))
        ))?;

    tx.commit().await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string() }))
    ))?;
    Ok(Json(user))
}

// --- Item handlers ---

#[utoipa::path(get, path = "/api/items", tag = "items", responses((status = 200, body = [Item])))]
pub async fn list_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    let rows = sqlx::query_as::<_, Item>("SELECT * FROM item ORDER BY item_id")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    Json(rows)
}

#[utoipa::path(post, path = "/api/items", tag = "items", responses((status = 200, body = Item)))]
pub async fn create_item(
    State(state): State<AppState>,
    Json(body): Json<CreateItem>,
) -> Json<Item> {
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO item (item_id, item_name, category, price, status, seller_id) VALUES ($1,$2,$3,$4,0,$5) RETURNING *"
    )
    .bind(&body.item_id)
    .bind(&body.item_name)
    .bind(&body.category)
    .bind(body.price)
    .bind(&body.seller_id)
    .fetch_one(&state.pool)
    .await
    .expect("Create item failed");
    Json(item)
}

#[utoipa::path(put, path = "/api/items/{item_id}/price", tag = "items", responses((status = 200, body = Item)))]
pub async fn update_price(
    State(state): State<AppState>,
    Path(path): Path<ItemPath>,
    Json(body): Json<UpdatePrice>,
) -> Json<Item> {
    let item = sqlx::query_as::<_, Item>(
        "UPDATE item SET price = $1 WHERE item_id = $2 RETURNING *"
    )
    .bind(body.price)
    .bind(&path.item_id)
    .fetch_one(&state.pool)
    .await
    .expect("Update price failed");
    Json(item)
}

#[utoipa::path(delete, path = "/api/items/{item_id}", tag = "items", responses((status = 200, body = Item)))]
pub async fn delete_item(
    State(state): State<AppState>,
    Path(path): Path<ItemPath>,
) -> Json<Item> {
    let item = sqlx::query_as::<_, Item>(
        "DELETE FROM item WHERE item_id = $1 AND status = 0 RETURNING *"
    )
    .bind(&path.item_id)
    .fetch_one(&state.pool)
    .await
    .expect("Delete failed (item may not exist or already sold)");
    Json(item)
}

#[utoipa::path(get, path = "/api/orders", tag = "orders", responses((status = 200, body = [Order])))]
pub async fn orders(State(state): State<AppState>) -> Json<Vec<Order>> {
    let rows = sqlx::query_as::<_, Order>("SELECT * FROM orders ORDER BY order_id")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    Json(rows)
}

#[utoipa::path(post, path = "/api/orders/buy", tag = "orders", responses((status = 200, body = Order)))]
pub async fn buy_item(
    State(state): State<AppState>,
    Json(body): Json<BuyItem>,
) -> Result<Json<Order>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let mut tx = state.pool.begin().await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string() }))
    ))?;

    let status: (i32,) = sqlx::query_as("SELECT status FROM item WHERE item_id = $1 FOR UPDATE")
        .bind(&body.item_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("item not found: {}", e) }))
        ))?;

    if status.0 != 0 {
        return Err((
            axum::http::StatusCode::CONFLICT,
            Json(serde_json::json!({ "error": "item already sold" }))
        ));
    }

    let max_id: (Option<String>,) = sqlx::query_as("SELECT MAX(order_id) FROM orders")
        .fetch_one(&mut *tx).await.unwrap_or((None,));
    let next = match max_id.0 {
        Some(id) => format!("o{:03}", id[1..].parse::<i32>().unwrap_or(0) + 1),
        None => "o001".into(),
    };

    sqlx::query("UPDATE item SET status = 1 WHERE item_id = $1")
        .bind(&body.item_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() }))
        ))?;

    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (order_id, item_id, buyer_id, order_date) VALUES ($1,$2,$3,$4) RETURNING *"
    )
    .bind(&next)
    .bind(&body.item_id)
    .bind(&body.buyer_id)
    .bind(body.order_date)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string() }))
    ))?;

    tx.commit().await.map_err(|e| (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string() }))
    ))?;

    Ok(Json(order))
}

#[utoipa::path(post, path = "/api/reset", tag = "system", responses((status = 200)))]
pub async fn reset_data(State(state): State<AppState>) -> Json<serde_json::Value> {
    db::reset(&state.pool).await;
    Json(serde_json::json!({ "ok": true }))
}
