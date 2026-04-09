use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ProfileVisibility {
    Public,
    FollowersOnly,
    Private,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PrivacySettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub profile_visibility: String,
    pub show_email: bool,
    pub show_phone: bool,
    pub allow_mentions: bool,
    pub allow_tags: bool,
    pub show_online_status: bool,
    pub show_activity: bool,
    pub allow_search_engines: bool,
    pub data_processing_consent: bool,
    pub marketing_emails_consent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePrivacySettingsRequest {
    pub profile_visibility: Option<String>,
    pub show_email: Option<bool>,
    pub show_phone: Option<bool>,
    pub allow_mentions: Option<bool>,
    pub allow_tags: Option<bool>,
    pub show_online_status: Option<bool>,
    pub show_activity: Option<bool>,
    pub allow_search_engines: Option<bool>,
    pub data_processing_consent: Option<bool>,
    pub marketing_emails_consent: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum DataExportStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DataExportRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: DataExportStatus,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct DataExportResponse {
    pub id: Uuid,
    pub status: DataExportStatus,
    pub download_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum AccountDeletionStatus {
    Pending,
    Processing,
    Completed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountDeletionRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reason: Option<String>,
    pub status: AccountDeletionStatus,
    pub requested_at: DateTime<Utc>,
    pub scheduled_deletion_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct RequestAccountDeletionRequest {
    pub reason: Option<String>,
}
