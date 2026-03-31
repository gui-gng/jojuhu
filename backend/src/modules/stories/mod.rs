pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;

use actix_web::web;
use sqlx::PgPool;

use self::repository::StoryRepository;
use self::service::StoryService;

pub fn configure_module(cfg: &mut web::ServiceConfig, pool: PgPool) {
    let repository = StoryRepository::new(pool.clone());
    let service = StoryService::new(repository);
    
    cfg.app_data(web::Data::new(service))
        .configure(routes::configure_routes);
}
