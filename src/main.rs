mod auth;
mod database;
mod models;
mod routes;

use crate::database::{AppState, connect_db};
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    let db_pool = connect_db().await.expect("Failed to connect to database");
    let app_state = AppState::new(db_pool);
    let app = create_router(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port");
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
