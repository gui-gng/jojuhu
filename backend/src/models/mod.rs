//! Core data models and API response types
//!
//! This module defines common data structures used across the application,
//! including pagination, API responses, and base models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod user;

/// Base model with common fields for database entities
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseModel {
    /// Unique identifier (UUID v4)
    pub id: Uuid,
    /// Timestamp when the record was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the record was last updated
    pub updated_at: DateTime<Utc>,
}

impl Default for BaseModel {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Standard API response wrapper for success/error responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Response data (only present on success)
    pub data: Option<T>,
    /// Error or informational message
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Creates a successful API response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    /// Creates an error API response with a message
    #[allow(dead_code)]
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

/// Paginated response containing a list of items with pagination metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Items in the current page
    pub items: Vec<T>,
    /// Total number of items across all pages
    pub total: i64,
    /// Current page number (1-indexed)
    pub page: i32,
    /// Number of items per page
    pub per_page: i32,
    /// Total number of pages
    pub total_pages: i32,
}

/// Query parameters for paginated endpoints
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationParams {
    /// Page number (1-indexed, defaults to 1)
    pub page: Option<i32>,
    /// Items per page (defaults to 20, max 100)
    pub per_page: Option<i32>,
}

impl PaginationParams {
    /// Calculates the SQL OFFSET value for the current page
    ///
    /// # Returns
    /// The offset in rows (0-indexed)
    pub fn get_offset(&self) -> i32 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page.unwrap_or(20).clamp(1, 100);
        (page - 1) * per_page
    }

    /// Returns the LIMIT value for SQL queries
    ///
    /// # Returns
    /// The number of items per page (clamped between 1 and 100)
    pub fn get_limit(&self) -> i32 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }
}
