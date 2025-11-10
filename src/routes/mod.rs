use crate::database::AppState;
use crate::routes::auth::{login, register};
use axum::Router;
use axum::routing::post;
use tower_http::cors::{Any, CorsLayer};

pub mod auth;
pub mod files;

// Make the function generic over State type
pub fn create_router(db_pool: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(db_pool)
        .layer(cors)
}
