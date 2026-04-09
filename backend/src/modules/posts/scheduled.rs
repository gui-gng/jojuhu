use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Scheduled Posts
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ScheduledPostStatus {
    Pending,
    Published,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScheduledPost {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub scheduled_for: DateTime<Utc>,
    pub status: ScheduledPostStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub published_post_id: Option<Uuid>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduledPostRequest {
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub scheduled_for: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScheduledPostRequest {
    pub content: Option<String>,
    pub media_urls: Option<Vec<String>>,
    pub scheduled_for: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ScheduledPostResponse {
    pub id: Uuid,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub scheduled_for: DateTime<Utc>,
    pub status: ScheduledPostStatus,
    pub created_at: DateTime<Utc>,
}

// Post Drafts
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostDraft {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: Option<String>,
    pub media_urls: Option<Vec<String>>,
    pub visibility: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SaveDraftRequest {
    pub content: Option<String>,
    pub media_urls: Option<Vec<String>>,
    pub visibility: Option<String>,
}

// Push Notification Tokens
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum DeviceType {
    Ios,
    Android,
    Web,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PushNotificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_token: String,
    pub device_type: String,
    pub device_name: Option<String>,
    pub is_active: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPushTokenRequest {
    pub device_token: String,
    pub device_type: String,
    pub device_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeactivatePushTokenRequest {
    pub device_token: String,
}

#[derive(Debug, Serialize)]
pub struct PushNotificationTokenResponse {
    pub id: Uuid,
    pub device_type: String,
    pub device_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
