use crate::auth::verify_jwt;
use crate::error_handler::error::AppError;
use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
}

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, AppError> {
    let token = extract_token_from_headers(request.headers())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization token".to_string()))?;

    let claims = verify_jwt(&token).map_err(|_| AppError::TokenValidation)?;

    let auth_user = AuthUser {
        user_id: claims.sub,
        username: claims.username,
    };
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("authorization")?.to_str().ok()?;
    auth_header.strip_prefix("Bearer ").map(|stripped| stripped.to_string())
}
