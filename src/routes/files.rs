use crate::auth::AuthUser;
use crate::database::AppState;
use axum::extract::Extension;
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;

pub async fn list_files(
    State(_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Ok(Json(json!({
        "status": "success",
        "message": format!("Showing files for user: {}", auth_user.username),
        "user_id": auth_user.user_id.to_string(),
        "files": [] //TBD
    })))
}

pub async fn upload_file(
    State(_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Ok(Json(json!({
        "status": "success",
        "message": format!("Upload endpoint for user: {}", auth_user.username),
        "user_id": auth_user.user_id.to_string()
    })))
}