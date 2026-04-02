use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::cache::RedisCache;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{ApiResponse, PaginationParams};
use crate::notifications::NotificationService;
use crate::modules::users::repository::UserRepository;
use crate::websocket::{WebSocketServer, WsMessage};

use super::models::{CreateCommentRequest, CreatePostRequest, CreateRepostRequest, FeedSort, TimelineFeedQuery, UpdatePostRequest};
use super::service::TimelineService;

pub async fn create_post(
    service: web::Data<TimelineService>,
    cache: Option<web::Data<RedisCache>>,
    ws_server: Option<web::Data<WebSocketServer>>,
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    request: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, AppError> {
    let author_id = user.user_id;
    let post = service.create_post(author_id, request.into_inner()).await?;
    
    // Invalidate following feed cache for user's followers
    if let Some(cache) = cache {
        let _ = cache.invalidate_feed(author_id).await;
    }
    
    // Send real-time notification to followers about new post
    if let Some(ref ws) = ws_server {
        let user_repo = UserRepository::new(pool.get_ref().clone());
        if let Ok(profile) = user_repo.get_profile_by_id(author_id).await {
            let author_name = profile.display_name.unwrap_or(profile.username);
            let content_preview = if post.content.len() > 50 {
                format!("{}...", &post.content[..50])
            } else {
                post.content.clone()
            };
            
            // Get follower IDs
            if let Ok(follower_ids) = user_repo.get_follower_ids(author_id).await {
                for follower_id in follower_ids {
                    let ws_msg = WsMessage::NewPost {
                        post_id: post.id,
                        author_id,
                        author_name: author_name.clone(),
                        content_preview: content_preview.clone(),
                    };
                    ws.send_to_user(follower_id, ws_msg).await;
                }
            }
        }
    }
    
    Ok(HttpResponse::Created().json(ApiResponse::success(post)))
}

pub async fn get_feed(
    service: web::Data<TimelineService>,
    cache: Option<web::Data<RedisCache>>,
    user: AuthenticatedUser,
    query: web::Query<TimelineFeedQuery>,
) -> Result<HttpResponse, AppError> {
    let offset = query.page.unwrap_or(1).saturating_sub(1) * query.per_page.unwrap_or(20);
    let limit = query.per_page.unwrap_or(20);
    let sort = query.sort.as_ref().unwrap_or(&FeedSort::Newest);

    // Only cache first page of feed
    if offset == 0 {
        if let Some(ref cache) = cache {
            if let Ok(Some(posts)) = cache.get_cached_feed(user.user_id).await {
                return Ok(HttpResponse::Ok().json(ApiResponse::success(posts)));
            }
        }
    }

    let posts = service.get_feed(user.user_id, offset, limit, sort).await?;
    
    // Cache first page
    if offset == 0 {
        if let Some(cache) = cache {
            let _ = cache.cache_user_feed(user.user_id, &posts).await;
        }
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_following_feed(
    service: web::Data<TimelineService>,
    cache: Option<web::Data<RedisCache>>,
    user: AuthenticatedUser,
    query: web::Query<TimelineFeedQuery>,
) -> Result<HttpResponse, AppError> {
    let offset = query.page.unwrap_or(1).saturating_sub(1) * query.per_page.unwrap_or(20);
    let limit = query.per_page.unwrap_or(20);
    let sort = query.sort.as_ref().unwrap_or(&FeedSort::Newest);

    // Cache key for following feed
    let cache_key = format!("following:{}", user.user_id);

    // Only cache first page
    if offset == 0 {
        if let Some(ref cache) = cache {
            if let Ok(Some(posts)) = cache.get::<Vec<super::models::PostResponse>>(&cache_key).await {
                return Ok(HttpResponse::Ok().json(ApiResponse::success(posts)));
            }
        }
    }

    let posts = service.get_following_feed(user.user_id, offset, limit, sort).await?;
    
    // Cache first page
    if offset == 0 {
        if let Some(cache) = cache {
            let _ = cache.set(&cache_key, &posts, std::time::Duration::from_secs(120)).await;
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_user_posts(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let author_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();

    let posts = service
        .get_user_posts(author_id, user.user_id, offset, limit)
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let post = service.get_post(post_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(post)))
}

pub async fn update_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let post = service
        .update_post(post_id, user.user_id, request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(post)))
}

pub async fn delete_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.delete_post(post_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn like_post(
    service: web::Data<TimelineService>,
    notification_service: web::Data<NotificationService>,
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.like_post(post_id, user.user_id).await?;
    
    // Get post to find author and send notification
    let post = service.repository.get_post_by_id(post_id).await?;
    if post.author_id != user.user_id {
        // Don't notify for self-likes
        let user_repo = UserRepository::new(pool.get_ref().clone());
        let liker_profile = user_repo.get_profile_by_id(user.user_id).await?;
        let liker_name = liker_profile.display_name
            .unwrap_or(liker_profile.username);
        
        let _ = notification_service
            .notify_post_like(user.user_id, &liker_name, post.author_id, post_id)
            .await;
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"liked": true}))))
}

