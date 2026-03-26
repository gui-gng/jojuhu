//! Search functionality
//!
//! This module provides full-text search capabilities across the platform
//! including posts, forums, topics, and users.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{PaginatedResponse, PaginationParams};

/// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// Search query string
    pub q: String,
    /// Type of content to search: "posts", "forums", "topics", "users", or "all"
    #[serde(default = "default_search_type")]
    pub search_type: String,
}

fn default_search_type() -> String {
    "all".to_string()
}

/// Search result item
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub id: uuid::Uuid,
    pub result_type: String,
    pub title: String,
    pub content: Option<String>,
    pub author_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub relevance_score: f32,
}

/// Perform search across the platform
pub async fn search(
    pool: web::Data<PgPool>,
    query: web::Query<SearchQuery>,
    pagination: web::Query<PaginationParams>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let search_query = &query.q;
    
    // Validate search query
    if search_query.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Search query cannot be empty".to_string(),
        ));
    }
    
    if search_query.len() < 2 {
        return Err(AppError::ValidationError(
            "Search query must be at least 2 characters".to_string(),
        ));
    }

    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    let results = match query.search_type.as_str() {
        "posts" => search_posts(&pool, search_query, offset, limit).await?,
        "forums" => search_forums(&pool, search_query, offset, limit).await?,
        "topics" => search_topics(&pool, search_query, offset, limit).await?,
        "users" => search_users(&pool, search_query, offset, limit).await?,
        _ => search_all(&pool, search_query, offset, limit).await?,
    };

    let total = results.len() as i64;

    Ok(HttpResponse::Ok().json(PaginatedResponse {
        items: results,
        total,
        page: pagination.page.unwrap_or(1),
        per_page: limit,
        total_pages: ((total as f32) / (limit as f32)).ceil() as i32,
    }))
}

/// Search posts
async fn search_posts(
    pool: &PgPool,
    query: &str,
    offset: i32,
    limit: i32,
) -> Result<Vec<SearchResult>, AppError> {
    let results = sqlx::query_as::<_, SearchResult>(
        r#"
        SELECT 
            p.id,
            'post' as result_type,
            COALESCE(u.username, 'Unknown') as author_name,
            p.content as content,
            '' as title,
            p.created_at,
            ts_rank_cd(to_tsvector('english', p.content), plainto_tsquery('english', $1)) as relevance_score
        FROM posts p
        JOIN users u ON p.author_id = u.id
        WHERE to_tsvector('english', p.content) @@ plainto_tsquery('english', $1)
            AND p.is_public = true
        ORDER BY relevance_score DESC, p.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(query)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(results)
}

/// Search forums
async fn search_forums(
    pool: &PgPool,
    query: &str,
    offset: i32,
    limit: i32,
) -> Result<Vec<SearchResult>, AppError> {
    let results = sqlx::query_as::<_, SearchResult>(
        r#"
        SELECT 
            f.id,
            'forum' as result_type,
            COALESCE(u.username, 'Unknown') as author_name,
            f.description as content,
            f.name as title,
            f.created_at,
            ts_rank_cd(
                to_tsvector('english', f.name || ' ' || COALESCE(f.description, '')),
                plainto_tsquery('english', $1)
            ) as relevance_score
        FROM forums f
        JOIN users u ON f.creator_id = u.id
        WHERE to_tsvector('english', f.name || ' ' || COALESCE(f.description, '')) 
            @@ plainto_tsquery('english', $1)
            AND f.is_public = true
        ORDER BY relevance_score DESC, f.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(query)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(results)
}

/// Search topics
async fn search_topics(
    pool: &PgPool,
    query: &str,
    offset: i32,
    limit: i32,
) -> Result<Vec<SearchResult>, AppError> {
    let results = sqlx::query_as::<_, SearchResult>(
        r#"
        SELECT 
            t.id,
            'topic' as result_type,
            COALESCE(u.username, 'Unknown') as author_name,
            t.content as content,
            t.title as title,
            t.created_at,
            ts_rank_cd(
                to_tsvector('english', t.title || ' ' || t.content),
                plainto_tsquery('english', $1)
            ) as relevance_score
        FROM topics t
        JOIN users u ON t.author_id = u.id
        JOIN forums f ON t.forum_id = f.id
        WHERE to_tsvector('english', t.title || ' ' || t.content) 
            @@ plainto_tsquery('english', $1)
            AND f.is_public = true
        ORDER BY relevance_score DESC, t.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(query)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(results)
}

/// Search users
async fn search_users(
    pool: &PgPool,
    query: &str,
    offset: i32,
    limit: i32,
) -> Result<Vec<SearchResult>, AppError> {
    let results = sqlx::query_as::<_, SearchResult>(
        r#"
        SELECT 
            u.id,
            'user' as result_type,
            u.username as author_name,
            u.bio as content,
            u.display_name as title,
            u.created_at,
            ts_rank_cd(
                to_tsvector('english', u.username || ' ' || COALESCE(u.display_name, '') || ' ' || COALESCE(u.bio, '')),
                plainto_tsquery('english', $1)
            ) as relevance_score
        FROM users u
        WHERE to_tsvector('english', u.username || ' ' || COALESCE(u.display_name, '') || ' ' || COALESCE(u.bio, '')) 
            @@ plainto_tsquery('english', $1)
        ORDER BY relevance_score DESC, u.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(query)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(results)
}

/// Search across all content types
async fn search_all(
    pool: &PgPool,
    query: &str,
    _offset: i32,
    limit: i32,
) -> Result<Vec<SearchResult>, AppError> {
    let mut results = Vec::new();
    
    // Search posts
    let posts = search_posts(pool, query, 0, limit / 4).await?;
    results.extend(posts);
    
    // Search forums
    let forums = search_forums(pool, query, 0, limit / 4).await?;
    results.extend(forums);
    
    // Search topics
    let topics = search_topics(pool, query, 0, limit / 4).await?;
    results.extend(topics);
    
    // Search users
    let users = search_users(pool, query, 0, limit / 4).await?;
    results.extend(users);
    
    // Sort by relevance score and take top results
    results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
    results.truncate(limit as usize);
    
    Ok(results)
}

/// Configure search routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/search")
            .route(web::get().to(search))
    );
}
