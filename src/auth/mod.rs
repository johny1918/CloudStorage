pub mod middleware;

use crate::models::auth::Claims;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
pub use middleware::{AuthUser, auth_middleware};
use uuid::Uuid;
use crate::error_handler::error::AppError;

const JWT_EXPIRATION_HOURS: i64 = 24; // Token valid for 24 hours

pub fn create_jwt(user_id: Uuid, username: &str) -> Result<String, AppError> {
    dotenv::dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").map_err(|_| AppError::Internal("JWT_SECRET must be set".to_string()))?;

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(JWT_EXPIRATION_HOURS))
        .expect("Failed to calculate expiration date")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        username: username.to_string(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    dotenv::dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").map_err(|_| AppError::Internal("DATABASE_URL must be set".to_string()))?;

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;

    Ok(token_data.claims)
}
pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(AppError::from)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash).map_err(AppError::from)
}
