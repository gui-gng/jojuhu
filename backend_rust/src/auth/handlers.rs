use actix_web::{web, HttpResponse};

use crate::auth::{login, register, LoginRequest};
use crate::config::Settings;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::CreateUserRequest;

/// Handler for user registration
pub async fn register_handler(
    pool: web::Data<sqlx::PgPool>,
    settings: web::Data<Settings>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    register(pool.get_ref(), settings.get_ref(), request.into_inner()).await
}

/// Handler for user login
pub async fn login_handler(
    pool: web::Data<sqlx::PgPool>,
    settings: web::Data<Settings>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    login(pool.get_ref(), settings.get_ref(), request.into_inner()).await
}

/// Handler for getting current user
pub async fn get_current_user_handler(
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    crate::auth::get_current_user(pool.get_ref(), user).await
}
