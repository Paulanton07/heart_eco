use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use argon2::{self, Config};
use rand;

/// User roles for authorization
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Customer,
    Admin,
}

/// Full user details as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Used for user registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}

/// Used for login attempts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

impl User {
    /// Create a new user with hashed password
    pub fn new(new_user: NewUser, role: UserRole) -> Result<Self, argon2::Error> {
        let salt = rand::random::<[u8; 32]>();
        let config = Config::default();
        
        let password_hash = argon2::hash_encoded(
            new_user.password.as_bytes(),
            &salt,
            &config,
        )?;
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            email: new_user.email,
            password_hash,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            role,
            phone: new_user.phone,
            created_at: now,
            updated_at: now,
        })
    }
    
    /// Verify user password
    pub fn verify_password(&self, password: &str) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(&self.password_hash, password.as_bytes())
    }
    
    /// Get user's full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    /// Check if user is an admin
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
}

