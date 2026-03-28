use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{
    Forum, ForumMember, ForumResponseRow, ForumRole, ReplyResponseRow, Topic, TopicReply,
    TopicResponseRow,
};

pub struct ForumRepository {
    pool: PgPool,
}

impl ForumRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn generate_slug(name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
    }

    pub async fn create_forum(
        &self,
        name: &str,
        description: Option<&str>,
        creator_id: Uuid,
        is_public: bool,
    ) -> Result<Forum, AppError> {
        let slug = Self::generate_slug(name);
        
        let forum = sqlx::query_as::<_, Forum>(
            r#"
            INSERT INTO forums (name, slug, description, creator_id, is_public)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(name)
        .bind(&slug)
        .bind(description)
        .bind(creator_id)
        .bind(is_public)
        .fetch_one(&self.pool)
        .await?;

        Ok(forum)
    }

    pub async fn get_forum_by_id(&self, forum_id: Uuid) -> Result<Forum, AppError> {
        let forum = sqlx::query_as::<_, Forum>(
            "SELECT * FROM forums WHERE id = $1"
        )
        .bind(forum_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(forum)
    }

    #[allow(dead_code)]
    pub async fn get_forum_by_slug(&self, slug: &str) -> Result<Forum, AppError> {
        let forum = sqlx::query_as::<_, Forum>(
            "SELECT * FROM forums WHERE slug = $1"
        )
        .bind(slug)
        .fetch_one(&self.pool)
        .await?;

        Ok(forum)
    }

    pub async fn get_forum_response_by_id(&self, forum_id: Uuid, user_id: Option<Uuid>) -> Result<Option<ForumResponseRow>, AppError> {
        let forum = sqlx::query_as::<_, ForumResponseRow>(
            r#"
            SELECT 
                f.id,
                f.name,
                f.slug,
                f.description,
                f.icon_url,
                f.cover_image_url,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as creator,
                f.is_public,
                f.members_count,
                f.topics_count,
                $2::uuid IS NOT NULL AND EXISTS(
                    SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $2
                ) as is_member,
                f.created_at
            FROM forums f
            JOIN users u ON f.creator_id = u.id
            WHERE f.id = $1
            "#
        )
        .bind(forum_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(forum)
    }

    pub async fn list_forums(
        &self,
        offset: i32,
        limit: i32,
        user_id: Option<Uuid>,
    ) -> Result<Vec<ForumResponseRow>, AppError> {
        let forums: Vec<ForumResponseRow> = sqlx::query_as::<_, ForumResponseRow>(
            r#"
            SELECT 
                f.id,
                f.name,
                f.slug,
                f.description,
                f.icon_url,
                f.cover_image_url,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as creator,
                f.is_public,
                f.members_count,
                f.topics_count,
                $4::uuid IS NOT NULL AND EXISTS(
                    SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $4
                ) as is_member,
                f.created_at
            FROM forums f
            JOIN users u ON f.creator_id = u.id
            WHERE f.is_public = true OR ($4::uuid IS NOT NULL AND EXISTS(
                SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $4
            ))
            ORDER BY f.created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .bind(user_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(forums)
    }

    /// Search forums by name or description
    pub async fn search_forums(
        &self,
        user_id: Option<Uuid>,
        search: Option<&str>,
        sort_by: &str,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<ForumResponseRow>, AppError> {
        let search_pattern = search.map(|s| format!("%{}%", s.to_lowercase()));
        
        let order_clause = match sort_by {
            "popular" => "f.members_count DESC",
            "most_active" => "f.topics_count DESC",
            "most_members" => "f.members_count DESC",
            _ => "f.created_at DESC", // newest
        };

        let sql = format!(
            r#"
            SELECT 
                f.id,
                f.name,
                f.slug,
                f.description,
                f.icon_url,
                f.cover_image_url,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as creator,
                f.is_public,
                f.members_count,
                f.topics_count,
                $4::uuid IS NOT NULL AND EXISTS(
                    SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $4
                ) as is_member,
                f.created_at
            FROM forums f
            JOIN users u ON f.creator_id = u.id
            WHERE (f.is_public = true OR ($4::uuid IS NOT NULL AND EXISTS(
                SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $4
            )))
            AND ($5::text IS NULL OR (LOWER(f.name) LIKE $5 OR LOWER(f.description) LIKE $5))
            ORDER BY {}
            LIMIT $1 OFFSET $2
            "#,
            order_clause
        );

        let forums = sqlx::query_as::<_, ForumResponseRow>(&sql)
            .bind(limit)
            .bind(offset)
            .bind(user_id)
            .bind(user_id)
            .bind(search_pattern)
            .fetch_all(&self.pool)
            .await?;

        Ok(forums)
    }

    /// Get trending forums (most active in last 30 days)
    pub async fn get_trending_forums(
        &self,
        user_id: Option<Uuid>,
        limit: i32,
    ) -> Result<Vec<ForumResponseRow>, AppError> {
        let forums = sqlx::query_as::<_, ForumResponseRow>(
            r#"
            SELECT 
                f.id,
                f.name,
                f.slug,
                f.description,
                f.icon_url,
                f.cover_image_url,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as creator,
                f.is_public,
                f.members_count,
                f.topics_count,
                $3::uuid IS NOT NULL AND EXISTS(
                    SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $3
                ) as is_member,
                f.created_at
            FROM forums f
            JOIN users u ON f.creator_id = u.id
            WHERE f.is_public = true OR ($3::uuid IS NOT NULL AND EXISTS(
                SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $3
            ))
            ORDER BY (
                SELECT COUNT(*) FROM topics t 
                WHERE t.forum_id = f.id AND t.created_at > NOW() - INTERVAL '30 days'
            ) DESC
            LIMIT $1
            "#
        )
        .bind(limit)
        .bind(user_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(forums)
    }

    /// Get forum count for search
    pub async fn get_search_count(
        &self,
        user_id: Option<Uuid>,
        search: Option<&str>,
    ) -> Result<i64, AppError> {
        let search_pattern = search.map(|s| format!("%{}%", s.to_lowercase()));

        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) 
            FROM forums f
            WHERE (f.is_public = true OR ($1::uuid IS NOT NULL AND EXISTS(
                SELECT 1 FROM forum_members WHERE forum_id = f.id AND user_id = $1
            )))
            AND ($2::text IS NULL OR (LOWER(f.name) LIKE $2 OR LOWER(f.description) LIKE $2))
            "#
        )
        .bind(user_id)
        .bind(search_pattern)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    pub async fn update_forum(
        &self,
        forum_id: Uuid,
        name: Option<&str>,
        description: Option<Option<&str>>,
        is_public: Option<bool>,
    ) -> Result<Forum, AppError> {
        let slug = name.map(Self::generate_slug);
        
        let forum = sqlx::query_as::<_, Forum>(
            r#"
            UPDATE forums 
            SET 
                name = COALESCE($2, name),
                slug = COALESCE($3, slug),
                description = COALESCE($4, description),
                is_public = COALESCE($5, is_public),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(forum_id)
        .bind(name)
        .bind(slug)
        .bind(description)
        .bind(is_public)
        .fetch_one(&self.pool)
        .await?;

        Ok(forum)
    }

    pub async fn delete_forum(&self, forum_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM forums WHERE id = $1")
            .bind(forum_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_member(
        &self,
        forum_id: Uuid,
        user_id: Uuid,
        role: ForumRole,
    ) -> Result<ForumMember, AppError> {
        let member = sqlx::query_as::<_, ForumMember>(
            r#"
            INSERT INTO forum_members (forum_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (forum_id, user_id) DO UPDATE SET role = $3
            RETURNING *
            "#
        )
        .bind(forum_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;

        Ok(member)
    }

    pub async fn remove_member(&self, forum_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM forum_members WHERE forum_id = $1 AND user_id = $2")
            .bind(forum_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_member(
        &self,
        forum_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<ForumMember>, AppError> {
        let member = sqlx::query_as::<_, ForumMember>(
            "SELECT * FROM forum_members WHERE forum_id = $1 AND user_id = $2"
        )
        .bind(forum_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    pub async fn create_topic(
        &self,
        forum_id: Uuid,
        author_id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<Topic, AppError> {
        let topic = sqlx::query_as::<_, Topic>(
            r#"
            INSERT INTO topics (forum_id, author_id, title, content)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(forum_id)
        .bind(author_id)
        .bind(title)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        Ok(topic)
    }

    pub async fn get_topic_by_id(&self, topic_id: Uuid) -> Result<Topic, AppError> {
        let topic = sqlx::query_as::<_, Topic>(
            "SELECT * FROM topics WHERE id = $1"
        )
        .bind(topic_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(topic)
    }

    pub async fn get_topic_response_by_id(&self, topic_id: Uuid) -> Result<Option<TopicResponseRow>, AppError> {
        let topic = sqlx::query_as::<_, TopicResponseRow>(
            r#"
            SELECT 
                t.id,
                t.forum_id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as author,
                t.title,
                t.content,
                t.is_pinned,
                t.is_locked,
                t.views_count,
                t.replies_count,
                t.created_at
            FROM topics t
            JOIN users u ON t.author_id = u.id
            WHERE t.id = $1
            "#
        )
        .bind(topic_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(topic)
    }

    pub async fn get_topics(
        &self,
        forum_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<TopicResponseRow>, AppError> {
        let topics: Vec<TopicResponseRow> = sqlx::query_as::<_, TopicResponseRow>(
            r#"
            SELECT 
                t.id,
                t.forum_id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as author,
                t.title,
                t.content,
                t.is_pinned,
                t.is_locked,
                t.views_count,
                t.replies_count,
                t.created_at
            FROM topics t
            JOIN users u ON t.author_id = u.id
            WHERE t.forum_id = $1
            ORDER BY t.is_pinned DESC, t.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(forum_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(topics)
    }

    pub async fn increment_topic_views(&self, topic_id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE topics SET views_count = views_count + 1 WHERE id = $1")
            .bind(topic_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_topic(&self, topic_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM topics WHERE id = $1")
            .bind(topic_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_reply(
        &self,
        topic_id: Uuid,
        author_id: Uuid,
        content: &str,
        parent_reply_id: Option<Uuid>,
    ) -> Result<TopicReply, AppError> {
        let reply = sqlx::query_as::<_, TopicReply>(
            r#"
            INSERT INTO topic_replies (topic_id, author_id, content, parent_reply_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(topic_id)
        .bind(author_id)
        .bind(content)
        .bind(parent_reply_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(reply)
    }

    pub async fn get_replies(
        &self,
        topic_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<ReplyResponseRow>, AppError> {
        let replies: Vec<ReplyResponseRow> = sqlx::query_as::<_, ReplyResponseRow>(
            r#"
            SELECT 
                tr.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as author,
                tr.content,
                tr.parent_reply_id,
                tr.likes_count,
                tr.created_at
            FROM topic_replies tr
            JOIN users u ON tr.author_id = u.id
            WHERE tr.topic_id = $1
            ORDER BY tr.created_at ASC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(topic_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(replies)
    }

    pub async fn get_reply_response_by_id(&self, reply_id: Uuid) -> Result<Option<ReplyResponseRow>, AppError> {
        let reply = sqlx::query_as::<_, ReplyResponseRow>(
            r#"
            SELECT 
                tr.id,
                json_build_object(
                    'id', u.id,
                    'username', u.username,
                    'display_name', u.display_name
                ) as author,
                tr.content,
                tr.parent_reply_id,
                tr.likes_count,
                tr.created_at
            FROM topic_replies tr
            JOIN users u ON tr.author_id = u.id
            WHERE tr.id = $1
            "#
        )
        .bind(reply_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(reply)
    }

    pub async fn delete_reply(&self, reply_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM topic_replies WHERE id = $1")
            .bind(reply_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_topic_lock(
        &self,
        topic_id: Uuid,
        is_locked: bool,
    ) -> Result<Topic, AppError> {
        let topic = sqlx::query_as::<_, Topic>(
            "UPDATE topics SET is_locked = $2, updated_at = NOW() WHERE id = $1 RETURNING *"
        )
        .bind(topic_id)
        .bind(is_locked)
        .fetch_one(&self.pool)
        .await?;

        Ok(topic)
    }

    pub async fn update_topic_pin(
        &self,
        topic_id: Uuid,
        is_pinned: bool,
    ) -> Result<Topic, AppError> {
        let topic = sqlx::query_as::<_, Topic>(
            "UPDATE topics SET is_pinned = $2, updated_at = NOW() WHERE id = $1 RETURNING *"
        )
        .bind(topic_id)
        .bind(is_pinned)
        .fetch_one(&self.pool)
        .await?;

        Ok(topic)
    }
}
