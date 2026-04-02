use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

use super::models::{CreatePollRequest, VotePollRequest};
use super::service::PollService;

pub async fn create_poll(
    service: web::Data<PollService>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<CreatePollRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let poll = service.create_poll(post_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(poll)))
}

pub async fn get_poll(
    service: web::Data<PollService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let poll = service.get_poll(post_id, Some(user.user_id)).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(poll)))
}

pub async fn vote_poll(
    service: web::Data<PollService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<VotePollRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let poll = service.vote(post_id, request.into_inner(), user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(poll)))
}

pub async fn delete_poll(
    service: web::Data<PollService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.delete_poll(post_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}