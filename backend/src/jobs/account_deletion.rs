use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::errors::AppError;

pub struct AccountDeletionJob {
    pool: PgPool,
}

impl AccountDeletionJob {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_pending_deletions(&self) -> Result<usize, AppError> {
        let deletions = self.get_pending_deletions().await?;
        
        if deletions.is_empty() {
            return Ok(0);
        }

        info!("Processing {} account deletion requests", deletions.len());
        let mut deleted_count = 0;

        for deletion in deletions {
            match self.delete_account(&deletion).await {
                Ok(_) => {
                    deleted_count += 1;
                    info!("Deleted account for user {}", deletion.user_id);
                }
                Err(e) => {
                    error!("Failed to delete account {}: {}", deletion.user_id, e);
                }
            }
        }

        Ok(deleted_count)
    }

    async fn get_pending_deletions(&self) -> Result<Vec<AccountDeletionRequest>, AppError> {
        let deletions = sqlx::query_as::<_, AccountDeletionRequest>(
            r#"
            SELECT * FROM account_deletion_requests 
            WHERE status = 'pending' 
            AND scheduled_deletion_at <= NOW()
            ORDER BY scheduled_deletion_at ASC
            LIMIT 10
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(deletions)
    }

    async fn delete_account(&self, deletion: &AccountDeletionRequest) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "UPDATE account_deletion_requests SET status = 'processing' WHERE id = $1"
        )
        .bind(deletion.id)
        .execute(&mut *tx)
        .await?;

        let user_id = deletion.user_id;

        Self::delete_user_messages(&mut tx, user_id).await?;
        Self::delete_user_comments(&mut tx, user_id).await?;
        Self::delete_user_posts(&mut tx, user_id).await?;
        Self::delete_user_likes(&mut tx, user_id).await?;
        Self::delete_user_followers(&mut tx, user_id).await?;
        Self::delete_user_notifications(&mut tx, user_id).await?;
        Self::delete_user_stories(&mut tx, user_id).await?;
        Self::delete_user_polls(&mut tx, user_id).await?;
        Self::delete_user_forum_content(&mut tx, user_id).await?;
        Self::delete_user_group_memberships(&mut tx, user_id).await?;
        Self::delete_user_privacy_settings(&mut tx, user_id).await?;
        Self::delete_user_push_tokens(&mut tx, user_id).await?;
        Self::delete_user_drafts(&mut tx, user_id).await?;
        Self::delete_user_scheduled_posts(&mut tx, user_id).await?;

        sqlx::query(
            "UPDATE account_deletion_requests SET status = 'completed', completed_at = NOW() WHERE id = $1"
        )
        .bind(deletion.id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "DELETE FROM users WHERE id = $1"
        )
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        info!("Account deletion completed for user {}", user_id);
        Ok(())
    }

    async fn delete_user_messages(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM messages WHERE sender_id = $1 OR recipient_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_comments(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM comments WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_posts(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM posts WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_likes(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM post_likes WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_followers(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM followers WHERE follower_id = $1 OR following_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_notifications(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM notifications WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_stories(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM stories WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_polls(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM poll_votes WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_forum_content(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM forum_replies WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        sqlx::query("DELETE FROM forum_topics WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_group_memberships(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM group_members WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_privacy_settings(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM privacy_settings WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_push_tokens(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM push_notification_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_drafts(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM post_drafts WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn delete_user_scheduled_posts(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM scheduled_posts WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountDeletionRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reason: Option<String>,
    pub status: String,
    pub requested_at: chrono::DateTime<Utc>,
    pub scheduled_deletion_at: chrono::DateTime<Utc>,
    pub completed_at: Option<chrono::DateTime<Utc>>,
}