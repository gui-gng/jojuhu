use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{Poll, PollOption, PollVote};

pub struct PollRepository {
    pool: PgPool,
}

impl PollRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_poll(
        &self,
        post_id: Uuid,
        question: &str,
        expires_at: Option<DateTime<Utc>>,
        is_multiple: bool,
    ) -> Result<Poll, AppError> {
        let poll = sqlx::query_as::<_, Poll>(
            r#"
            INSERT INTO polls (post_id, question, expires_at, is_multiple)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(post_id)
        .bind(question)
        .bind(expires_at)
        .bind(is_multiple)
        .fetch_one(&self.pool)
        .await?;

        Ok(poll)
    }

    pub async fn add_option(&self, poll_id: Uuid, option_text: &str, position: i32) -> Result<PollOption, AppError> {
        let option = sqlx::query_as::<_, PollOption>(
            r#"
            INSERT INTO poll_options (poll_id, option_text, position)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(poll_id)
        .bind(option_text)
        .bind(position)
        .fetch_one(&self.pool)
        .await?;

        Ok(option)
    }

    pub async fn get_poll_by_post_id(&self, post_id: Uuid) -> Result<Option<Poll>, AppError> {
        let poll = sqlx::query_as::<_, Poll>(
            "SELECT * FROM polls WHERE post_id = $1"
        )
        .bind(post_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(poll)
    }

    pub async fn get_poll_options(&self, poll_id: Uuid) -> Result<Vec<PollOption>, AppError> {
        let options = sqlx::query_as::<_, PollOption>(
            "SELECT * FROM poll_options WHERE poll_id = $1 ORDER BY position"
        )
        .bind(poll_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(options)
    }

    pub async fn vote(&self, poll_id: Uuid, option_id: Uuid, user_id: Uuid) -> Result<PollVote, AppError> {
        let vote = sqlx::query_as::<_, PollVote>(
            r#"
            INSERT INTO poll_votes (poll_id, option_id, user_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(poll_id)
        .bind(option_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Increment votes count on option
        sqlx::query(
            "UPDATE poll_options SET votes_count = votes_count + 1 WHERE id = $1"
        )
        .bind(option_id)
        .execute(&self.pool)
        .await?;

        Ok(vote)
    }

    pub async fn has_user_voted(&self, poll_id: Uuid, user_id: Uuid) -> Result<Option<Uuid>, AppError> {
        let option_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT option_id FROM poll_votes WHERE poll_id = $1 AND user_id = $2"
        )
        .bind(poll_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(option_id)
    }

    pub async fn get_total_votes(&self, poll_id: Uuid) -> Result<i32, AppError> {
        let count: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM poll_votes WHERE poll_id = $1"
        )
        .bind(poll_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    pub async fn delete_poll(&self, poll_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM polls WHERE id = $1")
            .bind(poll_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}