use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{ConversationSummaryRow, Message, MessageResponseRow, ThreadUser};

pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_message(
        &self,
        sender_id: Uuid,
        recipient_id: Uuid,
        content: &str,
    ) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (sender_id, recipient_id, content)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(sender_id)
        .bind(recipient_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    #[allow(dead_code)]
    pub async fn get_message_by_id(&self, message_id: Uuid) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE id = $1"
        )
        .bind(message_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    pub async fn get_message_response_by_id(&self, message_id: Uuid) -> Result<Option<MessageResponseRow>, AppError> {
        let message = sqlx::query_as::<_, MessageResponseRow>(
            r#"
            SELECT 
                m.id,
                m.sender_id,
                s.username as sender_username,
                m.recipient_id,
                r.username as recipient_username,
                m.content,
                m.is_read,
                m.created_at
            FROM messages m
            JOIN users s ON m.sender_id = s.id
            JOIN users r ON m.recipient_id = r.id
            WHERE m.id = $1
            "#
        )
        .bind(message_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(message)
    }

    pub async fn get_conversation(
        &self,
        user_id: Uuid,
        other_user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<MessageResponseRow>, AppError> {
        let messages = sqlx::query_as::<_, MessageResponseRow>(
            r#"
            SELECT 
                m.id,
                m.sender_id,
                s.username as sender_username,
                m.recipient_id,
                r.username as recipient_username,
                m.content,
                m.is_read,
                m.created_at
            FROM messages m
            JOIN users s ON m.sender_id = s.id
            JOIN users r ON m.recipient_id = r.id
            WHERE (m.sender_id = $1 AND m.recipient_id = $2)
               OR (m.sender_id = $2 AND m.recipient_id = $1)
            ORDER BY m.created_at DESC
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(user_id)
        .bind(other_user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }

    pub async fn get_conversations(&self, user_id: Uuid) -> Result<Vec<ConversationSummaryRow>, AppError> {
        let conversations = sqlx::query_as::<_, ConversationSummaryRow>(
            r#"
            WITH last_messages AS (
                SELECT DISTINCT ON (LEAST(sender_id, recipient_id), GREATEST(sender_id, recipient_id))
                    id,
                    sender_id,
                    recipient_id,
                    content,
                    created_at
                FROM messages
                WHERE sender_id = $1 OR recipient_id = $1
                ORDER BY LEAST(sender_id, recipient_id), GREATEST(sender_id, recipient_id), created_at DESC
            ),
            unread_counts AS (
                SELECT 
                    CASE 
                        WHEN sender_id = $1 THEN recipient_id
                        ELSE sender_id
                    END as other_user_id,
                    COUNT(*) FILTER (WHERE recipient_id = $1 AND NOT is_read) as unread_count
                FROM messages
                WHERE sender_id = $1 OR recipient_id = $1
                GROUP BY other_user_id
            )
            SELECT 
                u.id as user_id,
                u.username,
                u.display_name,
                u.avatar_url,
                lm.content as last_message,
                lm.created_at as last_message_at,
                COALESCE(uc.unread_count, 0) as unread_count
            FROM last_messages lm
            JOIN users u ON u.id = CASE 
                WHEN lm.sender_id = $1 THEN lm.recipient_id 
                ELSE lm.sender_id 
            END
            LEFT JOIN unread_counts uc ON uc.other_user_id = u.id
            ORDER BY lm.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(conversations)
    }

    pub async fn mark_as_read(&self, message_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE messages SET is_read = true WHERE id = $1 AND recipient_id = $2"
        )
        .bind(message_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_message(&self, message_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query(
            "DELETE FROM messages WHERE id = $1 AND (sender_id = $2 OR recipient_id = $2)"
        )
        .bind(message_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFoundError("Message not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_thread_user(&self, user_id: Uuid) -> Result<ThreadUser, AppError> {
        let user = sqlx::query_as::<_, ThreadUser>(
            "SELECT id, username, display_name, avatar_url FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
