use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;

use super::models::{ConversationSummary, ConversationSummaryRow, MessageResponse, MessageResponseRow, SendMessageRequest};
use super::repository::MessageRepository;

/// Maximum message content length
const MAX_MESSAGE_LENGTH: usize = 2000;
/// Minimum message content length
const MIN_MESSAGE_LENGTH: usize = 1;

pub struct MessageService {
    repository: MessageRepository,
}

impl MessageService {
    pub fn new(repository: MessageRepository) -> Self {
        Self { repository }
    }

    pub async fn send_message(
        &self,
        sender_id: Uuid,
        mut request: SendMessageRequest,
    ) -> Result<MessageResponse, AppError> {
        // Validate content length
        validate_content_length(&request.content, MIN_MESSAGE_LENGTH, MAX_MESSAGE_LENGTH, "Message content")?;
        
        // Check for suspicious patterns
        validate_input_safety(&request.content)
            .map_err(AppError::ValidationError)?;
        
        // Sanitize content to prevent XSS
        request.content = sanitize_input(&request.content);

        let message = self
            .repository
            .create_message(sender_id, request.recipient_id, &request.content)
            .await?;

        let row = self.get_message_response(message.id).await?;
        Ok(row.into())
    }

    pub async fn get_conversation(
        &self,
        user_id: Uuid,
        other_user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<super::models::MessageThread, AppError> {
        let rows: Vec<MessageResponseRow> = self
            .repository
            .get_conversation(user_id, other_user_id, offset, limit)
            .await?;
        let messages: Vec<MessageResponse> = rows.into_iter().map(Into::into).collect();

        let other_user = self.repository.get_thread_user(other_user_id).await?;

        Ok(super::models::MessageThread {
            user: other_user,
            messages,
        })
    }

    pub async fn get_conversations(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ConversationSummary>, AppError> {
        let rows: Vec<ConversationSummaryRow> = self.repository.get_conversations(user_id).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn mark_message_as_read(
        &self,
        message_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        self.repository.mark_as_read(message_id, user_id).await
    }

    pub async fn delete_message(
        &self,
        message_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        self.repository.delete_message(message_id, user_id).await
    }

    pub async fn delete_conversation(
        &self,
        user_id: Uuid,
        other_user_id: Uuid,
    ) -> Result<(), AppError> {
        self.repository.delete_conversation(user_id, other_user_id).await
    }

    async fn get_message_response(&self, message_id: Uuid) -> Result<MessageResponseRow, AppError> {
        self.repository.get_message_response_by_id(message_id).await?
            .ok_or_else(|| AppError::NotFoundError("Message not found".to_string()))
    }
}
