//! Unit tests for utility modules

use jojuhu_backend::config::{JwtSettings, Settings};
use jojuhu_backend::utils::auth::{decode_token, generate_token, Claims};
use jojuhu_backend::utils::validators::{validate_password, validate_username};
use jojuhu_backend::utils::{hash_password, verify_password};
use uuid::Uuid;

fn create_test_settings() -> Settings {
    Settings {
        database: jojuhu_backend::config::DatabaseSettings {
            url: "postgres://localhost/test".to_string(),
        },
        server: jojuhu_backend::config::ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        jwt: JwtSettings {
            secret: "test_secret_key_for_testing_only_make_it_long_enough".to_string(),
            expiration_hours: 24,
        },
        email: None,
        environment: "test".to_string(),
        cors: Some(jojuhu_backend::config::CorsSettings {
            allowed_origins: vec!["*".to_string()],
        }),
    }
}

// ==================== Auth Tests ====================

#[test]
fn test_claims_new() {
    let user_id = Uuid::new_v4();
    let username = "testuser".to_string();
    let email = "test@example.com".to_string();

    let claims = Claims::new(user_id, username.clone(), email.clone(), 24);

    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.username, username);
    assert_eq!(claims.email, email);
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_generate_and_decode_token() {
    let settings = create_test_settings();
    let user_id = Uuid::new_v4();
    let username = "testuser".to_string();
    let email = "test@example.com".to_string();

    let token = generate_token(user_id, username.clone(), email.clone(), &settings)
        .expect("Failed to generate token");

    assert!(!token.is_empty());

    let decoded_claims = decode_token(&token, &settings).expect("Failed to decode token");

    assert_eq!(decoded_claims.sub, user_id);
    assert_eq!(decoded_claims.username, username);
    assert_eq!(decoded_claims.email, email);
}

#[test]
fn test_decode_token_with_wrong_secret_fails() {
    let settings = create_test_settings();
    let user_id = Uuid::new_v4();
    let username = "testuser".to_string();
    let email = "test@example.com".to_string();

    let token =
        generate_token(user_id, username, email, &settings).expect("Failed to generate token");

    let mut wrong_settings = create_test_settings();
    wrong_settings.jwt.secret = "wrong_secret_key_for_testing_only_make_it_long".to_string();

    let result = decode_token(&token, &wrong_settings);
    assert!(result.is_err());
}

#[test]
fn test_decode_invalid_token_fails() {
    let settings = create_test_settings();
    let result = decode_token("invalid_token", &settings);
    assert!(result.is_err());
}

// ==================== Validator Tests ====================

#[test]
fn test_validate_username_valid() {
    assert!(validate_username("john").is_ok());
    assert!(validate_username("john_doe").is_ok());
    assert!(validate_username("JohnDoe123").is_ok());
    assert!(validate_username("user_123").is_ok());
}

#[test]
fn test_validate_username_too_short() {
    let result = validate_username("ab");
    assert!(result.is_err());
}

#[test]
fn test_validate_username_too_long() {
    let long_username = "a".repeat(33);
    let result = validate_username(&long_username);
    assert!(result.is_err());
}

#[test]
fn test_validate_username_max_length() {
    let max_username = "a".repeat(32);
    let result = validate_username(&max_username);
    assert!(result.is_ok());
}

#[test]
fn test_validate_username_invalid_chars() {
    assert!(validate_username("john@doe").is_err());
    assert!(validate_username("john doe").is_err());
    assert!(validate_username("john-doe").is_err());
    assert!(validate_username("john.doe").is_err());
    assert!(validate_username("john!doe").is_err());
}

#[test]
fn test_validate_password_valid() {
    assert!(validate_password("password123").is_ok());
    assert!(validate_password("12345678").is_ok());
    assert!(validate_password("MyP@ssw0rd!").is_ok());
}

#[test]
fn test_validate_password_too_short() {
    let result = validate_password("1234567");
    assert!(result.is_err());
}

#[test]
fn test_validate_password_too_long() {
    let long_password = "a".repeat(129);
    let result = validate_password(&long_password);
    assert!(result.is_err());
}

#[test]
fn test_validate_password_exactly_min_length() {
    let password = "a".repeat(8);
    let result = validate_password(&password);
    assert!(result.is_ok());
}

#[test]
fn test_validate_password_exactly_max_length() {
    let password = "a".repeat(128);
    let result = validate_password(&password);
    assert!(result.is_ok());
}

// ==================== Password Hash Tests ====================

#[test]
fn test_hash_password_success() {
    let password = "my_secure_password123";
    let hash = hash_password(password);

    assert!(hash.is_ok());
    let hash = hash.unwrap();
    assert!(!hash.is_empty());
    assert!(hash.starts_with("$2b$"));
}

