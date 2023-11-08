use axum::{
    extract::Extension,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use chrono::Local;
use pwhash::bcrypt;
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::routes::map_db_error;
use crate::utils::auth;
use crate::utils::jwt;
use crate::utils::models;

use super::ActiveUsers;

pub async fn create_user(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Json(new_user): Json<models::NewUser>,
) -> Result<String, (StatusCode, String)> {
    if auth::check_user_exists(&db_pool, &new_user.user_email).await? {
        return Err((
            StatusCode::BAD_REQUEST,
            "Unable to create user, another user is using the same email".to_owned(),
        ));
    }

    // Create a new user with a new UUID and hashed password
    let user = models::User::new(&new_user);

    sqlx::query_as!(
        models::User,
        "
        INSERT INTO users(user_id, username, user_email, user_password_hash) 
        VALUES ($1, $2, $3, $4);
        ",
        user.user_id,
        user.username,
        user.user_email,
        user.user_password_hash,
    )
    .execute(&db_pool)
    .await
    .map_err(map_db_error)?;

    Ok("User created successfully".to_owned())
}

pub async fn login(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<ActiveUsers>,
    Json(request_user): Json<models::RequestUser>,
) -> Result<String, (StatusCode, String)> {
    // Emails are typically not case sensitive, so we lowercase them
    let user_email_lowercase = request_user.user_email.to_lowercase();
    let user_option = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE user_email=$1;",
        user_email_lowercase
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(map_db_error)?;

    match user_option {
        Some(user) if bcrypt::verify(request_user.user_password, &user.user_password_hash) => {
            let new_active_user_id = user.user_id;
            let new_active_user_token = jwt::create_jwt()?;

            active_users.lock().await.insert(
                new_active_user_token.to_owned(),
                new_active_user_id.to_owned(),
            );

            Ok(new_active_user_token)
        }
        Some(_) => Err((
            StatusCode::UNAUTHORIZED,
            "Unable to login. The provided password is incorrect".to_owned(),
        )),
        None => Err((
            StatusCode::NOT_FOUND,
            "Unable to login. The user with the provided email address does not exist".to_owned(),
        )),
    }
}

pub async fn logout(
    Extension(active_users): Extension<ActiveUsers>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<String, (StatusCode, String)> {
    auth::remove_active_user(authorization.token().to_owned(), active_users).await?;

    Ok("Successfully logged out".to_owned())
}

pub async fn create_address(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<ActiveUsers>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(address): Json<models::Address>,
) -> Result<String, (StatusCode, String)> {
    let authed_user_id =
        auth::authenticate_user(authorization.token().to_owned(), active_users).await?;

    sqlx::query!(
        "
        INSERT INTO addresses (user_id, unit, street, city, postal_code, state_province, country)
        VALUES ($1, $2, $3, $4, $5, $6, $7);
        ",
        authed_user_id,
        address.unit,
        address.street,
        address.city,
        address.postal_code,
        address.state_province,
        address.country,
    )
    .execute(&db_pool)
    .await
    .map_err(map_db_error)?;

    Ok("Address created successfully".to_owned())
}

pub async fn add_personal_info(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<ActiveUsers>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(personal_info): Json<models::PersonalInfo>,
) -> Result<String, (StatusCode, String)> {
    let authed_user_id =
        auth::authenticate_user(authorization.token().to_owned(), active_users).await?;

    let personal_info_exists = sqlx::query!(
        "SELECT user_id FROM personal_info WHERE user_id = $1",
        authed_user_id
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(map_db_error)?;

    if personal_info_exists.is_none() {
        sqlx::query!(
            "
            INSERT INTO personal_info (user_id, first_name, last_name, gender)
            VALUES ($1, $2, $3, $4)
            ",
            authed_user_id,
            personal_info.first_name,
            personal_info.last_name,
            personal_info.gender,
        )
        .execute(&db_pool)
        .await
        .map_err(map_db_error)?;

        Ok("Personal info created successfully".to_owned())
    } else {
        sqlx::query!(
            "
            UPDATE personal_info
            SET first_name = $1,
                last_name = $2,
                gender = $3
            WHERE user_id = $4
            ",
            personal_info.first_name,
            personal_info.last_name,
            personal_info.gender,
            authed_user_id,
        )
        .execute(&db_pool)
        .await
        .map_err(map_db_error)?;

        Ok("Personal info updated successfully".to_owned())
    }
}

pub async fn add_to_cart(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<ActiveUsers>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(cart_item): Json<models::CartItem>,
) -> Result<String, (StatusCode, String)> {
    let authed_user_id =
        auth::authenticate_user(authorization.token().to_owned(), active_users).await?;

    let quantity_exists = sqlx::query!(
        "SELECT quantity FROM cart_items WHERE user_id = $1 AND product_id = $2",
        authed_user_id,
        cart_item.product_id,
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(map_db_error)?;

    match quantity_exists {
        Some(quantity) => {
            let new_quantity = quantity.quantity + cart_item.quantity;

            sqlx::query!(
                "
                UPDATE cart_items
                SET quantity = $1
                WHERE user_id = $2 AND product_id = $3
                ",
                new_quantity,
                authed_user_id,
                cart_item.product_id,
            )
            .execute(&db_pool)
            .await
            .map_err(map_db_error)?;

            Ok("Cart item updated successfully".to_owned())
        }
        None => {
            sqlx::query!(
                "
                INSERT INTO cart_items (user_id, product_id, quantity)
                VALUES ($1, $2, $3)
                ",
                authed_user_id,
                cart_item.product_id,
                cart_item.quantity,
            )
            .execute(&db_pool)
            .await
            .map_err(map_db_error)?;

            Ok("Cart item added successfully".to_owned())
        }
    }
}

pub async fn create_order(
    Extension(db_pool): Extension<Pool<Sqlite>>,
    Extension(active_users): Extension<ActiveUsers>,
    authorization: TypedHeader<Authorization<Bearer>>,
) -> Result<String, (StatusCode, String)> {
    let authed_user_id =
        auth::authenticate_user(authorization.token().to_owned(), active_users).await?;

    let user_cart_items = sqlx::query_as!(
        models::CartItem,
        "SELECT * FROM cart_items WHERE user_id = $1",
        authed_user_id,
    )
    .fetch_all(&db_pool)
    .await
    .map_err(map_db_error)?;

    if user_cart_items.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Cart is empty".to_owned()));
    }

    let local_time_now = Local::now().naive_local();
    let new_order_id = sqlx::query!(
        "
        INSERT INTO orders (creation_time, order_status)
        VALUES ($1, $2)
        RETURNING order_id
        ",
        local_time_now,
        models::OrderStatus::Pending,
    )
    .fetch_one(&db_pool)
    .await
    .map_err(map_db_error)?
    .order_id;

    // Add cart items to the order and tally up the total cost
    let mut total_cost = 0.0;
    for cart_item in user_cart_items {
        let product_cost_option = sqlx::query!(
            "
            SELECT SUM(cart_items.quantity * products.price) AS product_cost
            FROM cart_items
            INNER JOIN products ON cart_items.product_id = products.product_id
            WHERE cart_items.product_id = $1
            ",
            cart_item.product_id,
        )
        .fetch_one(&db_pool)
        .await
        .map_err(map_db_error)?
        .product_cost;

        // product_cost_option is an Option because it comes from a JOIN sql statement
        // It should, in theory, never be None, but just in case, we handle it
        match product_cost_option {
            Some(product_cost) => total_cost += product_cost,
            None => total_cost += 0.0,
        }

        sqlx::query!(
            "
            INSERT INTO order_items (order_id, product_id, quantity)
            VALUES ($1, $2, $3)
            ",
            new_order_id,
            cart_item.product_id,
            cart_item.quantity,
        )
        .execute(&db_pool)
        .await
        .map_err(map_db_error)?;
    }

    sqlx::query!(
        "UPDATE orders SET total_cost = $1 WHERE order_id = $2",
        total_cost,
        new_order_id,
    )
    .execute(&db_pool)
    .await
    .map_err(map_db_error)?;

    sqlx::query!("DELETE FROM cart_items WHERE user_id = $1", authed_user_id,)
        .execute(&db_pool)
        .await
        .map_err(map_db_error)?;

    Ok(format!(
        "Order created successfully. Order ID: {}",
        new_order_id
    ))
}
