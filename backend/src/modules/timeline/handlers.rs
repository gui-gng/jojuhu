use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{ApiResponse, PaginationParams};

use super::models::{CreateCommentRequest, CreatePostRequest, UpdatePostRequest};
use super::service::TimelineService;

pub async fn create_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    request: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, AppError> {
    let post = service.create_post(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(post)))
}

pub async fn get_feed(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();

    let posts = service.get_feed(user.user_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_following_feed(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();

    let posts = service.get_following_feed(user.user_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_user_posts(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let author_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();

    let posts = service
        .get_user_posts(author_id, user.user_id, offset, limit)
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let post = service.get_post(post_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(post)))
}

pub async fn update_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let post = service
        .update_post(post_id, user.user_id, request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(post)))
}

pub async fn delete_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.delete_post(post_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn like_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.like_post(post_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"liked": true}))))
}

pub async fn unlike_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.unlike_post(post_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"liked": false}))))
}

pub async fn add_comment(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<CreateCommentRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let comment = service
        .add_comment(post_id, user.user_id, request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(comment)))
}

pub async fn get_comments(
    service: web::Data<TimelineService>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();

    let comments = service.get_comments(post_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(comments)))
}

pub async fn delete_comment(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_post_id, comment_id) = path.into_inner();
    service.delete_comment(comment_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
