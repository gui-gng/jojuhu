use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{
    errors::AppError,
    middleware::auth::AuthenticatedUser,
    models::{ApiResponse, PaginationParams},
};

use super::{
    models::{UpdateAvatarRequest, UpdateProfileRequest},
    service::UserService,
};

/// Get current user's profile
#[get("/me")]
pub async fn get_my_profile(
    user: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let profile = service.get_my_profile(user.user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(profile)))
}

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

/// Update current user's profile
#[put("/me")]
pub async fn update_profile(
    user: AuthenticatedUser,
    request: web::Json<UpdateProfileRequest>,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    service.update_profile(user.user_id, request.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "Profile updated successfully"
    }))))
}

/// Update current user's avatar
#[put("/me/avatar")]
pub async fn update_avatar(
    user: AuthenticatedUser,
    request: web::Json<UpdateAvatarRequest>,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    service
        .update_avatar(user.user_id, request.avatar_url.clone())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "Avatar updated successfully"
    }))))
}

/// Follow a user
#[post("/{id}/follow")]
pub async fn follow_user(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();

    service.follow_user(user.user_id, target_user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "User followed successfully"
    }))))
}

/// Unfollow a user
#[delete("/{id}/follow")]
pub async fn unfollow_user(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let target_user_id = path.into_inner();

    service.unfollow_user(user.user_id, target_user_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "User unfollowed successfully"
    }))))
}

/// Get user's followers
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
