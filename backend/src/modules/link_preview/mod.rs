pub mod handlers;
pub mod models;
pub mod service;

use actix_web::web;
use sqlx::PgPool;

use self::service::LinkPreviewService;

pub fn configure_module(cfg: &mut web::ServiceConfig, pool: PgPool) {
    let service = LinkPreviewService::new(pool);

    cfg.app_data(web::Data::new(service))
        .configure(routes::configure_routes);
}

mod routes {
    use actix_web::web;

    use super::handlers;

    pub fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/link-preview")
                .route("", web::post().to(handlers::get_link_preview)),
        );
    }
}