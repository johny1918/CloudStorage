use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub status: String,
    pub token: String,
    pub username: String,
}