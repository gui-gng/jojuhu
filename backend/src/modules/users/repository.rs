use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{MyProfile, UpdateProfileRequest, UserInfo, UserProfile};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get user profile with counts
    pub async fn get_user_profile(
        &self,
        user_id: Uuid,
        current_user_id: Option<Uuid>,
    ) -> Result<Option<UserProfile>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT 
                u.id,
                u.username,
                u.display_name,
                u.bio,
                u.avatar_url,
                u.created_at,
                COALESCE(followers.count, 0) as followers_count,
                COALESCE(following.count, 0) as following_count,
                COALESCE(posts.count, 0) as posts_count,
                CASE 
                    WHEN $2::uuid IS NOT NULL THEN
                        EXISTS(
                            SELECT 1 FROM follows f 
                            WHERE f.follower_id = $2 AND f.following_id = u.id
                        )
                    ELSE FALSE
                END as is_following,
                COALESCE(mutual.count, 0) as mutual_friends_count
            FROM users u
            LEFT JOIN (
                SELECT following_id, COUNT(*) as count 
                FROM follows 
                GROUP BY following_id
            ) followers ON followers.following_id = u.id
            LEFT JOIN (
                SELECT follower_id, COUNT(*) as count 
                FROM follows 
                GROUP BY follower_id
            ) following ON following.follower_id = u.id
            LEFT JOIN (
                SELECT author_id, COUNT(*) as count 
                FROM posts 
                GROUP BY author_id
            ) posts ON posts.author_id = u.id
            LEFT JOIN (
                SELECT COUNT(*) as count
                FROM follows f1
                WHERE f1.follower_id = $1
                AND EXISTS(
                    SELECT 1 FROM follows f2 
                    WHERE f2.follower_id = $2 AND f2.following_id = f1.following_id
                )
            ) mutual ON $2::uuid IS NOT NULL
            WHERE u.id = $1
            "#,
        )
        .bind(user_id)
        .bind(current_user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        match result {
            Some(row) => Ok(Some(UserProfile {
                id: row.get("id"),
                username: row.get("username"),
                display_name: row.get("display_name"),
                bio: row.get("bio"),
                avatar_url: row.get("avatar_url"),
                created_at: row.get("created_at"),
                followers_count: row.get("followers_count"),
                following_count: row.get("following_count"),
                posts_count: row.get("posts_count"),
                is_following: row.get("is_following"),
                mutual_friends_count: row.get("mutual_friends_count"),
            })),
            None => Ok(None),
        }
    }

    /// Get my profile (authenticated user) with counts
    pub async fn get_my_profile(&self,
        user_id: Uuid,
    ) -> Result<Option<MyProfile>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT 
                u.id,
                u.username,
                u.email,
                u.display_name,
                u.bio,
                u.avatar_url,
                u.created_at,
                COALESCE(followers.count, 0) as followers_count,
                COALESCE(following.count, 0) as following_count,
                COALESCE(posts.count, 0) as posts_count
            FROM users u
            LEFT JOIN (
                SELECT following_id, COUNT(*) as count 
                FROM follows 
                GROUP BY following_id
            ) followers ON followers.following_id = u.id
            LEFT JOIN (
                SELECT follower_id, COUNT(*) as count 
                FROM follows 
                GROUP BY follower_id
            ) following ON following.follower_id = u.id
            LEFT JOIN (
                SELECT author_id, COUNT(*) as count 
                FROM posts 
                GROUP BY author_id
            ) posts ON posts.author_id = u.id
            WHERE u.id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        match result {
            Some(row) => Ok(Some(MyProfile {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                display_name: row.get("display_name"),
                bio: row.get("bio"),
                avatar_url: row.get("avatar_url"),
                created_at: row.get("created_at"),
                followers_count: row.get("followers_count"),
                following_count: row.get("following_count"),
                posts_count: row.get("posts_count"),
            })),
            None => Ok(None),
        }
    }

    /// Update user profile
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        request: &UpdateProfileRequest,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE users 
            SET 
                display_name = COALESCE($1, display_name),
                bio = COALESCE($2, bio),
                is_private = COALESCE($3, is_private),
                updated_at = NOW()
            WHERE id = $4
            "#,
        )
        .bind(&request.display_name)
        .bind(&request.bio)
        .bind(request.is_private)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    /// Update avatar URL
    pub async fn update_avatar(&self, user_id: Uuid, avatar_url: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE users 
            SET avatar_url = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(avatar_url)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    /// Follow a user
    pub async fn follow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, AppError> {
        // Prevent self-following
        if follower_id == following_id {
            return Ok(false);
        }

        let result = sqlx::query(
            r#"
            INSERT INTO follows (follower_id, following_id)
            VALUES ($1, $2)
            ON CONFLICT (follower_id, following_id) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result.is_some())
    }

    /// Unfollow a user
    pub async fn unfollow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM follows 
            WHERE follower_id = $1 AND following_id = $2
            RETURNING id
            "#,
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result.is_some())
    }

    #[allow(dead_code)]
    /// Check if user is following another user
    pub async fn is_following(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM follows 
                WHERE follower_id = $1 AND following_id = $2
            )
            "#,
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result)
    }

    /// Get followers list
    pub async fn get_followers(
        &self,
        user_id: Uuid,
        current_user_id: Option<Uuid>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<UserInfo>, i64), AppError> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(
            r#"
            SELECT 
                u.id,
                u.username,
                u.display_name,
                u.avatar_url,
                CASE 
                    WHEN $3::uuid IS NOT NULL THEN
                        EXISTS(
                            SELECT 1 FROM follows f2 
                            WHERE f2.follower_id = $3 AND f2.following_id = u.id
                        )
                    ELSE FALSE
                END as is_following,
                COALESCE(mutual.count, 0) as mutual_friends_count
            FROM follows f
            JOIN users u ON u.id = f.follower_id
            LEFT JOIN (
                SELECT f1.following_id as user_id, COUNT(*) as count
                FROM follows f1
                WHERE f1.follower_id = $1
                AND EXISTS(
                    SELECT 1 FROM follows f2 
                    WHERE f2.follower_id = $3 AND f2.following_id = f1.following_id
                )
                GROUP BY f1.following_id
            ) mutual ON $3::uuid IS NOT NULL AND mutual.user_id = u.id
            WHERE f.following_id = $1
            ORDER BY f.created_at DESC
            LIMIT $2 OFFSET $4
            "#,
        )
        .bind(user_id)
        .bind(per_page)
        .bind(current_user_id)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let users: Vec<UserInfo> = rows
            .into_iter()
            .map(|row| UserInfo {
                id: row.get("id"),
                username: row.get("username"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
                is_following: row.get("is_following"),
                mutual_friends_count: row.get("mutual_friends_count"),
            })
            .collect();

        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM follows WHERE following_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok((users, total))
    }

    /// Get following list
    pub async fn get_following(
        &self,
        user_id: Uuid,
        current_user_id: Option<Uuid>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<UserInfo>, i64), AppError> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(
            r#"
            SELECT 
                u.id,
                u.username,
                u.display_name,
                u.avatar_url,
                CASE 
                    WHEN $3::uuid IS NOT NULL THEN
                        EXISTS(
                            SELECT 1 FROM follows f2 
                            WHERE f2.follower_id = $3 AND f2.following_id = u.id
                        )
                    ELSE FALSE
                END as is_following,
                COALESCE(mutual.count, 0) as mutual_friends_count
            FROM follows f
            JOIN users u ON u.id = f.following_id
            LEFT JOIN (
                SELECT f1.following_id as user_id, COUNT(*) as count
                FROM follows f1
                WHERE f1.follower_id = $1
                AND EXISTS(
                    SELECT 1 FROM follows f2 
                    WHERE f2.follower_id = $3 AND f2.following_id = f1.following_id
                )
                GROUP BY f1.following_id
            ) mutual ON $3::uuid IS NOT NULL AND mutual.user_id = u.id
            WHERE f.follower_id = $1
            ORDER BY f.created_at DESC
            LIMIT $2 OFFSET $4
            "#,
        )
        .bind(user_id)
        .bind(per_page)
        .bind(current_user_id)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let users: Vec<UserInfo> = rows
            .into_iter()
            .map(|row| UserInfo {
                id: row.get("id"),
                username: row.get("username"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
                is_following: row.get("is_following"),
                mutual_friends_count: row.get("mutual_friends_count"),
            })
            .collect();

        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM follows WHERE follower_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok((users, total))
    }

    /// Block a user
    pub async fn block_user(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<bool, AppError> {
        // Prevent self-blocking
        if blocker_id == blocked_id {
            return Err(AppError::ValidationError(
                "Cannot block yourself".to_string(),
            ));
        }

        let result = sqlx::query(
            "INSERT INTO user_blocks (blocker_id, blocked_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result.rows_affected() > 0)
    }

    /// Unblock a user
    pub async fn unblock_user(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM user_blocks WHERE blocker_id = $1 AND blocked_id = $2"
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    /// Check if a user is blocked
    #[allow(dead_code)]
    pub async fn is_blocked(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM user_blocks 
                WHERE blocker_id = $1 AND blocked_id = $2
            )
            "#,
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result)
    }

    /// Get list of blocked users
    pub async fn get_blocked_users(
        &self,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<UserInfo>, i64), AppError> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(
            r#"
            SELECT 
                u.id,
                u.username,
                u.display_name,
                u.avatar_url,
                FALSE as is_following,
                0 as mutual_friends_count
            FROM user_blocks ub
            JOIN users u ON u.id = ub.blocked_id
            WHERE ub.blocker_id = $1
            ORDER BY ub.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let users: Vec<UserInfo> = rows
            .into_iter()
            .map(|row| UserInfo {
                id: row.get("id"),
                username: row.get("username"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
                is_following: row.get("is_following"),
                mutual_friends_count: row.get("mutual_friends_count"),
            })
            .collect();

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM user_blocks WHERE blocker_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok((users, total))
    }

    /// Get minimal user info by ID (for notifications)
    pub async fn get_profile_by_id(&self, user_id: Uuid) -> Result<UserInfo, AppError> {
        let row = sqlx::query(
            r#"
            SELECT 
                u.id,
                u.username,
                u.display_name,
                u.avatar_url
            FROM users u
            WHERE u.id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(UserInfo {
            id: row.get("id"),
            username: row.get("username"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_following: false,
            mutual_friends_count: 0,
        })
    }

    /// Find user by username
    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserInfo>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT 
                u.id,
                u.username,
                u.display_name,
                u.avatar_url
            FROM users u
            WHERE u.username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        match row {
            Some(r) => Ok(Some(UserInfo {
                id: r.get("id"),
                username: r.get("username"),
                display_name: r.get("display_name"),
                avatar_url: r.get("avatar_url"),
                is_following: false,
                mutual_friends_count: 0,
            })),
            None => Ok(None),
        }
    }
}
