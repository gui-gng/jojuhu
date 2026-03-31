use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{Story, StoryViewer, StoryWithUser};

pub struct StoryRepository {
    pool: PgPool,
}

impl StoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        media_url: &str,
        media_type: &str,
        caption: Option<&str>,
    ) -> Result<Story, AppError> {
        let story = sqlx::query_as::<_, Story>(
            r#"
            INSERT INTO stories (user_id, media_url, media_type, caption)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(media_url)
        .bind(media_type)
        .bind(caption)
        .fetch_one(&self.pool)
        .await?;

        Ok(story)
    }

    pub async fn get_story_with_user(
        &self,
        story_id: Uuid,
        viewer_id: Uuid,
    ) -> Result<StoryWithUser, AppError> {
        let story = sqlx::query_as::<_, StoryWithUser>(
            r#"
            SELECT 
                s.id,
                s.user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                s.media_url,
                s.media_type,
                s.caption,
                s.created_at,
                s.expires_at,
                s.view_count,
                EXISTS(
                    SELECT 1 FROM story_viewers sv 
                    WHERE sv.story_id = s.id AND sv.viewer_id = $2
                ) as is_viewed
            FROM stories s
            JOIN users u ON s.user_id = u.id
            WHERE s.id = $1 AND s.expires_at > NOW()
            "#
        )
        .bind(story_id)
        .bind(viewer_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::NotFoundError("Story not found or expired".to_string()))?;

        Ok(story)
    }

    pub async fn get_active_stories(
        &self,
        user_id: Uuid,
        viewer_id: Uuid,
    ) -> Result<Vec<StoryWithUser>, AppError> {
        let stories = sqlx::query_as::<_, StoryWithUser>(
            r#"
            SELECT 
                s.id,
                s.user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                s.media_url,
                s.media_type,
                s.caption,
                s.created_at,
                s.expires_at,
                s.view_count,
                EXISTS(
                    SELECT 1 FROM story_viewers sv 
                    WHERE sv.story_id = s.id AND sv.viewer_id = $2
                ) as is_viewed
            FROM stories s
            JOIN users u ON s.user_id = u.id
            WHERE s.user_id = $1
            AND s.expires_at > NOW()
            ORDER BY s.created_at DESC
            "#
        )
        .bind(user_id)
        .bind(viewer_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(stories)
    }

    pub async fn get_following_stories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<StoryWithUser>, AppError> {
        let stories = sqlx::query_as::<_, StoryWithUser>(
            r#"
            SELECT 
                s.id,
                s.user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                s.media_url,
                s.media_type,
                s.caption,
                s.created_at,
                s.expires_at,
                s.view_count,
                EXISTS(
                    SELECT 1 FROM story_viewers sv 
                    WHERE sv.story_id = s.id AND sv.viewer_id = $1
                ) as is_viewed
            FROM stories s
            JOIN users u ON s.user_id = u.id
            JOIN follows f ON s.user_id = f.following_id
            WHERE f.follower_id = $1
            AND s.expires_at > NOW()
            ORDER BY s.created_at DESC
            LIMIT 50
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(stories)
    }

    /// Get all active stories grouped by user (for feed)
    pub async fn get_stories_feed(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<StoryWithUser>, AppError> {
        // Get stories from followed users, one story per user (most recent)
        let stories = sqlx::query_as::<_, StoryWithUser>(
            r#"
            SELECT DISTINCT ON (s.user_id)
                s.id,
                s.user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                s.media_url,
                s.media_type,
                s.caption,
                s.created_at,
                s.expires_at,
                s.view_count,
                EXISTS(
                    SELECT 1 FROM story_viewers sv 
                    WHERE sv.story_id = s.id AND sv.viewer_id = $1
                ) as is_viewed
            FROM stories s
            JOIN users u ON s.user_id = u.id
            LEFT JOIN follows f ON s.user_id = f.following_id AND f.follower_id = $1
            WHERE s.expires_at > NOW()
            AND (s.user_id = $1 OR f.follower_id = $1)
            ORDER BY s.user_id, s.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(stories)
    }

    pub async fn record_view(
        &self,
        story_id: Uuid,
        viewer_id: Uuid,
        reaction: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO story_viewers (story_id, viewer_id, reaction)
            VALUES ($1, $2, $3)
            ON CONFLICT (story_id, viewer_id) DO NOTHING
            "#
        )
        .bind(story_id)
        .bind(viewer_id)
        .bind(reaction)
        .execute(&self.pool)
        .await?;

        // Increment view count
        sqlx::query(
            "UPDATE stories SET view_count = view_count + 1 WHERE id = $1"
        )
        .bind(story_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_viewers(
        &self,
        story_id: Uuid,
    ) -> Result<Vec<StoryViewer>, AppError> {
        let viewers = sqlx::query_as::<_, StoryViewer>(
            r#"
            SELECT 
                u.id as user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                sv.viewed_at,
                sv.reaction
            FROM story_viewers sv
            JOIN users u ON sv.viewer_id = u.id
            WHERE sv.story_id = $1
            ORDER BY sv.viewed_at DESC
            "#
        )
        .bind(story_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(viewers)
    }

    pub async fn delete(&self, story_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query(
            "DELETE FROM stories WHERE id = $1 AND user_id = $2"
        )
        .bind(story_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFoundError("Story not found".to_string()));
        }

        Ok(())
    }
}