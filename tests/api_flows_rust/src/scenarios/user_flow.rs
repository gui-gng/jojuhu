use anyhow::Result;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use reqwest::StatusCode;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::{Comment, Forum, Group, Message, Post, TestResults, TestUser, Topic};
use crate::utils::api_client::ApiClient;
use crate::utils::logging::{log_error, log_info, log_success};

/// Actions a user can perform
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserAction {
    CreatePost,
    LikePost,
    CommentOnPost,
    FollowUser,
    UpdateProfile,
    CreateForum,
    JoinForum,
    CreateTopic,
    CreateGroup,
    JoinGroup,
    SendMessage,
    ViewFeed,
    BrowseProfiles,
}

impl UserAction {
    /// Get a random action weighted by likelihood
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let actions = vec![
            (UserAction::CreatePost, 25),
            (UserAction::LikePost, 20),
            (UserAction::CommentOnPost, 15),
            (UserAction::FollowUser, 10),
            (UserAction::ViewFeed, 10),
            (UserAction::BrowseProfiles, 5),
            (UserAction::UpdateProfile, 3),
            (UserAction::CreateForum, 3),
            (UserAction::JoinForum, 3),
            (UserAction::CreateTopic, 2),
            (UserAction::CreateGroup, 2),
            (UserAction::JoinGroup, 1),
            (UserAction::SendMessage, 1),
        ];

        let total_weight: i32 = actions.iter().map(|(_, w)| w).sum();
        let random_val = rng.gen_range(0..total_weight);

        let mut cumulative = 0;
        for (action, weight) in actions {
            cumulative += weight;
            if random_val < cumulative {
                return action;
            }
        }

        UserAction::CreatePost
    }

    /// Get random delay after this action (in milliseconds)
    pub fn delay<R: Rng>(&self, rng: &mut R) -> u64 {
        match self {
            UserAction::CreatePost => rng.gen_range(2000..8000),      // 2-8s
            UserAction::LikePost => rng.gen_range(500..2000),         // 0.5-2s
            UserAction::CommentOnPost => rng.gen_range(3000..10000),  // 3-10s
            UserAction::FollowUser => rng.gen_range(1000..3000),      // 1-3s
            UserAction::UpdateProfile => rng.gen_range(5000..15000),  // 5-15s
            UserAction::CreateForum => rng.gen_range(5000..12000),    // 5-12s
            UserAction::JoinForum => rng.gen_range(1000..4000),       // 1-4s
            UserAction::CreateTopic => rng.gen_range(4000..12000),    // 4-12s
            UserAction::CreateGroup => rng.gen_range(5000..15000),    // 5-15s
            UserAction::JoinGroup => rng.gen_range(1000..4000),       // 1-4s
            UserAction::SendMessage => rng.gen_range(2000..6000),     // 2-6s
            UserAction::ViewFeed => rng.gen_range(3000..8000),        // 3-8s
            UserAction::BrowseProfiles => rng.gen_range(2000..5000),  // 2-5s
        }
    }
}

/// Shared state between all user actors
pub struct SharedState {
    pub posts: Mutex<Vec<Post>>,
    pub forums: Mutex<Vec<Forum>>,
    pub groups: Mutex<Vec<Group>>,
    pub topics: Mutex<Vec<Topic>>,
    pub comments: Mutex<Vec<Comment>>,
    pub messages: Mutex<Vec<Message>>,
    pub all_users: Vec<TestUser>,
    pub base_url: String,
}

impl SharedState {
    pub fn new(users: Vec<TestUser>, base_url: String) -> Self {
        Self {
            posts: Mutex::new(Vec::new()),
            forums: Mutex::new(Vec::new()),
            groups: Mutex::new(Vec::new()),
            topics: Mutex::new(Vec::new()),
            comments: Mutex::new(Vec::new()),
            messages: Mutex::new(Vec::new()),
            all_users: users,
            base_url,
        }
    }
}

/// A user actor that performs actions like a real user
pub struct UserActor {
    user: TestUser,
    client: ApiClient,
    state: Arc<SharedState>,
    rng: rand::rngs::StdRng,
    actions_completed: u32,
    max_actions: u32,
}

