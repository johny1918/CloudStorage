use crate::error_handler::error::AppError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use serde_json::json;

pub mod error;

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TokenValidation => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::FileSystem(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::FileTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            AppError::InvalidFileType => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            AppError::Database(e) => format!("Database error: {}", e),
            AppError::Unauthorized(msg) => msg.clone(),
            AppError::InvalidCredentials => "Invalid username or password".to_string(),
            AppError::TokenCreation => "Failed to create authentication token".to_string(),
            AppError::TokenValidation => "Invalid or expired token".to_string(),
            AppError::BadRequest(msg) => msg.clone(),
            AppError::NotFound(msg) => msg.clone(),
            AppError::FileSystem(msg) => msg.clone(),
            AppError::FileTooLarge => "File size exceeds limit".to_string(),
            AppError::InvalidFileType => "File type not allowed".to_string(),
            AppError::Internal(msg) => format!("Internal server error: {}", msg),
        }
    }
}

// Convert AppError to HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.message();

        let body = Json(json!({
            "status": "error",
            "message": message,
            "code": status.as_u16()
        }));

        (status, body).into_response()
    }
}

// Automatic conversions from other error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<BcryptError> for AppError {
    fn from(err: BcryptError) -> Self {
        AppError::Internal(format!("Password hashing error: {}", err))
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        AppError::Internal(format!("JWT error: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem(format!("IO error: {}", err))
    }
}
