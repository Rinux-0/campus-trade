use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers;
use crate::models;
use crate::queries;
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::users,
        handlers::create_user,
        handlers::update_user,
        handlers::delete_user,
        handlers::list_items,
        handlers::create_item,
        handlers::update_price,
        handlers::delete_item,
        handlers::orders,
        handlers::buy_item,
        handlers::reset_data,
        queries::search,
        queries::sold_view,
        queries::unsold_view,
        queries::item_count,
        queries::category_count,
        queries::avg_price,
        queries::top_seller,
    ),
    components(schemas(
        models::User,
        models::Item,
        models::Order,
        handlers::CreateItem,
        handlers::UpdatePrice,
        handlers::BuyItem,
        handlers::CreateUser,
        handlers::UpdateUser,
        queries::SearchRow,
        queries::SoldItemView,
        queries::UnsoldItemView,
        queries::CategoryCount,
        queries::AvgPrice,
        queries::TopSeller,
    ))
)]
pub struct ApiDoc;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/api/users", axum::routing::get(handlers::users).post(handlers::create_user))
        .route("/api/users/{user_id}", axum::routing::put(handlers::update_user).delete(handlers::delete_user))
        .route("/api/items", axum::routing::get(handlers::list_items).post(handlers::create_item))
        .route("/api/items/{item_id}", axum::routing::delete(handlers::delete_item))
        .route("/api/items/{item_id}/price", axum::routing::put(handlers::update_price))
        .route("/api/orders", axum::routing::get(handlers::orders))
        .route("/api/orders/buy", axum::routing::post(handlers::buy_item))
        .route("/api/reset", axum::routing::post(handlers::reset_data))
        .route("/api/queries/search", axum::routing::get(queries::search))
        .route("/api/queries/views/sold", axum::routing::get(queries::sold_view))
        .route("/api/queries/views/unsold", axum::routing::get(queries::unsold_view))
        .route("/api/queries/count", axum::routing::get(queries::item_count))
        .route("/api/queries/category-count", axum::routing::get(queries::category_count))
        .route("/api/queries/avg-price", axum::routing::get(queries::avg_price))
        .route("/api/queries/top-seller", axum::routing::get(queries::top_seller))
        .merge(SwaggerUi::new("/api").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
