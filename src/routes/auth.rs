use crate::auth::{create_jwt, hash_password, verify_password};
use crate::database::AppState;
use crate::error_handler::error::AppError;
use crate::models::auth::LoginResponse;
use crate::models::user::{LoginUser, RegisterUser};
use axum::Json;
use axum::extract::State;
use serde_json::json;

pub async fn register(
    State(state): State<AppState>,
    Json(user_data): Json<RegisterUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    let password_hash = hash_password(&user_data.password)?;

    sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
        user_data.username,
        user_data.email,
        password_hash,
    )
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "status": "success",
        "message": format!("User {} created successfully", user_data.username)
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(login_data): Json<LoginUser>,
) -> Result<Json<LoginResponse>, AppError> {
    let user_result = sqlx::query!(
        "SELECT id, username, email, password_hash FROM users WHERE username = $1",
        login_data.username
    )
    .fetch_optional(&state.db)
    .await?;

    let user = user_result.ok_or(AppError::InvalidCredentials)?;

    let is_valid = verify_password(&login_data.password, &user.password_hash)?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = create_jwt(user.id, &user.username)?;

    Ok(Json(LoginResponse {
        status: "success".to_string(),
        token,
        username: user.username,
    }))
}
