use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
    version: String,
}

#[tokio::main]
async fn main() {
    // Initialize logging (so we can see what's happening)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rsedu_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Load configuration
    let config = config::Config::from_env()
        .expect("Failed to load configuration");

    tracing::info!("🚀 Starting rsEdu Backend");
    tracing::info!("📚 Environment: {}", config.environment);

    // Build our API routes
    let api_routes = api::routes();  // Get the router from api module
    let app = Router::new()
    .route("/health", get(health_check))
    .route("/test", get(|| async { "Test works!" }))  // Simple test
    .nest("/api/v1", api_routes)
    .layer(TraceLayer::new_for_http());

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("🌐 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

// Health check endpoint - tells us if the server is running
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "rsEdu API is running".to_string(),
        version: "0.1.0".to_string(),
    })
}
