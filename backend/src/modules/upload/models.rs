use serde::{Deserialize, Serialize};

/// Request to generate a presigned upload URL
#[derive(Debug, Deserialize)]
pub struct PresignedUrlRequest {
    pub file_name: String,
    pub content_type: String,
    pub upload_type: UploadType,
}

/// Type of upload
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum UploadType {
    Avatar,
    PostImage,
}

impl UploadType {
    pub fn folder(&self) -> &'static str {
        match self {
            UploadType::Avatar => "avatars",
            UploadType::PostImage => "posts",
        }
    }

    pub fn max_size(&self) -> usize {
        match self {
            UploadType::Avatar => 5 * 1024 * 1024,     // 5MB
            UploadType::PostImage => 10 * 1024 * 1024, // 10MB
        }
    }

    pub fn allowed_content_types(&self) -> Vec<&'static str> {
        vec!["image/jpeg", "image/png", "image/webp", "image/gif"]
    }
}

/// Response with presigned URL
#[derive(Debug, Serialize)]
pub struct PresignedUrlResponse {
    pub upload_url: String,
    pub file_url: String,
    pub key: String,
    pub expires_in: u64,
}

/// Request to confirm upload completion
#[derive(Debug, Deserialize)]
pub struct ConfirmUploadRequest {
    pub key: String,
}

/// Upload validation result
#[derive(Debug)]
pub struct UploadValidation {
    pub is_valid: bool,
    pub error: Option<String>,
}

impl PresignedUrlRequest {
    /// Validate the upload request
    pub fn validate(&self) -> UploadValidation {
        // Check content type
        if !self
            .upload_type
            .allowed_content_types()
            .contains(&self.content_type.as_str())
        {
            return UploadValidation {
                is_valid: false,
                error: Some(format!(
                    "Invalid content type. Allowed: {:?}",
                    self.upload_type.allowed_content_types()
                )),
            };
        }

        // Validate file extension
        let valid_extensions = ["jpg", "jpeg", "png", "webp", "gif"];
        let extension = self
            .file_name
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase();

        if !valid_extensions.contains(&extension.as_str()) {
            return UploadValidation {
                is_valid: false,
                error: Some(format!(
                    "Invalid file extension. Allowed: {:?}",
                    valid_extensions
                )),
            };
        }

        UploadValidation {
            is_valid: true,
            error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_type_folder() {
        assert_eq!(UploadType::Avatar.folder(), "avatars");
        assert_eq!(UploadType::PostImage.folder(), "posts");
    }

    #[test]
    fn test_upload_type_max_size() {
        assert_eq!(UploadType::Avatar.max_size(), 5 * 1024 * 1024);
        assert_eq!(UploadType::PostImage.max_size(), 10 * 1024 * 1024);
    }

    #[test]
    fn test_presigned_url_validation() {
        let request = PresignedUrlRequest {
            file_name: "test.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            upload_type: UploadType::Avatar,
        };
        let validation = request.validate();
        assert!(validation.is_valid);

        let request = PresignedUrlRequest {
            file_name: "test.txt".to_string(),
            content_type: "image/jpeg".to_string(),
            upload_type: UploadType::Avatar,
        };
        let validation = request.validate();
        assert!(!validation.is_valid);

        let request = PresignedUrlRequest {
            file_name: "test.jpg".to_string(),
            content_type: "text/plain".to_string(),
            upload_type: UploadType::Avatar,
        };
        let validation = request.validate();
        assert!(!validation.is_valid);
    }
}
