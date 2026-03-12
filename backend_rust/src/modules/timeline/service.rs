use uuid::Uuid;

use crate::errors::AppError;


use super::models::{
    CommentResponse, CommentResponseRow, CreateCommentRequest, CreatePostRequest, PostResponse, PostResponseRow, UpdatePostRequest,
};
use super::repository::TimelineRepository;

pub struct TimelineService {
    repository: TimelineRepository,
}

impl TimelineService {
    pub fn new(repository: TimelineRepository) -> Self {
        Self { repository }
    }

    pub async fn create_post(
        &self,
        author_id: Uuid,
        request: CreatePostRequest,
    ) -> Result<PostResponse, AppError> {
        if request.content.trim().is_empty() {
            return Err(AppError::ValidationError("Post content cannot be empty".to_string()));
        }

        let post = self
            .repository
            .create_post(
                author_id,
                &request.content,
                request.media_urls,
                request.is_public.unwrap_or(true),
            )
            .await?;

        let row: PostResponseRow = self.repository.get_post_response(post.id, author_id).await?;
        Ok(row.into())
    }

    pub async fn get_feed(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<PostResponse>, AppError> {
        let rows: Vec<PostResponseRow> = self.repository.get_feed(user_id, offset, limit).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_user_posts(
        &self,
        author_id: Uuid,
        requesting_user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<PostResponse>, AppError> {
        let rows: Vec<PostResponseRow> = self.repository
            .get_user_posts(author_id, requesting_user_id, offset, limit)
            .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_post(
        &self,
        post_id: Uuid,
        requesting_user_id: Uuid,
    ) -> Result<PostResponse, AppError> {
        let row: PostResponseRow = self.repository.get_post_response(post_id, requesting_user_id).await?;
        Ok(row.into())
    }

    pub async fn update_post(
        &self,
        post_id: Uuid,
        author_id: Uuid,
        request: UpdatePostRequest,
    ) -> Result<PostResponse, AppError> {
        let post = self.repository.get_post_by_id(post_id).await?;

        if post.author_id != author_id {
            return Err(AppError::AuthorizationError(
                "You can only update your own posts".to_string(),
            ));
        }

        let updated = self
            .repository
            .update_post(
                post_id,
                request.content.as_deref(),
                request.is_public,
            )
            .await?;

        let row = self.repository.get_post_response(updated.id, author_id).await?;
        Ok(row.into())
    }

    pub async fn delete_post(&self, post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let post = self.repository.get_post_by_id(post_id).await?;

        if post.author_id != user_id {
            return Err(AppError::AuthorizationError(
                "You can only delete your own posts".to_string(),
            ));
        }

        self.repository.delete_post(post_id).await
    }

    pub async fn like_post(&self, post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.create_like(post_id, user_id).await?;
        Ok(())
    }

    pub async fn unlike_post(&self, post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.delete_like(post_id, user_id).await
    }

    pub async fn add_comment(
        &self,
        post_id: Uuid,
        author_id: Uuid,
        request: CreateCommentRequest,
    ) -> Result<CommentResponse, AppError> {
        if request.content.trim().is_empty() {
            return Err(AppError::ValidationError("Comment content cannot be empty".to_string()));
        }

        let comment = self
            .repository
            .create_comment(post_id, author_id, &request.content, request.parent_comment_id)
            .await?;

        self.repository.get_comment_response_by_id(comment.id).await?
            .map(Into::into)
            .ok_or_else(|| AppError::NotFoundError("Comment not found".to_string()))
    }

    pub async fn get_comments(
        &self,
        post_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<CommentResponse>, AppError> {
        let rows: Vec<CommentResponseRow> = self.repository.get_comments(post_id, offset, limit).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn delete_comment(
        &self,
        comment_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        self.repository.delete_comment(comment_id, user_id).await
    }
}
