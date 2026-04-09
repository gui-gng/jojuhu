use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{
    AccountDeletionRequest, DataExportRequest, DataExportStatus,
    PrivacySettings, UpdatePrivacySettingsRequest,
};

pub struct PrivacyRepository {
    pool: PgPool,
}

impl PrivacyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Privacy Settings
    pub async fn get_privacy_settings(&self, user_id: Uuid) -> Result<Option<PrivacySettings>, AppError> {
        let settings = sqlx::query_as::<_, PrivacySettings>(
            "SELECT * FROM privacy_settings WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(settings)
    }

    pub async fn create_default_privacy_settings(&self, user_id: Uuid) -> Result<PrivacySettings, AppError> {
        let settings = sqlx::query_as::<_, PrivacySettings>(
            r#"
            INSERT INTO privacy_settings (user_id)
            VALUES ($1)
            ON CONFLICT (user_id) DO UPDATE SET updated_at = NOW()
            RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(settings)
    }

    pub async fn update_privacy_settings(
        &self,
        user_id: Uuid,
        request: UpdatePrivacySettingsRequest,
    ) -> Result<PrivacySettings, AppError> {
        let settings = sqlx::query_as::<_, PrivacySettings>(
            r#"
            UPDATE privacy_settings
            SET
                profile_visibility = COALESCE($2, profile_visibility),
                show_email = COALESCE($3, show_email),
                show_phone = COALESCE($4, show_phone),
                allow_mentions = COALESCE($5, allow_mentions),
                allow_tags = COALESCE($6, allow_tags),
                show_online_status = COALESCE($7, show_online_status),
                show_activity = COALESCE($8, show_activity),
                allow_search_engines = COALESCE($9, allow_search_engines),
                data_processing_consent = COALESCE($10, data_processing_consent),
                marketing_emails_consent = COALESCE($11, marketing_emails_consent),
                updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(request.profile_visibility)
        .bind(request.show_email)
        .bind(request.show_phone)
        .bind(request.allow_mentions)
        .bind(request.allow_tags)
        .bind(request.show_online_status)
        .bind(request.show_activity)
        .bind(request.allow_search_engines)
        .bind(request.data_processing_consent)
        .bind(request.marketing_emails_consent)
        .fetch_one(&self.pool)
        .await?;

        Ok(settings)
    }

    // Data Export
    pub async fn create_data_export_request(&self, user_id: Uuid) -> Result<DataExportRequest, AppError> {
        let request = sqlx::query_as::<_, DataExportRequest>(
            r#"
            INSERT INTO data_export_requests (user_id)
            VALUES ($1)
            RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(request)
    }

    pub async fn get_data_export_request(&self, request_id: Uuid) -> Result<Option<DataExportRequest>, AppError> {
        let request = sqlx::query_as::<_, DataExportRequest>(
            "SELECT * FROM data_export_requests WHERE id = $1"
        )
        .bind(request_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(request)
    }

    pub async fn get_user_data_export_requests(
        &self,
        user_id: Uuid,
        limit: i32,
    ) -> Result<Vec<DataExportRequest>, AppError> {
        let requests = sqlx::query_as::<_, DataExportRequest>(
            "SELECT * FROM data_export_requests WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2"
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(requests)
    }

    pub async fn update_data_export_status(
        &self,
        request_id: Uuid,
        status: DataExportStatus,
        file_path: Option<&str>,
        file_size: Option<i64>,
    ) -> Result<DataExportRequest, AppError> {
        let request = sqlx::query_as::<_, DataExportRequest>(
            r#"
            UPDATE data_export_requests
            SET status = $2,
                file_path = COALESCE($3, file_path),
                file_size = COALESCE($4, file_size),
                completed_at = CASE WHEN $2 = 'completed' THEN NOW() ELSE completed_at END
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(request_id)
        .bind(status)
        .bind(file_path)
        .bind(file_size)
        .fetch_one(&self.pool)
        .await?;

        Ok(request)
    }

    // Account Deletion
    pub async fn create_deletion_request(
        &self,
        user_id: Uuid,
        reason: Option<&str>,
    ) -> Result<AccountDeletionRequest, AppError> {
        let scheduled_deletion = Utc::now() + chrono::Duration::days(30);

        let request = sqlx::query_as::<_, AccountDeletionRequest>(
            r#"
            INSERT INTO account_deletion_requests (user_id, reason, scheduled_deletion_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(reason)
        .bind(scheduled_deletion)
        .fetch_one(&self.pool)
        .await?;

        Ok(request)
    }

    pub async fn get_deletion_request(&self, user_id: Uuid) -> Result<Option<AccountDeletionRequest>, AppError> {
        let request = sqlx::query_as::<_, AccountDeletionRequest>(
            "SELECT * FROM account_deletion_requests WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(request)
    }

    pub async fn cancel_deletion_request(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE account_deletion_requests SET status = 'cancelled' WHERE user_id = $1 AND status = 'pending'"
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_pending_deletions(&self) -> Result<Vec<AccountDeletionRequest>, AppError> {
        let requests = sqlx::query_as::<_, AccountDeletionRequest>(
            "SELECT * FROM account_deletion_requests WHERE status = 'pending' AND scheduled_deletion_at <= NOW()"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(requests)
    }

    pub async fn complete_deletion(&self, request_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE account_deletion_requests SET status = 'completed', completed_at = NOW() WHERE id = $1"
        )
        .bind(request_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}