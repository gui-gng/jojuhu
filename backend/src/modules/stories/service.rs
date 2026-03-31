use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;

use super::models::{CreateStoryRequest, Story, StoryResponse, StoryViewer, ViewStoryRequest};
use super::repository::StoryRepository;

/// Maximum story caption length
const MAX_CAPTION_LENGTH: usize = 500;
/// Valid media types
const VALID_MEDIA_TYPES: &[&str] = &["image", "video"];

pub struct StoryService {
    repository: StoryRepository,
}

impl StoryService {
    pub fn new(repository: StoryRepository) -> Self {
        Self { repository }
    }

    pub async fn create_story(
        &self,
        user_id: Uuid,
        mut request: CreateStoryRequest,
    ) -> Result<StoryResponse, AppError> {
        // Validate media URL is not empty
        if request.media_url.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Media URL is required".to_string(),
            ));
        }

        // Validate media type
        let media_type = request
            .media_type
            .as_deref()
            .unwrap_or("image")
            .to_lowercase();
        if !VALID_MEDIA_TYPES.contains(&media_type.as_str()) {
            return Err(AppError::ValidationError(format!(
                "Invalid media type. Must be one of: {}",
                VALID_MEDIA_TYPES.join(", ")
            )));
        }

        // Validate and sanitize caption if provided
        if let Some(ref caption) = request.caption {
            if !caption.trim().is_empty() {
                validate_content_length(caption, 0, MAX_CAPTION_LENGTH, "Caption")?;
                validate_input_safety(caption).map_err(AppError::ValidationError)?;
                request.caption = Some(sanitize_input(caption));
            }
        }

        let story = self
            .repository
            .create(
                user_id,
                &request.media_url,
                &media_type,
                request.caption.as_deref(),
            )
            .await?;

        self.build_story_response(story, user_id, false).await
    }

    pub async fn get_user_stories(
        &self,
        user_id: Uuid,
        viewer_id: Uuid,
    ) -> Result<Vec<StoryResponse>, AppError> {
        let stories = self.repository.get_active_stories(user_id, viewer_id).await?;
        let mut responses = Vec::new();

        for story in stories {
            let is_viewed = self.is_story_viewed(story.id, viewer_id).await?;
            let response = self.build_story_response(story, viewer_id, is_viewed).await?;
            responses.push(response);
        }

        Ok(responses)
    }

    pub async fn get_following_stories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<StoryResponse>, AppError> {
        let stories = self.repository.get_following_stories(user_id).await?;
        let mut responses = Vec::new();

        for story in stories {
            let is_viewed = self.is_story_viewed(story.id, user_id).await?;
            let response = self.build_story_response(story, user_id, is_viewed).await?;
            responses.push(response);
        }

        Ok(responses)
    }

    pub async fn view_story(
        &self,
        story_id: Uuid,
        viewer_id: Uuid,
        request: ViewStoryRequest,
    ) -> Result<(), AppError> {
        // Validate reaction if provided
        if let Some(ref reaction) = request.reaction {
            validate_input_safety(reaction).map_err(AppError::ValidationError)?;
        }

        self.repository
            .record_view(story_id, viewer_id, request.reaction.as_deref())
            .await
    }

    pub async fn get_story_viewers(
        &self,
        story_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<StoryViewer>, AppError> {
        // Verify the user owns the story
        let stories = self.repository.get_active_stories(user_id, user_id).await?;
        if !stories.iter().any(|s| s.id == story_id) {
            // Also check if the story exists at all (even if expired)
            let all_stories = self.repository.get_following_stories(user_id).await?;
            if !all_stories.iter().any(|s| s.id == story_id) {
                return Err(AppError::NotFoundError("Story not found".to_string()));
            }
        }

        self.repository.get_viewers(story_id).await
    }

    pub async fn delete_story(&self, story_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.delete(story_id, user_id).await
    }

    async fn is_story_viewed(&self, story_id: Uuid, viewer_id: Uuid) -> Result<bool, AppError> {
        // Check if user has viewed the story by looking at viewers
        let viewers = self.repository.get_viewers(story_id).await?;
        Ok(viewers.iter().any(|v| v.user_id == viewer_id))
    }

    async fn build_story_response(
        &self,
        story: Story,
        _viewer_id: Uuid,
        is_viewed: bool,
    ) -> Result<StoryResponse, AppError> {
        // Get user info from a query - we'll need to add this to repository
        // For now, we'll construct a basic response
        // In a real implementation, you'd want to fetch user details
        Ok(StoryResponse {
            id: story.id,
            user_id: story.user_id,
            username: String::new(), // Will be populated by handler
            display_name: None,
            avatar_url: None,
            media_url: story.media_url,
            media_type: story.media_type,
            caption: story.caption,
            created_at: story.created_at,
            expires_at: story.expires_at,
            view_count: story.view_count,
            is_viewed,
        })
    }

    #[allow(dead_code)]
    async fn get_user_info(
        &self,
        _user_id: Uuid,
    ) -> Result<(String, Option<String>, Option<String>), AppError> {
        // This should be implemented to get user details
        // For now, return empty strings
        // In a real implementation, you'd fetch from user repository
        Ok((String::new(), None, None))
    }
}
