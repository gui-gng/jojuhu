use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

use super::scheduled::{
    CreateScheduledPostRequest, RegisterPushTokenRequest, UpdateScheduledPostRequest,
};
use super::service::ScheduledPostsService;

pub async fn create_scheduled_post(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    request: web::Json<CreateScheduledPostRequest>,
) -> Result<HttpResponse, AppError> {
    let post = service.create_scheduled_post(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(post)))
}

pub async fn get_scheduled_posts(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    query: web::Query<super::routes::PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    
    let posts = service.get_scheduled_posts(user.user_id, limit, offset).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_scheduled_post(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    match service.get_scheduled_post(user.user_id, post_id).await? {
        Some(post) => Ok(HttpResponse::Ok().json(ApiResponse::success(post))),
        None => Err(AppError::NotFoundError("Scheduled post not found".to_string())),
    }
}

pub async fn update_scheduled_post(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<UpdateScheduledPostRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let post = service.update_scheduled_post(user.user_id, post_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(post)))
}

pub async fn cancel_scheduled_post(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.cancel_scheduled_post(user.user_id, post_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"cancelled": true}))))
}

pub async fn save_draft(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    request: web::Json<super::scheduled::SaveDraftRequest>,
) -> Result<HttpResponse, AppError> {
    let req = request.into_inner();
    let draft = service
        .save_draft(user.user_id, req.content, req.media_urls, req.visibility)
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(draft)))
}

pub async fn get_draft(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let draft = service.get_draft(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(draft)))
}

pub async fn delete_draft(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    service.delete_draft(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"deleted": true}))))
}

pub async fn register_push_token(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    request: web::Json<RegisterPushTokenRequest>,
) -> Result<HttpResponse, AppError> {
    let token = service.register_push_token(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(token)))
}

pub async fn deactivate_push_token(
    service: web::Data<ScheduledPostsService>,
    user: AuthenticatedUser,
    request: web::Json<super::scheduled::DeactivatePushTokenRequest>,
) -> Result<HttpResponse, AppError> {
    service.deactivate_push_token(user.user_id, request.device_token.clone()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"deactivated": true}))))
}