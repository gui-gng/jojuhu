use actix_web::{web, HttpResponse};

use crate::auth::{login, register, LoginRequest};
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
