use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{ApiResponse, PaginationParams};
use crate::modules::users::repository::UserRepository;

use super::models::{CreateGroupRequest, UpdateGroupRequest};
use super::service::GroupService;

pub async fn create_group(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    request: web::Json<CreateGroupRequest>,
) -> Result<HttpResponse, AppError> {
    let group = service.create_group(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(group)))
}

pub async fn get_group(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    let group = service.get_group(group_id, Some(user.user_id)).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(group)))
}

pub async fn list_groups(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();
    let groups = service.list_groups(offset, limit, Some(user.user_id)).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(groups)))
}

pub async fn update_group(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<UpdateGroupRequest>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    let group = service.update_group(group_id, user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(group)))
}

pub async fn delete_group(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    service.delete_group(group_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn join_group(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    service.join_group(group_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"joined": true}))))
}

pub async fn leave_group(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    service.leave_group(group_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"left": true}))))
}

pub async fn get_members(
    service: web::Data<GroupService>,
    pool: web::Data<sqlx::PgPool>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();
    
    let user_repo = UserRepository::new(pool.get_ref().clone());
    let members = service.get_members(group_id, offset, limit, &user_repo).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(members)))
}