use actix_web::{delete, post, web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;
use crate::modules::users::service::UserService;

use super::models::{ConfirmUploadRequest, PresignedUrlRequest};
use super::service::UploadService;

/// Generate a presigned URL for uploading a file
#[post("/presigned-url")]
pub async fn generate_presigned_url(
    user: AuthenticatedUser,
    service: web::Data<UploadService>,
    request: web::Json<PresignedUrlRequest>,
) -> Result<HttpResponse, AppError> {
    let response = service
        .generate_presigned_url(user.user_id, request.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Confirm avatar upload and update user profile
#[post("/confirm-avatar")]
pub async fn confirm_avatar_upload(
    user: AuthenticatedUser,
    service: web::Data<UploadService>,
    user_service: web::Data<UserService>,
    request: web::Json<ConfirmUploadRequest>,
) -> Result<HttpResponse, AppError> {
    // Verify the file exists in storage
    let exists = service.file_exists(&request.key).await?;
    if !exists {
        return Err(AppError::NotFoundError("File not found in storage".to_string()));
    }

    // Get the public URL
    let file_url = service.get_public_url(&request.key);

    // Update user's avatar
    user_service.update_avatar(user.user_id, file_url.clone()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "Avatar updated successfully",
        "avatar_url": file_url
    }))))
}

/// Delete a file from storage
#[delete("/files/{key}")]
pub async fn delete_file(
    user: AuthenticatedUser,
    service: web::Data<UploadService>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let key = path.into_inner();
    
    // Security check: ensure user can only delete their own files
    // Keys are formatted as: {folder}/{user_id}/{file_id}.{ext}
    let key_parts: Vec<&str> = key.split('/').collect();
    if key_parts.len() >= 2 {
        if let Ok(file_owner_id) = Uuid::parse_str(key_parts[1]) {
            if file_owner_id != user.user_id {
                return Err(AppError::AuthorizationError(
                    "You can only delete your own files".to_string(),
                ));
            }
        }
    }

    service.delete_file(&key).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "message": "File deleted successfully"
    }))))
}
