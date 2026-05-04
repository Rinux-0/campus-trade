use axum::{Json, extract::{Query, State}};
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use utoipa::{IntoParams, ToSchema};

use crate::AppState;

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct SearchRow {
    pub item_id: String,
    pub item_name: String,
    pub category: String,
    pub price: i32,
    pub status: i32,
    pub seller_id: String,
    pub order_id: Option<String>,
    pub buyer_id: Option<String>,
    pub buyer_name: Option<String>,
    pub order_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AvgPrice {
    pub avg_price: Option<f64>,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct TopSeller {
    pub seller_id: String,
    pub count: i64,
}

#[derive(Deserialize, IntoParams)]
pub struct SearchParams {
    pub status: Option<i32>,
    pub category: Option<String>,
    pub seller_id: Option<String>,
    pub buyer_id: Option<String>,
    pub item_id: Option<String>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

fn csv_vals(raw: &Option<String>) -> Vec<String> {
    raw.as_ref().map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()).unwrap_or_default()
}

#[utoipa::path(get, path = "/api/queries/search", tag = "queries", params(SearchParams), responses((status = 200, body = [SearchRow])))]
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Json<Vec<SearchRow>> {
    let mut qb = QueryBuilder::new(
        r#"SELECT item.item_id, item.item_name, item.category, item.price, item.status, item.seller_id,
                  orders.order_id, orders.buyer_id, u.user_name AS buyer_name, orders.order_date
           FROM item
           LEFT JOIN orders ON item.item_id = orders.item_id
           LEFT JOIN "user" u ON orders.buyer_id = u.user_id"#
    );
    let mut has_where = false;

    if let Some(s) = params.status {
        qb.push(" WHERE item.status = ").push_bind(s);
        has_where = true;
    }

    let ms: Vec<String> = csv_vals(&params.category);
    if !ms.is_empty() {
        if has_where { qb.push(" AND "); } else { qb.push(" WHERE "); } has_where = true;
        qb.push("item.category IN (");
        for (i, v) in ms.iter().enumerate() {
            if i > 0 { qb.push(", "); }
            qb.push_bind(v);
        }
        qb.push(")");
    }

    let ss: Vec<String> = csv_vals(&params.seller_id);
    if !ss.is_empty() {
        if has_where { qb.push(" AND "); } else { qb.push(" WHERE "); } has_where = true;
        qb.push("item.seller_id IN (");
        for (i, v) in ss.iter().enumerate() {
            if i > 0 { qb.push(", "); }
            qb.push_bind(v);
        }
        qb.push(")");
    }

    let bs: Vec<String> = csv_vals(&params.buyer_id);
    if !bs.is_empty() {
        if has_where { qb.push(" AND "); } else { qb.push(" WHERE "); } has_where = true;
        qb.push("orders.buyer_id IN (");
        for (i, v) in bs.iter().enumerate() {
            if i > 0 { qb.push(", "); }
            qb.push_bind(v);
        }
        qb.push(")");
    }

    let item_ids: Vec<String> = csv_vals(&params.item_id);
    if !item_ids.is_empty() {
        if has_where { qb.push(" AND "); } else { qb.push(" WHERE "); } has_where = true;
        qb.push("item.item_id IN (");
        for (i, v) in item_ids.iter().enumerate() {
            if i > 0 { qb.push(", "); }
            qb.push_bind(v);
        }
        qb.push(")");
    }

    let min_p = params.min_price.unwrap_or(0);
    let max_p = params.max_price.unwrap_or(i32::MAX);
    if has_where { qb.push(" AND "); } else { qb.push(" WHERE "); } has_where = true;
    qb.push("item.price >= ").push_bind(min_p);
    qb.push(" AND item.price <= ").push_bind(max_p);

    if let Some(ref df) = params.date_from {
        if !df.is_empty() {
            if has_where { qb.push(" AND "); } else { qb.push(" WHERE "); } has_where = true;
            qb.push("orders.order_date >= ").push_bind(df);
        }
    }

    if let Some(ref dt) = params.date_to {
        if !dt.is_empty() {
            if has_where { qb.push(" AND "); } else { qb.push(" WHERE "); } has_where = true;
            qb.push("orders.order_date <= ").push_bind(dt);
        }
    }

    qb.push(" ORDER BY item.item_id");
    let _ = has_where;

    let rows = qb.build_query_as::<SearchRow>()
        .fetch_all(&state.pool).await.unwrap_or_default();
    Json(rows)
}

// === Views ===

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct SoldItemView {
    pub item_id: String,
    pub item_name: String,
    pub price: i32,
    pub category: String,
    pub order_id: String,
    pub buyer_id: String,
    pub buyer_name: String,
    pub order_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct UnsoldItemView {
    pub item_id: String,
    pub item_name: String,
    pub category: String,
    pub price: i32,
    pub seller_id: String,
}

#[utoipa::path(get, path = "/api/queries/views/sold", tag = "queries", responses((status = 200, body = [SoldItemView])))]
pub async fn sold_view(State(state): State<AppState>) -> Json<Vec<SoldItemView>> {
    let rows = sqlx::query_as::<_, SoldItemView>("SELECT * FROM sold_items_view")
        .fetch_all(&state.pool).await.unwrap_or_default();
    Json(rows)
}

#[utoipa::path(get, path = "/api/queries/views/unsold", tag = "queries", responses((status = 200, body = [UnsoldItemView])))]
pub async fn unsold_view(State(state): State<AppState>) -> Json<Vec<UnsoldItemView>> {
    let rows = sqlx::query_as::<_, UnsoldItemView>("SELECT * FROM unsold_items_view")
        .fetch_all(&state.pool).await.unwrap_or_default();
    Json(rows)
}

// === Aggregation ===

#[utoipa::path(get, path = "/api/queries/count", tag = "queries", responses((status = 200)))]
pub async fn item_count(State(state): State<AppState>) -> Json<serde_json::Value> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM item")
        .fetch_one(&state.pool).await.unwrap_or((0,));
    Json(serde_json::json!({ "total": row.0 }))
}

#[utoipa::path(get, path = "/api/queries/category-count", tag = "queries", responses((status = 200, body = [CategoryCount])))]
pub async fn category_count(State(state): State<AppState>) -> Json<Vec<CategoryCount>> {
    let rows = sqlx::query_as::<_, CategoryCount>(
        "SELECT category, COUNT(*)::bigint AS count FROM item GROUP BY category ORDER BY count DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();
    Json(rows)
}

#[utoipa::path(get, path = "/api/queries/avg-price", tag = "queries", responses((status = 200, body = AvgPrice)))]
pub async fn avg_price(State(state): State<AppState>) -> Json<AvgPrice> {
    let row: (Option<f64>,) = sqlx::query_as("SELECT AVG(price::float8) FROM item")
        .fetch_one(&state.pool).await.unwrap_or((None,));
    Json(AvgPrice { avg_price: row.0 })
}

#[utoipa::path(get, path = "/api/queries/top-seller", tag = "queries", responses((status = 200, body = TopSeller)))]
pub async fn top_seller(State(state): State<AppState>) -> Json<TopSeller> {
    let row = sqlx::query_as::<_, TopSeller>(
        "SELECT seller_id, COUNT(*)::bigint AS count FROM item GROUP BY seller_id ORDER BY count DESC LIMIT 1"
    ).fetch_optional(&state.pool).await.unwrap_or(None)
    .unwrap_or(TopSeller { seller_id: String::new(), count: 0 });
    Json(row)
}
