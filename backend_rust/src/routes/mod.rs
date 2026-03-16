//! Route configuration module
//! 
//! This module centralizes all route definitions for the application,
//! making it easier to understand the API structure at a glance.

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;

// Auth handlers are used through auth::handlers
use crate::auth::handlers::{get_current_user_handler, login_handler, register_handler};
use crate::config::Settings;
use crate::utils::auth::validator;

/// Configure all application routes
pub fn configure(cfg: &mut web::ServiceConfig, pool: PgPool, _settings: Settings) {
    // Public routes (no auth required)
    cfg.service(
        web::scope("/api/v1/auth")
            .route("/register", web::post().to(register_handler))
            .route("/login", web::post().to(login_handler))
    );
    
    // Health check (no auth required)
    cfg.route("/health", web::get().to(health_check));

    // Protected routes (auth required)
    let auth = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("/api/v1")
            .wrap(auth)
            .route("/me", web::get().to(get_current_user_handler))
            .configure(|c| crate::modules::messages::configure_module(c, pool.clone()))
            .configure(|c| crate::modules::timeline::configure_module(c, pool.clone()))
            .configure(|c| crate::modules::forums::configure_module(c, pool.clone()))
    );
}

/// Health check endpoint
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "social_network"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert!(response.status().is_success());
    }
}
