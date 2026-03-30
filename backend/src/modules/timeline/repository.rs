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
}
