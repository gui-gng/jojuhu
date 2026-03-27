use actix_web::{get, post, put, delete, web, HttpMessage, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{ApiResponse, PaginationParams};

use super::service::UserService;

/// Get user profile by ID
#[get("/{id}")]
pub async fn get_user_profile(
    path: web::Path<Uuid>,
    req: HttpRequest,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    
    // Try to get current user ID from request if authenticated
    let current_user_id = req
        .extensions()
        .get::<AuthenticatedUser>()
        .map(|u| u.user_id);

    let profile = service
        .get_user_profile(target_user_id, current_user_id)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(profile)))
}

/// Get authenticated user's profile
#[get("/me")]
pub async fn get_my_profile(
    user: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let profile = service.get_my_profile(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(profile)))
}

/// Update user profile
#[put("/me")]
pub async fn update_profile(
    user: AuthenticatedUser,
    request: web::Json<super::models::UpdateProfileRequest>,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    service.update_profile(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"message": "Profile updated"}))))
}

/// Update user avatar
#[put("/me/avatar")]
pub async fn update_avatar(
    user: AuthenticatedUser,
    request: web::Json<super::models::UpdateAvatarRequest>,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    service.update_avatar(user.user_id, request.avatar_url.clone()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"message": "Avatar updated"}))))
}

/// Follow a user
#[post("/{id}/follow")]
pub async fn follow_user(
    path: web::Path<Uuid>,
    user: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    service.follow_user(user.user_id, target_user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"following": true}))))
}

/// Unfollow a user
#[delete("/{id}/follow")]
pub async fn unfollow_user(
    path: web::Path<Uuid>,
    user: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    service.unfollow_user(user.user_id, target_user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"following": false}))))
}

/// Get user's followers list
#[get("/{id}/followers")]
pub async fn get_followers(
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
    req: HttpRequest,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    let page = query.page.unwrap_or(1) as i64;
    let per_page = query.per_page.unwrap_or(20).min(100) as i64;

    // Try to get current user ID from request if authenticated
    let current_user_id = req
        .extensions()
        .get::<AuthenticatedUser>()
        .map(|u| u.user_id);

    let response = service
        .get_followers(target_user_id, current_user_id, page, per_page)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Get user's following list
#[get("/{id}/following")]
pub async fn get_following(
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
    req: HttpRequest,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    let page = query.page.unwrap_or(1) as i64;
    let per_page = query.per_page.unwrap_or(20).min(100) as i64;

    // Try to get current user ID from request if authenticated
    let current_user_id = req
        .extensions()
        .get::<AuthenticatedUser>()
        .map(|u| u.user_id);

    let response = service
        .get_following(target_user_id, current_user_id, page, per_page)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Block a user
#[post("/{id}/block")]
pub async fn block_user(
    path: web::Path<Uuid>,
    user: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    let blocked = service.block_user(user.user_id, target_user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"blocked": blocked}))))
}

/// Unblock a user
#[delete("/{id}/block")]
pub async fn unblock_user(
    path: web::Path<Uuid>,
    user: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();
    service.unblock_user(user.user_id, target_user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"blocked": false}))))
}

/// Get blocked users list
#[get("/me/blocked")]
pub async fn get_blocked_users(
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1) as i64;
    let per_page = query.per_page.unwrap_or(20).min(100) as i64;

    let response = service
        .get_blocked_users(user.user_id, page, per_page)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}