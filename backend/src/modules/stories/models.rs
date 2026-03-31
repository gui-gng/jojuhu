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

/// Story with user details (from database join)
#[derive(Debug, sqlx::FromRow)]
pub struct StoryWithUser {
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
    pub is_viewed: Option<bool>,
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

impl From<StoryWithUser> for StoryResponse {
    fn from(story: StoryWithUser) -> Self {
        Self {
            id: story.id,
            user_id: story.user_id,
            username: story.username,
            display_name: story.display_name,
            avatar_url: story.avatar_url,
            media_url: story.media_url,
            media_type: story.media_type,
            caption: story.caption,
            created_at: story.created_at,
            expires_at: story.expires_at,
            view_count: story.view_count,
            is_viewed: story.is_viewed.unwrap_or(false),
        }
    }
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
