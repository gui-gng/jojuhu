use sqlx::PgPool;

use crate::errors::AppError;

use super::models::{LinkPreviewRequest, LinkPreviewResponse};

pub struct LinkPreviewService {
    #[allow(dead_code)]
    pool: PgPool,
}

impl LinkPreviewService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_preview(&self, request: LinkPreviewRequest) -> Result<LinkPreviewResponse, AppError> {
        let url = request.url.trim().to_string();

        // Validate URL
        if url.is_empty() {
            return Err(AppError::ValidationError("URL cannot be empty".to_string()));
        }

        // Basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::ValidationError("URL must start with http:// or https://".to_string()));
        }

        // For now, return a basic preview - in production, this would fetch the URL
        // and parse OpenGraph meta tags. We'll use a placeholder for demo.
        Ok(LinkPreviewResponse {
            url: url.clone(),
            title: Some(Self::extract_domain(&url)),
            description: None,
            image: None,
            site_name: None,
            success: true,
        })
    }

    fn extract_domain(url: &str) -> String {
        // Extract domain from URL for display
        url.split("://")
            .nth(1)
            .unwrap_or(url)
            .split('/')
            .next()
            .unwrap_or(url)
            .to_string()
    }
}

#[allow(dead_code)]
pub fn extract_urls(text: &str) -> Vec<String> {
    let mut urls = Vec::new();
    
    // Simple URL extraction - find http/https URLs
    let words: Vec<&str> = text.split_whitespace().collect();
    for word in words {
        if word.starts_with("http://") || word.starts_with("https://") {
            // Remove trailing punctuation
            let url = word.trim_end_matches(['.', ',', ';', ')']);
            urls.push(url.to_string());
        }
    }
    
    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_urls() {
        let text = "Check out https://example.com and http://test.org";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(urls.contains(&"https://example.com".to_string()));
    }
}