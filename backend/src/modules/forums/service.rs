use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;

use super::models::{
    CreateForumRequest, CreateReplyRequest, CreateTopicRequest, ForumListResponse, ForumResponse,
    ForumRole, ReplyResponse, ReplyResponseRow, TopicResponse, TopicResponseRow, UpdateForumRequest,
};
use super::repository::ForumRepository;

/// Maximum forum/topic title length
const MAX_TITLE_LENGTH: usize = 200;
/// Maximum content length for topics/replies
const MAX_CONTENT_LENGTH: usize = 10000;
/// Minimum content length
const MIN_CONTENT_LENGTH: usize = 1;

pub struct ForumService {
    repository: ForumRepository,
}

impl ForumService {
    pub fn new(repository: ForumRepository) -> Self {
        Self { repository }
    }

    pub async fn create_forum(
        &self,
        creator_id: Uuid,
        mut request: CreateForumRequest,
    ) -> Result<ForumResponse, AppError> {
        // Validate and sanitize forum name
        validate_content_length(&request.name, MIN_CONTENT_LENGTH, MAX_TITLE_LENGTH, "Forum name")?;
        validate_input_safety(&request.name).map_err(AppError::ValidationError)?;
        request.name = sanitize_input(&request.name);

        // Sanitize description if provided
        if let Some(ref mut desc) = request.description {
            validate_content_length(desc, MIN_CONTENT_LENGTH, MAX_CONTENT_LENGTH, "Forum description")?;
            validate_input_safety(desc).map_err(AppError::ValidationError)?;
            *desc = sanitize_input(desc);
        }

        let forum = self
            .repository
            .create_forum(
                &request.name,
                request.description.as_deref(),
                creator_id,
                request.is_public.unwrap_or(true),
            )
            .await?;

        // Add creator as admin
        self.repository
            .add_member(forum.id, creator_id, ForumRole::Admin)
            .await?;

        let row = self.get_forum_response(forum.id, Some(creator_id)).await?;
        Ok(row)
    }

