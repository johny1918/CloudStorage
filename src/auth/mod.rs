pub mod middleware;

use crate::models::auth::Claims;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
pub use middleware::{AuthUser, auth_middleware};
use uuid::Uuid;

const JWT_EXPIRATION_HOURS: i64 = 24; // Token valid for 24 hours

pub fn create_jwt(user_id: Uuid, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv::dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").expect("DATABASE_URL must be set");

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(JWT_EXPIRATION_HOURS))
        .expect("Failed to calculate expiration date")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        username: username.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    dotenv::dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").expect("DATABASE_URL must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|v| v.claims)
}
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}
