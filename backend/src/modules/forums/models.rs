use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Forum {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub creator_id: Uuid,
    pub is_public: bool,
    pub members_count: i32,
    pub topics_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateForumRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateForumRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ForumResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub creator: ForumCreator,
    pub is_public: bool,
    pub members_count: i32,
    pub topics_count: i32,
    pub is_member: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ForumResponseRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub creator: serde_json::Value,
    pub is_public: bool,
    pub members_count: i32,
    pub topics_count: i32,
    pub is_member: bool,
    pub created_at: DateTime<Utc>,
}

impl From<ForumResponseRow> for ForumResponse {
    fn from(row: ForumResponseRow) -> Self {
        let creator: ForumCreator = serde_json::from_value(row.creator).unwrap_or(ForumCreator {
            id: Uuid::nil(),
            username: "unknown".to_string(),
            display_name: None,
        });
        Self {
            id: row.id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            icon_url: row.icon_url,
            cover_image_url: row.cover_image_url,
            creator,
            is_public: row.is_public,
            members_count: row.members_count,
            topics_count: row.topics_count,
            is_member: row.is_member,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumCreator {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ForumMember {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub user_id: Uuid,
    pub role: ForumRole,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "forum_role", rename_all = "lowercase")]
pub enum ForumRole {
    Admin,
    Moderator,
    Member,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Topic {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub views_count: i32,
    pub replies_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct TopicResponse {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub author: ForumCreator,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub views_count: i32,
    pub replies_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopicResponseRow {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub author: serde_json::Value,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub views_count: i32,
    pub replies_count: i32,
    pub created_at: DateTime<Utc>,
}

impl From<TopicResponseRow> for TopicResponse {
    fn from(row: TopicResponseRow) -> Self {
        let author: ForumCreator = serde_json::from_value(row.author).unwrap_or(ForumCreator {
            id: Uuid::nil(),
            username: "unknown".to_string(),
            display_name: None,
        });
        Self {
            id: row.id,
            forum_id: row.forum_id,
            author,
            title: row.title,
            content: row.content,
            is_pinned: row.is_pinned,
            is_locked: row.is_locked,
            views_count: row.views_count,
            replies_count: row.replies_count,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicReply {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub parent_reply_id: Option<Uuid>,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReplyRequest {
    pub content: String,
    pub parent_reply_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct ReplyResponse {
    pub id: Uuid,
    pub author: ForumCreator,
    pub content: String,
    pub parent_reply_id: Option<Uuid>,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Query parameters for forum search and discovery
#[derive(Debug, Deserialize)]
pub struct ForumSearchQuery {
    pub search: Option<String>,
    pub sort_by: Option<ForumSortBy>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForumSortBy {
    Newest,
    Popular,
    MostMembers,
    MostActive,
}

impl Default for ForumSortBy {
    fn default() -> Self {
        Self::Newest
    }
}

/// Paginated forum list response
#[derive(Debug, Serialize)]
pub struct ForumListResponse {
    pub forums: Vec<ForumResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub has_more: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReplyResponseRow {
    pub id: Uuid,
    pub author: serde_json::Value,
    pub content: String,
    pub parent_reply_id: Option<Uuid>,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
}

impl From<ReplyResponseRow> for ReplyResponse {
    fn from(row: ReplyResponseRow) -> Self {
        let author: ForumCreator = serde_json::from_value(row.author).unwrap_or(ForumCreator {
            id: Uuid::nil(),
            username: "unknown".to_string(),
            display_name: None,
        });
        Self {
            id: row.id,
            author,
            content: row.content,
            parent_reply_id: row.parent_reply_id,
            likes_count: row.likes_count,
            created_at: row.created_at,
        }
    }
}
