use actix_web::web;

use super::handlers;

#[derive(Debug, serde::Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/scheduled-posts")
            .route("", web::post().to(handlers::create_scheduled_post))
            .route("", web::get().to(handlers::get_scheduled_posts))
            .route("/{id}", web::get().to(handlers::get_scheduled_post))
            .route("/{id}", web::put().to(handlers::update_scheduled_post))
            .route(
                "/{id}/cancel",
                web::post().to(handlers::cancel_scheduled_post),
            ),
    )
    .service(
        web::scope("/drafts")
            .route("", web::post().to(handlers::save_draft))
            .route("", web::get().to(handlers::get_draft))
            .route("", web::delete().to(handlers::delete_draft)),
    )
    .service(
        web::scope("/push-tokens")
            .route("", web::post().to(handlers::register_push_token))
            .route(
                "/deactivate",
                web::post().to(handlers::deactivate_push_token),
            ),
    );
}
