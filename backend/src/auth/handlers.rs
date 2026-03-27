use actix_web::{web, HttpResponse};

use crate::auth::{
    confirm_password_reset, login, register, request_password_reset, resend_verification_email,
    verify_email, LoginRequest,
};
use crate::auth::models::{ConfirmPasswordResetRequest, RequestPasswordResetRequest, VerifyEmailRequest};
use crate::config::Settings;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::middleware::validation::{validate_content_length, validate_non_empty_string};
use crate::models::user::CreateUserRequest;
use crate::utils::validators::{validate_password, validate_username};

/// Handler for user registration
///
/// Validates the request before creating a new user account.
/// Performs the following validations:
/// - Username: 3-32 alphanumeric characters + underscores
/// - Email: Valid email format (checked by struct validation)
/// - Password: 8-128 characters
pub async fn register_handler(
    pool: web::Data<sqlx::PgPool>,
    settings: web::Data<Settings>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let req = request.into_inner();

    // Validate username
    validate_username(&req.username).map_err(|e| {
        AppError::ValidationError(format!("Invalid username: {:?}", e))
    })?;

    // Validate password
    validate_password(&req.password).map_err(|e| {
        AppError::ValidationError(format!("Invalid password: {:?}", e))
    })?;

    // Validate display name if provided
    if let Some(ref display_name) = req.display_name {
        validate_content_length(display_name, 1, 100, "Display name")?;
    }

    register(pool.get_ref(), settings.get_ref(), req).await
}

/// Handler for user login
///
/// Validates credentials format before attempting authentication.
/// Note: Actual authentication is performed by the auth service.
pub async fn login_handler(
    pool: web::Data<sqlx::PgPool>,
    settings: web::Data<Settings>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let req = request.into_inner();

    // Validate username/email format
    validate_non_empty_string(&req.username_or_email, "Username or email")?;
    validate_content_length(&req.username_or_email, 3, 100, "Username or email")?;

    // Validate password is not empty
    validate_non_empty_string(&req.password, "Password")?;

    login(pool.get_ref(), settings.get_ref(), req).await
}

/// Handler for getting current user information
///
/// Returns the profile information for the authenticated user.
pub async fn get_current_user_handler(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    crate::auth::get_current_user(pool.get_ref(), user).await
}

/// Handler for requesting password reset
///
/// Sends a password reset email to the user if the email exists.
pub async fn request_password_reset_handler(
    pool: web::Data<sqlx::PgPool>,
    request: web::Json<RequestPasswordResetRequest>,
) -> Result<HttpResponse, AppError> {
    request_password_reset(pool.get_ref(), &request.email).await?;

    // Always return success to prevent email enumeration
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "If an account exists with this email, a password reset link has been sent"
    })))
}

/// Handler for confirming password reset
///
/// Resets the password using the token from the email.
pub async fn confirm_password_reset_handler(
    pool: web::Data<sqlx::PgPool>,
    request: web::Json<ConfirmPasswordResetRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate password
    validate_password(&request.new_password)?;

    confirm_password_reset(pool.get_ref(), &request.token, &request.new_password).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password reset successfully"
    })))
}

/// Handler for verifying email
///
/// Verifies the user's email address using the token from the email.
pub async fn verify_email_handler(
    pool: web::Data<sqlx::PgPool>,
    request: web::Json<VerifyEmailRequest>,
) -> Result<HttpResponse, AppError> {
    verify_email(pool.get_ref(), &request.token).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Email verified successfully"
    })))
}

/// Handler for resending verification email
///
/// Resends the verification email to the authenticated user.
pub async fn resend_verification_handler(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    resend_verification_email(pool.get_ref(), user.user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Verification email sent"
    })))
}