#[test]
fn test_verify_password_correct() {
    let password = "my_secure_password123";
    let hash = hash_password(password).expect("Failed to hash password");

    let result = verify_password(password, &hash);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_verify_password_incorrect() {
    let password = "my_secure_password123";
    let wrong_password = "wrong_password";
    let hash = hash_password(password).expect("Failed to hash password");

    let result = verify_password(wrong_password, &hash);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_hash_password_produces_different_hashes() {
    let password = "my_secure_password123";

    let hash1 = hash_password(password).expect("Failed to hash password");
    let hash2 = hash_password(password).expect("Failed to hash password");

    assert_ne!(
        hash1, hash2,
        "Same password should produce different hashes due to salt"
    );

    assert!(verify_password(password, &hash1).unwrap());
    assert!(verify_password(password, &hash2).unwrap());
}

#[test]
fn test_verify_password_with_invalid_hash_fails() {
    let password = "my_secure_password123";
    let invalid_hash = "invalid_hash_format";

    let result = verify_password(password, invalid_hash);
    assert!(result.is_err());
}

#[test]
fn test_empty_password() {
    let password = "";
    let hash = hash_password(password);

    assert!(hash.is_ok());
    let hash = hash.unwrap();

    let result = verify_password(password, &hash);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

// ==================== Security Tests ====================

use jojuhu_backend::middleware::security::{sanitize_input, validate_input_safety};
use jojuhu_backend::middleware::validation::{validate_content_length, validate_non_empty_string};

#[test]
fn test_sanitize_input_removes_script_tags() {
    let input = "<script>alert('xss')</script>Hello";
    let cleaned = sanitize_input(input);
    assert!(!cleaned.contains("<script>"));
    assert!(cleaned.contains("Hello"));
}

#[test]
fn test_sanitize_input_removes_event_handlers() {
    let input = "<img src=x onerror=alert('xss')>";
    let cleaned = sanitize_input(input);
    assert!(!cleaned.contains("onerror"));
    // ammonia keeps safe HTML tags like <img> but strips dangerous attributes
}

#[test]
fn test_sanitize_input_preserves_safe_html() {
    let input = "<p>Hello <strong>World</strong></p>";
    let cleaned = sanitize_input(input);
    assert!(cleaned.contains("Hello"));
    assert!(cleaned.contains("World"));
}

#[test]
fn test_sanitize_input_empty_string() {
    let input = "";
    let cleaned = sanitize_input(input);
    assert_eq!(cleaned, "");
}

#[test]
fn test_validate_input_safety_detects_xss() {
    let input = "<img src=x onerror=alert('xss')>";
    assert!(validate_input_safety(input).is_err());
}

#[test]
fn test_validate_input_safety_detects_javascript_protocol() {
    let input = "javascript:alert('xss')";
    assert!(validate_input_safety(input).is_err());
}

#[test]
fn test_validate_input_safety_detects_sql_injection_select() {
    let input = "'; SELECT * FROM users; --";
    assert!(validate_input_safety(input).is_err());
}

#[test]
fn test_validate_input_safety_detects_sql_injection_drop() {
    let input = "'; DROP TABLE users; --";
    assert!(validate_input_safety(input).is_err());
}

#[test]
fn test_validate_input_safety_detects_sql_injection_union() {
    let input = "' UNION SELECT * FROM passwords --";
    assert!(validate_input_safety(input).is_err());
}

#[test]
fn test_validate_input_safety_detects_sql_injection_or() {
    let input = "admin' OR '1'='1";
    assert!(validate_input_safety(input).is_err());
}

#[test]
fn test_validate_input_safety_accepts_safe_input() {
    let input = "Hello, this is a safe message!";
    assert!(validate_input_safety(input).is_ok());
}

#[test]
fn test_validate_input_safety_accepts_code_snippets() {
    let input = "Check out this Rust code: fn main() { println!(\"Hello\"); }";
    assert!(validate_input_safety(input).is_ok());
}

// ==================== Validation Middleware Tests ====================

#[test]
fn test_validate_non_empty_string_valid() {
    assert!(validate_non_empty_string("Hello", "Field").is_ok());
    assert!(validate_non_empty_string("  Hello  ", "Field").is_ok());
    assert!(validate_non_empty_string("A", "Field").is_ok());
}

#[test]
fn test_validate_non_empty_string_empty() {
    let result = validate_non_empty_string("", "Username");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Username cannot be empty"));
}

#[test]
fn test_validate_non_empty_string_whitespace_only() {
    let result = validate_non_empty_string("   ", "Content");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Content cannot be empty"));
}

#[test]
fn test_validate_non_empty_string_tabs_and_newlines() {
    let result = validate_non_empty_string("\t\n\r", "Field");
    assert!(result.is_err());
}

#[test]
fn test_validate_content_length_valid() {
    assert!(validate_content_length("Hello", 1, 100, "Field").is_ok());
    assert!(validate_content_length("A", 1, 10, "Field").is_ok());
    assert!(validate_content_length(&"A".repeat(100), 1, 100, "Field").is_ok());
}

#[test]
fn test_validate_content_length_too_short() {
    let result = validate_content_length("Hi", 5, 100, "Description");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Description must be at least 5 characters"));
}

#[test]
fn test_validate_content_length_too_long() {
    let input = "A".repeat(101);
    let result = validate_content_length(&input, 1, 100, "Bio");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Bio must be no more than 100 characters"));
}

#[test]
fn test_validate_content_length_exactly_at_bounds() {
    assert!(validate_content_length(&"A".repeat(5), 5, 100, "Field").is_ok());
    assert!(validate_content_length(&"A".repeat(100), 5, 100, "Field").is_ok());
}

#[test]
fn test_validate_content_length_with_unicode() {
    let unicode = "Hello 世界! 🎉";
    assert!(validate_content_length(unicode, 1, 100, "Field").is_ok());
}

#[test]
fn test_validate_content_length_unicode_vs_bytes() {
    // Unicode characters count as 1, not by byte length
    let unicode = "世界"; // 6 bytes but 2 characters
    assert!(validate_content_length(unicode, 2, 2, "Field").is_ok());
    assert!(validate_content_length(unicode, 3, 10, "Field").is_err());
}
