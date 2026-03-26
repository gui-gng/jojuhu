//! Unit tests for service layer validation logic

use social_network::errors::AppError;

// ==================== Message Service Tests ====================

fn validate_message_content(content: &str) -> Result<(), AppError> {
    if content.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Message content cannot be empty".to_string(),
        ));
    }
    if content.len() > 10000 {
        return Err(AppError::ValidationError(
            "Message content too long".to_string(),
        ));
    }
    Ok(())
}

#[test]
fn test_validate_message_content_valid() {
    assert!(validate_message_content("Hello!").is_ok());
    assert!(validate_message_content("This is valid.").is_ok());
    
    let long_content = "a".repeat(10000);
    assert!(validate_message_content(&long_content).is_ok());
}

#[test]
fn test_validate_message_content_empty() {
    assert!(validate_message_content("").is_err());
    assert!(validate_message_content("   ").is_err());
    assert!(validate_message_content("\t").is_err());
    assert!(validate_message_content("\n").is_err());
}

#[test]
fn test_validate_message_content_too_long() {
    let long_content = "a".repeat(10001);
    let result = validate_message_content(&long_content);
    assert!(result.is_err());
}

// ==================== Timeline Service Tests ====================

fn validate_post_content(content: &str) -> Result<(), AppError> {
    if content.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Post content cannot be empty".to_string(),
        ));
    }
    if content.len() > 5000 {
        return Err(AppError::ValidationError(
            "Post content too long".to_string(),
        ));
    }
    Ok(())
}

fn validate_comment_content(content: &str) -> Result<(), AppError> {
    if content.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Comment content cannot be empty".to_string(),
        ));
    }
    if content.len() > 1000 {
        return Err(AppError::ValidationError(
            "Comment too long".to_string(),
        ));
    }
    Ok(())
}

#[test]
fn test_validate_post_content_valid() {
    assert!(validate_post_content("Hello world!").is_ok());
    assert!(validate_post_content("This is my first post.").is_ok());
}

#[test]
fn test_validate_post_content_empty() {
    assert!(validate_post_content("").is_err());
    assert!(validate_post_content("   ").is_err());
    assert!(validate_post_content("\t\n").is_err());
}

#[test]
fn test_validate_post_content_too_long() {
    let long_content = "a".repeat(5001);
    assert!(validate_post_content(&long_content).is_err());
}

#[test]
fn test_validate_post_max_length() {
    let max_content = "a".repeat(5000);
    assert!(validate_post_content(&max_content).is_ok());
}

#[test]
fn test_validate_comment_content_valid() {
    assert!(validate_comment_content("Nice post!").is_ok());
    assert!(validate_comment_content("Great content!").is_ok());
}

#[test]
fn test_validate_comment_content_empty() {
    assert!(validate_comment_content("").is_err());
}

#[test]
fn test_validate_comment_content_too_long() {
    let long_comment = "a".repeat(1001);
    assert!(validate_comment_content(&long_comment).is_err());
}

#[test]
fn test_validate_comment_max_length() {
    let max_comment = "a".repeat(1000);
    assert!(validate_comment_content(&max_comment).is_ok());
}

#[test]
fn test_validate_post_with_unicode() {
    let unicode_content = "Hello 世界! 🎉 Привет мир!";
    assert!(validate_post_content(unicode_content).is_ok());
}

// ==================== Forum Service Tests ====================

fn validate_forum_name(name: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Forum name cannot be empty".to_string(),
        ));
    }
    if name.len() > 100 {
        return Err(AppError::ValidationError(
            "Forum name too long".to_string(),
        ));
    }
    Ok(())
}

fn validate_topic_title(title: &str) -> Result<(), AppError> {
    if title.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Topic title cannot be empty".to_string(),
        ));
    }
    if title.len() > 200 {
        return Err(AppError::ValidationError(
            "Topic title too long".to_string(),
        ));
    }
    Ok(())
}

fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
}

#[test]
fn test_validate_forum_name_valid() {
    assert!(validate_forum_name("Rust Programming").is_ok());
    assert!(validate_forum_name("General Discussion").is_ok());
}

#[test]
fn test_validate_forum_name_empty() {
    assert!(validate_forum_name("").is_err());
    assert!(validate_forum_name("   ").is_err());
}

#[test]
fn test_validate_forum_name_too_long() {
    let long_name = "a".repeat(101);
    assert!(validate_forum_name(&long_name).is_err());
}

#[test]
fn test_validate_topic_title_valid() {
    assert!(validate_topic_title("How to learn Rust?").is_ok());
    assert!(validate_topic_title("Welcome to the forum").is_ok());
}

#[test]
fn test_validate_topic_title_empty() {
    assert!(validate_topic_title("   ").is_err());
}

#[test]
fn test_generate_slug() {
    assert_eq!(generate_slug("Hello World"), "hello-world");
    assert_eq!(generate_slug("Rust Programming Forum"), "rust-programming-forum");
    assert_eq!(generate_slug("C++ Tips & Tricks"), "c-tips--tricks");
    assert_eq!(generate_slug("Web Development 101"), "web-development-101");
    // Note: underscore is NOT alphanumeric, and not '-', so it's removed
    assert_eq!(generate_slug("UNDER_SCORE"), "underscore");
}

#[test]
fn test_generate_slug_empty() {
    assert_eq!(generate_slug(""), "");
}

#[test]
fn test_generate_slug_only_special_chars() {
    assert_eq!(generate_slug("!@#$%"), "");
}
