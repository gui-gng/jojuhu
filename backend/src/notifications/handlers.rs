use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;
use crate::notifications::NotificationService;

/// Get user notifications
#[get("")]
pub async fn get_notifications(
    service: web::Data<NotificationService>,
    user: AuthenticatedUser,
    query: web::Query<GetNotificationsQuery>,
) -> Result<HttpResponse, AppError> {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).min(100);
    let unread_only = query.unread_only.unwrap_or(false);
    
    let notifications = service
        .get_notifications(user.user_id, unread_only, offset, limit)
        .await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(notifications)))
}

/// Get unread notification count
#[get("/count")]
pub async fn get_unread_count(
    service: web::Data<NotificationService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let count = service.get_unread_count(user.user_id).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "unread_count": count
    }))))
}

/// Mark notification as read
#[post("/{id}/read")]
pub async fn mark_as_read(
    service: web::Data<NotificationService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();
    service.mark_as_read(notification_id, user.user_id).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "Notification marked as read"
    }))))
}

/// Mark all notifications as read
#[post("/read-all")]
pub async fn mark_all_as_read(
    service: web::Data<NotificationService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    service.mark_all_as_read(user.user_id).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "All notifications marked as read"
    }))))
}

/// Delete notification
#[delete("/{id}")]
pub async fn delete_notification(
    service: web::Data<NotificationService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let notification_id = path.into_inner();
    service.delete_notification(notification_id, user.user_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

/// Query parameters for getting notifications
#[derive(Debug, serde::Deserialize)]
pub struct GetNotificationsQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub unread_only: Option<bool>,
}