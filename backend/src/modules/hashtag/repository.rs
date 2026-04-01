use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{Hashtag, HashtagResponse, TrendingHashtag};

pub struct HashtagRepository {
    pool: PgPool,
}

impl HashtagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[allow(dead_code)]
    pub async fn find_or_create(&self, name: &str) -> Result<Hashtag, AppError> {
        let name_lower = name.to_lowercase();
        
        // Try to find existing hashtag
        if let Some(hashtag) = self.find_by_name(&name_lower).await? {
            return Ok(hashtag);
        }
        
        // Create new hashtag
        let hashtag = sqlx::query_as::<_, Hashtag>(
            r#"
            INSERT INTO hashtags (name)
            VALUES ($1)
            RETURNING *
            "#
        )
        .bind(&name_lower)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(hashtag)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Hashtag>, AppError> {
        let hashtag = sqlx::query_as::<_, Hashtag>(
            "SELECT * FROM hashtags WHERE name = $1"
        )
        .bind(name.to_lowercase())
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(hashtag)
    }

    #[allow(dead_code)]
    pub async fn link_hashtag_to_post(&self, post_id: Uuid, hashtag_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO post_hashtags (post_id, hashtag_id)
            VALUES ($1, $2)
            ON CONFLICT (post_id, hashtag_id) DO NOTHING
            "#
        )
        .bind(post_id)
        .bind(hashtag_id)
        .execute(&self.pool)
        .await?;
        
        // Increment usage count
        sqlx::query(
            "UPDATE hashtags SET usage_count = usage_count + 1, updated_at = NOW() WHERE id = $1"
        )
        .bind(hashtag_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn unlink_hashtag_from_post(&self, post_id: Uuid, hashtag_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM post_hashtags WHERE post_id = $1 AND hashtag_id = $2"
        )
        .bind(post_id)
        .bind(hashtag_id)
        .execute(&self.pool)
        .await?;
        
        // Decrement usage count
        sqlx::query(
            "UPDATE hashtags SET usage_count = GREATEST(usage_count - 1, 0), updated_at = NOW() WHERE id = $1"
        )
        .bind(hashtag_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_post_hashtags(&self, post_id: Uuid) -> Result<Vec<HashtagResponse>, AppError> {
        let hashtags = sqlx::query_as::<_, HashtagResponse>(
            r#"
            SELECT h.name, h.usage_count
            FROM hashtags h
            JOIN post_hashtags ph ON h.id = ph.hashtag_id
            WHERE ph.post_id = $1
            ORDER BY h.name
            "#
        )
        .bind(post_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(hashtags)
    }

    pub async fn get_trending_hashtags(&self, limit: i32) -> Result<Vec<TrendingHashtag>, AppError> {
        let hashtags = sqlx::query_as::<_, TrendingHashtag>(
            r#"
            SELECT 
                h.name,
                h.usage_count,
                COUNT(DISTINCT ph.post_id) as posts_count
            FROM hashtags h
            JOIN post_hashtags ph ON h.id = ph.hashtag_id
            JOIN posts p ON ph.post_id = p.id
            WHERE p.created_at > NOW() - INTERVAL '7 days'
            GROUP BY h.id, h.name, h.usage_count
            ORDER BY posts_count DESC, h.usage_count DESC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(hashtags)
    }

    pub async fn search_hashtags(&self, query: &str, limit: i32) -> Result<Vec<HashtagResponse>, AppError> {
        let hashtags = sqlx::query_as::<_, HashtagResponse>(
            r#"
            SELECT name, usage_count
            FROM hashtags
            WHERE name LIKE $1
            ORDER BY usage_count DESC
            LIMIT $2
            "#
        )
        .bind(format!("{}%", query.to_lowercase()))
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(hashtags)
    }

    pub async fn get_hashtag_posts_count(&self, name: &str) -> Result<i32, AppError> {
        let count: i32 = sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT ph.post_id)::int
            FROM post_hashtags ph
            JOIN hashtags h ON ph.hashtag_id = h.id
            WHERE h.name = $1
            "#
        )
        .bind(name.to_lowercase())
        .fetch_one(&self.pool)
        .await?;
        
        Ok(count)
    }
}