use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
}
enum EncodeDecode {
    Encode(EncodingKey),
    Decode(DecodingKey),
}

pub fn create_jwt() -> Result<String, (StatusCode, String)> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::days(1);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claims = Claims { exp, iat };

    let EncodeDecode::Encode(key) = get_secret_key(true) else {
        unreachable!()
    };

    let token = encode(&Header::default(), &claims, &key);

    match token {
        Ok(token_string) => Ok(token_string),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unable to create JWT token: {}", err),
        )),
    }
}

pub fn is_valid(token: &str) -> Result<(), (StatusCode, String)> {
    let EncodeDecode::Decode(key) = get_secret_key(false) else {
        unreachable!()
    };

    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => (
                StatusCode::UNAUTHORIZED,
                "Your login token has expired. Please login again.".to_owned(),
            ),
            _ => (
                StatusCode::UNAUTHORIZED,
                "The token provided is invalid. Please login again".to_owned(),
            ),
        },
    )?;

    Ok(())
}

// Pass in true for encode, false for decode
fn get_secret_key(encode_or_decode: bool) -> EncodeDecode {
    dotenv::dotenv().expect("unable to load .env file");
    let secret = &env::var("JWT_SECRET").unwrap();

    match encode_or_decode {
        true => EncodeDecode::Encode(EncodingKey::from_secret(secret.as_bytes())),
        false => EncodeDecode::Decode(DecodingKey::from_secret(secret.as_bytes())),
    }
}
