use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use reqwest::StatusCode;
use serde_json::Value;
use uuid::Uuid;

use crate::models::{Message, TestResults, TestUser};
use crate::utils::api_client::ApiClient;
use crate::utils::logging::{log_error, log_info, log_success};

pub struct MessageScenarios {
    base_url: String,
}

impl MessageScenarios {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn run_all(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<Vec<Message>> {
        log_info("Starting Messages Tests...");

        let mut messages = Vec::new();

        // Test 1: Send messages
        match self.test_send_messages(users, results).await {
            Ok(sent_messages) => {
                messages = sent_messages;
                log_success(&format!("Sent {} messages", messages.len()));
            }
            Err(e) => log_error(&format!("Send messages test failed: {}", e)),
        }

        // Test 2: Get conversations
        if !messages.is_empty() {
            match self.test_get_conversations(users, results).await {
                Ok(_) => log_success("Get conversations tests completed"),
                Err(e) => log_error(&format!("Get conversations test failed: {}", e)),
            }
        }

        // Test 3: Reply to messages
        if !messages.is_empty() {
            match self.test_reply_to_messages(users, &messages, results).await {
                Ok(_) => log_success("Reply to messages tests completed"),
                Err(e) => log_error(&format!("Reply to messages test failed: {}", e)),
            }
        }

        Ok(messages)
    }

    async fn test_send_messages(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<Vec<Message>> {
        log_info("Testing: Send Messages");

        let sample_messages = vec![
            "Hey! How are you doing? 👋",
            "Great to connect with you here! 🎉",
            "Thanks for the follow! Looking forward to your posts 💯",
            "Hi! I saw your profile and thought I'd say hello ☀️",
            "Hello! Love your content. Keep it up! 🌟",
            "Hey there! How's your day going? 😊",
            "Hi! Just wanted to introduce myself. Nice to meet you! 🤝",
            "Hello! Your recent post was really interesting 🧠",
            "Hey! Would love to connect and chat more 💬",
            "Hi there! Welcome to the community 🎈",
            "Hello! Thanks for being part of this platform 🙏",
            "Hey! Looking forward to seeing more from you 👀",
            "Hi! Hope you're having an amazing day! ✨",
            "Hello! Just dropping by to say hi 👋",
            "Hey there! Let's connect and share ideas 💡",
        ];

        let mut sent_messages = Vec::new();
        let mut rng = thread_rng();

        let user_tokens: Vec<(String, String, Uuid)> = users
            .iter()
            .filter(|u| u.token.is_some() && u.user_id.is_some())
            .map(|u| (u.username.clone(), u.token.clone().unwrap(), u.user_id.unwrap()))
            .collect();

        if user_tokens.len() < 2 {
            log_info("Not enough users to test messaging");
            return Ok(sent_messages);
        }

        for (sender_name, sender_token, sender_id) in &user_tokens {
            let client = ApiClient::with_token(self.base_url.clone(), sender_token.clone());

            let other_users: Vec<&(String, String, Uuid)> = user_tokens
                .iter()
                .filter(|(name, _, _)| name != sender_name)
                .collect();

            if other_users.len() >= 2 {
                let num_recipients = rng.gen_range(2..=3).min(other_users.len());
                let recipients: Vec<&(String, String, Uuid)> = other_users
                    .choose_multiple(&mut rng, num_recipients)
                    .cloned()
                    .collect();

                for (recipient_name, _, recipient_id) in recipients {
                    let num_messages = rng.gen_range(2..=4);

                    for i in 0..num_messages {
                        let content = if i == 0 {
                            sample_messages.choose(&mut rng).unwrap().to_string()
                        } else {
                            "Just following up! 👆".to_string()
                        };

                        let response = client.send_message(*recipient_id, &content).await?;

                        match response.status() {
                            StatusCode::CREATED => {
                                let body: Value = response.json().await?;
                                if let Some(data) = body.get("data") {
                                    if let Some(id) = data.get("id").and_then(|id| id.as_str()) {
                                        sent_messages.push(Message {
                                            id: Uuid::parse_str(id)?,
                                            sender_id: *sender_id,
                                            recipient_id: *recipient_id,
                                            sender_username: sender_name.clone(),
                                            recipient_username: recipient_name.clone(),
                                            content: content.clone(),
                                            created_at: chrono::Utc::now(),
                                            is_read: false,
                                        });
                                    }
                                }
                            }
                            status => {
                                log_error(&format!("Failed to send message: HTTP {}", status));
                            }
                        }

                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    }
                }
            }
        }

        log_success(&format!("Sent {} messages total", sent_messages.len()));

        if !sent_messages.is_empty() {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(sent_messages)
    }

    async fn test_get_conversations(
        &self,
        users: &[TestUser],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: Get Conversations");

        let mut success_count = 0;

        for user in users.iter().filter(|u| u.token.is_some()).take(5) {
            let client =
                ApiClient::with_token(self.base_url.clone(), user.token.clone().unwrap());

            let response = client.get_conversations().await?;

            match response.status() {
                StatusCode::OK => {
                    success_count += 1;
                }
                status => {
                    log_error(&format!("Failed to get conversations: HTTP {}", status));
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

    async fn test_reply_to_messages(
        &self,
        users: &[TestUser],
        messages: &[Message],
        results: &mut TestResults,
    ) -> Result<()> {
        log_info("Testing: Reply to Messages");

        let reply_messages = vec![
            "Hey! Thanks for reaching out! I'm doing great 😊",
            "Nice to meet you too! Love the community here 🎉",
            "Thanks! I appreciate the kind words 💯",
            "I'm doing well, thanks for asking! How about you? ☀️",
            "Thanks! I try to post interesting content 🌟",
            "My day is going great! Hope yours is too 🎈",
            "Nice to meet you as well! Welcome aboard 🤝",
            "Thanks! I'm glad you found it interesting 🧠",
            "Absolutely! Would love to chat more 💬",
            "Thanks for the warm welcome! 🙏",
            "Looking forward to your posts as well! 👀",
            "Having an amazing day, thank you! Hope you are too ✨",
            "Hi back! Thanks for saying hello 👋",
            "Thanks! Always happy to share ideas 💡",
            "Thanks so much! This is a great community ❤️",
        ];

        let mut success_count = 0;
        let mut rng = thread_rng();

        // Get unique conversations from messages
        let conversations: Vec<(Uuid, Uuid)> = messages
            .iter()
            .map(|m| {
                if rng.gen_bool(0.5) {
                    (m.recipient_id, m.sender_id)
                } else {
                    (m.sender_id, m.recipient_id)
                }
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .take(10)
            .collect();

        for (sender_id, recipient_id) in conversations {
            if let Some((_sender_name, sender_token)) = users
                .iter()
                .find(|u| u.user_id == Some(sender_id))
                .map(|u| (u.username.clone(), u.token.clone().unwrap()))
            {
                let client = ApiClient::with_token(self.base_url.clone(), sender_token);

                let content = reply_messages.choose(&mut rng).unwrap();

                let response = client.send_message(recipient_id, content).await?;

                if response.status() == StatusCode::CREATED {
                    success_count += 1;
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            }
        }

        log_info(&format!("Sent {} replies", success_count));

        if success_count > 0 {
            results.record_pass();
        } else {
            results.record_fail();
        }

        Ok(())
    }
}
