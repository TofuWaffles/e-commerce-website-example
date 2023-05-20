mod get_handlers;
mod post_handlers;

use axum::{
    extract::Extension,
    http::{StatusCode, Uri},
    routing::{get, get_service, post},
    Router,
};
use http::{Method, header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE}};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

pub async fn create_router() -> Router {
    // Create the database pool
    let db_pool = create_db_pool().await;

    // The Cors Layer tells the client what methods are supported and from where
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // A hashmap to hold users that are currently logged in
    let active_users = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    Router::new()
        .route("/create_user", post(post_handlers::create_user))
        .route("/login", post(post_handlers::login))
        .route("/logout", post(post_handlers::logout))
        .route("/get_products", get(get_handlers::get_products))
        .route("/get_addresses", get(get_handlers::get_addresses))
        .route("/create_address", post(post_handlers::create_address))
        .route("/get_personal_info", get(get_handlers::get_personal_info))
        .route("/add_personal_info", post(post_handlers::add_personal_info))
        .route("/get_cart", get(get_handlers::get_cart))
        .route("/add_to_cart", post(post_handlers::add_to_cart))
        // .route("/get_orders", get(get_handlers::get_orders))
        .route("/orders/:order_id", get(get_handlers::get_order_items))
        .route("/create_order", post(post_handlers::create_order))
        .nest_service("/", get_service(ServeDir::new("assets/pages")))
        .nest_service("/images", get_service(ServeDir::new("assets/images")))
        .nest_service("/scripts", get_service(ServeDir::new("assets/scripts")))
        .layer(Extension(active_users))
        .layer(Extension(db_pool))
        .layer(cors)
        .fallback(handler_404) // Fallback for get requests for pages that don't exist
}

async fn create_db_pool() -> Pool<Sqlite> {
    // Load the environment variables from the .env file
    dotenv::dotenv().expect("unable to load .env file");

    // Grab the database URL from the env var file
    let db_url = env::var("DATABASE_URL").expect("Missing `DATABASE_URL` env var.");

    if !Sqlite::database_exists(&db_url).await.unwrap() {
        println!("Creating databse {:?}", &db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Successfully created the database"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    // Create the database pool
    SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Unable to create the database pool")
}

async fn handler_404(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!(
            "Oi, what are you doin' snoopin' around here? {} doesn't exist, mate!",
            uri
        ),
    )
}

// Maps database errors to a status code and a message
fn map_db_error(error: sqlx::Error) -> (StatusCode, String) {
    let error_message = format!("Database error: {}", error);
    eprint!("{}", error_message);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong. Please try again later".to_owned(),
    )
}
