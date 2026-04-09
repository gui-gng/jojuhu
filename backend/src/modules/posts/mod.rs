use actix_web::web;
use sqlx::PgPool;

pub mod handlers;
pub mod repository;
pub mod routes;
pub mod scheduled;
pub mod service;

use self::repository::ScheduledPostsRepository;
use self::service::ScheduledPostsService;

pub fn configure_module(cfg: &mut web::ServiceConfig, pool: PgPool) {
    let repository = ScheduledPostsRepository::new(pool);
    let service = ScheduledPostsService::new(repository);

    cfg.app_data(web::Data::new(service))
        .configure(routes::configure_routes);
}