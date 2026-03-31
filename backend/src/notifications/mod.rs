use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

pub mod handlers;

/// Notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
pub enum NotificationType {
    NewFollower,
    PostLike,
    PostComment,
    NewMessage,
    ForumReply,
    Mention,
    System,
}

/// Notification model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

/// Create notification request
#[derive(Debug, Deserialize)]
pub struct CreateNotificationRequest {
    pub recipient_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Notification service
pub struct NotificationService {
    pool: PgPool,
    // TODO: v0.2.0 - Enable when WebSocket is integrated
    // ws_server: Option<WebSocketServer>,
}

impl NotificationService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Create a new notification
    pub async fn create_notification(
        &self,
        request: CreateNotificationRequest,
    ) -> Result<Notification, AppError> {
        let notification = sqlx::query_as::<_, Notification>(
            r#"
            INSERT INTO notifications 
                (recipient_id, sender_id, notification_type, title, message, data)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(request.recipient_id)
        .bind(request.sender_id)
        .bind(format!("{:?}", request.notification_type).to_lowercase())
        .bind(&request.title)
        .bind(&request.message)
        .bind(request.data)
        .fetch_one(&self.pool)
        .await?;
        
        // TODO: v0.2.0 - Send real-time notification via WebSocket
        // if let Some(ref ws_server) = self.ws_server {
        //     let ws_msg = WsMessage::Notification {
        //         id: notification.id,
        //         notification_type: notification.notification_type.clone(),
        //         title: notification.title.clone(),
        //         message: notification.message.clone(),
        //         data: notification.data.clone(),
        //     };
        //     ws_server.send_to_user(request.recipient_id, ws_msg).await;
        // }
        
        // TODO: Send push notification if user has tokens
        
        Ok(notification)
    }
    
    /// Get user notifications
    pub async fn get_notifications(
        &self,
        user_id: Uuid,
        unread_only: bool,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Notification>, AppError> {
        let notifications = if unread_only {
            sqlx::query_as::<_, Notification>(
                r#"
                SELECT * FROM notifications 
                WHERE recipient_id = $1 AND is_read = FALSE
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Notification>(
                r#"
                SELECT * FROM notifications 
                WHERE recipient_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };
        
        Ok(notifications)
    }
    
    /// Get unread notification count
    pub async fn get_unread_count(&self,
        user_id: Uuid,
    ) -> Result<i64, AppError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM notifications WHERE recipient_id = $1 AND is_read = FALSE"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }
    
    /// Mark notification as read
    pub async fn mark_as_read(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE notifications 
            SET is_read = TRUE, read_at = NOW()
            WHERE id = $1 AND recipient_id = $2
            "#
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Mark all notifications as read
    pub async fn mark_all_as_read(
        &self,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE notifications 
            SET is_read = TRUE, read_at = NOW()
            WHERE recipient_id = $1 AND is_read = FALSE
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Delete notification
    pub async fn delete_notification(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM notifications WHERE id = $1 AND recipient_id = $2"
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // Helper methods for creating specific notification types
    
    pub async fn notify_new_follower(
        &self,
        follower_id: Uuid,
        follower_name: &str,
        target_user_id: Uuid,
    ) -> Result<(), AppError> {
        self.create_notification(CreateNotificationRequest {
            recipient_id: target_user_id,
            sender_id: Some(follower_id),
            notification_type: NotificationType::NewFollower,
            title: "New Follower".to_string(),
            message: format!("{} started following you", follower_name),
            data: Some(serde_json::json!({
                "follower_id": follower_id,
                "follower_name": follower_name,
            })),
        }).await?;
        
        Ok(())
    }
    
    pub async fn notify_post_like(
        &self,
        liker_id: Uuid,
        liker_name: &str,
        post_author_id: Uuid,
        post_id: Uuid,
    ) -> Result<(), AppError> {
        self.create_notification(CreateNotificationRequest {
            recipient_id: post_author_id,
            sender_id: Some(liker_id),
            notification_type: NotificationType::PostLike,
            title: "New Like".to_string(),
            message: format!("{} liked your post", liker_name),
            data: Some(serde_json::json!({
                "liker_id": liker_id,
                "liker_name": liker_name,
                "post_id": post_id,
            })),
        }).await?;
        
        Ok(())
    }
    
    pub async fn notify_post_comment(
        &self,
        commenter_id: Uuid,
        commenter_name: &str,
        post_author_id: Uuid,
        post_id: Uuid,
        comment_preview: &str,
    ) -> Result<(), AppError> {
        self.create_notification(CreateNotificationRequest {
            recipient_id: post_author_id,
            sender_id: Some(commenter_id),
            notification_type: NotificationType::PostComment,
            title: "New Comment".to_string(),
            message: format!("{} commented: {}", commenter_name, comment_preview),
            data: Some(serde_json::json!({
                "commenter_id": commenter_id,
                "commenter_name": commenter_name,
                "post_id": post_id,
                "comment_preview": comment_preview,
            })),
        }).await?;
        
        Ok(())
    }
    
    pub async fn notify_new_message(
        &self,
        sender_id: Uuid,
        sender_name: &str,
        recipient_id: Uuid,
        conversation_id: Uuid,
        message_preview: &str,
    ) -> Result<(), AppError> {
        self.create_notification(CreateNotificationRequest {
            recipient_id,
            sender_id: Some(sender_id),
            notification_type: NotificationType::NewMessage,
            title: "New Message".to_string(),
            message: format!("{}: {}", sender_name, message_preview),
            data: Some(serde_json::json!({
                "sender_id": sender_id,
                "sender_name": sender_name,
                "conversation_id": conversation_id,
            })),
        }).await?;
        
        Ok(())
    }
    
    pub async fn notify_mention(
        &self,
        mentioner_id: Uuid,
        mentioner_name: &str,
        mentioned_user_id: Uuid,
        content_type: &str, // "post" or "comment"
        content_id: Uuid,
    ) -> Result<(), AppError> {
        self.create_notification(CreateNotificationRequest {
            recipient_id: mentioned_user_id,
            sender_id: Some(mentioner_id),
            notification_type: NotificationType::Mention,
            title: "New Mention".to_string(),
            message: format!("{} mentioned you in a {}", mentioner_name, content_type),
            data: Some(serde_json::json!({
                "mentioner_id": mentioner_id,
                "mentioner_name": mentioner_name,
                "content_type": content_type,
                "content_id": content_id,
            })),
        }).await?;
        
        Ok(())
    }
}