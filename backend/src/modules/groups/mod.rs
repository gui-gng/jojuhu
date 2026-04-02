pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;

use actix_web::web;
use sqlx::PgPool;

use self::repository::GroupRepository;
use self::service::GroupService;

pub fn configure_module(cfg: &mut web::ServiceConfig, pool: PgPool) {
    let repository = GroupRepository::new(pool);
    let service = GroupService::new(repository);

    cfg.app_data(web::Data::new(service))
        .configure(routes::configure_routes);
}