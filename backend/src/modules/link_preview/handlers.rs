use actix_web::{web, HttpResponse};

use crate::errors::AppError;
use crate::models::ApiResponse;

use super::models::LinkPreviewRequest;
use super::service::LinkPreviewService;

pub async fn get_link_preview(
    service: web::Data<LinkPreviewService>,
    request: web::Json<LinkPreviewRequest>,
) -> Result<HttpResponse, AppError> {
    let preview = service.get_preview(request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(preview)))
}