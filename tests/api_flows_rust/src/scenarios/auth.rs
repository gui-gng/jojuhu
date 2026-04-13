use anyhow::Result;
use reqwest::StatusCode;
use serde_json::Value;
use uuid::Uuid;

use crate::models::{TestUser, TestResults};
use crate::utils::api_client::ApiClient;
use crate::utils::logging::{log_info, log_success, log_error, log_warning};

pub struct AuthScenarios {
    base_url: String,
}

impl AuthScenarios {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn run_all(&self, results: &mut TestResults) -> Result<Vec<TestUser>> {
        log_info("Starting Authentication Tests...");
        
        let mut registered_users = Vec::new();
        
        // Test 1: Health check
        match self.test_health_check().await {
            Ok(true) => {
                log_success("Health check passed");
                results.record_pass();
            }
            Ok(false) => {
                log_error("Health check failed - backend unreachable");
                results.record_fail();
                return Ok(registered_users);
            }
            Err(e) => {
                log_error(&format!("Health check error: {}", e));
                results.record_fail();
                return Ok(registered_users);
            }
        }
        
        // Test 2: User Registration
        match self.test_user_registration(results).await {
            Ok(users) => {
                registered_users = users;
            }
            Err(e) => {
                log_error(&format!("Registration test failed: {}", e));
            }
        }
        
        // Test 3: User Login
        if !registered_users.is_empty() {
            match self.test_user_login(&mut registered_users, results).await {
                Ok(_) => log_success("Login tests completed"),
                Err(e) => log_error(&format!("Login test failed: {}", e)),
            }
        }
        
        // Test 4: Profile retrieval
        if !registered_users.is_empty() {
            match self.test_profile_retrieval(&registered_users, results).await {
                Ok(_) => log_success("Profile retrieval tests completed"),
                Err(e) => log_error(&format!("Profile retrieval failed: {}", e)),
            }
        }
        
        // Test 5: Registration failure cases
        match self.test_registration_failures(results).await {
            Ok(_) => log_success("Registration failure tests completed"),
            Err(e) => log_error(&format!("Registration failure tests failed: {}", e)),
        }
        
        // Test 6: Login failure cases
        if !registered_users.is_empty() {
            match self.test_login_failures(&registered_users, results).await {
                Ok(_) => log_success("Login failure tests completed"),
                Err(e) => log_error(&format!("Login failure tests failed: {}", e)),
            }
        }
        
        log_info(&format!("Authentication tests complete. Registered {} users", registered_users.len()));
        
        Ok(registered_users)
    }

    async fn test_health_check(&self) -> Result<bool> {
        let client = ApiClient::new(self.base_url.clone());
        client.health_check().await
    }

