//! Route configuration module
//!
//! This module centralizes all route definitions for the application,
//! making it easier to understand the API structure at a glance.

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;

// Auth handlers are used through auth::handlers
use crate::auth::handlers::{
    confirm_password_reset_handler, get_current_user_handler, login_handler,
    register_handler, request_password_reset_handler, resend_verification_handler,
    verify_email_handler,
};
use crate::config::Settings;
use crate::docs::{openapi_json, swagger_ui};
use crate::modules::search;
use crate::notifications::{handlers as notification_handlers, NotificationService};
use crate::utils::auth::validator;
use crate::websocket::{websocket_handler, WebSocketServer};

/// Configure all application routes
pub fn configure(
    cfg: &mut web::ServiceConfig, 
    pool: PgPool, 
    _settings: Settings
) {
    // Initialize WebSocket server
    let ws_server = WebSocketServer::new();
    let ws_data = web::Data::new(ws_server);
    
    // Initialize notification service
    let notification_service = NotificationService::new(pool.clone(), Some(ws_server));
    let notification_data = web::Data::new(notification_service);
    
    // API Documentation (public)
    cfg.route("/docs", web::get().to(swagger_ui));
    cfg.route("/api-docs/openapi.json", web::get().to(openapi_json));
    
    // Public routes (no auth required)
    cfg.service(
        web::scope("/api/v1/auth")
            .route("/register", web::post().to(register_handler))
            .route("/login", web::post().to(login_handler))
            .route("/password-reset", web::post().to(request_password_reset_handler))
            .route("/password-reset/confirm", web::post().to(confirm_password_reset_handler))
            .route("/verify-email", web::post().to(verify_email_handler))
    );
    
    // Health check (no auth required)
    cfg.route("/health", web::get().to(health_check));
    
    // WebSocket endpoint (auth required via bearer)
    cfg.route("/ws", web::get().to(websocket_handler));

    // Protected routes (auth required)
    let auth = HttpAuthentication::bearer(validator);
    let user_rate_limit = crate::middleware::rate_limit::UserRateLimit::new(100, 60);
    
    cfg.service(
        web::scope("/api/v1")
            .app_data(notification_data.clone())
            .wrap(user_rate_limit)
            .wrap(auth)
            .route("/me", web::get().to(get_current_user_handler))
            .route("/me/resend-verification", web::post().to(resend_verification_handler))
            // Notifications
            .service(
                web::scope("/notifications")
                    .route("", web::get().to(notification_handlers::get_notifications))
                    .route("/count", web::get().to(notification_handlers::get_unread_count))
                    .route("/read-all", web::post().to(notification_handlers::mark_all_as_read))
                    .route("/{id}/read", web::post().to(notification_handlers::mark_as_read))
                    .route("/{id}", web::delete().to(notification_handlers::delete_notification))
            )
            .configure(|c| crate::modules::users::configure(c, pool.clone()))
            .configure(|c| crate::modules::upload::configure_module(c, pool.clone()))
            .configure(|c| crate::modules::messages::configure_module(c, pool.clone()))
            .configure(|c| crate::modules::timeline::configure_module(c, pool.clone()))
            .configure(|c| crate::modules::forums::configure_module(c, pool.clone()))
            .configure(search::configure_routes)
    );
}

/// Health check endpoint
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "jojuhu_backend"
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