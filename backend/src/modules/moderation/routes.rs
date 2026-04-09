use actix_web::web;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/moderation")
            // Dashboard
            .route("/dashboard", web::get().to(handlers::get_dashboard_stats))
            // Reports
            .route("/reports", web::post().to(handlers::create_report))
            .route("/reports", web::get().to(handlers::list_reports))
            .route(
                "/reports/pending",
                web::get().to(handlers::get_pending_reports_count),
            )
            .route("/reports/{report_id}", web::get().to(handlers::get_report))
            .route(
                "/reports/{report_id}/assign",
                web::post().to(handlers::assign_report),
            )
            .route(
                "/reports/{report_id}/resolve",
                web::post().to(handlers::resolve_report),
            )
            .route(
                "/reports/{report_id}/dismiss",
                web::post().to(handlers::dismiss_report),
            )
            // Suspensions
            .route("/suspensions", web::post().to(handlers::suspend_user))
            .route(
                "/suspensions/{user_id}",
                web::get().to(handlers::check_user_suspension),
            )
            .route(
                "/suspensions/{user_id}/history",
                web::get().to(handlers::get_user_suspensions),
            ),
    );
}
