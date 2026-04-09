use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::scheduled::{
    CreateScheduledPostRequest, PostDraft, PushNotificationToken,
    RegisterPushTokenRequest, ScheduledPost,
    UpdateScheduledPostRequest,
};

pub struct ScheduledPostsRepository {
    pool: PgPool,
}

impl ScheduledPostsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_scheduled_post(
        &self,
        user_id: Uuid,
        request: CreateScheduledPostRequest,
    ) -> Result<ScheduledPost, AppError> {
        if request.scheduled_for <= Utc::now() {
            return Err(AppError::ValidationError(
                "Scheduled time must be in the future".to_string(),
            ));
        }

        let post = sqlx::query_as::<_, ScheduledPost>(
            r#"
            INSERT INTO scheduled_posts (user_id, content, media_urls, scheduled_for)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&request.content)
        .bind(&request.media_urls)
        .bind(request.scheduled_for)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn get_scheduled_posts(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<ScheduledPost>, AppError> {
        let posts = sqlx::query_as::<_, ScheduledPost>(
            r#"
            SELECT * FROM scheduled_posts
            WHERE user_id = $1
            ORDER BY scheduled_for ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    pub async fn get_scheduled_post(
        &self,
        user_id: Uuid,
        post_id: Uuid,
    ) -> Result<Option<ScheduledPost>, AppError> {
        let post = sqlx::query_as::<_, ScheduledPost>(
            "SELECT * FROM scheduled_posts WHERE id = $1 AND user_id = $2",
        )
        .bind(post_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn update_scheduled_post(
        &self,
        user_id: Uuid,
        post_id: Uuid,
        request: UpdateScheduledPostRequest,
    ) -> Result<ScheduledPost, AppError> {
        if let Some(scheduled_for) = request.scheduled_for {
            if scheduled_for <= Utc::now() {
                return Err(AppError::ValidationError(
                    "Scheduled time must be in the future".to_string(),
                ));
            }
        }

        let post = sqlx::query_as::<_, ScheduledPost>(
            r#"
            UPDATE scheduled_posts
            SET
                content = COALESCE($3, content),
                media_urls = COALESCE($4, media_urls),
                scheduled_for = COALESCE($5, scheduled_for),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2 AND status = 'pending'
            RETURNING *
            "#,
        )
        .bind(post_id)
        .bind(user_id)
        .bind(request.content)
        .bind(request.media_urls)
        .bind(request.scheduled_for)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn cancel_scheduled_post(
        &self,
        user_id: Uuid,
        post_id: Uuid,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE scheduled_posts SET status = 'cancelled', updated_at = NOW() 
             WHERE id = $1 AND user_id = $2 AND status = 'pending'",
        )
        .bind(post_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFoundError("Scheduled post not found or already processed".to_string()));
        }

        Ok(())
    }

    pub async fn get_pending_scheduled_posts(&self) -> Result<Vec<ScheduledPost>, AppError> {
        let posts = sqlx::query_as::<_, ScheduledPost>(
            "SELECT * FROM scheduled_posts WHERE status = 'pending' AND scheduled_for <= NOW()",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    pub async fn mark_scheduled_post_published(
        &self,
        post_id: Uuid,
        published_post_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE scheduled_posts SET status = 'published', published_at = NOW(), 
             published_post_id = $2, updated_at = NOW() WHERE id = $1",
        )
        .bind(post_id)
        .bind(published_post_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Post Drafts
    pub async fn save_draft(
        &self,
        user_id: Uuid,
        content: Option<String>,
        media_urls: Option<Vec<String>>,
        visibility: Option<String>,
    ) -> Result<PostDraft, AppError> {
        let draft = sqlx::query_as::<_, PostDraft>(
            r#"
            INSERT INTO post_drafts (user_id, content, media_urls, visibility)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id) DO UPDATE SET
                content = COALESCE(EXCLUDED.content, post_drafts.content),
                media_urls = COALESCE(EXCLUDED.media_urls, post_drafts.media_urls),
                visibility = COALESCE(EXCLUDED.visibility, post_drafts.visibility),
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(content)
        .bind(media_urls)
        .bind(visibility)
        .fetch_one(&self.pool)
        .await?;

        Ok(draft)
    }

    pub async fn get_draft(&self, user_id: Uuid) -> Result<Option<PostDraft>, AppError> {
        let draft = sqlx::query_as::<_, PostDraft>(
            "SELECT * FROM post_drafts WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(draft)
    }

    pub async fn delete_draft(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM post_drafts WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // Push Notification Tokens
    pub async fn register_push_token(
        &self,
        user_id: Uuid,
        request: RegisterPushTokenRequest,
    ) -> Result<PushNotificationToken, AppError> {
        let token = sqlx::query_as::<_, PushNotificationToken>(
            r#"
            INSERT INTO push_notification_tokens (user_id, device_token, device_type, device_name)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, device_token) DO UPDATE SET
                device_type = EXCLUDED.device_type,
                device_name = COALESCE(EXCLUDED.device_name, push_notification_tokens.device_name),
                is_active = TRUE,
                last_used_at = NOW()
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&request.device_token)
        .bind(&request.device_type)
        .bind(&request.device_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn get_user_push_tokens(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<PushNotificationToken>, AppError> {
        let tokens = sqlx::query_as::<_, PushNotificationToken>(
            "SELECT * FROM push_notification_tokens WHERE user_id = $1 AND is_active = TRUE",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }

    pub async fn deactivate_push_token(
        &self,
        user_id: Uuid,
        device_token: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE push_notification_tokens SET is_active = FALSE 
             WHERE user_id = $1 AND device_token = $2",
        )
        .bind(user_id)
        .bind(device_token)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}