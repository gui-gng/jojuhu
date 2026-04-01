use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/timeline")
            // Feed
            .route("/feed", web::get().to(handlers::get_feed))
            .route("/following", web::get().to(handlers::get_following_feed))
            .route("/for-you", web::get().to(handlers::get_for_you_feed))
            .route("/trending", web::get().to(handlers::get_trending_posts))
            // Posts
            .route("/posts", web::post().to(handlers::create_post))
            .route("/posts/{post_id}", web::get().to(handlers::get_post))
            .route("/posts/{post_id}", web::put().to(handlers::update_post))
            .route("/posts/{post_id}", web::delete().to(handlers::delete_post))
            // User posts
            .route(
                "/users/{user_id}/posts",
                web::get().to(handlers::get_user_posts),
            )
            // Likes
            .route("/posts/{post_id}/like", web::post().to(handlers::like_post))
            .route(
                "/posts/{post_id}/like",
                web::delete().to(handlers::unlike_post),
            )
            // Comments
            .route(
                "/posts/{post_id}/comments",
                web::get().to(handlers::get_comments),
            )
            .route(
                "/posts/{post_id}/comments",
                web::post().to(handlers::add_comment),
            )
            .route(
                "/posts/{post_id}/comments/{comment_id}",
                web::delete().to(handlers::delete_comment),
            )
            // Reposts
            .route("/reposts", web::post().to(handlers::create_repost))
            .route("/reposts/me", web::get().to(handlers::get_my_reposts))
            .route(
                "/reposts/{post_id}",
                web::delete().to(handlers::delete_repost),
            )
            // Suggestions
            .route(
                "/suggestions/users",
                web::get().to(handlers::get_suggested_users),
            ),
    );
}
