use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/messages")
            .route("", web::post().to(handlers::send_message))
            .route("/conversations", web::get().to(handlers::get_conversations))
            .route(
                "/conversations/{user_id}",
                web::get().to(handlers::get_conversation),
            )
            .route(
                "/conversations/{user_id}",
                web::delete().to(handlers::delete_conversation),
            )
            .route("/{message_id}/read", web::put().to(handlers::mark_as_read))
            .route("/{message_id}", web::delete().to(handlers::delete_message)),
    );
}
