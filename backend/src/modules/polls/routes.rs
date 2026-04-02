use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/polls")
            .route("/{post_id}", web::post().to(handlers::create_poll))
            .route("/{post_id}", web::get().to(handlers::get_poll))
            .route("/{post_id}/vote", web::post().to(handlers::vote_poll))
            .route("/{post_id}", web::delete().to(handlers::delete_poll)),
    );
}
