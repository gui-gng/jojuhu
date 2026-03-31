use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stories")
            .route("", web::post().to(handlers::create_story))
            .route("/me", web::get().to(handlers::get_my_stories))
            .route("/feed", web::get().to(handlers::get_following_stories))
            .route("/user/{user_id}", web::get().to(handlers::get_user_stories))
            .route("/{story_id}", web::delete().to(handlers::delete_story))
            .route("/{story_id}/view", web::post().to(handlers::view_story))
            .route(
                "/{story_id}/viewers",
                web::get().to(handlers::get_story_viewers),
            ),
    );
}
