use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::errors::AppError;

pub struct ScheduledPostsJob {
    pool: PgPool,
}

impl ScheduledPostsJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_pending_posts(&self) -> Result<usize, AppError> {
        let posts = self.get_pending_posts().await?;
        
        if posts.is_empty() {
            return Ok(0);
        }

        info!("Processing {} scheduled posts", posts.len());
        let mut published_count = 0;

        for post in posts {
            match self.publish_post(&post).await {
                Ok(_) => {
                    published_count += 1;
                    info!("Published scheduled post {}", post.id);
                }
                Err(e) => {
                    error!("Failed to publish scheduled post {}: {}", post.id, e);
                    self.mark_post_failed(&post.id, &e.to_string()).await?;
                }
            }
        }

        Ok(published_count)
    }

    async fn get_pending_posts(&self) -> Result<Vec<ScheduledPost>, AppError> {
        let posts = sqlx::query_as::<_, ScheduledPost>(
            r#"
            SELECT * FROM scheduled_posts 
            WHERE status = 'pending' 
            AND scheduled_for <= NOW()
            ORDER BY scheduled_for ASC
            LIMIT 50
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    async fn publish_post(&self, scheduled_post: &ScheduledPost) -> Result<Uuid, AppError> {
        let mut tx = self.pool.begin().await?;

        let post_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO posts (user_id, content, media_urls, visibility, created_at)
            VALUES ($1, $2, $3, 'public', NOW())
            RETURNING id
            "#
        )
        .bind(scheduled_post.user_id)
        .bind(&scheduled_post.content)
        .bind(&scheduled_post.media_urls)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE scheduled_posts
            SET status = 'published', published_at = NOW(), published_post_id = $1, updated_at = NOW()
            WHERE id = $2
            "#
        )
        .bind(post_id)
        .bind(scheduled_post.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(post_id)
    }

    async fn mark_post_failed(&self, post_id: &Uuid, error_message: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE scheduled_posts
            SET status = 'failed', error_message = $1, updated_at = NOW()
            WHERE id = $2
            "#
        )
        .bind(error_message)
        .bind(post_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ScheduledPost {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub scheduled_for: chrono::DateTime<Utc>,
    pub status: String,
    pub published_at: Option<chrono::DateTime<Utc>>,
    pub published_post_id: Option<Uuid>,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}