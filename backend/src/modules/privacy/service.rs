use uuid::Uuid;

use crate::errors::AppError;

use super::models::{
    AccountDeletionRequest, DataExportRequest, PrivacySettings, UpdatePrivacySettingsRequest,
};
use super::repository::PrivacyRepository;

pub struct PrivacyService {
    repository: PrivacyRepository,
}

impl PrivacyService {
    pub fn new(repository: PrivacyRepository) -> Self {
        Self { repository }
    }

    pub async fn get_privacy_settings(&self, user_id: Uuid) -> Result<PrivacySettings, AppError> {
        match self.repository.get_privacy_settings(user_id).await? {
            Some(settings) => Ok(settings),
            None => self.repository.create_default_privacy_settings(user_id).await,
        }
    }

    pub async fn update_privacy_settings(
        &self,
        user_id: Uuid,
        request: UpdatePrivacySettingsRequest,
    ) -> Result<PrivacySettings, AppError> {
        // Ensure privacy settings exist
        if self.repository.get_privacy_settings(user_id).await?.is_none() {
            self.repository.create_default_privacy_settings(user_id).await?;
        }

        self.repository.update_privacy_settings(user_id, request).await
    }

    pub async fn request_data_export(&self, user_id: Uuid) -> Result<DataExportRequest, AppError> {
        // Check for pending request
        if let Some(existing) = self.repository.get_user_data_export_requests(user_id, 1).await?.first() {
            if matches!(existing.status, super::models::DataExportStatus::Pending | super::models::DataExportStatus::Processing) {
                return Err(AppError::ValidationError(
                    "A data export request is already in progress".to_string()
                ));
            }
        }

        self.repository.create_data_export_request(user_id).await
    }

    pub async fn get_data_export_status(&self, user_id: Uuid) -> Result<Option<DataExportRequest>, AppError> {
        self.repository.get_user_data_export_requests(user_id, 1).await.map(|mut v| v.pop())
    }

    pub async fn request_account_deletion(
        &self,
        user_id: Uuid,
        reason: Option<String>,
    ) -> Result<AccountDeletionRequest, AppError> {
        // Check for existing deletion request
        if let Some(existing) = self.repository.get_deletion_request(user_id).await? {
            if matches!(existing.status, super::models::AccountDeletionStatus::Pending) {
                return Err(AppError::ValidationError(
                    "An account deletion request is already pending".to_string()
                ));
            }
        }

        self.repository.create_deletion_request(user_id, reason.as_deref()).await
    }

    pub async fn cancel_account_deletion(&self, user_id: Uuid) -> Result<(), AppError> {
        self.repository.cancel_deletion_request(user_id).await
    }

    pub async fn get_deletion_request(&self, user_id: Uuid) -> Result<Option<AccountDeletionRequest>, AppError> {
        self.repository.get_deletion_request(user_id).await
    }
}