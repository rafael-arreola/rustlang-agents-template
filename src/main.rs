mod api;
mod infra;
mod tools;
mod agents;
mod state;

use std::sync::Arc;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 1. Initialize Tracing (Logging)
    infra::telemetry::init_tracing();

    // 2. Initialize Orchestrator (It handles its own models internally)
    let orchestrator = agents::orchestrator::Orchestrator::new();

    // 3. Initialize State (Shared Memory)
    let state = Arc::new(state::AppState::new(orchestrator));

    // 4. Setup Router (The Nervous System)
    let app = api::routes::app_router(state);

    // 5. Start Server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{}", port);
    
    tracing::info!("🚀 Server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}