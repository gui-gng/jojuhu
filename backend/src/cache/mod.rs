use redis::{aio::ConnectionManager, Client, RedisResult};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// Redis cache wrapper
#[derive(Clone)]
#[allow(dead_code)]
pub struct RedisCache {
    client: Client,
}

#[allow(dead_code)]
impl RedisCache {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }
    
    pub async fn get_connection(&self,
    ) -> RedisResult<ConnectionManager> {
        ConnectionManager::new(self.client.clone()).await
    }
    
    /// Get value from cache
    pub async fn get<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> RedisResult<Option<T>> {
        let mut conn = self.get_connection().await?;
        let value: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await?;
        
        match value {
            Some(v) => {
                match serde_json::from_str(&v) {
                    Ok(data) => Ok(Some(data)),
                    Err(_) => Ok(None),
                }
            }
            None => Ok(None),
        }
    }
    
    /// Set value in cache with expiration
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        let serialized = serde_json::to_string(value).unwrap_or_default();
        
        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl.as_secs() as usize)
            .arg(serialized)
            .query_async::<_, ()>(&mut conn)
            .await?;
        
        Ok(())
    }
    
    /// Delete key from cache
    pub async fn del(&self,
        key: &str,
    ) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        redis::cmd("DEL")
            .arg(key)
            .query_async::<_, ()>(&mut conn)
            .await?;
        Ok(())
    }
    
    /// Check if key exists
    pub async fn exists(&self,
        key: &str,
    ) -> RedisResult<bool> {
        let mut conn = self.get_connection().await?;
        let exists: i32 = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await?;
        Ok(exists == 1)
    }
    
    /// Cache feed for user
    pub async fn cache_user_feed(
        &self,
        user_id: uuid::Uuid,
        posts: &Vec<crate::modules::timeline::models::PostResponse>,
    ) -> RedisResult<()> {
        let key = format!("feed:user:{}", user_id);
        self.set(&key, posts, Duration::from_secs(300)).await
    }
    
    /// Get cached feed for user
    pub async fn get_cached_feed(
        &self,
        user_id: uuid::Uuid,
    ) -> RedisResult<Option<Vec<crate::modules::timeline::models::PostResponse>>> {
        let key = format!("feed:user:{}", user_id);
        self.get(&key).await
    }
    
    /// Invalidate user feed cache
    pub async fn invalidate_feed(
        &self,
        user_id: uuid::Uuid,
    ) -> RedisResult<()> {
        let key = format!("feed:user:{}", user_id);
        self.del(&key).await
    }
    
    /// Cache user profile
    pub async fn cache_user_profile(
        &self,
        user_id: uuid::Uuid,
        profile: &crate::modules::users::models::UserProfile,
    ) -> RedisResult<()> {
        let key = format!("profile:user:{}", user_id);
        self.set(&key, profile, Duration::from_secs(600)).await
    }
    
    /// Get cached user profile
    pub async fn get_cached_profile(
        &self,
        user_id: uuid::Uuid,
    ) -> RedisResult<Option<crate::modules::users::models::UserProfile>> {
        let key = format!("profile:user:{}", user_id);
        self.get(&key).await
    }
}