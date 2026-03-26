use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{ApiResponse, PaginationParams};

use super::models::SendMessageRequest;
use super::service::MessageService;

pub async fn send_message(
    service: web::Data<MessageService>,
    user: AuthenticatedUser,
    request: web::Json<SendMessageRequest>,
) -> Result<HttpResponse, AppError> {
    let message = service.send_message(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(message)))
}

pub async fn get_conversations(
    service: web::Data<MessageService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let conversations = service.get_conversations(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(conversations)))
}

pub async fn get_conversation(
    service: web::Data<MessageService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let other_user_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();

    let thread = service
        .get_conversation(user.user_id, other_user_id, offset, limit)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(thread)))
}

pub async fn mark_as_read(
    service: web::Data<MessageService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();
    service.mark_message_as_read(message_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"message": "Marked as read"}))))
}

pub async fn delete_message(
    service: web::Data<MessageService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();
    service.delete_message(message_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
