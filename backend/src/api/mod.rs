use axum::{routing::get, Router, Json, extract::State};
use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Serialize)]
struct ApiInfo {
    name: String,
    version: String,
    description: String,
    database_connected: bool,
}

async fn api_info(State(db): State<DatabaseConnection>) -> Json<ApiInfo> {
    // Test if database is connected by pinging it
    let db_connected = db.ping().await.is_ok();
    
    Json(ApiInfo {
        name: "rsEdu API".to_string(),
        version: "0.1.0".to_string(),
        description: "School Management System API".to_string(),
        database_connected: db_connected,
    })
}

pub fn routes() -> Router<DatabaseConnection> {
    tracing::info!("📋 Registering API routes");
    Router::new()
        .route("/info", get(api_info))
}