impl UserActor {
    pub fn new(user: TestUser, state: Arc<SharedState>, max_actions: u32) -> Self {
        let client = ApiClient::with_token(
            state.base_url.clone(),
            user.token.clone().unwrap_or_default(),
        );

        // Use StdRng which is Send + Sync
        let rng = rand::rngs::StdRng::from_entropy();

        Self {
            user,
            client,
            state,
            rng,
            actions_completed: 0,
            max_actions,
        }
    }

    /// Run the user's activity loop
    pub async fn run(&mut self) -> Result<()> {
        log_info(&format!(
            "👤 User {} started their session ({} actions planned)",
            self.user.username, self.max_actions
        ));

        while self.actions_completed < self.max_actions {
            let action = UserAction::random(&mut self.rng);
            let delay_ms = action.delay(&mut self.rng);

            // Perform the action
            match self.perform_action(action).await {
                Ok(true) => {
                    self.actions_completed += 1;
                }
                Ok(false) => {
                    // Action skipped (e.g., no targets available)
                }
                Err(e) => {
                    log_error(&format!(
                        "❌ User {} failed action {:?}: {}",
                        self.user.username, action, e
                    ));
                }
            }

            // Wait before next action (random delay)
            if self.actions_completed < self.max_actions {
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
        }

        log_success(&format!(
            "✅ User {} completed {} actions",
            self.user.username, self.actions_completed
        ));

        Ok(())
    }

    /// Perform a specific action
    async fn perform_action(&mut self, action: UserAction) -> Result<bool> {
        match action {
            UserAction::CreatePost => self.create_post().await,
            UserAction::LikePost => self.like_random_post().await,
            UserAction::CommentOnPost => self.comment_on_random_post().await,
            UserAction::FollowUser => self.follow_random_user().await,
            UserAction::UpdateProfile => self.update_profile().await,
            UserAction::CreateForum => self.create_forum().await,
            UserAction::JoinForum => self.join_random_forum().await,
            UserAction::CreateTopic => self.create_topic_in_random_forum().await,
            UserAction::CreateGroup => self.create_group().await,
            UserAction::JoinGroup => self.join_random_group().await,
            UserAction::SendMessage => self.send_message_to_random_user().await,
            UserAction::ViewFeed => self.view_feed().await,
            UserAction::BrowseProfiles => self.browse_profiles().await,
        }
    }

    async fn create_post(&mut self) -> Result<bool> {
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

        let content = sample_posts.choose(&mut self.rng).unwrap();
        let is_public = self.rng.gen_bool(0.75);

        let response = self.client.create_post(content, is_public).await?;

        if response.status() == StatusCode::CREATED {
            let body: Value = response.json().await?;
            if let Some(data) = body.get("data") {
                if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                    let post = Post {
                        id: Uuid::parse_str(id)?,
                        content: content.to_string(),
                        author_id: self.user.user_id.unwrap_or_else(Uuid::new_v4),
                        author_username: self.user.username.clone(),
                        is_public,
                        created_at: chrono::Utc::now(),
                        likes_count: 0,
                        comments_count: 0,
                    };

                    self.state.posts.lock().await.push(post);
                    log_success(&format!("📝 {} created a post", self.user.username));
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    async fn like_random_post(&mut self) -> Result<bool> {
        let posts = self.state.posts.lock().await;

        // Find posts not created by this user
        let other_posts: Vec<Post> = posts
            .iter()
            .filter(|p| p.author_username != self.user.username)
            .cloned()
            .collect();

        drop(posts);

        if other_posts.is_empty() {
            return Ok(false);
        }

        if let Some(post) = other_posts.choose(&mut self.rng) {
            let response = self.client.like_post(post.id).await?;

            if response.status() == StatusCode::OK {
                log_success(&format!("❤️ {} liked {}'s post", self.user.username, post.author_username));
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn comment_on_random_post(&mut self) -> Result<bool> {
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

        let posts = self.state.posts.lock().await;

        // Find posts not created by this user
        let other_posts: Vec<Post> = posts
            .iter()
            .filter(|p| p.author_username != self.user.username)
            .cloned()
            .collect();

        drop(posts);

        if other_posts.is_empty() {
            return Ok(false);
        }

        if let Some(post) = other_posts.choose(&mut self.rng) {
            let comment = sample_comments.choose(&mut self.rng).unwrap();
            let response = self.client.add_comment(post.id, comment).await?;

            if response.status() == StatusCode::CREATED {
                let body: Value = response.json().await?;
                if let Some(data) = body.get("data") {
                    if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                        let comment_obj = Comment {
                            id: Uuid::parse_str(id)?,
                            post_id: post.id,
                            content: comment.to_string(),
                            author_username: self.user.username.clone(),
                            created_at: chrono::Utc::now(),
                        };

                        self.state.comments.lock().await.push(comment_obj);
                        log_success(&format!(
                            "💬 {} commented on {}'s post",
                            self.user.username, post.author_username
                        ));
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    async fn follow_random_user(&mut self) -> Result<bool> {
        // Find users not followed yet (simplified - just pick random)
        let other_users: Vec<&TestUser> = self
            .state
            .all_users
            .iter()
            .filter(|u| u.user_id.is_some() && u.username != self.user.username)
            .collect();

        if other_users.is_empty() {
            return Ok(false);
        }

        if let Some(target_user) = other_users.choose(&mut self.rng) {
            let response = self.client.follow_user(target_user.user_id.unwrap()).await?;

            if response.status() == StatusCode::OK {
                log_success(&format!(
                    "👥 {} followed {}",
                    self.user.username, target_user.username
                ));
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn update_profile(&mut self) -> Result<bool> {
        let bios = vec![
            "Hello! I'm new here 👋",
            "Love coding and coffee ☕️",
            "Living life to the fullest! ✨",
            "Tech enthusiast 💻",
            "Coffee addict ☕️🚀",
            "Just exploring the world 🌍",
        ];

        let bio = bios.choose(&mut self.rng).unwrap();
        let response = self.client.update_profile(None, Some(bio)).await?;

        if response.status() == StatusCode::OK {
            log_success(&format!("✏️ {} updated their profile", self.user.username));
            return Ok(true);
        }

        Ok(false)
    }

    async fn create_forum(&mut self) -> Result<bool> {
        let forum_templates = vec![
            ("Technology Enthusiasts", "Discuss the latest in tech, programming, and innovation"),
            ("Photography Lovers", "Share your best shots and get feedback from the community"),
            ("Book Club", "Discover new books and discuss your favorite reads"),
            ("Fitness & Health", "Tips, motivation, and support for a healthy lifestyle"),
            ("Travel Adventures", "Share travel stories, tips, and recommendations"),
            ("Food & Cooking", "Recipes, restaurant reviews, and culinary discussions"),
            ("Music Discovery", "Find new music and discuss your favorite artists"),
            ("Gaming Community", "Discuss games, strategies, and find teammates"),
        ];

        if let Some(template) = forum_templates.choose(&mut self.rng) {
            let name = format!("{} - {}", template.0, Uuid::new_v4().to_string()[..6].to_string());
            let is_public = self.rng.gen_bool(0.75);

            let response = self.client.create_forum(&name, template.1, is_public).await?;

            if response.status() == StatusCode::CREATED {
                let body: Value = response.json().await?;
                if let Some(data) = body.get("data") {
                    if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                        let forum = Forum {
                            id: Uuid::parse_str(id)?,
                            name: name.clone(),
                            description: template.1.to_string(),
                            is_public,
                            creator: self.user.username.clone(),
                            created_at: chrono::Utc::now(),
                        };

                        self.state.forums.lock().await.push(forum);
                        log_success(&format!("🏛️ {} created forum: {}", self.user.username, name));
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    async fn join_random_forum(&mut self) -> Result<bool> {
        let forums = self.state.forums.lock().await;

        // Find forums not created by this user
        let other_forums: Vec<Forum> = forums
            .iter()
            .filter(|f| f.creator != self.user.username)
            .cloned()
            .collect();

        drop(forums);

        if other_forums.is_empty() {
            return Ok(false);
        }

        if let Some(forum) = other_forums.choose(&mut self.rng) {
            let response = self.client.join_forum(forum.id).await?;

            if response.status() == StatusCode::OK {
                log_success(&format!(
                    "📌 {} joined forum: {}",
                    self.user.username, forum.name
                ));
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn create_topic_in_random_forum(&mut self) -> Result<bool> {
        let sample_topics = vec![
            ("Welcome to our community!", "Hello everyone! Feel free to introduce yourself!"),
            ("Best practices for beginners", "What are your best tips for someone just starting out?"),
            ("Weekly challenge", "Let's start our first weekly challenge!"),
            ("Resources and recommendations", "Share your favorite resources with the community."),
            ("Q&A Session", "Ask any questions you have!"),
            ("Success stories", "Share your achievements to inspire others!"),
        ];

        let forums = self.state.forums.lock().await.clone();

        if forums.is_empty() {
            return Ok(false);
        }

        if let Some(forum) = forums.choose(&mut self.rng) {
            if let Some(template) = sample_topics.choose(&mut self.rng) {
                let title = format!("{} - {}", template.0, Uuid::new_v4().to_string()[..6].to_string());

                let response = self.client.create_topic(forum.id, &title, template.1).await?;

                if response.status() == StatusCode::CREATED {
                    let body: Value = response.json().await?;
                    if let Some(data) = body.get("data") {
                        if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                            let topic = Topic {
                                id: Uuid::parse_str(id)?,
                                forum_id: forum.id,
                                title: title.clone(),
                                content: template.1.to_string(),
                                author: self.user.username.clone(),
                                created_at: chrono::Utc::now(),
                            };

                            self.state.topics.lock().await.push(topic);
                            log_success(&format!(
                                "📋 {} created topic in {}",
                                self.user.username, forum.name
                            ));
                            return Ok(true);
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    async fn create_group(&mut self) -> Result<bool> {
        let group_templates = vec![
            ("Rust Developers", "A group for Rust programming enthusiasts"),
            ("Photography Club", "Share and discuss photography"),
            ("Book Worms", "For those who love reading"),
            ("Fitness Buddies", "Support group for fitness goals"),
            ("Travel Enthusiasts", "Share travel experiences and tips"),
        ];

        if let Some(template) = group_templates.choose(&mut self.rng) {
            let name = format!("{} - {}", template.0, Uuid::new_v4().to_string()[..6].to_string());

            let response = self.client.create_group(&name, template.1).await?;

            if response.status() == StatusCode::CREATED {
                let body: Value = response.json().await?;
                if let Some(data) = body.get("data") {
                    if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                        let group = Group {
                            id: Uuid::parse_str(id)?,
                            name: name.clone(),
                            description: template.1.to_string(),
                            creator: self.user.username.clone(),
                            created_at: chrono::Utc::now(),
                        };

                        self.state.groups.lock().await.push(group);
                        log_success(&format!("👥 {} created group: {}", self.user.username, name));
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    async fn join_random_group(&mut self) -> Result<bool> {
        let groups = self.state.groups.lock().await;

        // Find groups not created by this user
        let other_groups: Vec<Group> = groups
            .iter()
            .filter(|g| g.creator != self.user.username)
            .cloned()
            .collect();

        drop(groups);

        if other_groups.is_empty() {
            return Ok(false);
        }

        if let Some(group) = other_groups.choose(&mut self.rng) {
            let response = self.client.join_group(group.id).await?;

            if response.status() == StatusCode::OK {
                log_success(&format!(
                    "🤝 {} joined group: {}",
                    self.user.username, group.name
                ));
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn send_message_to_random_user(&mut self) -> Result<bool> {
        let sample_messages = vec![
            "Hey! How are you doing? 👋",
            "Great to connect with you here! 🎉",
            "Thanks for the follow! 💯",
            "Hi! I saw your profile and thought I'd say hello ☀️",
            "Hello! Love your content. Keep it up! 🌟",
        ];

        // Find users not self
        let other_users: Vec<&TestUser> = self
            .state
            .all_users
            .iter()
            .filter(|u| u.user_id.is_some() && u.username != self.user.username)
            .collect();

        if other_users.is_empty() {
            return Ok(false);
        }

        if let Some(target_user) = other_users.choose(&mut self.rng) {
            let content = sample_messages.choose(&mut self.rng).unwrap();
            let response = self.client.send_message(target_user.user_id.unwrap(), content).await?;

            if response.status() == StatusCode::CREATED {
                let body: Value = response.json().await?;
                if let Some(data) = body.get("data") {
                    if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                        let message = Message {
                            id: Uuid::parse_str(id)?,
                            sender_id: self.user.user_id.unwrap_or_else(Uuid::new_v4),
                            recipient_id: target_user.user_id.unwrap(),
                            sender_username: self.user.username.clone(),
                            recipient_username: target_user.username.clone(),
                            content: content.to_string(),
                            created_at: chrono::Utc::now(),
                            is_read: false,
                        };

                        self.state.messages.lock().await.push(message);
                        log_success(&format!(
                            "💌 {} sent message to {}",
                            self.user.username, target_user.username
                        ));
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    async fn view_feed(&mut self) -> Result<bool> {
        let response = self.client.get_feed(1, 10).await?;

        if response.status() == StatusCode::OK {
            log_info(&format!("📰 {} viewed their feed", self.user.username));
            return Ok(true);
        }

        Ok(false)
    }

    async fn browse_profiles(&mut self) -> Result<bool> {
        // Pick a few random users to view
        let other_users: Vec<&TestUser> = self
            .state
            .all_users
            .iter()
            .filter(|u| u.user_id.is_some() && u.username != self.user.username)
            .take(3)
            .collect();

        if other_users.is_empty() {
            return Ok(false);
        }

        let mut viewed = 0;
        for target_user in other_users {
            let response = self.client.get_user_profile(target_user.user_id.unwrap()).await?;
            if response.status() == StatusCode::OK {
                viewed += 1;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        if viewed > 0 {
            log_info(&format!(
                "👀 {} browsed {} profiles",
                self.user.username, viewed
            ));
            return Ok(true);
        }

        Ok(false)
    }
}

/// Run a realistic user flow simulation
pub async fn run_realistic_user_flow(
    users: Vec<TestUser>,
    base_url: String,
    results: &mut TestResults,
) -> Result<()> {
    log_info(&format!(
        "🎭 Starting realistic user flow simulation with {} users...",
        users.len()
    ));

    // Filter only authenticated users
    let authenticated_users: Vec<TestUser> = users
        .clone()
        .into_iter()
        .filter(|u| u.token.is_some() && u.user_id.is_some())
        .collect();

    if authenticated_users.is_empty() {
        log_error("❌ No authenticated users available for simulation");
        return Ok(());
    }

    log_info(&format!(
        "👥 {} authenticated users will participate",
        authenticated_users.len()
    ));

    // Create shared state
    let state = Arc::new(SharedState::new(authenticated_users.clone(), base_url));

    // Create user actors with varying activity levels
    let mut handles = vec![];

    for (i, user) in authenticated_users.iter().enumerate() {
        // Vary the number of actions per user (power users vs casual users)
        let max_actions = match i % 5 {
            0 => 15, // Power user
            1 | 2 => 10, // Regular user
            _ => 5,  // Casual user
        };

        let state_clone = Arc::clone(&state);
        let user_clone = user.clone();

        let handle = tokio::spawn(async move {
            let mut actor = UserActor::new(user_clone, state_clone, max_actions);
            if let Err(e) = actor.run().await {
                log_error(&format!("User actor error: {}", e));
            }
        });

        handles.push(handle);

        // Stagger user starts to simulate real-world login patterns
        if i % 3 == 0 {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    // Wait for all users to complete
    for handle in handles {
        let _ = handle.await;
    }

    // Collect results from shared state
    let posts = state.posts.lock().await.clone();
    let forums = state.forums.lock().await.clone();
    let groups = state.groups.lock().await.clone();
    let topics = state.topics.lock().await.clone();
    let comments = state.comments.lock().await.clone();
    let messages = state.messages.lock().await.clone();

    results.created_posts = posts;
    results.created_forums = forums;
    results.created_groups = groups;
    results.created_topics = topics;
    results.created_comments = comments;
    results.created_messages = messages;

    log_success(&format!(
        "✅ Simulation complete! Created: {} posts, {} forums, {} groups, {} topics, {} comments, {} messages",
        results.created_posts.len(),
        results.created_forums.len(),
        results.created_groups.len(),
        results.created_topics.len(),
        results.created_comments.len(),
        results.created_messages.len()
    ));

    Ok(())
}
