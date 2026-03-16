//! Unit tests for utility modules

use social_network::config::{JwtSettings, Settings};
use social_network::utils::auth::{generate_token, decode_token, Claims};
use social_network::utils::validators::{validate_username, validate_password};
use social_network::utils::{hash_password, verify_password};
use uuid::Uuid;

fn create_test_settings() -> Settings {
    Settings {
        database: social_network::config::DatabaseSettings {
            url: "postgres://localhost/test".to_string(),
        },
        server: social_network::config::ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        jwt: JwtSettings {
            secret: "test_secret_key_for_testing_only_make_it_long_enough".to_string(),
            expiration_hours: 24,
        },
        environment: "test".to_string(),
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
    
    let decoded_claims = decode_token(&token, &settings)
        .expect("Failed to decode token");
    
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
    
    let token = generate_token(user_id, username, email, &settings)
        .expect("Failed to generate token");
    
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
    
    assert_ne!(hash1, hash2, "Same password should produce different hashes due to salt");
    
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
