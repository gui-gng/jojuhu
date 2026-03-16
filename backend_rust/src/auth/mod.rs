pub mod handlers;
pub mod models;



use crate::config::Settings;
use crate::middleware::auth::AuthenticatedUser;
use crate::utils::auth::generate_token;
use crate::utils::{hash_password, verify_password};
use crate::errors::AppError;
use crate::models::user::{CreateUserRequest, User, UserResponse};
use crate::models::ApiResponse;
use actix_web::HttpResponse;
use sqlx::PgPool;

/// Register a new user
pub async fn register(
    pool: &PgPool,
    settings: &Settings,
    request: CreateUserRequest,
) -> Result<HttpResponse, AppError> {
    // Validate input
    if request.username.len() < 3 || request.username.len() > 32 {
        return Err(AppError::ValidationError(
            "Username must be between 3 and 32 characters".to_string(),
        ));
    }

    if request.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Hash password
    let password_hash = hash_password(&request.password)
        .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash, display_name)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&request.username)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(request.display_name.as_ref().unwrap_or(&request.username))
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.constraint().is_some() => {
            AppError::ConflictError("Username or email already exists".to_string())
        }
        _ => e.into(),
    })?;

    // Generate token
    let token = generate_token(user.id, user.username.clone(), user.email.clone(), settings)?;

    let response = serde_json::json!({
        "user": UserResponse::from(user),
        "token": token
    });

    Ok(HttpResponse::Created().json(ApiResponse::success(response)))
}

/// Login request payload
#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

/// Login a user
pub async fn login(
    pool: &PgPool,
    settings: &Settings,
    request: LoginRequest,
) -> Result<HttpResponse, AppError> {
    // Find user by username or email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1 OR email = $1",
    )
    .bind(&request.username_or_email)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    // Verify password
    let valid = verify_password(&request.password, &user.password_hash)
        .map_err(|_| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    if !valid {
        return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
    }

    // Generate token
    let token = generate_token(user.id, user.username.clone(), user.email.clone(), settings)?;

    let response = serde_json::json!({
        "user": UserResponse::from(user),
        "token": token
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Get current authenticated user
pub async fn get_current_user(
    pool: &PgPool,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user.user_id)
        .fetch_one(pool)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(UserResponse::from(user))))
}
