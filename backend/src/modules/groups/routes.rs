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
            .route("/{group_id}/members", web::get().to(handlers::get_members))
            .route("/{group_id}/invite", web::post().to(handlers::invite_user))
            .route(
                "/{group_id}/join-requests",
                web::post().to(handlers::request_join_group),
            )
            .route(
                "/{group_id}/join-requests",
                web::get().to(handlers::get_join_requests),
            )
            .route(
                "/{group_id}/members/{user_id}/role",
                web::put().to(handlers::update_member_role),
            )
            .route("/invitations", web::get().to(handlers::get_my_invitations))
            .route(
                "/invitations/{invitation_id}/accept",
                web::post().to(handlers::accept_invitation),
            )
            .route(
                "/invitations/{invitation_id}/reject",
                web::post().to(handlers::reject_invitation),
            )
            .route(
                "/join-requests/{request_id}/approve",
                web::post().to(handlers::approve_join_request),
            )
            .route(
                "/join-requests/{request_id}/reject",
                web::post().to(handlers::reject_join_request),
            ),
    );
}
