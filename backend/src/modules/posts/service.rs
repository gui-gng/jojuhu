use uuid::Uuid;

use crate::errors::AppError;

use super::repository::ScheduledPostsRepository;
use super::scheduled::{
    CreateScheduledPostRequest, PostDraft, PushNotificationTokenResponse,
    RegisterPushTokenRequest, ScheduledPostResponse,
    UpdateScheduledPostRequest,
};

pub struct ScheduledPostsService {
    repository: ScheduledPostsRepository,
}

impl ScheduledPostsService {
    pub fn new(repository: ScheduledPostsRepository) -> Self {
        Self { repository }
    }

    pub async fn create_scheduled_post(
        &self,
        user_id: Uuid,
        request: CreateScheduledPostRequest,
    ) -> Result<ScheduledPostResponse, AppError> {
        let post = self.repository.create_scheduled_post(user_id, request).await?;
        Ok(ScheduledPostResponse {
            id: post.id,
            content: post.content,
            media_urls: post.media_urls,
            scheduled_for: post.scheduled_for,
            status: post.status,
            created_at: post.created_at,
        })
    }

    pub async fn get_scheduled_posts(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<ScheduledPostResponse>, AppError> {
        let posts = self.repository.get_scheduled_posts(user_id, limit, offset).await?;
        Ok(posts
            .into_iter()
            .map(|p| ScheduledPostResponse {
                id: p.id,
                content: p.content,
                media_urls: p.media_urls,
                scheduled_for: p.scheduled_for,
                status: p.status,
                created_at: p.created_at,
            })
            .collect())
    }

    pub async fn get_scheduled_post(
        &self,
        user_id: Uuid,
        post_id: Uuid,
    ) -> Result<Option<ScheduledPostResponse>, AppError> {
        let post = self.repository.get_scheduled_post(user_id, post_id).await?;
        Ok(post.map(|p| ScheduledPostResponse {
            id: p.id,
            content: p.content,
            media_urls: p.media_urls,
            scheduled_for: p.scheduled_for,
            status: p.status,
            created_at: p.created_at,
        }))
    }

    pub async fn update_scheduled_post(
        &self,
        user_id: Uuid,
        post_id: Uuid,
        request: UpdateScheduledPostRequest,
    ) -> Result<ScheduledPostResponse, AppError> {
        let post = self.repository.update_scheduled_post(user_id, post_id, request).await?;
        Ok(ScheduledPostResponse {
            id: post.id,
            content: post.content,
            media_urls: post.media_urls,
            scheduled_for: post.scheduled_for,
            status: post.status,
            created_at: post.created_at,
        })
    }

    pub async fn cancel_scheduled_post(&self, user_id: Uuid, post_id: Uuid) -> Result<(), AppError> {
        self.repository.cancel_scheduled_post(user_id, post_id).await
    }

    pub async fn save_draft(
        &self,
        user_id: Uuid,
        content: Option<String>,
        media_urls: Option<Vec<String>>,
        visibility: Option<String>,
    ) -> Result<PostDraft, AppError> {
        self.repository.save_draft(user_id, content, media_urls, visibility).await
    }

    pub async fn get_draft(&self, user_id: Uuid) -> Result<Option<PostDraft>, AppError> {
        self.repository.get_draft(user_id).await
    }

    pub async fn delete_draft(&self, user_id: Uuid) -> Result<(), AppError> {
        self.repository.delete_draft(user_id).await
    }

    pub async fn register_push_token(
        &self,
        user_id: Uuid,
        request: RegisterPushTokenRequest,
    ) -> Result<PushNotificationTokenResponse, AppError> {
        let token = self.repository.register_push_token(user_id, request).await?;
        Ok(PushNotificationTokenResponse {
            id: token.id,
            device_type: token.device_type,
            device_name: token.device_name,
            is_active: token.is_active,
            created_at: token.created_at,
        })
    }

    pub async fn deactivate_push_token(&self, user_id: Uuid, device_token: String) -> Result<(), AppError> {
        self.repository.deactivate_push_token(user_id, &device_token).await
    }
}