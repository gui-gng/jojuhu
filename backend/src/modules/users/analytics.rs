use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

use super::service::UserService;

#[derive(Debug, Serialize)]
pub struct UserAnalyticsResponse {
    pub posts_count: i64,
    pub followers_count: i64,
    pub following_count: i64,
    pub posts_this_week: i64,
    pub posts_this_month: i64,
    pub avg_likes_per_post: f64,
    pub avg_comments_per_post: f64,
}

#[get("/me/analytics")]
pub async fn get_user_analytics(
    user: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let analytics = service.get_user_analytics(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(analytics)))
}