pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;

use actix_web::web;
use sqlx::PgPool;

use self::repository::HashtagRepository;
use self::service::HashtagService;

pub fn configure_module(cfg: &mut web::ServiceConfig, pool: PgPool) {
    let repository = HashtagRepository::new(pool.clone());
    let service = HashtagService::new(pool);
    
    cfg.app_data(web::Data::new(repository))
        .app_data(web::Data::new(service))
        .configure(routes::configure_routes);
}