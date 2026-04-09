use actix_web::{web, HttpResponse};

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

use super::models::{RequestAccountDeletionRequest, UpdatePrivacySettingsRequest};
use super::service::PrivacyService;

pub async fn get_privacy_settings(
    service: web::Data<PrivacyService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let settings = service.get_privacy_settings(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(settings)))
}

pub async fn update_privacy_settings(
    service: web::Data<PrivacyService>,
    user: AuthenticatedUser,
    request: web::Json<UpdatePrivacySettingsRequest>,
) -> Result<HttpResponse, AppError> {
    let settings = service.update_privacy_settings(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(settings)))
}

pub async fn request_data_export(
    service: web::Data<PrivacyService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let request = service.request_data_export(user.user_id).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(request)))
}

pub async fn get_data_export_status(
    service: web::Data<PrivacyService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let request = service.get_data_export_status(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(request)))
}

pub async fn request_account_deletion(
    service: web::Data<PrivacyService>,
    user: AuthenticatedUser,
    request: web::Json<RequestAccountDeletionRequest>,
) -> Result<HttpResponse, AppError> {
    let deletion_request = service
        .request_account_deletion(user.user_id, request.reason.clone())
        .await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(deletion_request)))
}

pub async fn cancel_account_deletion(
    service: web::Data<PrivacyService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    service.cancel_account_deletion(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"cancelled": true}))))
}

pub async fn get_deletion_request(
    service: web::Data<PrivacyService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let request = service.get_deletion_request(user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(request)))
}