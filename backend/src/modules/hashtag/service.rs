use sqlx::PgPool;

use crate::errors::AppError;
use crate::utils::hashtags::extract_hashtags;

use super::models::{HashtagResponse, TrendingHashtag};
use super::repository::HashtagRepository;

pub struct HashtagService {
    repository: HashtagRepository,
}

impl HashtagService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: HashtagRepository::new(pool),
        }
    }

    pub async fn process_post_hashtags(
        &self,
        post_id: uuid::Uuid,
        content: &str,
    ) -> Result<(), AppError> {
        let hashtags = extract_hashtags(content);

        for tag in hashtags {
            let hashtag = self.repository.find_or_create(&tag).await?;
            self.repository
                .link_hashtag_to_post(post_id, hashtag.id)
                .await?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn remove_post_hashtags(
        &self,
        post_id: uuid::Uuid,
        content: &str,
    ) -> Result<(), AppError> {
        let hashtags = extract_hashtags(content);

        for tag in hashtags {
            if let Some(hashtag) = self.repository.find_by_name(&tag).await? {
                self.repository
                    .unlink_hashtag_from_post(post_id, hashtag.id)
                    .await?;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_post_hashtags(
        &self,
        post_id: uuid::Uuid,
    ) -> Result<Vec<HashtagResponse>, AppError> {
        self.repository.get_post_hashtags(post_id).await
    }

    pub async fn get_trending_hashtags(
        &self,
        limit: i32,
    ) -> Result<Vec<TrendingHashtag>, AppError> {
        self.repository.get_trending_hashtags(limit).await
    }

    pub async fn search_hashtags(
        &self,
        query: &str,
        limit: i32,
    ) -> Result<Vec<HashtagResponse>, AppError> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

        self.repository.search_hashtags(query, limit).await
    }

    pub async fn get_hashtag_stats(&self, name: &str) -> Result<(i32, i32), AppError> {
        let hashtag = self.repository.find_by_name(name).await?;

        match hashtag {
            Some(h) => {
                let posts_count = self.repository.get_hashtag_posts_count(name).await?;
                Ok((h.usage_count, posts_count))
            }
            None => Ok((0, 0)),
        }
    }
}
