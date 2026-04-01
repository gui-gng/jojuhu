use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;

use super::models::{
    CommentResponse, CommentResponseRow, CreateCommentRequest, CreatePostRequest, CreateRepostRequest, FeedSort, PostResponse, PostResponseRow, RepostResponse, UpdatePostRequest,
};
use super::repository::TimelineRepository;

/// Maximum post content length
const MAX_POST_LENGTH: usize = 5000;
/// Maximum comment content length
const MAX_COMMENT_LENGTH: usize = 1000;
/// Minimum content length
const MIN_CONTENT_LENGTH: usize = 1;

pub struct TimelineService {
    pub repository: TimelineRepository,
}

impl TimelineService {
    pub fn new(repository: TimelineRepository) -> Self {
        Self { repository }
    }

    pub async fn create_post(
        &self,
        author_id: Uuid,
        mut request: CreatePostRequest,
    ) -> Result<PostResponse, AppError> {
        // Validate content length
        validate_content_length(&request.content, MIN_CONTENT_LENGTH, MAX_POST_LENGTH, "Post content")?;
        
        // Check for suspicious patterns
        validate_input_safety(&request.content)
            .map_err(AppError::ValidationError)?;
        
        // Sanitize content
        request.content = sanitize_input(&request.content);

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
        sort: &FeedSort,
    ) -> Result<Vec<PostResponse>, AppError> {
        let rows: Vec<PostResponseRow> = self.repository.get_feed(user_id, offset, limit, sort).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_following_feed(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
        sort: &FeedSort,
    ) -> Result<Vec<PostResponse>, AppError> {
        let rows: Vec<PostResponseRow> = self.repository.get_following_feed(user_id, offset, limit, sort).await?;
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
        mut request: CreateCommentRequest,
    ) -> Result<CommentResponse, AppError> {
        // Validate content length
        validate_content_length(
            &request.content,
            MIN_CONTENT_LENGTH,
            MAX_COMMENT_LENGTH,
            "Comment content",
        )?;

        // Check for suspicious patterns
        validate_input_safety(&request.content).map_err(AppError::ValidationError)?;

        // Sanitize content
        request.content = sanitize_input(&request.content);

        let comment = self
            .repository
            .create_comment(post_id, author_id, &request.content, request.parent_comment_id)
            .await?;

        self.repository
            .get_comment_response_by_id(comment.id)
            .await?
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

    // ==================== Repost operations ====================

    /// Create a repost (with optional quote text)
    pub async fn create_repost(
        &self,
        user_id: Uuid,
        request: CreateRepostRequest,
    ) -> Result<RepostResponse, AppError> {
        // Validate quote text if provided
        if let Some(ref quote) = request.quote_text {
            if !quote.trim().is_empty() {
                validate_content_length(quote, 1, 500, "Quote text")?;
                validate_input_safety(quote).map_err(AppError::ValidationError)?;
            }
        }

        // Check if original post exists
        let _ = self.repository.get_post_by_id(request.original_post_id).await?;

        // Check if user already reposted this post
        if self.repository.is_reposted(request.original_post_id, user_id).await? {
            return Err(AppError::ValidationError(
                "You have already reposted this post".to_string(),
            ));
        }

        let quote_text = request.quote_text.as_deref().filter(|q| !q.trim().is_empty());
        
        let _repost = self.repository
            .create_repost(request.original_post_id, user_id, quote_text)
            .await?;

        self.repository
            .get_user_reposts(user_id, 0, 1)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| AppError::NotFoundError("Repost not found".to_string()))
    }

    /// Remove a repost
    pub async fn delete_repost(
        &self,
        user_id: Uuid,
        post_id: Uuid,
    ) -> Result<(), AppError> {
        self.repository.delete_repost(post_id, user_id).await
    }

    /// Check if a post is reposted by user
    #[allow(dead_code)]
    pub async fn is_reposted(
        &self,
        post_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, AppError> {
        self.repository.is_reposted(post_id, user_id).await
    }

    /// Get user's reposts
    pub async fn get_user_reposts(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<RepostResponse>, AppError> {
        self.repository.get_user_reposts(user_id, offset, limit).await
    }

    // ==================== Algorithmic Feed ====================

    /// Get personalized "For You" feed with engagement-based ranking
    pub async fn get_for_you_feed(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<PostResponse>, AppError> {
        let rows = self.repository.get_for_you_feed(user_id, offset, limit).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get trending posts from the last 7 days
    pub async fn get_trending_posts(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<PostResponse>, AppError> {
        let rows = self.repository.get_trending_posts(user_id, offset, limit).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Get suggested users to follow
    pub async fn get_suggested_users(
        &self,
        user_id: Uuid,
        limit: i32,
    ) -> Result<Vec<super::repository::SuggestedUser>, AppError> {
        self.repository.get_suggested_users(user_id, limit).await
    }
}
