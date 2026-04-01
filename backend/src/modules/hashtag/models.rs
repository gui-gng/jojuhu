use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Hashtag {
    pub id: Uuid,
    pub name: String,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct HashtagResponse {
    pub name: String,
    pub usage_count: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TrendingHashtag {
    pub name: String,
    pub usage_count: i32,
    pub posts_count: i32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct HashtagSearchResult {
    pub name: String,
    pub posts_count: i32,
}
