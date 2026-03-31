use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{Story, StoryViewer};

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

    pub async fn get_active_stories(
        &self,
        user_id: Uuid,
        _viewer_id: Uuid,
    ) -> Result<Vec<Story>, AppError> {
        let stories = sqlx::query_as::<_, Story>(
            r#"
            SELECT s.* FROM stories s
            WHERE s.user_id = $1
            AND s.expires_at > NOW()
            ORDER BY s.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(stories)
    }

    pub async fn get_following_stories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Story>, AppError> {
        let stories = sqlx::query_as::<_, Story>(
            r#"
            SELECT s.* FROM stories s
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
        sqlx::query(
            "DELETE FROM stories WHERE id = $1 AND user_id = $2"
        )
        .bind(story_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}