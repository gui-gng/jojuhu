//! Message models
//!
//! This module defines data structures for the direct messaging system
//! including messages, conversations, and thread information.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core message entity representing a single message
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    /// Unique identifier for the message
    pub id: Uuid,
    /// ID of the user who sent the message
    pub sender_id: Uuid,
    /// ID of the recipient user
    pub recipient_id: Uuid,
    /// Message content (encrypted at rest)
    pub content: String,
    /// Whether the message has been read by the recipient
    pub is_read: bool,
    /// When the message was sent
    pub created_at: DateTime<Utc>,
    /// When the message was last edited (if applicable)
    pub updated_at: DateTime<Utc>,
}

/// Database row for message responses with sender/recipient info
#[derive(Debug, sqlx::FromRow)]
pub struct MessageResponseRow {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_username: String,
    pub recipient_id: Uuid,
    pub recipient_username: String,
    pub content: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

/// Request to send a new message
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    /// ID of the intended recipient
    pub recipient_id: Uuid,
    /// Message content (max 2000 chars)
    pub content: String,
}

/// Request to update/edit an existing message
#[derive(Debug, Deserialize)]
pub struct UpdateMessageRequest {
    /// New content for the message
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_username: String,
    pub recipient_id: Uuid,
    pub recipient_username: String,
    pub content: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl From<MessageResponseRow> for MessageResponse {
    fn from(row: MessageResponseRow) -> Self {
        Self {
            id: row.id,
            sender_id: row.sender_id,
            sender_username: row.sender_username,
            recipient_id: row.recipient_id,
            recipient_username: row.recipient_username,
            content: row.content,
            is_read: row.is_read,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ConversationSummaryRow {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub last_message: String,
    pub last_message_at: DateTime<Utc>,
    pub unread_count: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ConversationSummary {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub last_message: String,
    pub last_message_at: DateTime<Utc>,
    pub unread_count: i64,
}

impl From<ConversationSummaryRow> for ConversationSummary {
    fn from(row: ConversationSummaryRow) -> Self {
        Self {
            user_id: row.user_id,
            username: row.username,
            display_name: row.display_name,
            avatar_url: row.avatar_url,
            last_message: row.last_message,
            last_message_at: row.last_message_at,
            unread_count: row.unread_count.unwrap_or(0),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageThread {
    pub user: ThreadUser,
    pub messages: Vec<MessageResponse>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ThreadUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}
