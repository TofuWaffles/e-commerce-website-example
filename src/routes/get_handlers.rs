use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};

use http::StatusCode;
use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex;

use crate::routes::map_db_error;
use crate::utils::auth;
use crate::utils::models;

pub async fn get_products(
    Extension(db_pool): Extension<Pool<Sqlite>>,
) -> Result<Json<Vec<models::Product>>, (StatusCode, String)> {
    let products = sqlx::query_as!(
        models::Product,
        r#"
        SELECT 
        product_id,
        product_name,
        product_description,
        product_category AS "product_category: models::ProductCategory",
        stock,
        price,
        img_path
        FROM products
        "#
    )
    .fetch_all(&db_pool)
    .await
    .map_err(map_db_error)?;

    Ok(Json(products))
}

pub async fn get_addresses(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<Arc<Mutex<HashMap<String, String>>>>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<models::Address>>, (StatusCode, String)> {
    let authed_user_id =
        auth::authenticate_user(authorization.token().to_string(), active_users).await?;

    let addresses = sqlx::query_as!(
        models::Address,
        "SELECT
        address_id,
        unit,
        street,
        city,
        postal_code,
        state_province,
        country
        FROM addresses WHERE user_id = $1",
        authed_user_id,
    )
    .fetch_all(&db_pool)
    .await
    .map_err(map_db_error)?;

    Ok(Json(addresses))
}

pub async fn get_personal_info(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<Arc<Mutex<HashMap<String, String>>>>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<Json<models::PersonalInfo>, (StatusCode, String)> {
    let authed_user_id =
        auth::authenticate_user(authorization.token().to_string(), active_users).await?;

    let personal_info_option = sqlx::query_as!(
        models::PersonalInfo,
        r#"
        SELECT
        first_name,
        last_name,
        gender AS "gender: models::Gender"
        FROM personal_info WHERE user_id = $1
        "#,
        authed_user_id,
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(map_db_error)?;

    match personal_info_option {
        Some(personal_info) => Ok(Json(personal_info)),
        None => Err((StatusCode::NOT_FOUND, "Personal info not found".to_owned())),
    }
}

pub async fn get_cart(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<Arc<Mutex<HashMap<String, String>>>>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<models::DisplayCartItem>>, (StatusCode, String)> {
    let authed_user_id =
        auth::authenticate_user(authorization.token().to_string(), active_users).await?;

    let cart = sqlx::query_as!(
        models::DisplayCartItem,
        "
        SELECT
        products.product_name,
        products.price,
        cart_items.quantity
        from products
        INNER JOIN cart_items ON cart_items.product_id = products.product_id
        WHERE cart_items.user_id = $1
        ",
        authed_user_id,
    )
    .fetch_all(&db_pool)
    .await
    .map_err(map_db_error)?;

    Ok(Json(cart))
}

// pub async fn get_orders(
//     Extension(db_pool): Extension<Pool<Sqlite>>,
//     Extension(active_users): Extension<Arc<Mutex<HashMap<String, String>>>>,
//     authorization: TypedHeader<Authorization<Bearer>>,
// ) -> Result<Json<Vec<models::Order>>, (StatusCode, String)> {
//     let authed_user_id =
//         auth::authenticate_user(authorization.token().to_string(), active_users).await?;

//     let orders = sqlx::query_as!(
//         models::Order,
//         r#"
//         SELECT 
//         orders.order_id,
//         orders.creation_time AS "creation_time: NaiveDateTime",
//         orders.total_cost,
//         orders.order_status AS "order_status: models::OrderStatus"
//         FROM orders
//         INNER JOIN addresses ON addresses.address_id = orders.address_id
//         WHERE addresses.user_id = $1
//         "#,
//         authed_user_id,
//     )
//     .fetch_all(&db_pool)
//     .await
//     .map_err(map_db_error)?;

//     Ok(Json(orders))
// }

pub async fn get_order_items(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<Arc<Mutex<HashMap<String, String>>>>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Path(order_id): Path<i64>,
) -> Result<Json<Vec<models::OrderItem>>, (StatusCode, String)> {
    auth::authenticate_user(authorization.token().to_string(), active_users).await?;

    let order_items = sqlx::query_as!(
        models::OrderItem,
        "SELECT * FROM order_items WHERE order_id = $1",
        order_id,
    )
    .fetch_all(&db_pool)
    .await
    .map_err(map_db_error)?;

    Ok(Json(order_items))
}
