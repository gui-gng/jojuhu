use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/privacy")
            .route("/settings", web::get().to(handlers::get_privacy_settings))
            .route(
                "/settings",
                web::put().to(handlers::update_privacy_settings),
            )
            .route(
                "/data-export",
                web::post().to(handlers::request_data_export),
            )
            .route(
                "/data-export",
                web::get().to(handlers::get_data_export_status),
            )
            .route(
                "/account-deletion",
                web::post().to(handlers::request_account_deletion),
            )
            .route(
                "/account-deletion",
                web::get().to(handlers::get_deletion_request),
            )
            .route(
                "/account-deletion/cancel",
                web::post().to(handlers::cancel_account_deletion),
            ),
    );
}