    async fn test_user_registration(&self, results: &mut TestResults) -> Result<Vec<TestUser>> {
        log_info("Testing: User Registration");
        
        let client = ApiClient::new(self.base_url.clone());
        let mut registered_users = Vec::new();
        
        // Load pre-generated users
        let users_data = crate::utils::load_test_users("data/test_users.json")?;
        
        // Calculate max users to register: ~80% of available, max 50, min 5
        let max_users = (users_data.users.len() as f32 * 0.8) as usize;
        let max_users = max_users.max(5).min(50);
        
        for (i, mut user) in users_data.users.into_iter().enumerate() {
            if i >= max_users {
                break;
            }
            
            let response = client.register(
                &user.username,
                &user.email,
                &user.password,
                &user.display_name
            ).await?;
            
            match response.status() {
                StatusCode::CREATED => {
                    let body: Value = response.json().await?;
                    if body.get("success").and_then(|s| s.as_bool()).unwrap_or(false) {
                        if let Some(data) = body.get("data") {
                            if let Some(user_id) = data.get("user_id").and_then(|id| id.as_str()) {
                                user.user_id = Some(Uuid::parse_str(user_id)?);
                            }
                        }
                        log_success(&format!("Registered: {}", user.username));
                        registered_users.push(user);
                        results.record_pass();
                    } else {
                        log_warning(&format!("Registration returned success=false for {}", user.username));
                        results.record_fail();
                    }
                }
                StatusCode::CONFLICT => {
                    log_warning(&format!("User {} already exists (conflict)", user.username));
                    results.record_fail();
                }
                status => {
                    let text = response.text().await?;
                    log_error(&format!("Registration failed for {}: HTTP {} - {}", 
                        user.username, status, text));
                    results.record_fail();
                }
            }
            
            // Small delay to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        Ok(registered_users)
    }

    async fn test_user_login(&self, users: &mut [TestUser], results: &mut TestResults) -> Result<()> {
        log_info("Testing: User Login");
        
        for user in users.iter_mut() {
            let client = ApiClient::new(self.base_url.clone());
            
            let response = client.login(&user.username, &user.password).await?;
            
            match response.status() {
                StatusCode::OK => {
                    let body: Value = response.json().await?;
                    if let Some(data) = body.get("data") {
                        if let Some(token) = data.get("token").and_then(|t| t.as_str()) {
                            user.token = Some(token.to_string());
                            if let Some(user_id) = data.get("user_id").and_then(|id| id.as_str()) {
                                user.user_id = Some(Uuid::parse_str(user_id)?);
                            }
                            log_success(&format!("Login successful: {}", user.username));
                            results.record_pass();
                        } else {
                            log_error(&format!("No token in login response for {}", user.username));
                            results.record_fail();
                        }
                    } else {
                        log_error(&format!("No data in login response for {}", user.username));
                        results.record_fail();
                    }
                }
                status => {
                    let text = response.text().await?;
                    log_error(&format!("Login failed for {}: HTTP {} - {}", 
                        user.username, status, text));
                    results.record_fail();
                }
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        Ok(())
    }

    async fn test_profile_retrieval(&self, users: &[TestUser], results: &mut TestResults) -> Result<()> {
        log_info("Testing: Profile Retrieval");
        
        let mut success_count = 0;
        // Scale test count based on available users (max 30% or 10, min 2)
        let test_count = users.iter().filter(|u| u.token.is_some()).count();
        let test_count = ((test_count as f32 * 0.3) as usize).max(2).min(10);
        
        for user in users.iter().filter(|u| u.token.is_some()).take(test_count) {
            let client = ApiClient::with_token(
                self.base_url.clone(),
                user.token.clone().unwrap()
            );
            
            let response = client.get_me().await?;
            
            match response.status() {
                StatusCode::OK => {
                    log_success(&format!("Retrieved profile for {}", user.username));
                    success_count += 1;
                }
                status => {
                    log_error(&format!("Failed to get profile for {}: HTTP {}", 
                        user.username, status));
                }
            }
        }
        
        if success_count == test_count {
            results.record_pass();
        } else {
            results.record_fail();
        }
        
        Ok(())
    }

    async fn test_registration_failures(&self, results: &mut TestResults) -> Result<()> {
        log_info("Testing: Registration Failure Cases");
        
        let client = ApiClient::new(self.base_url.clone());
        let mut passed = 0;
        let mut total = 0;
        
        // Test 1: Duplicate username
        total += 1;
        let response = client.register(
            "testuser123",
            "unique1@test.com",
            "Password123!",
            "Test User"
        ).await?;
        
        if response.status() == StatusCode::CONFLICT || response.status() == StatusCode::BAD_REQUEST {
            log_success("Duplicate username correctly rejected");
            passed += 1;
        } else {
            log_warning(&format!("Expected 409/400 for duplicate username, got {}", response.status()));
        }
        
        // Test 2: Invalid email format
        total += 1;
        let response = client.register(
            "newuser_test_123",
            "not-an-email",
            "Password123!",
            "Test User"
        ).await?;
        
        if response.status() == StatusCode::BAD_REQUEST {
            log_success("Invalid email format correctly rejected");
            passed += 1;
        } else {
            log_warning(&format!("Expected 400 for invalid email, got {}", response.status()));
        }
        
        // Test 3: Short password
        total += 1;
        let response = client.register(
            "newuser_test_456",
            "test@test.com",
            "123",
            "Test User"
        ).await?;
        
        if response.status() == StatusCode::BAD_REQUEST {
            log_success("Short password correctly rejected");
            passed += 1;
        } else {
            log_warning(&format!("Expected 400 for short password, got {}", response.status()));
        }
        
        // Test 4: Missing fields
        total += 1;
        let response = client.post("/api/v1/auth/register", serde_json::json!({})).await?;
        
        if response.status() == StatusCode::BAD_REQUEST {
            log_success("Missing fields correctly rejected");
            passed += 1;
        } else {
            log_warning(&format!("Expected 400 for missing fields, got {}", response.status()));
        }
        
        log_info(&format!("Failure case tests: {}/{}", passed, total));
        
        if passed == total {
            results.record_pass();
        } else {
            results.record_fail();
        }
        
        Ok(())
    }

    async fn test_login_failures(&self, users: &[TestUser], results: &mut TestResults) -> Result<()> {
        log_info("Testing: Login Failure Cases");
        
        let client = ApiClient::new(self.base_url.clone());
        let mut passed = 0;
        let mut total = 0;
        
        if let Some(user) = users.first() {
            // Test 1: Wrong password
            total += 1;
            let response = client.login(&user.username, "WrongPassword123!").await?;
            if response.status() == StatusCode::UNAUTHORIZED {
                log_success("Wrong password correctly rejected");
                passed += 1;
            } else {
                log_warning(&format!("Expected 401 for wrong password, got {}", response.status()));
            }
            
            // Test 2: Non-existent user
            total += 1;
            let response = client.login("nonexistentuser123456789", "Password123!").await?;
            if response.status() == StatusCode::UNAUTHORIZED {
                log_success("Non-existent user correctly rejected");
                passed += 1;
            } else {
                log_warning(&format!("Expected 401 for non-existent user, got {}", response.status()));
            }
            
            // Test 3: Empty credentials
            total += 1;
            let response = client.post("/api/v1/auth/login", serde_json::json!({
                "username_or_email": "",
                "password": ""
            })).await?;
            if response.status() == StatusCode::BAD_REQUEST {
                log_success("Empty credentials correctly rejected");
                passed += 1;
            } else {
                log_warning(&format!("Expected 400 for empty credentials, got {}", response.status()));
            }
        }
        
        // Test 4: Protected endpoint without token
        total += 1;
        let response = client.get("/api/v1/me").await?;
        if response.status() == StatusCode::UNAUTHORIZED {
            log_success("Protected endpoint correctly rejects unauthenticated requests");
            passed += 1;
        } else {
            log_warning(&format!("Expected 401 for unauthenticated request, got {}", response.status()));
        }
        
        log_info(&format!("Login failure tests: {}/{}", passed, total));
        
        if passed == total {
            results.record_pass();
        } else {
            results.record_fail();
        }
        
        Ok(())
    }
}
