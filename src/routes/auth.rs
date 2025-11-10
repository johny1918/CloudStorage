use crate::auth::hash_password;
use crate::database::AppState;
use crate::models::user::{RegisterUser};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::json;

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
