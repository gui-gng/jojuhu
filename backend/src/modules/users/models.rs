use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Public user profile information
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub followers_count: i64,
    pub following_count: i64,
    pub posts_count: i64,
    pub is_following: bool,
    pub mutual_friends_count: i64,
}

/// User profile for the authenticated user (includes private info)
#[derive(Debug, Serialize)]
pub struct MyProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub followers_count: i64,
    pub following_count: i64,
    pub posts_count: i64,
}

/// Request to update user profile
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub is_private: Option<bool>,
}

/// Request to update avatar
#[derive(Debug, Deserialize)]
pub struct UpdateAvatarRequest {
    pub avatar_url: String,
}

/// Simple user info for lists
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub is_following: bool,
    pub mutual_friends_count: i64,
}

/// Paginated list of users
#[derive(Debug, Serialize)]
pub struct UsersListResponse {
    pub users: Vec<UserInfo>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct VerificationRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl UpdateProfileRequest {
    /// Validate the update request
    pub fn validate(&self) -> Result<(), String> {
        // Validate display_name length
        if let Some(ref name) = self.display_name {
            if name.len() > 100 {
                return Err("Display name must be at most 100 characters".to_string());
            }
        }

        // Validate bio length
        if let Some(ref bio) = self.bio {
            if bio.len() > 500 {
                return Err("Bio must be at most 500 characters".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_profile_validation() {
        let request = UpdateProfileRequest {
            display_name: Some("a".repeat(101)),
            bio: None,
            is_private: None,
        };
        assert!(request.validate().is_err());

        let request = UpdateProfileRequest {
            display_name: Some("Valid Name".to_string()),
            bio: Some("a".repeat(501)),
            is_private: None,
        };
        assert!(request.validate().is_err());

        let request = UpdateProfileRequest {
            display_name: Some("Valid Name".to_string()),
            bio: Some("Valid bio".to_string()),
            is_private: Some(false),
        };
        assert!(request.validate().is_ok());
    }
}
