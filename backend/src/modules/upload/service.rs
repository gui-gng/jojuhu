use s3::creds::Credentials;
use s3::Bucket;
use s3::Region;
use std::time::Duration;
use uuid::Uuid;

use crate::errors::AppError;

use super::models::{PresignedUrlRequest, PresignedUrlResponse, UploadType};

pub struct UploadService {
    bucket: Bucket,
    public_url: String,
}

impl UploadService {
    pub fn new(
        endpoint: String,
        bucket_name: String,
        access_key: String,
        secret_key: String,
        region: Option<String>,
    ) -> Result<Self, AppError> {
        let credentials = Credentials::new(
            Some(&access_key),
            Some(&secret_key),
            None,
            None,
            None,
        )
        .map_err(|e| AppError::InternalError(format!("Failed to create credentials: {}", e)))?;

        let region = match region {
            Some(r) => Region::Custom { region: r, endpoint: endpoint.clone() },
            None => Region::Custom {
                region: "us-east-1".to_string(),
                endpoint: endpoint.clone(),
            },
        };

        let bucket = Bucket::new(&bucket_name, region, credentials)
            .map_err(|e| AppError::InternalError(format!("Failed to create bucket: {}", e)))?
            .with_path_style();

        // Construct public URL (for MinIO, this is usually endpoint/bucket_name)
        let public_url = format!("{}/{}", endpoint, bucket_name);

        Ok(Self { bucket, public_url })
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

        // Generate presigned URL (expires in 5 minutes)
        let presigned_url = self
            .bucket
            .presign_put(
                &key,
                request.upload_type.max_size() as u32,
                Some(Duration::from_secs(300)),
                Some(&[("content-type", &request.content_type)]),
            )
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to generate presigned URL: {}", e)))?;

        // Construct public file URL
        let file_url = format!("{}/{}", self.public_url, key);

        Ok(PresignedUrlResponse {
            upload_url: presigned_url,
            file_url,
            key,
            expires_in: 300,
        })
    }

    /// Delete a file from storage
    pub async fn delete_file(&self, key: &str) -> Result<(), AppError> {
        self.bucket
            .delete_object(key)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to delete file: {}", e)))?;

        Ok(())
    }

    /// Check if a file exists
    pub async fn file_exists(&self, key: &str) -> Result<bool, AppError> {
        let result = self
            .bucket
            .head_object(key)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to check file: {}", e)))?;

        Ok(result.1.is_some())
    }

    /// Get the public URL for a file
    pub fn get_public_url(&self, key: &str) -> String {
        format!("{}/{}", self.public_url, key)
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
