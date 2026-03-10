use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/timeline")
            // Feed
            .route("/feed", web::get().to(handlers::get_feed))
            // Posts
            .route("/posts", web::post().to(handlers::create_post))
            .route("/posts/{post_id}", web::get().to(handlers::get_post))
            .route("/posts/{post_id}", web::put().to(handlers::update_post))
            .route("/posts/{post_id}", web::delete().to(handlers::delete_post))
            // User posts
            .route("/users/{user_id}/posts", web::get().to(handlers::get_user_posts))
            // Likes
            .route("/posts/{post_id}/like", web::post().to(handlers::like_post))
            .route("/posts/{post_id}/like", web::delete().to(handlers::unlike_post))
            // Comments
            .route("/posts/{post_id}/comments", web::get().to(handlers::get_comments))
            .route("/posts/{post_id}/comments", web::post().to(handlers::add_comment))
            .route("/posts/{post_id}/comments/{comment_id}", web::delete().to(handlers::delete_comment)),
    );
}