pub async fn unlike_post(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.unlike_post(post_id, user.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"liked": false}))))
}

pub async fn add_comment(
    service: web::Data<TimelineService>,
    notification_service: web::Data<NotificationService>,
    ws_server: Option<web::Data<WebSocketServer>>,
    pool: web::Data<sqlx::PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    request: web::Json<CreateCommentRequest>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let author_id = user.user_id;
    let content = request.content.clone();
    let comment = service
        .add_comment(post_id, author_id, request.into_inner())
        .await?;
    
    // Get post to find author and send notification
    let post = service.repository.get_post_by_id(post_id).await?;
    if post.author_id != author_id {
        // Don't notify for self-comments
        let user_repo = UserRepository::new(pool.get_ref().clone());
        let commenter_profile = user_repo.get_profile_by_id(author_id).await?;
        let commenter_name = commenter_profile.display_name
            .unwrap_or(commenter_profile.username);
        
        let preview = if content.len() > 50 {
            format!("{}...", &content[..50])
        } else {
            content.clone()
        };
        
        let _ = notification_service
            .notify_post_comment(author_id, &commenter_name, post.author_id, post_id, &preview)
            .await;
        
        // Check for mentions in the comment
        let _ = check_and_notify_mentions(
            &content,
            author_id,
            &commenter_name,
            "comment",
            post_id,
            &notification_service,
            &pool,
        ).await;
        
        // Send real-time comment notification via WebSocket
        if let Some(ref ws) = ws_server {
            let ws_msg = WsMessage::NewComment {
                post_id,
                comment_id: comment.id,
                author_id,
                author_name: commenter_name.clone(),
                content: content.clone(),
            };
            ws.send_to_user(post.author_id, ws_msg).await;
        }
    }
    
    Ok(HttpResponse::Created().json(ApiResponse::success(comment)))
}

pub async fn get_comments(
    service: web::Data<TimelineService>,
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    let offset = query.get_offset();
    let limit = query.get_limit();

    let comments = service.get_comments(post_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(comments)))
}

pub async fn delete_comment(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_post_id, comment_id) = path.into_inner();
    service.delete_comment(comment_id, user.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

async fn check_and_notify_mentions(
    content: &str,
    author_id: Uuid,
    author_name: &str,
    content_type: &str,
    content_id: Uuid,
    notification_service: &NotificationService,
    pool: &web::Data<sqlx::PgPool>,
) -> Result<(), AppError> {
    use regex::Regex;
    use std::sync::LazyLock;
    
    static MENTION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"@([a-zA-Z0-9_]+)").unwrap()
    });
    
    let user_repo = UserRepository::new(pool.get_ref().clone());
    
    for cap in MENTION_REGEX.captures_iter(content) {
        if let Some(username) = cap.get(1) {
            if let Ok(Some(mentioned_user)) = user_repo.find_by_username(username.as_str()).await {
                if mentioned_user.id != author_id {
                    let _ = notification_service
                        .notify_mention(author_id, author_name, mentioned_user.id, content_type, content_id)
                        .await;
                }
            }
        }
    }
    
    Ok(())
}

// ==================== Repost handlers ====================

pub async fn create_repost(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    request: web::Json<CreateRepostRequest>,
) -> Result<HttpResponse, AppError> {
    let repost = service.create_repost(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(repost)))
}

pub async fn delete_repost(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let post_id = path.into_inner();
    service.delete_repost(user.user_id, post_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_my_reposts(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();
    
    let reposts = service.get_user_reposts(user.user_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(reposts)))
}

// ==================== Algorithmic Feed handlers ====================

pub async fn get_for_you_feed(
    service: web::Data<TimelineService>,
    cache: Option<web::Data<RedisCache>>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();

    // Cache first page of For You feed
    let cache_key = format!("for_you:{}", user.user_id);
    
    if offset == 0 {
        if let Some(ref cache) = cache {
            if let Ok(Some(posts)) = cache.get::<Vec<super::models::PostResponse>>(&cache_key).await {
                return Ok(HttpResponse::Ok().json(ApiResponse::success(posts)));
            }
        }
    }

    let posts = service.get_for_you_feed(user.user_id, offset, limit).await?;
    
    // Cache first page for 2 minutes
    if offset == 0 {
        if let Some(cache) = cache {
            let _ = cache.set(&cache_key, &posts, std::time::Duration::from_secs(120)).await;
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_trending_posts(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let offset = query.get_offset();
    let limit = query.get_limit();
    
    let posts = service.get_trending_posts(user.user_id, offset, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(posts)))
}

pub async fn get_suggested_users(
    service: web::Data<TimelineService>,
    user: AuthenticatedUser,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let limit = query.per_page.unwrap_or(10).min(50);
    
    let users = service.get_suggested_users(user.user_id, limit).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(users)))
}
