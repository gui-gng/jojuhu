use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/groups")
            .route("", web::post().to(handlers::create_group))
            .route("", web::get().to(handlers::list_groups))
            .route("/{group_id}", web::get().to(handlers::get_group))
            .route("/{group_id}", web::put().to(handlers::update_group))
            .route("/{group_id}", web::delete().to(handlers::delete_group))
            .route("/{group_id}/join", web::post().to(handlers::join_group))
            .route("/{group_id}/leave", web::post().to(handlers::leave_group))
            .route("/{group_id}/members", web::get().to(handlers::get_members)),
    );
}
