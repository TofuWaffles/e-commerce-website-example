use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{routes::ActiveUsers, utils::jwt};

pub async fn authenticate_user(
    token: String,
    active_users: ActiveUsers,
) -> Result<String, (StatusCode, String)> {
    jwt::is_valid(&token)?;

    match active_users.lock().await.get(&token) {
        Some(user_id) => Ok(user_id.to_string()),
        None => Err((
            StatusCode::UNAUTHORIZED,
            "A user was not found for the given session token. Please login again".to_owned(),
        )),
    }
}

pub async fn remove_active_user(
    token: String,
    active_users: ActiveUsers,
) -> Result<(), (StatusCode, String)> {
    match active_users.lock().await.remove(&token) {
        Some(_) => Ok(()),
        None => Err((
            StatusCode::UNAUTHORIZED,
            "A user was not found for the given session token.".to_owned(),
        )),
    }
}

pub async fn check_user_exists(
    db_pool: &Pool<Sqlite>,
    email: &String,
) -> Result<bool, (StatusCode, String)> {
    let user_exists_result = sqlx::query!(
        "SELECT username FROM users WHERE user_email=$1 LIMIT 1;",
        email
    )
    .fetch_optional(db_pool)
    .await;

    match user_exists_result {
        Ok(user_exists) => Ok(user_exists.is_some()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to check if the user exists".to_owned(),
        )),
    }
}
