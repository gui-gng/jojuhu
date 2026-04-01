use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::errors::AppError;
use crate::models::ApiResponse;

use super::service::HashtagService;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub limit: Option<i32>,
}

pub async fn search_hashtags(
    service: web::Data<HashtagService>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    let search_query = query.q.as_deref().unwrap_or("");
    let limit = query.limit.unwrap_or(20).min(100);
    
    let hashtags = service.search_hashtags(search_query, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(hashtags)))
}

pub async fn get_trending_hashtags(
    service: web::Data<HashtagService>,
) -> Result<HttpResponse, AppError> {
    let hashtags = service.get_trending_hashtags(10).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(hashtags)))
}

pub async fn get_hashtag_by_name(
    service: web::Data<HashtagService>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let name = path.into_inner();
    let (usage_count, posts_count) = service.get_hashtag_stats(&name).await?;
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
        "name": name.to_lowercase(),
        "usage_count": usage_count,
        "posts_count": posts_count
    }))))
}