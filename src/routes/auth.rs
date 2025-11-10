use crate::auth::{create_jwt, hash_password, verify_password};
use crate::database::AppState;
use crate::models::user::{LoginUser, RegisterUser, UserLogin};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::json;
use crate::models::auth::LoginResponse;

pub async fn register(
    State(state): State<AppState>,
    Json(user_data): Json<RegisterUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let password_hash = hash_password(user_data.password.as_str())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to hash password: {}", e)))?;

    sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
        user_data.username,
        user_data.email,
        password_hash,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ 
        "status": "success",
        "message": format!("User {} created successfully", user_data.username)
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(login_data): Json<LoginUser>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {

    let user_result = sqlx::query!(
        "SELECT id, username, email, password_hash FROM users WHERE username = $1",
        login_data.username
    )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = user_result
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()))?;

    let is_valid = verify_password(&login_data.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Password verification failed: {}", e)))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()));
    }

    let token = create_jwt(user.id, &user.username)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JWT creation failed: {}", e)))?;

    // Step 5: Return success response
    Ok(Json(LoginResponse {
        status: "success".to_string(),
        token,
        username: user.username,
    }))
}
