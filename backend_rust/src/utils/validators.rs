//! Input validation utilities
//!
//! This module provides validation functions for user inputs including
//! usernames, passwords, and other data types.

use validator::ValidationError;

/// Validates a username according to project requirements.
///
/// # Rules
/// - Minimum 3 characters
/// - Maximum 32 characters
/// - Only alphanumeric characters and underscores allowed
///
/// # Example
/// ```
/// use social_network::utils::validators::validate_username;
///
/// assert!(validate_username("john_doe").is_ok());
/// assert!(validate_username("ab").is_err()); // Too short
/// ```
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::new("username_too_short"));
    }
    if username.len() > 32 {
        return Err(ValidationError::new("username_too_long"));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::new("username_invalid_chars"));
    }
    Ok(())
}

/// Validates a password according to security requirements.
///
/// # Rules
/// - Minimum 8 characters (OWASP recommendation)
/// - Maximum 128 characters (prevent DoS attacks)
///
/// # Security Note
/// Password complexity requirements are intentionally minimal here.
/// Modern security practices favor length over complexity rules.
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }
    if password.len() > 128 {
        return Err(ValidationError::new("password_too_long"));
    }
    Ok(())
}
