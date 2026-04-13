use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUser {
    pub user_id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub token: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TestUser {
    pub fn new(username: String, email: String, password: String, display_name: String) -> Self {
        Self {
            user_id: None,
            username,
            email,
            password,
            display_name,
            token: None,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUsersData {
    pub created_at: DateTime<Utc>,
    pub api_base_url: String,
    pub user_count: usize,
    pub users: Vec<TestUser>,
}

impl TestUsersData {
    pub fn new(api_base_url: String, users: Vec<TestUser>) -> Self {
        Self {
            created_at: Utc::now(),
            api_base_url,
            user_count: users.len(),
            users,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub content: String,
    pub author_id: Uuid,
    pub author_username: String,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub likes_count: i64,
    pub comments_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub content: String,
    pub author_username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forum {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub creator: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub sender_username: String,
    pub recipient_username: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub test_run_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub created_users: Vec<TestUser>,
    pub created_posts: Vec<Post>,
    pub created_comments: Vec<Comment>,
    pub created_forums: Vec<Forum>,
    pub created_topics: Vec<Topic>,
    pub created_messages: Vec<Message>,
    pub created_groups: Vec<Group>,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            test_run_id: Uuid::new_v4(),
            started_at: Utc::now(),
            completed_at: None,
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            created_users: Vec::new(),
            created_posts: Vec::new(),
            created_comments: Vec::new(),
            created_forums: Vec::new(),
            created_topics: Vec::new(),
            created_messages: Vec::new(),
            created_groups: Vec::new(),
        }
    }

    pub fn record_pass(&mut self) {
        self.total_tests += 1;
        self.passed_tests += 1;
    }

    pub fn record_fail(&mut self) {
        self.total_tests += 1;
        self.failed_tests += 1;
    }

    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
    }
}
