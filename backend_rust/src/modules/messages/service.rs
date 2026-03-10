use uuid::Uuid;

use crate::errors::AppError;

use super::models::{ConversationSummary, ConversationSummaryRow, MessageResponse, MessageResponseRow, SendMessageRequest};
use super::repository::MessageRepository;

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
        request: SendMessageRequest,
    ) -> Result<MessageResponse, AppError> {
        if request.content.trim().is_empty() {
            return Err(AppError::ValidationError("Message content cannot be empty".to_string()));
        }

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

    async fn get_message_response(&self, message_id: Uuid) -> Result<MessageResponseRow, AppError> {
        let rows: Vec<MessageResponseRow> = self
            .repository
            .get_conversation(message_id, message_id, 0, 1)
            .await?;

        rows.into_iter().next().map(Into::into).ok_or_else(|| {
            AppError::NotFoundError("Message not found".to_string())
        })
    }
}
