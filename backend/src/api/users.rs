use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::dto::user::{CreateUserRequest, UserResponse, UsersListResponse};
use crate::repositories::user_repository::UserRepository;

// GET /api/v1/users - List all users
pub async fn list_users(
    State(db): State<DatabaseConnection>,
) -> Result<Json<UsersListResponse>, StatusCode> {
    match UserRepository::find_all(&db).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// GET /api/v1/users/:id - Get user by ID
pub async fn get_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, StatusCode> {
    match UserRepository::find_by_id(&db, id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// POST /api/v1/users - Create new user
pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, String)> {
    // Validate input
    if let Err(e) = payload.validate() {
        tracing::error!("Validation error: {:?}", e);
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    match UserRepository::create(&db, payload).await {
        Ok(user) => {
            tracing::info!("User created: {}", user.email);
            Ok((StatusCode::CREATED, Json(user)))
        }
        Err(e) => {
            tracing::error!("Database error creating user: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))
        }
    }
}

// DELETE /api/v1/users/:id - Delete user
pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match UserRepository::delete(&db, id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}