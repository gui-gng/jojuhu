use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    AuthenticationError(String),
    AuthorizationError(String),
    NotFoundError(String),
    ConflictError(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::AuthorizationError(msg) => write!(f, "Authorization error: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "Not found: {}", msg),
            AppError::ConflictError(msg) => write!(f, "Conflict: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::DatabaseError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
            AppError::ValidationError(msg) => {
                (actix_web::http::StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::AuthenticationError(msg) => {
                (actix_web::http::StatusCode::UNAUTHORIZED, msg.clone())
            }
            AppError::AuthorizationError(msg) => {
                (actix_web::http::StatusCode::FORBIDDEN, msg.clone())
            }
            AppError::NotFoundError(msg) => (actix_web::http::StatusCode::NOT_FOUND, msg.clone()),
            AppError::ConflictError(msg) => (actix_web::http::StatusCode::CONFLICT, msg.clone()),
            AppError::InternalError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
        };

        HttpResponse::build(status).json(json!({
            "success": false,
            "error": message
        }))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::NotFoundError("Record not found".to_string()),
            sqlx::Error::Database(db_err) => {
                if db_err.constraint().is_some() {
                    AppError::ConflictError(db_err.to_string())
                } else {
                    AppError::DatabaseError(db_err.to_string())
                }
            }
            _ => AppError::DatabaseError(error.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::ValidationError(error.to_string())
    }
}