    pub async fn get_forum(
        &self,
        forum_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<ForumResponse, AppError> {
        self.get_forum_response(forum_id, user_id).await
    }

    pub async fn list_forums(
        &self,
        offset: i32,
        limit: i32,
        user_id: Option<Uuid>,
    ) -> Result<Vec<ForumResponse>, AppError> {
        let rows = self.repository.list_forums(offset, limit, user_id).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Search forums with filtering and sorting
    pub async fn search_forums(
        &self,
        user_id: Option<Uuid>,
        search: Option<&str>,
        sort_by: &str,
        page: i64,
        per_page: i64,
    ) -> Result<super::models::ForumListResponse, AppError> {
        let offset = ((page - 1) * per_page) as i32;
        let limit = per_page as i32;

        let rows = self
            .repository
            .search_forums(user_id, search, sort_by, offset, limit)
            .await?;

        let total = self.repository.get_search_count(user_id, search).await?;

        let forums: Vec<ForumResponse> = rows.into_iter().map(Into::into).collect();
        let has_more = (page * per_page) < total;

        Ok(super::models::ForumListResponse {
            forums,
            total,
            page,
            per_page,
            has_more,
        })
    }

    /// Get trending forums
    pub async fn get_trending_forums(
        &self,
        user_id: Option<Uuid>,
        limit: i32,
    ) -> Result<Vec<ForumResponse>, AppError> {
        let rows = self.repository.get_trending_forums(user_id, limit).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn update_forum(
        &self,
        forum_id: Uuid,
        user_id: Uuid,
        request: UpdateForumRequest,
    ) -> Result<ForumResponse, AppError> {
        let member = self
            .repository
            .get_member(forum_id, user_id)
            .await?;

        match member {
            Some(m) if matches!(m.role, ForumRole::Admin | ForumRole::Moderator) => (),
            _ => return Err(AppError::AuthorizationError(
                "Only admins and moderators can update forums".to_string()
            )),
        }

        let forum = self
            .repository
            .update_forum(
                forum_id,
                request.name.as_deref(),
                request.description.as_deref().map(Some),
                request.is_public,
            )
            .await?;

        let row = self.get_forum_response(forum.id, Some(user_id)).await?;
        Ok(row)
    }

    pub async fn delete_forum(&self, forum_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let forum = self.repository.get_forum_by_id(forum_id).await?;

        if forum.creator_id != user_id {
            return Err(AppError::AuthorizationError(
                "Only the creator can delete a forum".to_string(),
            ));
        }

        self.repository.delete_forum(forum_id).await
    }

    pub async fn join_forum(&self, forum_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let forum = self.repository.get_forum_by_id(forum_id).await?;

        if !forum.is_public {
            return Err(AppError::AuthorizationError(
                "This is a private forum".to_string(),
            ));
        }

        self.repository
            .add_member(forum_id, user_id, ForumRole::Member)
            .await?;

        Ok(())
    }

    pub async fn leave_forum(&self, forum_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.remove_member(forum_id, user_id).await
    }

    pub async fn create_topic(
        &self,
        forum_id: Uuid,
        author_id: Uuid,
        mut request: CreateTopicRequest,
    ) -> Result<TopicResponse, AppError> {
        // Validate and sanitize title
        validate_content_length(&request.title, MIN_CONTENT_LENGTH, MAX_TITLE_LENGTH, "Topic title")?;
        validate_input_safety(&request.title).map_err(AppError::ValidationError)?;
        request.title = sanitize_input(&request.title);

        // Validate and sanitize content
        validate_content_length(&request.content, MIN_CONTENT_LENGTH, MAX_CONTENT_LENGTH, "Topic content")?;
        validate_input_safety(&request.content).map_err(AppError::ValidationError)?;
        request.content = sanitize_input(&request.content);

        self.check_forum_membership(forum_id, author_id).await?;

        let topic = self
            .repository
            .create_topic(forum_id, author_id, &request.title, &request.content)
            .await?;

        let row = self.get_topic_response(topic.id).await?;
        Ok(row.into())
    }

    pub async fn get_topic(&self, topic_id: Uuid) -> Result<TopicResponse, AppError> {
        self.repository.increment_topic_views(topic_id).await.ok();
        let row = self.get_topic_response(topic_id).await?;
        Ok(row.into())
    }

    pub async fn get_topics(
        &self,
        forum_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<TopicResponse>, AppError> {
        let rows = self.repository.get_topics(forum_id, offset, limit).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn delete_topic(&self, topic_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let topic = self.repository.get_topic_by_id(topic_id).await?;
        let forum = self.repository.get_forum_by_id(topic.forum_id).await?;

        if topic.author_id != user_id && forum.creator_id != user_id {
            return Err(AppError::AuthorizationError(
                "You can only delete your own topics".to_string(),
            ));
        }

        self.repository.delete_topic(topic_id).await
    }

    pub async fn create_reply(
        &self,
        topic_id: Uuid,
        author_id: Uuid,
        mut request: CreateReplyRequest,
    ) -> Result<ReplyResponse, AppError> {
        let topic = self.repository.get_topic_by_id(topic_id).await?;

        if topic.is_locked {
            return Err(AppError::AuthorizationError(
                "This topic is locked".to_string(),
            ));
        }

        self.check_forum_membership(topic.forum_id, author_id).await?;

        // Validate and sanitize content
        validate_content_length(
            &request.content,
            MIN_CONTENT_LENGTH,
            MAX_CONTENT_LENGTH,
            "Reply content",
        )?;
        validate_input_safety(&request.content).map_err(AppError::ValidationError)?;
        request.content = sanitize_input(&request.content);

        let reply = self
            .repository
            .create_reply(topic_id, author_id, &request.content, request.parent_reply_id)
            .await?;

        let row: ReplyResponseRow = self.get_reply_response(reply.id).await?;
        Ok(row.into())
    }

    pub async fn get_replies(
        &self,
        topic_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<ReplyResponse>, AppError> {
        let rows = self.repository.get_replies(topic_id, offset, limit).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn delete_reply(&self, reply_id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        self.repository.delete_reply(reply_id).await
    }

    pub async fn lock_topic(&self, topic_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let topic = self.repository.get_topic_by_id(topic_id).await?;
        self.check_moderator_permission(topic.forum_id, user_id).await?;
        
        self.repository.update_topic_lock(topic_id, true).await?;
        Ok(())
    }

    pub async fn unlock_topic(&self, topic_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let topic = self.repository.get_topic_by_id(topic_id).await?;
        self.check_moderator_permission(topic.forum_id, user_id).await?;
        
        self.repository.update_topic_lock(topic_id, false).await?;
        Ok(())
    }

    pub async fn pin_topic(&self, topic_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let topic = self.repository.get_topic_by_id(topic_id).await?;
        self.check_moderator_permission(topic.forum_id, user_id).await?;
        
        self.repository.update_topic_pin(topic_id, true).await?;
        Ok(())
    }

    pub async fn unpin_topic(&self, topic_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let topic = self.repository.get_topic_by_id(topic_id).await?;
        self.check_moderator_permission(topic.forum_id, user_id).await?;
        
        self.repository.update_topic_pin(topic_id, false).await?;
        Ok(())
    }

    async fn check_forum_membership(&self, forum_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let forum = self.repository.get_forum_by_id(forum_id).await?;

        if !forum.is_public {
            let member = self.repository.get_member(forum_id, user_id).await?;
            if member.is_none() {
                return Err(AppError::AuthorizationError(
                    "You must be a member to participate in this forum".to_string(),
                ));
            }
        }

        Ok(())
    }

    async fn check_moderator_permission(&self, forum_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let member = self
            .repository
            .get_member(forum_id, user_id)
            .await?;

        match member {
            Some(m) if matches!(m.role, ForumRole::Admin | ForumRole::Moderator) => Ok(()),
            _ => Err(AppError::AuthorizationError(
                "Moderator permission required".to_string()
            )),
        }
    }

    async fn get_forum_response(
        &self,
        forum_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<ForumResponse, AppError> {
        self.repository.get_forum_response_by_id(forum_id, user_id).await?
            .map(Into::into)
            .ok_or_else(|| AppError::NotFoundError("Forum not found".to_string()))
    }

    async fn get_topic_response(&self, topic_id: Uuid) -> Result<TopicResponseRow, AppError> {
        self.repository.get_topic_response_by_id(topic_id).await?
            .ok_or_else(|| AppError::NotFoundError("Topic not found".to_string()))
    }

    async fn get_reply_response(&self, reply_id: Uuid) -> Result<ReplyResponseRow, AppError> {
        self.repository.get_reply_response_by_id(reply_id).await?
            .ok_or_else(|| AppError::NotFoundError("Reply not found".to_string()))
    }
}
