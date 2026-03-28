use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/forums")
            // Forums
            .route("", web::post().to(handlers::create_forum))
            .route("", web::get().to(handlers::list_forums))
            .route("/search", web::get().to(handlers::search_forums))
            .route("/trending", web::get().to(handlers::get_trending_forums))
            .route("/{forum_id}", web::get().to(handlers::get_forum))
            .route("/{forum_id}", web::put().to(handlers::update_forum))
            .route("/{forum_id}", web::delete().to(handlers::delete_forum))
            .route("/{forum_id}/join", web::post().to(handlers::join_forum))
            .route("/{forum_id}/leave", web::post().to(handlers::leave_forum))
            // Topics
            .route("/{forum_id}/topics", web::get().to(handlers::get_topics))
            .route("/{forum_id}/topics", web::post().to(handlers::create_topic))
            .route(
                "/{forum_id}/topics/{topic_id}",
                web::get().to(handlers::get_topic),
            )
            .route(
                "/{forum_id}/topics/{topic_id}",
                web::delete().to(handlers::delete_topic),
            )
            .route(
                "/{forum_id}/topics/{topic_id}/lock",
                web::post().to(handlers::lock_topic),
            )
            .route(
                "/{forum_id}/topics/{topic_id}/unlock",
                web::post().to(handlers::unlock_topic),
            )
            .route(
                "/{forum_id}/topics/{topic_id}/pin",
                web::post().to(handlers::pin_topic),
            )
            .route(
                "/{forum_id}/topics/{topic_id}/unpin",
                web::post().to(handlers::unpin_topic),
            )
            // Replies
            .route(
                "/{forum_id}/topics/{topic_id}/replies",
                web::get().to(handlers::get_replies),
            )
            .route(
                "/{forum_id}/topics/{topic_id}/replies",
                web::post().to(handlers::create_reply),
            )
            .route(
                "/{forum_id}/topics/{topic_id}/replies/{reply_id}",
                web::delete().to(handlers::delete_reply),
            ),
    );
}
