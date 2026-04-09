use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

use super::models::{CreateModerationActionRequest, CreateReportRequest, ReportQuery, SuspendUserRequest};
use super::service::ModerationService;

pub async fn create_report(
    service: web::Data<ModerationService>,
    user: AuthenticatedUser,
    request: web::Json<CreateReportRequest>,
) -> Result<HttpResponse, AppError> {
    let report = service.create_report(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(report)))
}

pub async fn get_report(
    service: web::Data<ModerationService>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let report = service.get_report(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(report)))
}

pub async fn list_reports(
    service: web::Data<ModerationService>,
    _user: AuthenticatedUser,
    query: web::Query<ReportQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    
    let reports = service
        .list_reports(query.status.clone(), query.report_type.clone(), page, per_page)
        .await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(reports)))
}

pub async fn assign_report(
    service: web::Data<ModerationService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let report_id = path.into_inner();
    let report = service.assign_report(report_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(report)))
}

pub async fn resolve_report(
    service: web::Data<ModerationService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<CreateModerationActionRequest>,
) -> Result<HttpResponse, AppError> {
    let report_id = path.into_inner();
    let (report, action) = service
        .resolve_report(
            report_id,
            user.user_id,
            request.action_type.clone(),
            request.target_user_id,
            request.target_content_id,
            request.reason.clone(),
        )
        .await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "report": report,
        "action": action
    }))))
}

pub async fn dismiss_report(
    service: web::Data<ModerationService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<SuspendUserRequest>,
) -> Result<HttpResponse, AppError> {
    let report_id = path.into_inner();
    let (report, action) = service
        .dismiss_report(report_id, user.user_id, Some(request.reason.clone()))
        .await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "report": report,
        "action": action
    }))))
}

pub async fn suspend_user(
    service: web::Data<ModerationService>,
    user: AuthenticatedUser,
    request: web::Json<SuspendUserRequest>,
) -> Result<HttpResponse, AppError> {
    let suspension = service.suspend_user(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(suspension)))
}

pub async fn check_user_suspension(
    service: web::Data<ModerationService>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let is_suspended = service.is_user_suspended(user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "suspended": is_suspended
    }))))
}

pub async fn get_user_suspensions(
    service: web::Data<ModerationService>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let suspensions = service.list_user_suspensions(user_id, 1, 20).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(suspensions)))
}

pub async fn get_pending_reports_count(
    service: web::Data<ModerationService>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let count = service.count_pending_reports().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "pending_reports": count
    }))))
}

pub async fn get_dashboard_stats(
    service: web::Data<ModerationService>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let pending_reports = service.count_pending_reports().await?;
    let reviewing_reports = service.count_reports(Some(super::models::ReportStatus::Reviewing)).await?;
    let total_reports = service.count_reports(None).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "pending_reports": pending_reports,
        "reviewing_reports": reviewing_reports,
        "total_reports": total_reports,
        "resolved_reports": total_reports - pending_reports - reviewing_reports
    }))))
}