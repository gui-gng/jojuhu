use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{
    ModerationAction, ModerationActionType, Report, ReportStatus, ReportType, UserSuspension,
};

pub struct ModerationRepository {
    pool: PgPool,
}

impl ModerationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_report(
        &self,
        reporter_id: Uuid,
        reported_user_id: Option<Uuid>,
        report_type: ReportType,
        content_id: Uuid,
        reason: &str,
        description: Option<&str>,
    ) -> Result<Report, AppError> {
        let report = sqlx::query_as::<_, Report>(
            r#"
            INSERT INTO reports (reporter_id, reported_user_id, report_type, content_id, reason, description)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(reporter_id)
        .bind(reported_user_id)
        .bind(report_type)
        .bind(content_id)
        .bind(reason)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(report)
    }

    pub async fn get_report_by_id(&self, report_id: Uuid) -> Result<Option<Report>, AppError> {
        let report = sqlx::query_as::<_, Report>(
            "SELECT * FROM reports WHERE id = $1"
        )
        .bind(report_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(report)
    }

    pub async fn list_reports(
        &self,
        status: Option<ReportStatus>,
        report_type: Option<ReportType>,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<Report>, AppError> {
        let reports = if let (Some(s), Some(t)) = (&status, &report_type) {
            sqlx::query_as::<_, Report>(
                r#"
                SELECT * FROM reports
                WHERE status = $1 AND report_type = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#
            )
            .bind(s)
            .bind(t)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else if let Some(s) = &status {
            sqlx::query_as::<_, Report>(
                r#"
                SELECT * FROM reports
                WHERE status = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(s)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else if let Some(t) = &report_type {
            sqlx::query_as::<_, Report>(
                r#"
                SELECT * FROM reports
                WHERE report_type = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(t)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Report>(
                r#"
                SELECT * FROM reports
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(reports)
    }

    pub async fn update_report_status(
        &self,
        report_id: Uuid,
        status: ReportStatus,
        assigned_to: Option<Uuid>,
    ) -> Result<Report, AppError> {
        let report = sqlx::query_as::<_, Report>(
            r#"
            UPDATE reports
            SET status = $1, assigned_to = COALESCE($2, assigned_to), updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#
        )
        .bind(status)
        .bind(assigned_to)
        .bind(report_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(report)
    }

    pub async fn create_moderation_action(
        &self,
        report_id: Option<Uuid>,
        moderator_id: Uuid,
        action_type: ModerationActionType,
        target_user_id: Option<Uuid>,
        target_content_id: Option<Uuid>,
        reason: Option<&str>,
    ) -> Result<ModerationAction, AppError> {
        let action = sqlx::query_as::<_, ModerationAction>(
            r#"
            INSERT INTO moderation_actions (report_id, moderator_id, action_type, target_user_id, target_content_id, reason)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(report_id)
        .bind(moderator_id)
        .bind(action_type)
        .bind(target_user_id)
        .bind(target_content_id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await?;

        Ok(action)
    }

    pub async fn list_moderation_actions(
        &self,
        moderator_id: Option<Uuid>,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<ModerationAction>, AppError> {
        let actions = if let Some(mid) = moderator_id {
            sqlx::query_as::<_, ModerationAction>(
                r#"
                SELECT * FROM moderation_actions
                WHERE moderator_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(mid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, ModerationAction>(
                r#"
                SELECT * FROM moderation_actions
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(actions)
    }

    pub async fn create_suspension(
        &self,
        user_id: Uuid,
        suspended_by: Uuid,
        reason: &str,
        ends_at: Option<DateTime<Utc>>,
    ) -> Result<UserSuspension, AppError> {
        let suspension = sqlx::query_as::<_, UserSuspension>(
            r#"
            INSERT INTO user_suspensions (user_id, suspended_by, reason, ends_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(suspended_by)
        .bind(reason)
        .bind(ends_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(suspension)
    }

    pub async fn get_active_suspension(&self, user_id: Uuid) -> Result<Option<UserSuspension>, AppError> {
        let suspension = sqlx::query_as::<_, UserSuspension>(
            r#"
            SELECT * FROM user_suspensions
            WHERE user_id = $1 AND (ends_at IS NULL OR ends_at > NOW())
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(suspension)
    }

    pub async fn list_user_suspensions(
        &self,
        user_id: Uuid,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<UserSuspension>, AppError> {
        let suspensions = sqlx::query_as::<_, UserSuspension>(
            r#"
            SELECT * FROM user_suspensions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(suspensions)
    }

    pub async fn is_user_suspended(&self, user_id: Uuid) -> Result<bool, AppError> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM user_suspensions
            WHERE user_id = $1 AND (ends_at IS NULL OR ends_at > NOW())
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    pub async fn count_reports(&self, status: Option<ReportStatus>) -> Result<i64, AppError> {
        let count = if let Some(s) = status {
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM reports WHERE status = $1"
            )
            .bind(s)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM reports")
                .fetch_one(&self.pool)
                .await?
        };

        Ok(count)
    }
}