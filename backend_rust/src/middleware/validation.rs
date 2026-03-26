//! Request validation middleware
//!
//! This module provides middleware for validating incoming request payloads
//! before they reach the handlers.

use crate::errors::AppError;

/// Validates that a string is not empty or whitespace-only
pub fn validate_non_empty_string(s: &str, field_name: &str) -> Result<(), AppError> {
    if s.trim().is_empty() {
        return Err(AppError::ValidationError(format!(
            "{} cannot be empty",
            field_name
        )));
    }
    Ok(())
}

/// Validates content length is within acceptable bounds
pub fn validate_content_length(content: &str, min: usize, max: usize, field_name: &str) -> Result<(), AppError> {
    let len = content.chars().count();
    if len < min {
        return Err(AppError::ValidationError(format!(
            "{} must be at least {} characters",
            field_name, min
        )));
    }
    if len > max {
        return Err(AppError::ValidationError(format!(
            "{} must be no more than {} characters",
            field_name, max
        )));
    }
    Ok(())
}
