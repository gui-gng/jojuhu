//! Unit tests for error handling

use actix_web::ResponseError;
use jojuhu_backend::errors::AppError;

#[test]
fn test_app_error_display() {
    let errors = vec![
        (
            AppError::DatabaseError("db error".to_string()),
            "Database error: db error",
        ),
        (
            AppError::ValidationError("invalid input".to_string()),
            "Validation error: invalid input",
        ),
        (
            AppError::AuthenticationError("bad creds".to_string()),
            "Authentication error: bad creds",
        ),
        (
            AppError::AuthorizationError("no access".to_string()),
            "Authorization error: no access",
        ),
        (
            AppError::NotFoundError("missing".to_string()),
            "Not found: missing",
        ),
        (
            AppError::ConflictError("duplicate".to_string()),
            "Conflict: duplicate",
        ),
        (
            AppError::InternalError("oops".to_string()),
            "Internal error: oops",
        ),
    ];

    for (error, expected) in errors {
        assert_eq!(format!("{}", error), expected);
    }
}

#[test]
fn test_app_error_status_codes() {
    let test_cases = vec![
        (AppError::DatabaseError("test".to_string()), 500),
        (AppError::ValidationError("test".to_string()), 400),
        (AppError::AuthenticationError("test".to_string()), 401),
        (AppError::AuthorizationError("test".to_string()), 403),
        (AppError::NotFoundError("test".to_string()), 404),
        (AppError::ConflictError("test".to_string()), 409),
        (AppError::InternalError("test".to_string()), 500),
    ];

    for (error, expected_status) in test_cases {
        let response = error.error_response();
        assert_eq!(response.status().as_u16(), expected_status);
    }
}

#[test]
fn test_sqlx_error_conversion_row_not_found() {
    let sqlx_error = sqlx::Error::RowNotFound;
    let app_error: AppError = sqlx_error.into();

    match app_error {
        AppError::NotFoundError(msg) => {
            assert!(msg.contains("Record not found"));
        }
        _ => panic!("Expected NotFoundError for RowNotFound"),
    }
}

#[test]
fn test_serde_json_error_conversion() {
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let app_error: AppError = json_error.into();

    match app_error {
        AppError::ValidationError(_) => {
            // Expected
        }
        _ => panic!("Expected ValidationError for JSON parse error"),
    }
}
