//! Authentication models
//!
//! This module contains data models specific to authentication.
//! For user models, see `crate::models::user`.

use serde::{Deserialize, Serialize};

/// Token response returned after successful authentication
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// Refresh token request
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Password reset request
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PasswordResetRequest {
    pub email: String,
}

/// Change password request
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}
