use serde::{Deserialize, Serialize};
use validator::Validate;

// Response DTO - what we send to clients
#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: String,
}

// Request DTO - what clients send to create a user
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(length(min = 2, message = "Full name must be at least 2 characters"))]
    pub full_name: String,
    
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: String,
}

// List response
#[derive(Debug, Serialize)]
pub struct UsersListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
}