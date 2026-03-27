//! Authentication models
//!
//! This module contains data models specific to authentication.
//! For user models, see `crate::models::user`.

use serde::{Deserialize, Serialize};

/// Token response returned after successful authentication
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    /// JWT access token
    pub token: String,
    /// Token type, typically "Bearer"
    pub token_type: String,
    /// Token expiration time in seconds
    pub expires_in: i64,
}

/// Refresh token request for obtaining new access tokens
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    /// The refresh token string
    pub refresh_token: String,
}

/// Password reset request (step 1 - request reset)
#[derive(Debug, Deserialize)]
pub struct RequestPasswordResetRequest {
    /// Email address of the account to reset
    pub email: String,
}

/// Password reset request (step 2 - confirm with token)
#[derive(Debug, Deserialize)]
pub struct ConfirmPasswordResetRequest {
    /// Reset token from email
    pub token: String,
    /// New password
    pub new_password: String,
}

/// Email verification request
#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    /// Verification token from email
    pub token: String,
}

/// Change password request for authenticated users
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    /// Current password for verification
    pub current_password: String,
    /// New password to set
    pub new_password: String,
}
