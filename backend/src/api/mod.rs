use axum::{routing::get, Router, Json};
use serde::Serialize;

#[derive(Serialize)]
struct ApiInfo {
    name: String,
    version: String,
    description: String,
}

async fn api_info() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "rsEdu API".to_string(),
        version: "0.1.0".to_string(),
        description: "School Management System API".to_string(),
    })
}

pub fn routes() -> Router {
    tracing::info!("📋 Registering API routes");
    Router::new()
        .route("/info", get(api_info))
        // Future routes will go here:
        // .route("/students", get(list_students))
        // .route("/teachers", get(list_teachers))
        // .route("/attendance", get(get_attendance))
}
