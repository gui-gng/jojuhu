use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::StatusCode;
use serde_json::Value;
use uuid::Uuid;

use crate::models::{Group, TestResults, TestUser};
use crate::utils::api_client::ApiClient;
use crate::utils::logging::{log_error, log_info, log_success};

pub struct GroupScenarios {
    base_url: String,
}

impl GroupScenarios {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn run_all(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<Vec<Group>> {
        log_info("Starting Groups Tests...");

        let mut groups = Vec::new();

        // Test 1: Create groups
        match self.test_create_groups(users, results).await {
            Ok(created_groups) => {
                groups = created_groups;
                log_success(&format!("Created {} groups", groups.len()));
            }
            Err(e) => log_error(&format!("Create groups test failed: {}", e)),
        }

        // Test 2: Join groups
        if !groups.is_empty() {
            match self.test_join_groups(users, &groups, results).await {
                Ok(_) => log_success("Join groups tests completed"),
                Err(e) => log_error(&format!("Join groups test failed: {}", e)),
            }
        }

        // Test 3: List groups
        match self.test_list_groups(users, results).await {
            Ok(_) => log_success("List groups tests completed"),
            Err(e) => log_error(&format!("List groups test failed: {}", e)),
        }

        Ok(groups)
    }

    async fn test_create_groups(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<Vec<Group>> {
        log_info("Testing: Create Groups");

        let sample_groups = vec![
            ("Rust Developers", "A group for Rust programming enthusiasts"),
            ("Photography Club", "Share and discuss photography"),
            ("Book Worms", "For those who love reading"),
            ("Fitness Buddies", "Support group for fitness goals"),
            ("Travel Enthusiasts", "Share travel experiences and tips"),
            ("Foodies United", "Discuss food, recipes, and restaurants"),
            ("Music Lovers", "Discover and discuss music"),
            ("Gaming Squad", "Find gaming buddies and discuss games"),
        ];

        let mut created_groups = Vec::new();

        // Scale groups based on user count: ~20-30% of users can create groups
        let user_count = users.iter().filter(|u| u.token.is_some()).count();
        let max_creators = ((user_count as f32 * 0.25) as usize).max(2).min(15); // Min 2, max 15 creators

        for (i, user) in users.iter().filter(|u| u.token.is_some()).enumerate() {
            if i >= max_creators {
                break;
            }

            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            if let Some(template) = sample_groups.get(i) {
                let name = format!(
                    "{} - {}",
                    template.0,
                    Uuid::new_v4().to_string()[..6].to_string()
                );

                let response = client.create_group(&name, template.1).await?;

                match response.status() {
                    StatusCode::CREATED => {
                        let body: Value = response.json().await?;
                        if let Some(data) = body.get("data") {
                            if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                                created_groups.push(Group {
                                    id: Uuid::parse_str(id)?,
                                    name: name.clone(),
                                    description: template.1.to_string(),
                                    creator: user.username.clone(),
                                    created_at: chrono::Utc::now(),
                                });
                                log_success(&format!(
                                    "{} created group: {}",
                                    user.username, name
                                ));
                            }
                        }
                    }
                    status => {
                        log_error(&format!("Failed to create group: HTTP {}", status));
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        if !created_groups.is_empty() {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(created_groups)
    }

    async fn test_join_groups(
        &self,
        users: &[TestUser],
        groups: &[Group],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: Join Groups");

        let mut success_count = 0;
        let mut rng = thread_rng();

        for user in users.iter().filter(|u| u.token.is_some()) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let other_groups: Vec<&Group> = groups
                .iter()
                .filter(|g| g.creator != user.username)
                .collect();

            if other_groups.len() >= 2 {
                // Scale joins based on available groups (25-50% of available, min 2, max 4)
                let num_to_join = ((other_groups.len() as f32 * 0.35) as usize)
                    .max(2)
                    .min(4)
                    .min(other_groups.len());
                let groups_to_join: Vec<&Group> = other_groups
                    .choose_multiple(&mut rng, num_to_join)
                    .cloned()
                    .collect();

                for group in groups_to_join {
                    let response = client.join_group(group.id).await?;

                    match response.status() {
                        StatusCode::OK => {
                            log_success(&format!(
                                "{} joined group: {}",
                                user.username, group.name
                            ));
                            success_count += 1;
                        }
                        StatusCode::CONFLICT => {
                            log_info(&format!(
                                "{} already member of: {}",
                                user.username, group.name
                            ));
                        }
                        status => {
                            log_error(&format!("Failed to join group: HTTP {}", status));
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            }
        }

        log_info(&format!("Created {} group joins", success_count));

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }

    async fn test_list_groups(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: List Groups");

        let mut success_count = 0;

        // Scale list tests based on user count (15% of users, min 2, max 8)
        let test_count = users.iter().filter(|u| u.token.is_some()).count();
        let test_count = ((test_count as f32 * 0.15) as usize).max(2).min(8);
        
        for user in users.iter().filter(|u| u.token.is_some()).take(test_count) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let response = client.get_groups(1, 10).await?;

            match response.status() {
                StatusCode::OK => {
                    success_count += 1;
                }
                status => {
                    log_error(&format!("Failed to list groups: HTTP {}", status));
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }
}
