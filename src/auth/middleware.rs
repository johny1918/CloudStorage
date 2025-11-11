use crate::auth::verify_jwt;
use crate::database::AppState;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
}

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let token = extract_token_from_headers(request.headers()).ok_or((
        StatusCode::UNAUTHORIZED,
        "Missing authorization token".to_string(),
    ))?;
    let claims = verify_jwt(&token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid or expired token".to_string(),
        )
    })?;
    let auth_user = AuthUser {
        user_id: claims.sub,
        username: claims.username,
    };
    request.extensions_mut().insert(auth_user);
    Ok(next.run(request).await)
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("authorization")?.to_str().ok()?;

    if let Some(stripped) = auth_header.strip_prefix("Bearer ") {
        Some(stripped.to_string())
    } else {
        None
    }
}
