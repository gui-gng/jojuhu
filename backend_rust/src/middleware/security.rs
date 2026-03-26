//! Security utilities
//!
//! This module provides security-related utilities including:
//! - Input sanitization (XSS prevention)
//! - Input safety validation

/// Sanitizes user input to prevent XSS attacks
///
/// Uses the ammonia crate to strip dangerous HTML while preserving safe content.
/// This should be called on any user-generated content before storing or displaying.
///
/// # Example
/// ```rust
/// use social_network::middleware::security::sanitize_input;
///
/// let clean = sanitize_input("<script>alert('xss')</script>Hello");
/// assert_eq!(clean, "Hello");
/// ```
pub fn sanitize_input(input: &str) -> String {
    ammonia::clean(input)
}

/// Validates that input doesn't contain suspicious patterns
///
/// Checks for common SQL injection and XSS patterns.
/// Returns true if the input appears safe.
pub fn validate_input_safety(input: &str) -> Result<(), String> {
    let suspicious_patterns = vec![
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "SELECT * FROM",
        "DROP TABLE",
        "INSERT INTO",
        "DELETE FROM",
        "UNION SELECT",
        "1=1",
        "' OR '",
        ";--",
    ];
    
    let lower_input = input.to_lowercase();
    
    for pattern in suspicious_patterns {
        if lower_input.contains(pattern) {
            return Err(format!("Input contains potentially dangerous pattern: {}", pattern));
        }
    }
    
    Ok(())
}

/// Sanitizes content fields in a request object
///
/// This is a helper function to sanitize string fields before processing.
/// Use it in handlers that accept user-generated content.
pub fn sanitize_content_fields(content: &mut String) {
    *content = sanitize_input(content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_input_removes_script_tags() {
        let input = "<script>alert('xss')</script>Hello";
        let cleaned = sanitize_input(input);
        assert!(!cleaned.contains("<script>"));
        assert!(cleaned.contains("Hello"));
    }

    #[test]
    fn test_validate_input_safety_detects_xss() {
        let input = "<img src=x onerror=alert('xss')>";
        assert!(validate_input_safety(input).is_err());
    }

    #[test]
    fn test_validate_input_safety_detects_sql_injection() {
        let input = "'; DROP TABLE users; --";
        assert!(validate_input_safety(input).is_err());
    }

    #[test]
    fn test_validate_input_safety_accepts_safe_input() {
        let input = "Hello, this is a safe message!";
        assert!(validate_input_safety(input).is_ok());
    }
}
