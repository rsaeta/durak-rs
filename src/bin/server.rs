use axum::Router;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use durak_rt::server::api::create_api_router;
use durak_rt::server::GameSessions;

#[tokio::main]
async fn main() {
    // Initialize game sessions storage
    let sessions: GameSessions = Arc::new(RwLock::new(HashMap::new()));

    // Create API router
    let api_router = create_api_router(sessions);

    // Create main router with static file serving and CORS
    let app = Router::new()
        .nest("/api", api_router)
        .nest_service("/", ServeDir::new("webapp"))
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
