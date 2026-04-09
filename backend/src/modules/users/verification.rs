use actix_web::{post, web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

use super::service::UserService;
use super::models::VerificationRequest;

#[derive(Debug, serde::Deserialize)]
pub struct SetVerifiedRequest {
    pub is_verified: bool,
}

#[post("/{id}/verify")]
pub async fn set_user_verified(
    service: web::Data<UserService>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<SetVerifiedRequest>,
) -> Result<HttpResponse, AppError> {
    // TODO: Add admin role check here
    // For now, any authenticated user can set verification (should be admin-only in production)
    service.set_user_verified(path.into_inner(), request.is_verified).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "verified": request.is_verified
    }))))
}

#[post("/me/verification")]
pub async fn request_verification(
    service: web::Data<UserService>,
    user: AuthenticatedUser,
    request: web::Json<VerificationRequest>,
) -> Result<HttpResponse, AppError> {
    let verification = service.request_verification(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(verification)))
}