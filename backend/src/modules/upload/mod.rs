pub mod handlers;
pub mod models;
pub mod service;

use actix_web::web;
use sqlx::PgPool;

use self::service::UploadService;

pub fn configure_module(cfg: &mut web::ServiceConfig, _pool: PgPool) {
    // Get MinIO configuration from environment
    let endpoint = std::env::var("MINIO_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:9000".to_string());
    let bucket_name = std::env::var("MINIO_BUCKET_NAME")
        .unwrap_or_else(|_| "jojuhu-uploads".to_string());
    let access_key = std::env::var("MINIO_ACCESS_KEY")
        .unwrap_or_else(|_| "minioadmin".to_string());
    let secret_key = std::env::var("MINIO_SECRET_KEY")
        .unwrap_or_else(|_| "minioadmin".to_string());

    // Create upload service
    let upload_service = UploadService::new(
        endpoint,
        bucket_name,
        access_key,
        secret_key,
        None,
    )
    .expect("Failed to initialize upload service");

    let service_data = web::Data::new(upload_service);

    cfg.app_data(service_data).service(
        web::scope("/upload")
            .service(handlers::generate_presigned_url)
            .service(handlers::confirm_avatar_upload)
            .service(handlers::delete_file),
    );
}
