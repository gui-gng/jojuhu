//! User models
//!
//! This module defines user-related data structures including the User entity,
//! creation requests, and response DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core user entity representing a registered user in the system
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// Unique identifier (UUID v4)
    pub id: Uuid,
    /// Unique username (alphanumeric + underscores, 3-32 chars)
    pub username: String,
    /// Verified email address
    pub email: String,
    /// Hashed password (never serialized in responses)
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// Optional display name (shown instead of username if set)
    pub display_name: Option<String>,
    /// User's bio or description
    pub bio: Option<String>,
    /// URL to user's avatar image
    pub avatar_url: Option<String>,
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Request to create a new user account
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    /// Desired username (must be unique)
    pub username: String,
    /// Email address (must be unique and valid)
    pub email: String,
    /// Plain-text password (will be hashed)
    pub password: String,
    /// Optional display name
    pub display_name: Option<String>,
}

/// Request to update user profile information
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    /// New display name (optional)
    pub display_name: Option<String>,
    /// New bio text (optional)
    pub bio: Option<String>,
    /// New avatar URL (optional)
    pub avatar_url: Option<String>,
}

/// User response DTO (excludes sensitive fields like password_hash)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            created_at: user.created_at,
        }
    }
}
