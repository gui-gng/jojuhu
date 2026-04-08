use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{ApiResponse, PaginationParams};

use super::models::{BanUserRequest, CreateForumRequest, CreateReplyRequest, CreateTopicRequest, ForumSearchQuery, UpdateForumRequest};
use super::service::ForumService;

pub async fn create_forum(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    request: web::Json<CreateForumRequest>,
) -> Result<HttpResponse, AppError> {
    let forum = service.create_forum(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(forum)))
}

pub async fn list_forums(
    service: web::Data<ForumService>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();

    let forums = service.list_forums(offset, limit, None).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(forums)))
}

pub async fn search_forums(
    service: web::Data<ForumService>,
    query: web::Query<ForumSearchQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1) as i64;
    let per_page = query.per_page.unwrap_or(20).min(100) as i64;
    let sort_by = match query.sort_by {
        Some(super::models::ForumSortBy::Popular) => "popular",
        Some(super::models::ForumSortBy::MostActive) => "most_active",
        Some(super::models::ForumSortBy::MostMembers) => "most_members",
        _ => "newest",
    };

    // Try to get current user ID from request if authenticated
    let current_user_id = req
        .extensions()
        .get::<AuthenticatedUser>()
        .map(|u| u.user_id);

    let response = service
        .search_forums(
            current_user_id,
            query.search.as_deref(),
            sort_by,
            page,
            per_page,
        )
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn get_trending_forums(
    service: web::Data<ForumService>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Try to get current user ID from request if authenticated
    let current_user_id = req
        .extensions()
        .get::<AuthenticatedUser>()
        .map(|u| u.user_id);

    let forums = service.get_trending_forums(current_user_id, 10).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(forums)))
}

pub async fn get_forum(
    service: web::Data<ForumService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let forum_id = path.into_inner();
    let forum = service.get_forum(forum_id, None).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(forum)))
}

pub async fn update_forum(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<UpdateForumRequest>,
) -> Result<HttpResponse, AppError> {
    let forum_id = path.into_inner();
    let forum = service
        .update_forum(forum_id, user.user_id, request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(forum)))
}

pub async fn delete_forum(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let forum_id = path.into_inner();
    service.delete_forum(forum_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn join_forum(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let forum_id = path.into_inner();
    service.join_forum(forum_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"joined": true}))))
}

pub async fn leave_forum(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let forum_id = path.into_inner();
    service.leave_forum(forum_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"joined": false}))))
}

pub async fn create_topic(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<CreateTopicRequest>,
) -> Result<HttpResponse, AppError> {
    let forum_id = path.into_inner();
    let topic = service
        .create_topic(forum_id, user.user_id, request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(topic)))
}

pub async fn get_topics(
    service: web::Data<ForumService>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let forum_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();

    let topics = service.get_topics(forum_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(topics)))
}

pub async fn get_topic(
    service: web::Data<ForumService>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    let topic = service.get_topic(topic_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(topic)))
}

pub async fn delete_topic(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    service.delete_topic(topic_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn create_reply(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    request: web::Json<CreateReplyRequest>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    let reply = service
        .create_reply(topic_id, user.user_id, request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(reply)))
}

pub async fn get_replies(
    service: web::Data<ForumService>,
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();

    let replies = service.get_replies(topic_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(replies)))
}

pub async fn delete_reply(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_, _, reply_id) = path.into_inner();
    service.delete_reply(reply_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn lock_topic(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    service.lock_topic(topic_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"locked": true}))))
}

pub async fn unlock_topic(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    service.unlock_topic(topic_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"locked": false}))))
}

pub async fn pin_topic(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    service.pin_topic(topic_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"pinned": true}))))
}

pub async fn unpin_topic(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_, topic_id) = path.into_inner();
    service.unpin_topic(topic_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"pinned": false}))))
}

// Forum Ban Handlers
pub async fn ban_user(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    request: web::Json<BanUserRequest>,
) -> Result<HttpResponse, AppError> {
    let (forum_id, user_to_ban) = path.into_inner();
    service
        .ban_user_from_forum(
            forum_id,
            user_to_ban,
            user.user_id,
            request.reason.clone(),
            request.expires_at,
        )
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"banned": true}))))
}

pub async fn unban_user(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (forum_id, user_to_unban) = path.into_inner();
    service.unban_user_from_forum(forum_id, user_to_unban, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"banned": false}))))
}

pub async fn check_ban_status(
    service: web::Data<ForumService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (forum_id, user_to_check) = path.into_inner();
    let is_banned = service.is_user_banned(forum_id, user_to_check).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"banned": is_banned}))))
}
