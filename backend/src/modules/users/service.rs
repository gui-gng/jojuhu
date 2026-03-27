use uuid::Uuid;

use crate::errors::AppError;

use super::{
    models::{MyProfile, UpdateProfileRequest, UserProfile, UsersListResponse},
    repository::UserRepository,
};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    /// Get user profile by ID
    pub async fn get_user_profile(
        &self,
        user_id: Uuid,
        current_user_id: Option<Uuid>,
    ) -> Result<UserProfile, AppError> {
        self.repository
            .get_user_profile(user_id, current_user_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
    }

    /// Get authenticated user's profile
    pub async fn get_my_profile(&self,
        user_id: Uuid,
    ) -> Result<MyProfile, AppError> {
        self.repository
            .get_my_profile(user_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
    }

    /// Update user profile
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        request: UpdateProfileRequest,
    ) -> Result<(), AppError> {
        // Validate input
        request.validate().map_err(AppError::ValidationError)?;

        // Sanitize input
        let sanitized_request = UpdateProfileRequest {
            display_name: request.display_name.map(|s| s.trim().to_string()),
            bio: request.bio.map(|s| s.trim().to_string()),
            is_private: request.is_private,
        };

        self.repository.update_profile(user_id, &sanitized_request).await
    }

    /// Update user avatar
    pub async fn update_avatar(
        &self,
        user_id: Uuid,
        avatar_url: String,
    ) -> Result<(), AppError> {
        // Validate URL format
        if !avatar_url.starts_with("http://") && !avatar_url.starts_with("https://") {
            return Err(AppError::ValidationError(
                "Invalid avatar URL format".to_string(),
            ));
        }

        self.repository.update_avatar(user_id, &avatar_url).await
    }

    /// Follow a user
    pub async fn follow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, AppError> {
        // Prevent self-following
        if follower_id == following_id {
            return Err(AppError::ValidationError(
                "Cannot follow yourself".to_string(),
            ));
        }

        let success = self.repository.follow_user(follower_id, following_id).await?;

        if !success {
            return Err(AppError::ValidationError(
                "Already following this user".to_string(),
            ));
        }

        Ok(true)
    }

    /// Unfollow a user
    pub async fn unfollow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, AppError> {
        let success = self
            .repository
            .unfollow_user(follower_id, following_id)
            .await?;

        if !success {
            return Err(AppError::ValidationError(
                "Not following this user".to_string(),
            ));
        }

        Ok(true)
    }

    /// Get followers list
    pub async fn get_followers(
        &self,
        user_id: Uuid,
        current_user_id: Option<Uuid>,
        page: i64,
        per_page: i64,
    ) -> Result<UsersListResponse, AppError> {
        // Validate pagination
        if page < 1 {
            return Err(AppError::ValidationError(
                "Page must be at least 1".to_string(),
            ));
        }
        if !(1..=100).contains(&per_page) {
            return Err(AppError::ValidationError(
                "Per page must be between 1 and 100".to_string(),
            ));
        }

        let (users, total) = self
            .repository
            .get_followers(user_id, current_user_id, page, per_page)
            .await?;

        let has_more = total > page * per_page;

        Ok(UsersListResponse {
            users,
            total,
            page,
            per_page,
            has_more,
        })
    }

    /// Get following list
    pub async fn get_following(
        &self,
        user_id: Uuid,
        current_user_id: Option<Uuid>,
        page: i64,
        per_page: i64,
    ) -> Result<UsersListResponse, AppError> {
        // Validate pagination
        if page < 1 {
            return Err(AppError::ValidationError(
                "Page must be at least 1".to_string(),
            ));
        }
        if !(1..=100).contains(&per_page) {
            return Err(AppError::ValidationError(
                "Per page must be between 1 and 100".to_string(),
            ));
        }

        let (users, total) = self
            .repository
            .get_following(user_id, current_user_id, page, per_page)
            .await?;

        let has_more = total > page * per_page;

        Ok(UsersListResponse {
            users,
            total,
            page,
            per_page,
            has_more,
        })
    }

    #[allow(dead_code)]
    /// Check if a user is following another
    pub async fn is_following(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, AppError> {
        self.repository.is_following(follower_id, following_id).await
    }

    /// Block a user
    pub async fn block_user(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
    ) -> Result<bool, AppError> {
        if blocker_id == blocked_id {
            return Err(AppError::ValidationError(
                "Cannot block yourself".to_string(),
            ));
        }
        self.repository.block_user(blocker_id, blocked_id).await
    }

    /// Unblock a user
    pub async fn unblock_user(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
    ) -> Result<(), AppError> {
        self.repository.unblock_user(blocker_id, blocked_id).await
    }

    /// Check if a user is blocked
    pub async fn is_blocked(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
    ) -> Result<bool, AppError> {
        self.repository.is_blocked(blocker_id, blocked_id).await
    }

    /// Get blocked users list
    pub async fn get_blocked_users(
        &self,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> Result<UsersListResponse, AppError> {
        let (users, total) = self.repository.get_blocked_users(user_id, page, per_page).await?;

        let has_more = (page * per_page) < total;

        Ok(UsersListResponse {
            users,
            total,
            page,
            per_page,
            has_more,
        })
    }
}
