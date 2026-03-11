use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub shares_count: i32,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostResponseRow {
    pub id: Uuid,
    pub author: serde_json::Value,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub shares_count: i32,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub is_liked: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub content: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PostResponse {
    pub id: Uuid,
    pub author: PostAuthor,
    pub content: String,
    pub media_urls: Option<Vec<String>>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub shares_count: i32,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub is_liked: bool,
}

impl From<PostResponseRow> for PostResponse {
    fn from(row: PostResponseRow) -> Self {
        let author: PostAuthor = serde_json::from_value(row.author).unwrap_or(PostAuthor {
            id: Uuid::nil(),
            username: "unknown".to_string(),
            display_name: None,
            avatar_url: None,
        });
        Self {
            id: row.id,
            author,
            content: row.content,
            media_urls: row.media_urls,
            likes_count: row.likes_count,
            comments_count: row.comments_count,
            shares_count: row.shares_count,
            is_public: row.is_public,
            created_at: row.created_at,
            is_liked: row.is_liked,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PostAuthor {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Like {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CommentResponseRow {
    pub id: Uuid,
    pub author: serde_json::Value,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub author: PostAuthor,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
}

impl From<CommentResponseRow> for CommentResponse {
    fn from(row: CommentResponseRow) -> Self {
        let author: PostAuthor = serde_json::from_value(row.author).unwrap_or(PostAuthor {
            id: Uuid::nil(),
            username: "unknown".to_string(),
            display_name: None,
            avatar_url: None,
        });
        Self {
            id: row.id,
            author,
            content: row.content,
            parent_comment_id: row.parent_comment_id,
            likes_count: row.likes_count,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TimelineFeedQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}
