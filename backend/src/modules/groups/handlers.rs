use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{ApiResponse, PaginationParams};
use crate::modules::users::repository::UserRepository;

use super::models::{CreateGroupRequest, InviteToGroupRequest, CreateJoinRequest, UpdateMemberRoleRequest, UpdateGroupRequest};
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

// Invitation handlers
pub async fn invite_user(
    service: web::Data<GroupService>,
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<InviteToGroupRequest>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    let user_repo = UserRepository::new(pool.get_ref().clone());
    
    let invitation = service
        .invite_user(group_id, user.user_id, request.user_id, &user_repo)
        .await?;
    
    Ok(HttpResponse::Created().json(ApiResponse::success(invitation)))
}

pub async fn get_my_invitations(
    service: web::Data<GroupService>,
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();
    let user_repo = UserRepository::new(pool.get_ref().clone());
    
    let invitations = service.get_my_invitations(user.user_id, offset, limit, &user_repo).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(invitations)))
}

pub async fn accept_invitation(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let invitation_id = path.into_inner();
    service.accept_invitation(invitation_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"accepted": true}))))
}

pub async fn reject_invitation(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let invitation_id = path.into_inner();
    service.reject_invitation(invitation_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"rejected": true}))))
}

// Join request handlers
pub async fn request_join_group(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<CreateJoinRequest>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    let join_request = service
        .request_join_group(group_id, user.user_id, request.message.clone())
        .await?;
    
    Ok(HttpResponse::Created().json(ApiResponse::success(join_request)))
}

pub async fn get_join_requests(
    service: web::Data<GroupService>,
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let group_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();
    let user_repo = UserRepository::new(pool.get_ref().clone());
    
    let requests = service.get_join_requests(group_id, user.user_id, offset, limit, &user_repo).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(requests)))
}

pub async fn approve_join_request(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let request_id = path.into_inner();
    service.approve_join_request(request_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"approved": true}))))
}

pub async fn reject_join_request(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let request_id = path.into_inner();
    service.reject_join_request(request_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"rejected": true}))))
}

// Role management handlers
pub async fn update_member_role(
    service: web::Data<GroupService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    request: web::Json<UpdateMemberRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let (group_id, target_user_id) = path.into_inner();
    service
        .update_member_role(group_id, user.user_id, target_user_id, request.role.clone())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"updated": true}))))
}