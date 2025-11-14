mod auth;
mod database;
mod error_handler;
mod models;
mod routes;

use crate::database::{AppState, connect_db};
use crate::routes::create_router;
use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let db_pool = connect_db().await.expect("Failed to connect to database");
    let app_state = AppState::new(db_pool);
    let api_router = create_router(app_state);

    let app = Router::new()
        // Mount API routes
        .nest("/api", api_router)
        // Serve static files for everything else
        .fallback_service(ServeDir::new("static"));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);
    println!("📁 Static files served from ./static");
    println!("📄 Access the web interface at http://localhost:3000");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .expect("Failed to start server");
}
