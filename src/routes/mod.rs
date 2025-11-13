use crate::auth::auth_middleware;
use crate::database::AppState;
use crate::routes::auth::{login, register};
use crate::routes::files::{list_files, upload_file};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post};
use tower_http::cors::{Any, CorsLayer};

pub mod auth;
pub mod files;

// Make the function generic over State type
pub fn create_router(db_pool: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    //Public routes, no auth required
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected_routes = Router::new()
        .route("/files", get(list_files))
        .route("/upload", post(upload_file))
        .route("/files/{file_id}", get(files::download_file))
        .route("/files/{file_id}", delete(files::delete_file))
        .route_layer(from_fn_with_state(db_pool.clone(), auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(db_pool)
        .layer(cors)
}
