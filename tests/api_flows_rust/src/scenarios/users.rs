use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::StatusCode;

use crate::models::{TestUser, TestResults};
use crate::utils::api_client::ApiClient;
use crate::utils::logging::{log_info, log_success, log_error};

pub struct UserScenarios {
    base_url: String,
}

impl UserScenarios {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn run_all(&self, users: &mut [TestUser], results: &mut TestResults) -> Result<()> {
        log_info("Starting User Management Tests...");

        // Test 1: Update profiles
        match self.test_update_profiles(users, results).await {
            Ok(_) => log_success("Profile update tests completed"),
            Err(e) => log_error(&format!("Profile update tests failed: {}", e)),
        }

        // Test 2: Follow/unfollow users
        match self.test_follow_unfollow(users, results).await {
            Ok(_) => log_success("Follow/unfollow tests completed"),
            Err(e) => log_error(&format!("Follow/unfollow tests failed: {}", e)),
        }

        // Test 3: Get user profiles
        match self.test_get_user_profiles(users, results).await {
            Ok(_) => log_success("Get user profile tests completed"),
            Err(e) => log_error(&format!("Get user profile tests failed: {}", e)),
        }

        Ok(())
    }

    async fn test_update_profiles(&self, users: &mut [TestUser], results: &mut TestResults) -> Result<()> {
        log_info("Testing: Update User Profiles");

        let mut success_count = 0;
        // Scale profile updates based on user count (20% of users, min 2, max 10)
        let test_count = users.iter().filter(|u| u.token.is_some()).count();
        let test_count = ((test_count as f32 * 0.2) as usize).max(2).min(10);
        let test_users: Vec<&mut TestUser> = users.iter_mut()
            .filter(|u| u.token.is_some())
            .take(test_count)
            .collect();

        for user in test_users {
            let client = ApiClient::with_token(
                self.base_url.clone(),
                user.token.clone().unwrap()
            );

            let new_bio = format!("Hello! I'm {} and I love testing APIs!", user.display_name);

            let response = client.update_profile(None, Some(&new_bio)).await?;

            match response.status() {
                StatusCode::OK => {
                    log_success(&format!("Updated profile for {}", user.username));
                    success_count += 1;
                }
                status => {
                    log_error(&format!("Failed to update profile for {}: HTTP {}",
                        user.username, status));
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }

    async fn test_follow_unfollow(&self, users: &mut [TestUser], results: &mut TestResults) -> Result<()> {
        log_info("Testing: Follow/Unfollow Users");

        let mut success_count = 0;
        let user_tokens: Vec<(String, String, uuid::Uuid)> = users.iter()
            .filter(|u| u.token.is_some() && u.user_id.is_some())
            .map(|u| (u.username.clone(), u.token.clone().unwrap(), u.user_id.unwrap()))
            .collect();

        if user_tokens.len() < 2 {
            log_info("Not enough users to test follow/unfollow");
            return Ok(());
        }

        // Scale follows based on user count
        // Each user follows 10-30% of other users, min 2, max 5
        let total_users = user_tokens.len();
        let max_follows = if total_users <= 10 {
            3
        } else if total_users <= 30 {
            4
        } else {
            5
        };

        for (username, token, _) in &user_tokens {
            let client = ApiClient::with_token(self.base_url.clone(), token.clone());

            let mut other_users: Vec<&(String, String, uuid::Uuid)> = user_tokens.iter()
                .filter(|(u, _, _)| u != username)
                .collect();

            other_users.shuffle(&mut thread_rng());

            let num_to_follow = ((other_users.len() as f32 * 0.2) as usize)
                .max(2)
                .min(max_follows)
                .min(other_users.len());

            for target in other_users.into_iter().take(num_to_follow) {
                let response = client.follow_user(target.2).await?;

                match response.status() {
                    StatusCode::OK => {
                        log_success(&format!("{} followed {}", username, target.0));
                        success_count += 1;
                    }
                    StatusCode::CONFLICT => {
                        log_info(&format!("{} already follows {}", username, target.0));
                    }
                    status => {
                        log_error(&format!("Failed to follow: HTTP {}", status));
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        log_info(&format!("Created {} follow relationships", success_count));

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }

    async fn test_get_user_profiles(&self, users: &[TestUser], results: &mut TestResults) -> Result<()> {
        log_info("Testing: Get User Profiles");

        let mut success_count = 0;

        // Use first user to get profiles of others
        if let Some(first_user) = users.iter().find(|u| u.token.is_some()) {
            let client = ApiClient::with_token(
                self.base_url.clone(),
                first_user.token.clone().unwrap()
            );

            // Scale profile retrieval tests (15% of users, min 3, max 12)
            let test_count = users.iter().filter(|u| u.user_id.is_some()).count();
            let test_count = ((test_count as f32 * 0.15) as usize).max(3).min(12);
            
            for target in users.iter().filter(|u| u.user_id.is_some()).take(test_count) {
                let response = client.get_user_profile(target.user_id.unwrap()).await?;

                match response.status() {
                    StatusCode::OK => {
                        log_success(&format!("Retrieved profile for {}", target.username));
                        success_count += 1;
                    }
                    status => {
                        log_error(&format!("Failed to get profile for {}: HTTP {}",
                            target.username, status));
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }
        }

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }
}
