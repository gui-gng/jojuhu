//! Social Network Backend
//!
//! A Rust-based backend API for a social network platform, built with Actix-web.
//!
//! ## Architecture
//!
//! The project follows a modular, domain-driven architecture:
//!
//! - `auth/` - Authentication logic (login, register, JWT)
//! - `cache/` - Redis caching module
//! - `config/` - Configuration management
//! - `docs/` - API documentation generation
//! - `errors/` - Error handling types
//! - `middleware/` - Authentication and logging middleware
//! - `models/` - Shared data models
//! - `modules/` - Domain modules (messages, timeline, forums, search)
//! - `notifications/` - Notification service and handlers
//! - `routes/` - Route configuration
//! - `utils/` - Utilities (auth, validators)
//! - `websocket/` - WebSocket server for real-time communication

pub mod auth;
pub mod cache;
pub mod config;
pub mod docs;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod modules;
pub mod notifications;
pub mod routes;
pub mod utils;
pub mod websocket;
