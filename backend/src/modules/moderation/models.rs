use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_type", rename_all = "snake_case")]
pub enum ReportType {
    Post,
    Comment,
    ForumTopic,
    ForumReply,
    UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_status", rename_all = "snake_case")]
pub enum ReportStatus {
    Pending,
    Reviewing,
    Resolved,
    Dismissed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "moderation_action_type", rename_all = "snake_case")]
pub enum ModerationActionType {
    ContentRemoved,
    ContentRestored,
    UserWarned,
    UserSuspended,
    UserBanned,
    ContentHidden,
    ReportDismissed,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Report {
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub reported_user_id: Option<Uuid>,
    pub report_type: ReportType,
    pub content_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
    pub status: ReportStatus,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReportRequest {
    pub report_type: ReportType,
    pub content_id: Uuid,
    pub reported_user_id: Option<Uuid>,
    pub reason: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReportRequest {
    pub status: Option<ReportStatus>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct ReportResponse {
    pub id: Uuid,
    pub reporter: ReporterInfo,
    pub reported_user: Option<ReporterInfo>,
    pub report_type: ReportType,
    pub content_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
    pub status: ReportStatus,
    pub assigned_to: Option<ReporterInfo>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ReporterInfo {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReportResponseRow {
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub reporter_username: String,
    pub reporter_display_name: Option<String>,
    pub reported_user_id: Option<Uuid>,
    pub reported_user_username: Option<String>,
    pub reported_user_display_name: Option<String>,
    pub report_type: ReportType,
    pub content_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
    pub status: ReportStatus,
    pub assigned_to: Option<Uuid>,
    pub assigned_to_username: Option<String>,
    pub assigned_to_display_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ModerationAction {
    pub id: Uuid,
    pub report_id: Option<Uuid>,
    pub moderator_id: Uuid,
    pub action_type: ModerationActionType,
    pub target_user_id: Option<Uuid>,
    pub target_content_id: Option<Uuid>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateModerationActionRequest {
    pub report_id: Option<Uuid>,
    pub action_type: ModerationActionType,
    pub target_user_id: Option<Uuid>,
    pub target_content_id: Option<Uuid>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ModerationActionResponse {
    pub id: Uuid,
    pub report_id: Option<Uuid>,
    pub moderator: ReporterInfo,
    pub action_type: ModerationActionType,
    pub target_user: Option<ReporterInfo>,
    pub target_content_id: Option<Uuid>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSuspension {
    pub id: Uuid,
    pub user_id: Uuid,
    pub suspended_by: Uuid,
    pub reason: String,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SuspendUserRequest {
    pub user_id: Uuid,
    pub reason: String,
    pub duration_hours: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UserSuspensionResponse {
    pub id: Uuid,
    pub user: ReporterInfo,
    pub suspended_by: ReporterInfo,
    pub reason: String,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    pub status: Option<ReportStatus>,
    pub report_type: Option<ReportType>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}
