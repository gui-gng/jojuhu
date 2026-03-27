pub mod handlers;
pub mod models;
pub mod repository;
pub mod service;

use actix_web::web;
use sqlx::PgPool;

use self::{repository::UserRepository, service::UserService};

/// Configure user routes
pub fn configure(cfg: &mut web::ServiceConfig, pool: PgPool) {
    // Create repository and service
    let repository = UserRepository::new(pool);
    let service = UserService::new(repository);

    // Add service to app data
    let service_data = web::Data::new(service);

    cfg.app_data(service_data.clone()).service(
        web::scope("/users")
            // Get current user profile
            .service(handlers::get_my_profile)
            // Update current user profile
            .service(handlers::update_profile)
            // Update avatar
            .service(handlers::update_avatar)
            // Follow/unfollow user
            .service(handlers::follow_user)
            .service(handlers::unfollow_user)
            // Get followers/following
            .service(handlers::get_followers)
            .service(handlers::get_following)
            // Get user profile by ID (must be last to not conflict with other routes)
            .service(handlers::get_user_profile),
    );
}
