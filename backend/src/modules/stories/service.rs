use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::{sanitize_input, validate_input_safety};
use crate::middleware::validation::validate_content_length;

use super::models::{CreateStoryRequest, StoryResponse, StoryViewer, ViewStoryRequest};
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

        // Fetch the story with user details
        let story_with_user = self.repository.get_story_with_user(story.id, user_id).await?;
        Ok(story_with_user.into())
    }

    pub async fn get_user_stories(
        &self,
        user_id: Uuid,
        viewer_id: Uuid,
    ) -> Result<Vec<StoryResponse>, AppError> {
        let stories = self.repository.get_active_stories(user_id, viewer_id).await?;
        Ok(stories.into_iter().map(Into::into).collect())
    }

    pub async fn get_following_stories(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<StoryResponse>, AppError> {
        let stories = self.repository.get_following_stories(user_id).await?;
        Ok(stories.into_iter().map(Into::into).collect())
    }

    pub async fn get_stories_feed(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<StoryResponse>, AppError> {
        let stories = self.repository.get_stories_feed(user_id).await?;
        Ok(stories.into_iter().map(Into::into).collect())
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
        // Verify the user owns the story by checking if it's in their active stories
        let stories = self.repository.get_active_stories(user_id, user_id).await?;
        let owns_story = stories.iter().any(|s| s.id == story_id);
        
        if !owns_story {
            return Err(AppError::AuthorizationError(
                "You can only view viewers of your own stories".to_string(),
            ));
        }

        self.repository.get_viewers(story_id).await
    }

    pub async fn delete_story(&self, story_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.delete(story_id, user_id).await
    }
}
