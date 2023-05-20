use chrono::naive::NaiveDateTime;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Used when a new user signs up to create a new user in the DB
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub user_email: String,
    pub user_password: String,
}

// Used when a user logs in to validate the user's credentials
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUser {
    pub user_email: String,
    pub user_password: String,
}

// An exact replica of the users table in the DB so make accessing the table easier
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub user_email: String,
    pub user_password_hash: String,
}

impl User {
    pub fn new(new_user: &NewUser) -> User {
        User {
            user_id: Uuid::new_v4().simple().to_string(),
            username: new_user.username.clone(),
            user_email: new_user.user_email.clone().to_lowercase(),
            user_password_hash: bcrypt::hash(new_user.user_password.clone()).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub address_id: i64,
    pub unit: String,
    pub street: String,
    pub city: String,
    pub postal_code: i64,
    pub state_province: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub order_id: i64,
    // pub address_id: i64,
    pub creation_time: NaiveDateTime,
    pub total_cost: Option<f64>,
    pub order_status: OrderStatus,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum ProductCategory {
    Meat,
    Seafood,
    Vegetable,
    Fruit,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub product_id: i64,
    pub product_name: String,
    pub product_description: Option<String>,
    pub product_category: ProductCategory,
    pub stock: i64,
    pub price: f64,
    pub img_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    #[serde(skip_deserializing)]
    pub user_id: String,
    pub product_id: i64,
    pub quantity: i64,
}

// Used to show each cart item to the user
// Grabbed from joining the cart_items and products tables
#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayCartItem {
    pub product_name: String,
    pub price: f64,
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i64,
}
