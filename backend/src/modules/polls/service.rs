use chrono::Utc;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;

use super::models::{CreatePollRequest, PollResponse, PollOptionResponse, VotePollRequest};
use super::repository::PollRepository;

const MIN_OPTIONS: usize = 2;
const MAX_OPTIONS: usize = 6;
const MIN_QUESTION_LENGTH: usize = 1;
const MAX_QUESTION_LENGTH: usize = 500;
const MIN_OPTION_LENGTH: usize = 1;
const MAX_OPTION_LENGTH: usize = 100;

pub struct PollService {
    repository: PollRepository,
}

impl PollService {
    pub fn new(repository: PollRepository) -> Self {
        Self { repository }
    }

    pub async fn create_poll(
        &self,
        post_id: Uuid,
        request: CreatePollRequest,
    ) -> Result<PollResponse, AppError> {
        // Validate question
        validate_content_length(
            &request.question,
            MIN_QUESTION_LENGTH,
            MAX_QUESTION_LENGTH,
            "Poll question",
        )?;
        validate_input_safety(&request.question).map_err(AppError::ValidationError)?;

        // Validate options count
        if request.options.len() < MIN_OPTIONS {
            return Err(AppError::ValidationError(
                format!("Poll must have at least {} options", MIN_OPTIONS),
            ));
        }
        if request.options.len() > MAX_OPTIONS {
            return Err(AppError::ValidationError(
                format!("Poll can have at most {} options", MAX_OPTIONS),
            ));
        }

        // Validate and sanitize each option
        let mut sanitized_options = Vec::new();
        for opt in &request.options {
            validate_content_length(opt, MIN_OPTION_LENGTH, MAX_OPTION_LENGTH, "Poll option")?;
            validate_input_safety(opt).map_err(AppError::ValidationError)?;
            sanitized_options.push(sanitize_input(opt));
        }

        // Calculate expiration time
        let expires_at = request.expires_in_hours.map(|hours| {
            Utc::now() + chrono::Duration::hours(hours as i64)
        });

        let is_multiple = request.is_multiple.unwrap_or(false);

        // Create poll
        let poll = self
            .repository
            .create_poll(post_id, &request.question, expires_at, is_multiple)
            .await?;

        // Add options
        for (i, option_text) in sanitized_options.iter().enumerate() {
            self.repository
                .add_option(poll.id, option_text, i as i32)
                .await?;
        }

        // Get the full response
        self.get_poll_response(post_id, None).await?
            .ok_or_else(|| AppError::NotFoundError("Poll not found".to_string()))
    }

    pub async fn get_poll(&self, post_id: Uuid, user_id: Option<Uuid>) -> Result<Option<PollResponse>, AppError> {
        self.get_poll_response(post_id, user_id).await
    }

    async fn get_poll_response(&self, post_id: Uuid, user_id: Option<Uuid>) -> Result<Option<PollResponse>, AppError> {
        let poll = match self.repository.get_poll_by_post_id(post_id).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        let options = self.repository.get_poll_options(poll.id).await?;
        let total_votes = self.repository.get_total_votes(poll.id).await?;
        let user_voted = match user_id {
            Some(uid) => self.repository.has_user_voted(poll.id, uid).await?,
            None => None,
        };

        let options_response: Vec<PollOptionResponse> = options
            .into_iter()
            .map(|opt| {
                let percentage = if total_votes > 0 {
                    (opt.votes_count as f32 / total_votes as f32) * 100.0
                } else {
                    0.0
                };
                PollOptionResponse {
                    id: opt.id,
                    option_text: opt.option_text,
                    position: opt.position,
                    votes_count: opt.votes_count,
                    percentage,
                }
            })
            .collect();

        Ok(Some(PollResponse {
            id: poll.id,
            post_id: poll.post_id,
            question: poll.question,
            options: options_response,
            expires_at: poll.expires_at,
            is_multiple: poll.is_multiple,
            total_votes,
            user_voted,
            created_at: poll.created_at,
        }))
    }

    pub async fn vote(&self, post_id: Uuid, request: VotePollRequest, user_id: Uuid) -> Result<PollResponse, AppError> {
        // Get poll
        let poll = self.repository.get_poll_by_post_id(post_id).await?
            .ok_or_else(|| AppError::NotFoundError("Poll not found".to_string()))?;

        // Check if poll is expired
        if let Some(expires_at) = poll.expires_at {
            if expires_at < Utc::now() {
                return Err(AppError::ValidationError("Poll has expired".to_string()));
            }
        }

        // Check if user already voted
        if let Some(_) = self.repository.has_user_voted(poll.id, user_id).await? {
            return Err(AppError::ValidationError("You have already voted on this poll".to_string()));
        }

        // Record vote
        self.repository.vote(poll.id, request.option_id, user_id).await?;

        // Return updated poll
        self.get_poll_response(post_id, Some(user_id))
            .await?
            .ok_or_else(|| AppError::NotFoundError("Poll not found".to_string()))
    }

    pub async fn delete_poll(&self, post_id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        let poll = self.repository.get_poll_by_post_id(post_id).await?
            .ok_or_else(|| AppError::NotFoundError("Poll not found".to_string()))?;

        // Note: In a real app, you'd check if user owns the poll/post
        self.repository.delete_poll(poll.id).await
    }
}