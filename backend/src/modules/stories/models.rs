use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Story {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_url: String,
    pub media_type: String,
    pub caption: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub view_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateStoryRequest {
    pub media_url: String,
    pub media_type: Option<String>,
    pub caption: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StoryResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub media_url: String,
    pub media_type: String,
    pub caption: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub view_count: i32,
    pub is_viewed: bool,
}

#[derive(Debug, Deserialize)]
pub struct ViewStoryRequest {
    pub reaction: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StoryViewer {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub viewed_at: DateTime<Utc>,
    pub reaction: Option<String>,
}
