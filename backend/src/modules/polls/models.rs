use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Poll {
    pub id: Uuid,
    pub post_id: Uuid,
    pub question: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_multiple: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PollOption {
    pub id: Uuid,
    pub poll_id: Uuid,
    pub option_text: String,
    pub position: i32,
    pub votes_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PollVote {
    pub id: Uuid,
    pub poll_id: Uuid,
    pub option_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePollRequest {
    pub question: String,
    pub options: Vec<String>,
    pub expires_in_hours: Option<i32>,
    pub is_multiple: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PollResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub question: String,
    pub options: Vec<PollOptionResponse>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_multiple: bool,
    pub total_votes: i32,
    pub user_voted: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PollOptionResponse {
    pub id: Uuid,
    pub option_text: String,
    pub position: i32,
    pub votes_count: i32,
    pub percentage: f32,
}

#[derive(Debug, Deserialize)]
pub struct VotePollRequest {
    pub option_id: Uuid,
}
