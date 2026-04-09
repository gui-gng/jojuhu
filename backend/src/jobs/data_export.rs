use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::errors::AppError;

pub struct DataExportJob {
    pool: PgPool,
}

impl DataExportJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_pending_exports(&self) -> Result<usize, AppError> {
        let exports = self.get_pending_exports().await?;
        
        if exports.is_empty() {
            return Ok(0);
        }

        info!("Processing {} data export requests", exports.len());
        let mut processed_count = 0;

        for export in exports {
            match self.generate_export(&export).await {
                Ok(_) => {
                    processed_count += 1;
                    info!("Generated data export for user {}", export.user_id);
                }
                Err(e) => {
                    error!("Failed to generate export {}: {}", export.id, e);
                    self.mark_export_failed(&export.id, &e.to_string()).await?;
                }
            }
        }

        Ok(processed_count)
    }

    async fn get_pending_exports(&self) -> Result<Vec<DataExportRequest>, AppError> {
        let exports = sqlx::query_as::<_, DataExportRequest>(
            r#"
            SELECT * FROM data_export_requests 
            WHERE status = 'pending'
            ORDER BY created_at ASC
            LIMIT 10
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(exports)
    }

    async fn generate_export(&self, export: &DataExportRequest) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "UPDATE data_export_requests SET status = 'processing' WHERE id = $1"
        )
        .bind(export.id)
        .execute(&mut *tx)
        .await?;

        let user_id = export.user_id;

        let user_data = self.collect_user_data(user_id).await?;

        let export_json = serde_json::to_string_pretty(&user_data)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize export: {}", e)))?;

        let file_size = export_json.len() as i64;
        let file_path = format!("/exports/{}_{}.json", user_id, Utc::now().timestamp());

        sqlx::query(
            r#"
            UPDATE data_export_requests
            SET status = 'completed', 
                file_path = $1, 
                file_size = $2,
                completed_at = NOW(),
                expires_at = NOW() + INTERVAL '7 days'
            WHERE id = $3
            "#
        )
        .bind(&file_path)
        .bind(file_size)
        .bind(export.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        info!("Data export completed: {} bytes for user {}", file_size, user_id);
        Ok(())
    }

    async fn collect_user_data(&self, user_id: Uuid) -> Result<serde_json::Value, AppError> {
        let posts: sqlx::Result<Option<serde_json::Value>> = sqlx::query_scalar(
            "SELECT json_agg(row_to_json(p)) FROM posts p WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        let comments: sqlx::Result<Option<serde_json::Value>> = sqlx::query_scalar(
            "SELECT json_agg(row_to_json(c)) FROM comments c WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        let likes: sqlx::Result<Option<serde_json::Value>> = sqlx::query_scalar(
            "SELECT json_agg(row_to_json(l)) FROM post_likes l WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        let followers: sqlx::Result<Option<serde_json::Value>> = sqlx::query_scalar(
            "SELECT json_agg(row_to_json(f)) FROM followers f WHERE following_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        let following: sqlx::Result<Option<serde_json::Value>> = sqlx::query_scalar(
            "SELECT json_agg(row_to_json(f)) FROM followers f WHERE follower_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        let messages_sent: sqlx::Result<Option<serde_json::Value>> = sqlx::query_scalar(
            "SELECT json_agg(row_to_json(m)) FROM messages m WHERE sender_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        let messages_received: sqlx::Result<Option<serde_json::Value>> = sqlx::query_scalar(
            "SELECT json_agg(row_to_json(m)) FROM messages m WHERE recipient_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        Ok(json!({
            "exported_at": Utc::now().to_rfc3339(),
            "user_id": user_id,
            "data": {
                "posts": posts.unwrap_or(None).unwrap_or(serde_json::Value::Null),
                "comments": comments.unwrap_or(None).unwrap_or(serde_json::Value::Null),
                "likes": likes.unwrap_or(None).unwrap_or(serde_json::Value::Null),
                "followers": followers.unwrap_or(None).unwrap_or(serde_json::Value::Null),
                "following": following.unwrap_or(None).unwrap_or(serde_json::Value::Null),
                "messages_sent": messages_sent.unwrap_or(None).unwrap_or(serde_json::Value::Null),
                "messages_received": messages_received.unwrap_or(None).unwrap_or(serde_json::Value::Null),
            }
        }))
    }

    async fn mark_export_failed(&self, export_id: &Uuid, error_message: &str) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE data_export_requests SET status = 'failed', error_message = $1 WHERE id = $2"
        )
        .bind(error_message)
        .bind(export_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DataExportRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub created_at: chrono::DateTime<Utc>,
    pub completed_at: Option<chrono::DateTime<Utc>>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}