use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{Comment, CommentResponseRow, FeedSort, Like, Post, PostResponseRow};

pub struct TimelineRepository {
    pool: PgPool,
}

impl TimelineRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_post(
        &self,
        author_id: Uuid,
        content: &str,
        media_urls: Option<Vec<String>>,
        is_public: bool,
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (author_id, content, media_urls, is_public)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(author_id)
        .bind(content)
        .bind(media_urls)
        .bind(is_public)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn get_post_by_id(&self, post_id: Uuid) -> Result<Post, AppError> {
        let post = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE id = $1"
        )
        .bind(post_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn get_post_response(
        &self,
        post_id: Uuid,
        requesting_user_id: Uuid,
    ) -> Result<PostResponseRow, AppError> {
        let post = sqlx::query_as::<_, PostResponseRow>(
            r#"
            SELECT 
                p.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name,
                    'avatar_url', u.avatar_url
                ) as author,
                p.content,
                p.media_urls,
                p.likes_count,
                p.comments_count,
                p.shares_count,
                p.is_public,
                p.created_at,
                EXISTS(
                    SELECT 1 FROM likes WHERE post_id = p.id AND user_id = $2
                ) as is_liked
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.id = $1
            "#
        )
        .bind(post_id)
        .bind(requesting_user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn get_feed(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
        sort: &FeedSort,
    ) -> Result<Vec<PostResponseRow>, AppError> {
        let order_clause = match sort {
            FeedSort::Newest => "p.created_at DESC",
            FeedSort::Oldest => "p.created_at ASC",
            FeedSort::Popular => "p.likes_count DESC, p.created_at DESC",
        };

        let sql = format!(
            r#"
            SELECT 
                p.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name,
                    'avatar_url', u.avatar_url
                ) as author,
                p.content,
                p.media_urls,
                p.likes_count,
                p.comments_count,
                p.shares_count,
                p.is_public,
                p.created_at,
                EXISTS(
                    SELECT 1 FROM likes WHERE post_id = p.id AND user_id = $1
                ) as is_liked
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.is_public = true 
               OR p.author_id = $1
               OR EXISTS(
                   SELECT 1 FROM follows WHERE follower_id = $1 AND following_id = p.author_id
               )
            ORDER BY {}
            LIMIT $2 OFFSET $3
            "#,
            order_clause
        );

        let posts = sqlx::query_as::<_, PostResponseRow>(&sql)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(posts)
    }

    pub async fn get_following_feed(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
        sort: &FeedSort,
    ) -> Result<Vec<PostResponseRow>, AppError> {
        let order_clause = match sort {
            FeedSort::Newest => "p.created_at DESC",
            FeedSort::Oldest => "p.created_at ASC",
            FeedSort::Popular => "p.likes_count DESC, p.created_at DESC",
        };

        let sql = format!(
            r#"
            SELECT 
                p.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name,
                    'avatar_url', u.avatar_url
                ) as author,
                p.content,
                p.media_urls,
                p.likes_count,
                p.comments_count,
                p.shares_count,
                p.is_public,
                p.created_at,
                EXISTS(
                    SELECT 1 FROM likes WHERE post_id = p.id AND user_id = $1
                ) as is_liked
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.author_id = $1
               OR EXISTS(
                   SELECT 1 FROM follows WHERE follower_id = $1 AND following_id = p.author_id
               )
            ORDER BY {}
            LIMIT $2 OFFSET $3
            "#,
            order_clause
        );

        let posts = sqlx::query_as::<_, PostResponseRow>(&sql)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(posts)
    }

    pub async fn get_user_posts(
        &self,
        author_id: Uuid,
        requesting_user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<PostResponseRow>, AppError> {
        let posts = sqlx::query_as::<_, PostResponseRow>(
            r#"
            SELECT 
                p.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name,
                    'avatar_url', u.avatar_url
                ) as author,
                p.content,
                p.media_urls,
                p.likes_count,
                p.comments_count,
                p.shares_count,
                p.is_public,
                p.created_at,
                EXISTS(
                    SELECT 1 FROM likes WHERE post_id = p.id AND user_id = $2
                ) as is_liked
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.author_id = $1 AND (p.is_public = true OR p.author_id = $2)
            ORDER BY p.created_at DESC
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(author_id)
        .bind(requesting_user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    pub async fn update_post(
        &self,
        post_id: Uuid,
        content: Option<&str>,
        is_public: Option<bool>,
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts 
            SET 
                content = COALESCE($2, content),
                is_public = COALESCE($3, is_public),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(post_id)
        .bind(content)
        .bind(is_public)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn delete_post(&self, post_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(post_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_like(&self, post_id: Uuid, user_id: Uuid) -> Result<Like, AppError> {
        let like = sqlx::query_as::<_, Like>(
            r#"
            INSERT INTO likes (post_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (post_id, user_id) DO NOTHING
            RETURNING *
            "#
        )
        .bind(post_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(like)
    }

    pub async fn delete_like(&self, post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM likes WHERE post_id = $1 AND user_id = $2")
            .bind(post_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_comment(
        &self,
        post_id: Uuid,
        author_id: Uuid,
        content: &str,
        parent_comment_id: Option<Uuid>,
    ) -> Result<Comment, AppError> {
        let comment = sqlx::query_as::<_, Comment>(
            r#"
            INSERT INTO comments (post_id, author_id, content, parent_comment_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(post_id)
        .bind(author_id)
        .bind(content)
        .bind(parent_comment_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    pub async fn get_comments(
        &self,
        post_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<CommentResponseRow>, AppError> {
        let comments = sqlx::query_as::<_, CommentResponseRow>(
            r#"
            SELECT 
                c.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name,
                    'avatar_url', u.avatar_url
                ) as author,
                c.content,
                c.parent_comment_id,
                c.likes_count,
                c.created_at
            FROM comments c
            JOIN users u ON c.author_id = u.id
            WHERE c.post_id = $1
            ORDER BY c.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(post_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(comments)
    }

    pub async fn get_comment_response_by_id(&self, comment_id: Uuid) -> Result<Option<CommentResponseRow>, AppError> {
        let comment = sqlx::query_as::<_, CommentResponseRow>(
            r#"
            SELECT 
                c.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name,
                    'avatar_url', u.avatar_url
                ) as author,
                c.content,
                c.parent_comment_id,
                c.likes_count,
                c.created_at
            FROM comments c
            JOIN users u ON c.author_id = u.id
            WHERE c.id = $1
            "#
        )
        .bind(comment_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(comment)
    }

    pub async fn delete_comment(&self, comment_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query(
            "DELETE FROM comments WHERE id = $1 AND author_id = $2"
        )
        .bind(comment_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFoundError("Comment not found".to_string()));
        }

        Ok(())
    }

    // ==================== Repost operations ====================

    pub async fn create_repost(
        &self,
        original_post_id: Uuid,
        reposter_id: Uuid,
        quote_text: Option<&str>,
    ) -> Result<super::models::Repost, AppError> {
        let repost = sqlx::query_as::<_, super::models::Repost>(
            r#"
            INSERT INTO reposts (original_post_id, reposter_id, quote_text)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(original_post_id)
        .bind(reposter_id)
        .bind(quote_text)
        .fetch_one(&self.pool)
        .await?;

        // Update repost count on original post
        sqlx::query(
            "UPDATE posts SET reposts_count = reposts_count + 1 WHERE id = $1"
        )
        .bind(original_post_id)
        .execute(&self.pool)
        .await?;

        Ok(repost)
    }

    pub async fn delete_repost(&self, original_post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query(
            "DELETE FROM reposts WHERE original_post_id = $1 AND reposter_id = $2"
        )
        .bind(original_post_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFoundError("Repost not found".to_string()));
        }

        // Update repost count on original post
        sqlx::query(
            "UPDATE posts SET reposts_count = GREATEST(reposts_count - 1, 0) WHERE id = $1"
        )
        .bind(original_post_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_reposts(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<super::models::RepostResponse>, AppError> {
        let reposts = sqlx::query_as::<_, RepostWithPostRow>(
            r#"
            SELECT 
                r.id,
                r.original_post_id,
                r.reposter_id,
                r.quote_text,
                r.created_at,
                p.id as post_id,
                p.content as post_content,
                p.media_urls as post_media_urls,
                p.likes_count as post_likes_count,
                p.comments_count as post_comments_count,
                p.shares_count as post_shares_count,
                p.is_public as post_is_public,
                p.created_at as post_created_at,
                p.author_id as post_author_id,
                pu.username as post_author_username,
                pu.display_name as post_author_display_name,
                pu.avatar_url as post_author_avatar_url,
                rp.username as reposter_username,
                rp.display_name as reposter_display_name,
                rp.avatar_url as reposter_avatar_url,
                EXISTS(SELECT 1 FROM likes WHERE post_id = p.id AND user_id = $1) as post_is_liked
            FROM reposts r
            JOIN posts p ON r.original_post_id = p.id
            JOIN users pu ON p.author_id = pu.id
            JOIN users rp ON r.reposter_id = rp.id
            WHERE r.reposter_id = $1
            ORDER BY r.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(reposts.into_iter().map(Into::into).collect())
    }

    pub async fn is_reposted(&self, post_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM reposts WHERE original_post_id = $1 AND reposter_id = $2)"
        )
        .bind(post_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }
}

/// Database row for repost with post details
#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct RepostWithPostRow {
    id: Uuid,
    original_post_id: Uuid,
    reposter_id: Uuid,
    quote_text: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    post_id: Uuid,
    post_content: String,
    post_media_urls: Option<Vec<String>>,
    post_likes_count: i32,
    post_comments_count: i32,
    post_shares_count: i32,
    post_is_public: bool,
    post_created_at: chrono::DateTime<chrono::Utc>,
    post_author_id: Uuid,
    post_author_username: String,
    post_author_display_name: Option<String>,
    post_author_avatar_url: Option<String>,
    reposter_username: String,
    reposter_display_name: Option<String>,
    reposter_avatar_url: Option<String>,
    post_is_liked: bool,
}

impl From<RepostWithPostRow> for super::models::RepostResponse {
    fn from(row: RepostWithPostRow) -> Self {
        use super::models::{PostAuthor, PostResponse, RepostResponse};
        
        RepostResponse {
            id: row.id,
            original_post: PostResponse {
                id: row.post_id,
                author: PostAuthor {
                    id: row.post_author_id,
                    username: row.post_author_username,
                    display_name: row.post_author_display_name,
                    avatar_url: row.post_author_avatar_url,
                },
                content: row.post_content,
                media_urls: row.post_media_urls,
                likes_count: row.post_likes_count,
                comments_count: row.post_comments_count,
                shares_count: row.post_shares_count,
                is_public: row.post_is_public,
                created_at: row.post_created_at,
                is_liked: row.post_is_liked,
            },
            quote_text: row.quote_text,
            created_at: row.created_at,
        }
    }
}
