use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use reqwest::StatusCode;
use serde_json::Value;
use uuid::Uuid;

use crate::models::{Comment, Post, TestResults, TestUser};
use crate::utils::api_client::ApiClient;
use crate::utils::logging::{log_error, log_info, log_success};

pub struct PostScenarios {
    base_url: String,
}

impl PostScenarios {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn run_all(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<(Vec<Post>, Vec<Comment>)> {
        log_info("Starting Timeline/Posts Tests...");

        let mut posts = Vec::new();
        let mut comments = Vec::new();

        // Test 1: Create posts
        match self.test_create_posts(users, results).await {
            Ok(created_posts) => {
                posts = created_posts;
                log_success(&format!("Created {} posts", posts.len()));
            }
            Err(e) => log_error(&format!("Create posts test failed: {}", e)),
        }

        // Test 2: Like posts
        if !posts.is_empty() {
            match self.test_like_posts(users, &posts, results).await {
                Ok(_) => log_success("Like posts tests completed"),
                Err(e) => log_error(&format!("Like posts test failed: {}", e)),
            }
        }

        // Test 3: Add comments
        if !posts.is_empty() {
            match self.test_add_comments(users, &posts, results).await {
                Ok(created_comments) => {
                    comments = created_comments;
                    log_success(&format!("Created {} comments", comments.len()));
                }
                Err(e) => log_error(&format!("Add comments test failed: {}", e)),
            }
        }

        // Test 4: Get feeds
        match self.test_get_feeds(users, results).await {
            Ok(_) => log_success("Get feeds tests completed"),
            Err(e) => log_error(&format!("Get feeds test failed: {}", e)),
        }

        // Test 5: Get post details
        if !posts.is_empty() {
            match self.test_get_post_details(users, &posts, results).await {
                Ok(_) => log_success("Get post details tests completed"),
                Err(e) => log_error(&format!("Get post details test failed: {}", e)),
            }
        }

        Ok((posts, comments))
    }

    async fn test_create_posts(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<Vec<Post>> {
        log_info("Testing: Create Posts");

        let sample_posts = vec![
            "Just joined Jojuhu! Excited to connect with everyone 🎉",
            "Beautiful day today! Hope you're all doing well ☀️",
            "Working on something exciting. Can't wait to share! 💡",
            "What's everyone up to this weekend? 🎈",
            "Just had the best coffee ever! ☕️",
            "Learning new things every day. Growth mindset! 🌱",
            "Anyone else love coding as much as I do? 💻",
            "Nature is amazing. Went for a hike today! 🏔️",
            "Good music + good company = perfect evening 🎵",
            "Remember to take breaks and stay hydrated! 💧",
            "Just finished reading an amazing book 📚",
            "The sunset today was absolutely beautiful 🌅",
            "Trying out new recipes in the kitchen today 👨‍🍳",
            "Technology is incredible when used for good 🚀",
            "Grateful for all the amazing people in my life ❤️",
        ];

        let mut created_posts = Vec::new();
        let mut rng = thread_rng();

        for user in users.iter().filter(|u| u.token.is_some()) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            // Scale posts per user based on total user count
            // 2-5 posts for up to 50 users, scale down for larger user sets
            let user_count = users.len();
            let max_posts = if user_count <= 10 {
                5
            } else if user_count <= 50 {
                4
            } else if user_count <= 100 {
                3
            } else {
                2
            };
            let num_posts = rng.gen_range(2..=max_posts);

            for i in 0..num_posts {
                let content = sample_posts.choose(&mut rng).unwrap();
                let is_public = rng.gen_bool(0.75);

                let response = client
                    .create_post(&format!("{} #{}", content, i + 1), is_public)
                    .await?;

                match response.status() {
                    StatusCode::CREATED => {
                        let body: Value = response.json().await?;
                        if let Some(data) = body.get("data") {
                            if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                                let post = Post {
                                    id: Uuid::parse_str(id)?,
                                    content: data
                                        .get("content")
                                        .and_then(|c| c.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    author_id: user.user_id.unwrap_or_else(Uuid::new_v4),
                                    author_username: user.username.clone(),
                                    is_public,
                                    created_at: chrono::Utc::now(),
                                    likes_count: 0,
                                    comments_count: 0,
                                };
                                created_posts.push(post);
                            }
                        }
                    }
                    status => {
                        log_error(&format!("Failed to create post: HTTP {}", status));
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        log_success(&format!("Created {} posts total", created_posts.len()));

        if !created_posts.is_empty() {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(created_posts)
    }

    async fn test_like_posts(
        &self,
        users: &[TestUser],
        posts: &[Post],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: Like Posts");

        let mut success_count = 0;
        let mut rng = thread_rng();

        for user in users.iter().filter(|u| u.token.is_some()) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let other_posts: Vec<&Post> = posts
                .iter()
                .filter(|p| p.author_username != user.username)
                .collect();

            if other_posts.len() >= 2 {
                // Scale likes based on available posts (10-30% of available)
                let num_to_like = ((other_posts.len() as f32 * 0.2) as usize)
                    .max(2)
                    .min(5)
                    .min(other_posts.len());
                let posts_to_like: Vec<&Post> = other_posts
                    .choose_multiple(&mut rng, num_to_like)
                    .cloned()
                    .collect();

                for post in posts_to_like {
                    let response = client.like_post(post.id).await?;

                    match response.status() {
                        StatusCode::OK => {
                            success_count += 1;
                        }
                        StatusCode::CONFLICT => {
                            // Already liked
                        }
                        status => {
                            log_error(&format!("Failed to like post: HTTP {}", status));
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            }
        }

        log_info(&format!("Created {} likes", success_count));

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }

    async fn test_add_comments(
        &self,
        users: &[TestUser],
        posts: &[Post],
        results: &mut TestResults,
    ) -> Result<Vec<Comment>> {
        log_info("Testing: Add Comments");

        let sample_comments = vec![
            "Great post! Thanks for sharing 👍",
            "I totally agree with this! 💯",
            "This is so true! Thanks for posting 🙏",
            "Love this! Keep them coming ❤️",
            "Interesting perspective! 🤔",
            "Couldn't have said it better myself! 👏",
            "This made my day! ☀️",
            "So relatable! Thanks for sharing 💭",
            "Absolutely love this! 🎉",
            "Great insight! Thanks for posting 🌟",
        ];

        let mut created_comments = Vec::new();
        let mut rng = thread_rng();

        for user in users.iter().filter(|u| u.token.is_some()) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let other_posts: Vec<&Post> = posts
                .iter()
                .filter(|p| p.author_username != user.username)
                .collect();

            if other_posts.len() >= 2 {
                // Scale comments based on available posts (5-20% of available)
                let num_to_comment = ((other_posts.len() as f32 * 0.15) as usize)
                    .max(1)
                    .min(3)
                    .min(other_posts.len());
                let posts_to_comment: Vec<&Post> = other_posts
                    .choose_multiple(&mut rng, num_to_comment)
                    .cloned()
                    .collect();

                for post in posts_to_comment {
                    let comment = sample_comments.choose(&mut rng).unwrap();

                    let response = client.add_comment(post.id, comment).await?;

                    match response.status() {
                        StatusCode::CREATED => {
                            let body: Value = response.json().await?;
                            if let Some(data) = body.get("data") {
                                if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                                    created_comments.push(Comment {
                                        id: Uuid::parse_str(id)?,
                                        post_id: post.id,
                                        content: comment.to_string(),
                                        author_username: user.username.clone(),
                                        created_at: chrono::Utc::now(),
                                    });
                                }
                            }
                        }
                        status => {
                            log_error(&format!("Failed to add comment: HTTP {}", status));
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            }
        }

        log_success(&format!("Created {} comments total", created_comments.len()));

        if !created_comments.is_empty() {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(created_comments)
    }

    async fn test_get_feeds(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: Get Feeds");

        let mut success_count = 0;

        // Scale feed tests based on user count (20% of users, min 2, max 10)
        let test_count = users.iter().filter(|u| u.token.is_some()).count();
        let test_count = ((test_count as f32 * 0.2) as usize).max(2).min(10);
        
        for user in users.iter().filter(|u| u.token.is_some()).take(test_count) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            // Test For You feed
            let response = client.get_feed(1, 10).await?;
            if response.status() == StatusCode::OK {
                success_count += 1;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        log_info(&format!("Successfully retrieved {} feeds", success_count));

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }

    async fn test_get_post_details(
        &self,
        users: &[TestUser],
        posts: &[Post],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: Get Post Details");

        let mut success_count = 0;

        // Scale post detail tests based on post count (10-20% of posts, min 3, max 15)
        let test_count = ((posts.len() as f32 * 0.15) as usize).max(3).min(15);
        
        for post in posts.iter().take(test_count) {
            if let Some(token) = users
                .iter()
                .find(|u| u.username == post.author_username)
                .and_then(|u| u.token.clone())
            {
                let client = ApiClient::with_token(self.base_url.clone(), token);

                let response = client
                    .get(&format!("/api/v1/timeline/posts/{}", post.id))
                    .await?;

                if response.status() == StatusCode::OK {
                    success_count += 1;
                }
            }
        }

        log_info(&format!("Retrieved {} post details", success_count));

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }
}
