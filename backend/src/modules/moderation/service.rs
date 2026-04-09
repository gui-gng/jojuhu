use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::security::sanitize_input;

use super::models::{
    CreateReportRequest, ModerationAction, ModerationActionType, Report, ReportStatus, ReportType,
    SuspendUserRequest, UserSuspension,
};
use super::repository::ModerationRepository;

pub struct ModerationService {
    repository: ModerationRepository,
}

impl ModerationService {
    pub fn new(repository: ModerationRepository) -> Self {
        Self { repository }
    }

    pub async fn create_report(
        &self,
        reporter_id: Uuid,
        mut request: CreateReportRequest,
    ) -> Result<Report, AppError> {
        // Validate and sanitize reason
        if request.reason.trim().is_empty() {
            return Err(AppError::ValidationError("Report reason is required".to_string()));
        }
        if request.reason.len() > 500 {
            return Err(AppError::ValidationError("Report reason too long".to_string()));
        }
        request.reason = sanitize_input(&request.reason);

        // Sanitize description if provided
        if let Some(ref mut desc) = request.description {
            *desc = sanitize_input(desc);
        }

        self.repository
            .create_report(
                reporter_id,
                request.reported_user_id,
                request.report_type,
                request.content_id,
                &request.reason,
                request.description.as_deref(),
            )
            .await
    }

    pub async fn get_report(&self, report_id: Uuid) -> Result<Report, AppError> {
        self.repository
            .get_report_by_id(report_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Report not found".to_string()))
    }

    pub async fn list_reports(
        &self,
        status: Option<ReportStatus>,
        report_type: Option<ReportType>,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Report>, AppError> {
        let offset = (page - 1) * per_page;
        self.repository
            .list_reports(status, report_type, offset, per_page)
            .await
    }

    pub async fn update_report_status(
        &self,
        report_id: Uuid,
        status: ReportStatus,
        assigned_to: Option<Uuid>,
    ) -> Result<Report, AppError> {
        self.repository
            .update_report_status(report_id, status, assigned_to)
            .await
    }

    pub async fn assign_report(
        &self,
        report_id: Uuid,
        moderator_id: Uuid,
    ) -> Result<Report, AppError> {
        self.repository
            .update_report_status(report_id, ReportStatus::Reviewing, Some(moderator_id))
            .await
    }

    pub async fn resolve_report(
        &self,
        report_id: Uuid,
        moderator_id: Uuid,
        action_type: ModerationActionType,
        target_user_id: Option<Uuid>,
        target_content_id: Option<Uuid>,
        reason: Option<String>,
    ) -> Result<(Report, ModerationAction), AppError> {
        // Create moderation action
        let action = self
            .repository
            .create_moderation_action(
                Some(report_id),
                moderator_id,
                action_type,
                target_user_id,
                target_content_id,
                reason.as_deref(),
            )
            .await?;

        // Update report status
        let report = self
            .repository
            .update_report_status(report_id, ReportStatus::Resolved, Some(moderator_id))
            .await?;

        Ok((report, action))
    }

    pub async fn dismiss_report(
        &self,
        report_id: Uuid,
        moderator_id: Uuid,
        reason: Option<String>,
    ) -> Result<(Report, ModerationAction), AppError> {
        // Create moderation action
        let action = self
            .repository
            .create_moderation_action(
                Some(report_id),
                moderator_id,
                ModerationActionType::ReportDismissed,
                None,
                None,
                reason.as_deref(),
            )
            .await?;

        // Update report status
        let report = self
            .repository
            .update_report_status(report_id, ReportStatus::Dismissed, Some(moderator_id))
            .await?;

        Ok((report, action))
    }

    pub async fn suspend_user(
        &self,
        moderator_id: Uuid,
        mut request: SuspendUserRequest,
    ) -> Result<UserSuspension, AppError> {
        // Validate reason
        if request.reason.trim().is_empty() {
            return Err(AppError::ValidationError("Suspension reason is required".to_string()));
        }
        if request.reason.len() > 500 {
            return Err(AppError::ValidationError("Suspension reason too long".to_string()));
        }
        request.reason = sanitize_input(&request.reason);

        // Calculate end time if duration specified
        let ends_at = request.duration_hours.map(|hours| {
            Utc::now() + Duration::hours(hours as i64)
        });

        self.repository
            .create_suspension(request.user_id, moderator_id, &request.reason, ends_at)
            .await
    }

    pub async fn is_user_suspended(&self, user_id: Uuid) -> Result<bool, AppError> {
        self.repository.is_user_suspended(user_id).await
    }

    pub async fn get_active_suspension(&self, user_id: Uuid) -> Result<Option<UserSuspension>, AppError> {
        self.repository.get_active_suspension(user_id).await
    }

    pub async fn list_user_suspensions(
        &self,
        user_id: Uuid,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<UserSuspension>, AppError> {
        let offset = (page - 1) * per_page;
        self.repository.list_user_suspensions(user_id, offset, per_page).await
    }

    pub async fn count_pending_reports(&self) -> Result<i64, AppError> {
        self.repository.count_reports(Some(ReportStatus::Pending)).await
    }

    pub async fn count_reports(&self, status: Option<ReportStatus>) -> Result<i64, AppError> {
        self.repository.count_reports(status).await
    }
}