use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hashtag")
            .route("/search", web::get().to(handlers::search_hashtags))
            .route("/trending", web::get().to(handlers::get_trending_hashtags))
            .route("/{name}", web::get().to(handlers::get_hashtag_by_name)),
    );
}
