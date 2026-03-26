//! Authentication middleware
//!
//! This module provides middleware for authenticating HTTP requests
//! and extracting authenticated user information from JWT tokens.

use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use std::future::{ready, Ready};

use crate::errors::AppError;
use crate::utils::auth::Claims;

/// Represents an authenticated user extracted from a valid JWT token.
///
/// This struct is automatically populated by the authentication middleware
/// and can be injected into request handlers using Actix-web's extractor system.
///
/// # Example
/// ```rust
/// use actix_web::{web, HttpResponse};
/// use social_network::middleware::auth::AuthenticatedUser;
///
/// async fn profile(user: AuthenticatedUser) -> HttpResponse {
///     HttpResponse::Ok().body(format!("Hello, {}", user.username))
/// }
/// ```
pub struct AuthenticatedUser {
    /// Unique identifier for the user (UUID v4)
    pub user_id: uuid::Uuid,
    /// User's display name (may differ from username)
    pub username: String,
    /// User's email address (verified)
    pub email: String,
}

impl From<Claims> for AuthenticatedUser {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            username: claims.username,
            email: claims.email,
        }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        
        match extensions.get::<Claims>() {
            Some(claims) => ready(Ok(AuthenticatedUser::from(claims.clone()))),
            None => ready(Err(AppError::AuthenticationError(
                "User not authenticated".to_string()
            ))),
        }
    }
}
