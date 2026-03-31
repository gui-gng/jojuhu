use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

use super::models::{CreateStoryRequest, ViewStoryRequest};
use super::service::StoryService;

pub async fn create_story(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
    request: web::Json<CreateStoryRequest>,
) -> Result<HttpResponse, AppError> {
    let story = service.create_story(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(story)))
}

pub async fn get_user_stories(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    let stories = service.get_user_stories(target_user_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(stories)))
}

pub async fn get_my_stories(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let stories = service.get_user_stories(user.user_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(stories)))
}

pub async fn get_stories_feed(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let stories = service.get_stories_feed(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(stories)))
}

pub async fn get_following_stories(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let stories = service.get_following_stories(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(stories)))
}

pub async fn view_story(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<ViewStoryRequest>,
) -> Result<HttpResponse, AppError> {
    let story_id = path.into_inner();
    service.view_story(story_id, user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"message": "Story viewed"}))))
}

pub async fn get_story_viewers(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let story_id = path.into_inner();
    let viewers = service.get_story_viewers(story_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(viewers)))
}

pub async fn delete_story(
    service: web::Data<StoryService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let story_id = path.into_inner();
    service.delete_story(story_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
