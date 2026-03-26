//! Common test utilities
//!
//! This module provides shared test helpers for both unit and integration tests.

use social_network::config::{DatabaseSettings, JwtSettings, ServerSettings, Settings};
use sqlx::PgPool;
use std::env;

/// Create test settings with default values
pub fn create_test_settings() -> Settings {
    Settings {
        database: DatabaseSettings {
            url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost:5432/social_network_test".to_string()),
        },
        server: ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        jwt: JwtSettings {
            secret: "test_secret_key_for_testing_only".to_string(),
            expiration_hours: 24,
        },
        environment: "test".to_string(),
    }
}

/// Create a test database pool
/// Note: This requires a running PostgreSQL instance
pub async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost:5432/social_network_test".to_string());
    
    PgPool::connect(&database_url).await
}

/// Test user data for creating test users
pub struct TestUserData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

impl Default for TestUserData {
    fn default() -> Self {
        Self {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: Some("Test User".to_string()),
        }
    }
}

impl TestUserData {
    pub fn new(suffix: &str) -> Self {
        Self {
            username: format!("testuser_{}", suffix),
            email: format!("test_{}@example.com", suffix),
            password: "password123".to_string(),
            display_name: Some(format!("Test User {}", suffix)),
        }
    }
}

/// Generate a unique identifier for test isolation
pub fn generate_test_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
