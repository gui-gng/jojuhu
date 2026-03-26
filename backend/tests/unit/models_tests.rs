//! Unit tests for data models

use social_network::models::user::CreateUserRequest;
use social_network::models::{ApiResponse, PaginatedResponse, PaginationParams};

#[test]
fn test_api_response_success() {
    let data = "test data";
    let response: ApiResponse<&str> = ApiResponse::success(data);

    assert!(response.success);
    assert_eq!(response.data, Some(data));
    assert_eq!(response.message, None);
}

#[test]
fn test_api_response_error() {
    let error_msg = "Something went wrong".to_string();
    let response: ApiResponse<String> = ApiResponse::error(error_msg.clone());

    assert!(!response.success);
    assert_eq!(response.data, None);
    assert_eq!(response.message, Some(error_msg));
}

#[test]
fn test_paginated_response_creation() {
    let items = vec!["item1", "item2", "item3"];
    let response = PaginatedResponse {
        items: items.clone(),
        total: 100,
        page: 1,
        per_page: 3,
        total_pages: 34,
    };

    assert_eq!(response.items.len(), 3);
    assert_eq!(response.total, 100);
    assert_eq!(response.page, 1);
    assert_eq!(response.per_page, 3);
    assert_eq!(response.total_pages, 34);
}

#[test]
fn test_create_user_request_defaults() {
    let request = CreateUserRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        display_name: None,
    };

    assert_eq!(request.username, "testuser");
    assert_eq!(request.email, "test@example.com");
    assert_eq!(request.password, "password123");
    assert_eq!(request.display_name, None);
}

#[test]
fn test_pagination_params_default() {
    let params = PaginationParams {
        page: None,
        per_page: None,
    };

    assert_eq!(params.get_offset(), 0);
    assert_eq!(params.get_limit(), 20);
}

#[test]
fn test_pagination_params_custom() {
    let params = PaginationParams {
        page: Some(3),
        per_page: Some(10),
    };

    assert_eq!(params.get_offset(), 20);
    assert_eq!(params.get_limit(), 10);
}

#[test]
fn test_pagination_params_page_zero_defaults_to_one() {
    let params = PaginationParams {
        page: Some(0),
        per_page: Some(10),
    };

    assert_eq!(params.get_offset(), 0);
}

#[test]
fn test_pagination_params_per_page_clamped_max() {
    let params = PaginationParams {
        page: Some(1),
        per_page: Some(200),
    };

    assert_eq!(params.get_limit(), 100);
}

#[test]
fn test_pagination_params_per_page_clamped_min() {
    let params = PaginationParams {
        page: Some(1),
        per_page: Some(0),
    };

    assert_eq!(params.get_limit(), 1);
}

#[test]
fn test_pagination_params_serialization() {
    let params = PaginationParams {
        page: Some(2),
        per_page: Some(25),
    };

    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains("\"page\":2"));
    assert!(json.contains("\"per_page\":25"));
}

#[test]
fn test_pagination_params_deserialization() {
    let json = r#"{"page":3,"per_page":50}"#;
    let params: PaginationParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.page, Some(3));
    assert_eq!(params.per_page, Some(50));
}
