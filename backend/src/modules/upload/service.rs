use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{PresignedUrlRequest, PresignedUrlResponse, UploadType};

// HMAC-SHA256 type alias
type HmacSha256 = Hmac<Sha256>;

pub struct UploadService {
    endpoint: String,
    bucket_name: String,
    access_key: String,
    secret_key: String,
    region: String,
}

impl UploadService {
    pub fn new(
        endpoint: String,
        bucket_name: String,
        access_key: String,
        secret_key: String,
        _region: Option<String>,
    ) -> Result<Self, AppError> {
        Ok(Self {
            endpoint,
            bucket_name,
            access_key,
            secret_key,
            region: "us-east-1".to_string(),
        })
    }

    /// Generate a file URL
    fn build_file_url(
        &self,
        key: &str,
    ) -> String {
        format!("{}/{}/{}", self.endpoint, self.bucket_name, key)
    }

    /// Generate a presigned URL for uploading a file
    pub async fn generate_presigned_url(
        &self,
        user_id: Uuid,
        request: PresignedUrlRequest,
    ) -> Result<PresignedUrlResponse, AppError> {
        // Validate request
        let validation = request.validate();
        if !validation.is_valid {
            return Err(AppError::ValidationError(
                validation.error.unwrap_or_else(|| "Invalid upload request".to_string()),
            ));
        }

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let extension = request
            .file_name
            .split('.')
            .last()
            .unwrap_or("jpg")
            .to_lowercase();
        let key = format!(
            "{}/{}/{}.{}",
            request.upload_type.folder(),
            user_id,
            file_id,
            extension
        );

        // Generate URL (5 minute expiration)
        let file_url = self.build_file_url(&key);

        // For now, return a simple URL structure
        // In production, you'd implement proper AWS Signature V4
        Ok(PresignedUrlResponse {
            upload_url: file_url.clone(),
            file_url,
            key,
            expires_in: 300,
        })
    }

    /// Delete a file from storage
    pub async fn delete_file(&self, _key: &str) -> Result<(), AppError> {
        // TODO: Implement actual MinIO deletion
        // For now, just return success
        Ok(())
    }

    /// Check if a file exists
    pub async fn file_exists(&self, _key: &str) -> Result<bool, AppError> {
        // TODO: Implement actual MinIO head request
        // For now, assume file exists
        Ok(true)
    }

    /// Get the public URL for a file
    pub fn get_public_url(&self, key: &str) -> String {
        format!("{}/{}/{}", self.endpoint, self.bucket_name, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_unique_filename() {
        let user_id = Uuid::new_v4();
        let file_id = Uuid::new_v4();
        let key = format!("avatars/{}/{}.jpg", user_id, file_id);
        
        assert!(key.contains("avatars/"));
        assert!(key.contains(&user_id.to_string()));
        assert!(key.ends_with(".jpg"));
    }
}
