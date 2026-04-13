use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use reqwest::StatusCode;
use serde_json::Value;
use uuid::Uuid;

use crate::models::{Forum, TestResults, TestUser, Topic};
use crate::utils::api_client::ApiClient;
use crate::utils::logging::{log_error, log_info, log_success};

pub struct ForumScenarios {
    base_url: String,
}

impl ForumScenarios {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn run_all(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<(Vec<Forum>, Vec<Topic>)> {
        log_info("Starting Forums Tests...");

        let mut forums = Vec::new();
        let mut topics = Vec::new();

        // Test 1: Create forums
        match self.test_create_forums(users, results).await {
            Ok(created_forums) => {
                forums = created_forums;
                log_success(&format!("Created {} forums", forums.len()));
            }
            Err(e) => log_error(&format!("Create forums test failed: {}", e)),
        }

        // Test 2: Join forums
        if !forums.is_empty() {
            match self.test_join_forums(users, &forums, results).await {
                Ok(_) => log_success("Join forums tests completed"),
                Err(e) => log_error(&format!("Join forums test failed: {}", e)),
            }
        }

        // Test 3: Create topics
        if !forums.is_empty() {
            match self.test_create_topics(users, &forums, results).await {
                Ok(created_topics) => {
                    topics = created_topics;
                    log_success(&format!("Created {} topics", topics.len()));
                }
                Err(e) => log_error(&format!("Create topics test failed: {}", e)),
            }
        }

        // Test 4: List forums
        match self.test_list_forums(users, results).await {
            Ok(_) => log_success("List forums tests completed"),
            Err(e) => log_error(&format!("List forums test failed: {}", e)),
        }

        Ok((forums, topics))
    }

    async fn test_create_forums(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<Vec<Forum>> {
        log_info("Testing: Create Forums");

        let sample_forums = vec![
            ("Technology Enthusiasts", "Discuss the latest in tech, programming, and innovation"),
            ("Photography Lovers", "Share your best shots and get feedback from the community"),
            ("Book Club", "Discover new books and discuss your favorite reads"),
            ("Fitness & Health", "Tips, motivation, and support for a healthy lifestyle"),
            ("Travel Adventures", "Share travel stories, tips, and recommendations"),
            ("Food & Cooking", "Recipes, restaurant reviews, and culinary discussions"),
            ("Music Discovery", "Find new music and discuss your favorite artists"),
            ("Gaming Community", "Discuss games, strategies, and find teammates"),
            ("Art & Design", "Showcase your creative work and get inspired"),
            ("Career Development", "Professional growth, job opportunities, and networking"),
        ];

        let mut created_forums = Vec::new();
        let mut rng = thread_rng();
        let mut forum_templates: Vec<(&&str, &&str)> = sample_forums.iter().map(|(a, b)| (a, b)).collect();
        forum_templates.shuffle(&mut rng);

        for (i, user) in users.iter().filter(|u| u.token.is_some()).enumerate() {
            if i >= 10 || forum_templates.is_empty() {
                break;
            }

            let client = ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let num_forums = rng.gen_range(1..=2).min(forum_templates.len());

            for _ in 0..num_forums {
                if let Some(template) = forum_templates.pop() {
                    let name = format!(
                        "{} - {}",
                        template.0,
                        Uuid::new_v4().to_string()[..8].to_string()
                    );
                    let is_public = rng.gen_bool(0.75);

                    let response = client.create_forum(&name, template.1, is_public).await?;

                    match response.status() {
                        StatusCode::CREATED => {
                            let body: Value = response.json().await?;
                            if let Some(data) = body.get("data") {
                                if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                                    created_forums.push(Forum {
                                        id: Uuid::parse_str(id)?,
                                        name: name.clone(),
                                        description: template.1.to_string(),
                                        is_public,
                                        creator: user.username.clone(),
                                        created_at: chrono::Utc::now(),
                                    });
                                    log_success(&format!(
                                        "{} created forum: {}",
                                        user.username, name
                                    ));
                                }
                            }
                        }
                        status => {
                            log_error(&format!("Failed to create forum: HTTP {}", status));
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        }

        if !created_forums.is_empty() {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(created_forums)
    }

    async fn test_join_forums(
        &self,
        users: &[TestUser],
        forums: &[Forum],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: Join Forums");

        let mut success_count = 0;
        let mut rng = thread_rng();

        for user in users.iter().filter(|u| u.token.is_some()) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let other_forums: Vec<&Forum> = forums
                .iter()
                .filter(|f| f.creator != user.username)
                .collect();

            if other_forums.len() >= 2 {
                let num_to_join = rng.gen_range(2..=4).min(other_forums.len());
                let forums_to_join: Vec<&Forum> = other_forums
                    .choose_multiple(&mut rng, num_to_join)
                    .cloned()
                    .collect();

                for forum in forums_to_join {
                    let response = client.join_forum(forum.id).await?;

                    match response.status() {
                        StatusCode::OK => {
                            log_success(&format!(
                                "{} joined forum: {}",
                                user.username, forum.name
                            ));
                            success_count += 1;
                        }
                        StatusCode::CONFLICT => {
                            log_info(&format!(
                                "{} already member of: {}",
                                user.username, forum.name
                            ));
                        }
                        status => {
                            log_error(&format!("Failed to join forum: HTTP {}", status));
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            }
        }

        log_info(&format!("Created {} forum joins", success_count));

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }

    async fn test_create_topics(
        &self,
        users: &[TestUser],
        forums: &[Forum],
        results: &mut TestResults,
    ) -> Result<Vec<Topic>> {
        log_info("Testing: Create Topics");

        let sample_topics = vec![
            ("Welcome to our community!", "Hello everyone! This is a place to share and learn. Feel free to introduce yourself!"),
            ("Best practices for beginners", "What are your best tips for someone just starting out? Share your wisdom!"),
            ("Weekly challenge", "Let's start our first weekly challenge! Post your progress here."),
            ("Resources and recommendations", "Share your favorite resources, tools, or recommendations with the community."),
            ("Q&A Session", "Ask any questions you have! Our community is here to help."),
            ("Success stories", "Share your achievements and success stories to inspire others!"),
            ("Tips and tricks", "What are some lesser-known tips that have helped you?"),
            ("Introduction thread", "New here? Introduce yourself and tell us a bit about you!"),
        ];

        let mut created_topics = Vec::new();
        let mut rng = thread_rng();

        for user in users.iter().filter(|u| u.token.is_some()) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            // Get forums this user can post in (created or joined)
            let user_forums: Vec<&Forum> = forums
                .iter()
                .filter(|f| f.creator == user.username || rng.gen_bool(0.3))
                .take(3)
                .collect();

            if !user_forums.is_empty() {
                let num_topics = rng.gen_range(1..=2).min(user_forums.len());
                let forums_for_topics: Vec<&Forum> = user_forums
                    .choose_multiple(&mut rng, num_topics)
                    .cloned()
                    .collect();

                for forum in forums_for_topics {
                    if let Some(template) = sample_topics.choose(&mut rng) {
                        let title = format!(
                            "{} - {}",
                            template.0,
                            Uuid::new_v4().to_string()[..6].to_string()
                        );

                        let response = client
                            .create_topic(forum.id, &title, template.1)
                            .await?;

                        match response.status() {
                            StatusCode::CREATED => {
                                let body: Value = response.json().await?;
                                if let Some(data) = body.get("data") {
                                    if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                                        created_topics.push(Topic {
                                            id: Uuid::parse_str(id)?,
                                            forum_id: forum.id,
                                            title: title.clone(),
                                            content: template.1.to_string(),
                                            author: user.username.clone(),
                                            created_at: chrono::Utc::now(),
                                        });
                                        log_success(&format!(
                                            "{} created topic in {}",
                                            user.username, forum.name
                                        ));
                                    }
                                }
                            }
                            status => {
                                log_error(&format!("Failed to create topic: HTTP {}", status));
                            }
                        }

                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    }
                }
            }
        }

        if !created_topics.is_empty() {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(created_topics)
    }

    async fn test_list_forums(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: List Forums");

        let mut success_count = 0;

        for user in users.iter().filter(|u| u.token.is_some()).take(5) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let response = client.get_forums(1, 10).await?;

            match response.status() {
                StatusCode::OK => {
                    success_count += 1;
                }
                status => {
                    log_error(&format!("Failed to list forums: HTTP {}", status));
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
