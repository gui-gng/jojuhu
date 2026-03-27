//! HTTP middleware modules
//!
//! This module provides middleware for authentication, logging,
//! request validation, and other cross-cutting concerns.

pub mod auth;
pub mod logging;
pub mod rate_limit;
pub mod security;
pub mod validation;
