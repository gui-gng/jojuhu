use anyhow::Result;
use reqwest::{Client, Method, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            client,
            base_url,
            token: None,
        }
    }

    pub fn with_token(base_url: String, token: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            client,
            base_url,
            token: Some(token),
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub async fn request<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<B>,
    ) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut request_builder = self.client.request(method, &url);
        
        if let Some(token) = &self.token {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }
        
        request_builder = request_builder.header("Content-Type", "application/json");
        
        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }
        
        Ok(request_builder.send().await?)
    }

    pub async fn get(&self, path: &str) -> Result<Response> {
        self.request::<serde_json::Value>(Method::GET, path, None).await
    }

    pub async fn post<B: Serialize>(&self, path: &str, body: B) -> Result<Response> {
        self.request(Method::POST, path, Some(body)).await
    }

    pub async fn put<B: Serialize>(&self, path: &str, body: B) -> Result<Response> {
        self.request(Method::PUT, path, Some(body)).await
    }

    pub async fn delete(&self, path: &str) -> Result<Response> {
        self.request::<serde_json::Value>(Method::DELETE, path, None).await
    }

    // Health check
    pub async fn health_check(&self) -> Result<bool> {
        let response = self.get("/health").await?;
        Ok(response.status() == StatusCode::OK)
    }

    // Auth endpoints
    pub async fn register(&self, username: &str, email: &str, password: &str, display_name: &str) -> Result<Response> {
        let body = json!({
            "username": username,
            "email": email,
            "password": password,
            "display_name": display_name,
        });
        self.post("/api/v1/auth/register", body).await
    }

    pub async fn login(&self, username_or_email: &str, password: &str) -> Result<Response> {
        let body = json!({
            "username_or_email": username_or_email,
            "password": password,
        });
        self.post("/api/v1/auth/login", body).await
    }

    // User endpoints
    pub async fn get_me(&self) -> Result<Response> {
        self.get("/api/v1/me").await
    }

    pub async fn get_user_profile(&self, user_id: Uuid) -> Result<Response> {
        self.get(&format!("/api/v1/users/{}", user_id)).await
    }

    pub async fn update_profile(&self, display_name: Option<&str>, bio: Option<&str>) -> Result<Response> {
        let mut body = serde_json::Map::new();
        if let Some(name) = display_name {
            body.insert("display_name".to_string(), json!(name));
        }
        if let Some(bio) = bio {
            body.insert("bio".to_string(), json!(bio));
        }
        self.put("/api/v1/users/me", body).await
    }

    pub async fn follow_user(&self, user_id: Uuid) -> Result<Response> {
        self.post(&format!("/api/v1/users/{}/follow", user_id), json!({})).await
    }

    pub async fn unfollow_user(&self, user_id: Uuid) -> Result<Response> {
        self.delete(&format!("/api/v1/users/{}/follow", user_id)).await
    }

    // Timeline endpoints
    pub async fn create_post(&self, content: &str, is_public: bool) -> Result<Response> {
        let body = json!({
            "content": content,
            "is_public": is_public,
        });
        self.post("/api/v1/timeline/posts", body).await
    }

    pub async fn get_feed(&self, page: u32, per_page: u32) -> Result<Response> {
        self.get(&format!("/api/v1/timeline/feed?page={}&per_page={}", page, per_page)).await
    }

    pub async fn like_post(&self, post_id: Uuid) -> Result<Response> {
        self.post(&format!("/api/v1/timeline/posts/{}/like", post_id), json!({})).await
    }

    pub async fn unlike_post(&self, post_id: Uuid) -> Result<Response> {
        self.delete(&format!("/api/v1/timeline/posts/{}/like", post_id)).await
    }

    pub async fn add_comment(&self, post_id: Uuid, content: &str) -> Result<Response> {
        let body = json!({
            "content": content,
        });
        self.post(&format!("/api/v1/timeline/posts/{}/comments", post_id), body).await
    }

    // Forum endpoints
    pub async fn create_forum(&self, name: &str, description: &str, is_public: bool) -> Result<Response> {
        let body = json!({
            "name": name,
            "description": description,
            "is_public": is_public,
        });
        self.post("/api/v1/forums", body).await
    }

    pub async fn get_forums(&self, page: u32, per_page: u32) -> Result<Response> {
        self.get(&format!("/api/v1/forums?page={}&per_page={}", page, per_page)).await
    }

    pub async fn join_forum(&self, forum_id: Uuid) -> Result<Response> {
        self.post(&format!("/api/v1/forums/{}/join", forum_id), json!({})).await
    }

    pub async fn create_topic(&self, forum_id: Uuid, title: &str, content: &str) -> Result<Response> {
        let body = json!({
            "title": title,
            "content": content,
        });
        self.post(&format!("/api/v1/forums/{}/topics", forum_id), body).await
    }

    // Message endpoints
    pub async fn send_message(&self, recipient_id: Uuid, content: &str) -> Result<Response> {
        let body = json!({
            "recipient_id": recipient_id,
            "content": content,
        });
        self.post("/api/v1/messages", body).await
    }

    pub async fn get_conversations(&self) -> Result<Response> {
        self.get("/api/v1/messages/conversations").await
    }

    pub async fn get_messages(&self, user_id: Uuid, page: u32, per_page: u32) -> Result<Response> {
        self.get(&format!("/api/v1/messages/conversations/{}?page={}&per_page={}", user_id, page, per_page)).await
    }

    // Group endpoints
    pub async fn create_group(&self, name: &str, description: &str) -> Result<Response> {
        let body = json!({
            "name": name,
            "description": description,
        });
        self.post("/api/v1/groups", body).await
    }

    pub async fn get_groups(&self, page: u32, per_page: u32) -> Result<Response> {
        self.get(&format!("/api/v1/groups?page={}&per_page={}", page, per_page)).await
    }

    pub async fn join_group(&self, group_id: Uuid) -> Result<Response> {
        self.post(&format!("/api/v1/groups/{}/join", group_id), json!({})).await
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenData {
    pub token: String,
    pub user_id: Uuid,
    pub token_type: String,
    pub expires_in: i64,
}
