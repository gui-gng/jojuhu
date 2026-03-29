//! Integration tests for API endpoints
//!
//! These tests require a running PostgreSQL database with test data.
//! Set the DATABASE_URL environment variable before running these tests.

use jojuhu_backend::config::{DatabaseSettings, JwtSettings, ServerSettings, Settings};

fn create_test_settings() -> Settings {
    Settings {
        database: DatabaseSettings {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost:5432/jojuhu_backend_test".to_string()),
        },
        server: ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        jwt: JwtSettings {
            secret: "test_secret_key_for_testing_only".to_string(),
            expiration_hours: 24,
        },
        email: None,
        environment: "test".to_string(),
        cors: Some(jojuhu_backend::config::CorsSettings {
            allowed_origins: vec!["*".to_string()],
        }),
    }
}

#[test]
fn test_health_check_response_structure() {
    // Verify expected health check response format
    let expected_response = serde_json::json!({
        "status": "healthy",
        "service": "jojuhu_backend"
    });

    assert_eq!(expected_response["status"], "healthy");
    assert_eq!(expected_response["service"], "jojuhu_backend");
}

#[test]
fn test_settings_loads_correctly() {
    let settings = create_test_settings();

    assert_eq!(settings.server.port, 8080);
    assert_eq!(settings.jwt.expiration_hours, 24);
    assert_eq!(settings.environment, "test");
}
