use sea_orm::*;
use crate::entities::{users, prelude::Users};
use crate::dto::user::{UserResponse, CreateUserRequest, UsersListResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub struct UserRepository;

impl UserRepository {
    // Get all users
    pub async fn find_all(db: &DatabaseConnection) -> Result<UsersListResponse, DbErr> {
        let users_list = Users::find().all(db).await?;
        
        let users: Vec<UserResponse> = users_list
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                role: user.role,
                is_active: user.is_active,
                created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
            .collect();
        
        let total = users.len();
        
        Ok(UsersListResponse { users, total })
    }
    
    // Get user by ID
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<UserResponse>, DbErr> {
        let user = Users::find_by_id(id).one(db).await?;
        
        Ok(user.map(|u| UserResponse {
            id: u.id,
            email: u.email,
            full_name: u.full_name,
            role: u.role,
            is_active: u.is_active,
            created_at: u.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }))
    }
    
    // Create new user
    pub async fn create(db: &DatabaseConnection, data: CreateUserRequest) -> Result<UserResponse, DbErr> {
        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(data.password.as_bytes(), &salt)
            .map_err(|_| DbErr::Custom("Failed to hash password".to_string()))?
            .to_string();
        
        // Create user
        let user = users::ActiveModel {
            email: Set(data.email),
            password_hash: Set(password_hash),
            full_name: Set(data.full_name),
            role: Set(data.role),
            is_active: Set(true),
            ..Default::default()
        };
        
        let result = user.insert(db).await?;
        
        Ok(UserResponse {
            id: result.id,
            email: result.email,
            full_name: result.full_name,
            role: result.role,
            is_active: result.is_active,
            created_at: result.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }
    
    // Delete user
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool, DbErr> {
        let user = Users::find_by_id(id).one(db).await?;
        
        match user {
            Some(user) => {
                let user: users::ActiveModel = user.into();
                user.delete(db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}