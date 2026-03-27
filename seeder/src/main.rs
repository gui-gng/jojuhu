use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

const API_BASE: &str = "http://localhost:8080/api/v1";

#[derive(Debug, Deserialize)]
struct UserSeed {
    username: String,
    email: String,
    password: String,
    display_name: String,
    bio: String,
}

#[derive(Debug, Deserialize)]
struct PostSeed {
    content: String,
    is_public: bool,
}

#[derive(Debug, Deserialize)]
struct ForumSeed {
    name: String,
    description: String,
    is_public: bool,
}

#[derive(Debug, Deserialize)]
struct MessageSeed {
    content: String,
}

#[derive(Debug, Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    display_name: Option<String>,
}

#[derive(Debug, Serialize)]
struct LoginRequest {
    username_or_email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct CreatePostRequest {
    content: String,
    is_public: Option<bool>,
}

#[derive(Debug, Serialize)]
struct CreateForumRequest {
    name: String,
    description: Option<String>,
    is_public: Option<bool>,
}

#[derive(Debug, Serialize)]
struct SendMessageRequest {
    recipient_id: Uuid,
    content: String,
}

#[derive(Debug, Serialize)]
struct UpdateProfileRequest {
    display_name: Option<String>,
    bio: Option<String>,
}

#[derive(Clone)]
struct User {
    id: Uuid,
    username: String,
    token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌱 Jojuhu Database Seeder\n");

    let client = Client::new();

    // Load seed data
    let users: Vec<UserSeed> = load_json("data/users.json")?;
    let posts: Vec<PostSeed> = load_json("data/posts.json")?;
    let forums: Vec<ForumSeed> = load_json("data/forums.json")?;
    let messages: Vec<MessageSeed> = load_json("data/messages.json")?;

    println!("📊 Seed data loaded:");
    println!("   - {} users", users.len());
    println!("   - {} posts", posts.len());
    println!("   - {} forums", forums.len());
    println!("   - {} messages\n", messages.len());

    // Step 1: Register users
    println!("👤 Step 1: Registering users...");
    let mut registered_users: Vec<User> = Vec::new();

    for user_seed in &users {
        match register_user(&client, user_seed).await {
            Ok((user_id, token)) => {
                println!("   ✅ Registered: {}", user_seed.username);
                registered_users.push(User {
                    id: user_id,
                    username: user_seed.username.clone(),
                    token,
                });
            }
            Err(_) => {
                // Try to login if registration fails (user might exist)
                match login_user(&client, &user_seed.username, &user_seed.password).await {
                    Ok((user_id, token)) => {
                        println!("   ⚠️  User exists, logged in: {}", user_seed.username);
                        registered_users.push(User {
                            id: user_id,
                            username: user_seed.username.clone(),
                            token,
                        });
                    }
                    Err(_) => {
                        println!("   ❌ Failed: {}", user_seed.username);
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("   → {} users ready\n", registered_users.len());

    // Step 2: Update profiles
    println!("✏️  Step 2: Updating user profiles...");
    for (i, user) in registered_users.iter().enumerate() {
        if let Some(user_seed) = users.get(i) {
            let update_req = UpdateProfileRequest {
                display_name: Some(user_seed.display_name.clone()),
                bio: Some(user_seed.bio.clone()),
            };

            match update_profile(&client, &user.token, &update_req).await {
                Ok(_) => println!("   ✅ Updated: {}", user.username),
                Err(_) => println!("   ❌ Failed: {}", user.username),
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    println!();

    // Step 3: Create follow relationships
    println!("👥 Step 3: Creating follow relationships...");
    let mut follow_count = 0;
    for user in &registered_users {
        let num_to_follow = rand::random::<usize>() % 4 + 2;
        let mut targets: Vec<&User> = registered_users
            .iter()
            .filter(|u| u.id != user.id)
            .collect();
        targets.shuffle(&mut rand::thread_rng());

        for target in targets.iter().take(num_to_follow) {
            if follow_user(&client, &user.token, target.id).await.is_ok() {
                follow_count += 1;
                println!("   ✅ {} -> {}", user.username, target.username);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }
    println!("   → {} follow relationships\n", follow_count);

    // Step 4: Create posts
    println!("📝 Step 4: Creating posts...");
    let mut post_count = 0;
    for post_seed in &posts {
        if let Some(user) = registered_users.choose(&mut rand::thread_rng()) {
            let post_req = CreatePostRequest {
                content: post_seed.content.clone(),
                is_public: Some(post_seed.is_public),
            };

            if create_post(&client, &user.token, &post_req).await.is_ok() {
                post_count += 1;
                println!("   ✅ Post by {}", user.username);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    println!("   → {} posts created\n", post_count);

    // Step 5: Create forums
    println!("🏛️  Step 5: Creating forums...");
    let mut forum_ids: Vec<Uuid> = Vec::new();
    for (i, forum_seed) in forums.iter().enumerate() {
        if let Some(user) = registered_users.get(i % registered_users.len()) {
            let forum_req = CreateForumRequest {
                name: forum_seed.name.clone(),
                description: Some(forum_seed.description.clone()),
                is_public: Some(forum_seed.is_public),
            };

            match create_forum(&client, &user.token, &forum_req).await {
                Ok(forum_id) => {
                    forum_ids.push(forum_id);
                    println!("   ✅ Forum: {}", forum_seed.name);
                }
                Err(_) => println!("   ⚠️  Forum exists: {}", forum_seed.name),
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    println!("   → {} forums\n", forum_ids.len());

    // Step 6: Have users join forums
    println!("🤝 Step 6: Users joining forums...");
    let mut join_count = 0;
    for forum_id in &forum_ids {
        let num_joiners = rand::random::<usize>() % 3 + 3;
        let mut joiners = registered_users.clone();
        joiners.shuffle(&mut rand::thread_rng());

        for user in joiners.iter().take(num_joiners) {
            if join_forum(&client, &user.token, *forum_id).await.is_ok() {
                join_count += 1;
                println!("   ✅ {} joined", user.username);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }
    println!("   → {} memberships\n", join_count);

    // Step 7: Send messages
    println!("💬 Step 7: Sending messages...");
    let mut message_count = 0;
    for msg_seed in &messages {
        if registered_users.len() >= 2 {
            let mut users_iter = registered_users.choose_multiple(
                &mut rand::thread_rng(), 2);
            if let (Some(sender), Some(recipient)) = (users_iter.next(), users_iter.next()) {
                let msg_req = SendMessageRequest {
                    recipient_id: recipient.id,
                    content: msg_seed.content.clone(),
                };

                if send_message(&client, &sender.token, &msg_req).await.is_ok() {
                    message_count += 1;
                    println!("   ✅ {} -> {}", sender.username, recipient.username);
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    println!("   → {} messages sent\n", message_count);

    // Summary
    println!("🎉 Seeding complete!");
    println!("   • Users: {}", registered_users.len());
    println!("   • Follows: {}", follow_count);
    println!("   • Posts: {}", post_count);
    println!("   • Forums: {}", forum_ids.len());
    println!("   • Memberships: {}", join_count);
    println!("   • Messages: {}", message_count);
    println!("\n✨ Database populated with test data!");

    Ok(())
}

fn load_json<T: serde::de::DeserializeOwned>(path: &str) -> Result<T> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;
    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path))
}

async fn register_user(client: &Client, user: &UserSeed) -> Result<(Uuid, String)> {
    let req = RegisterRequest {
        username: user.username.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
        display_name: Some(user.display_name.clone()),
    };

    let resp = client
        .post(format!("{}/auth/register", API_BASE))
        .json(&req)
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Registration failed");
    }

    let json: Value = resp.json().await?;
    let user_id = json
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .context("Failed to extract user ID")?;

    let (_, token) = login_user(client, &user.username, &user.password).await?;
    Ok((user_id, token))
}

async fn login_user(client: &Client, username: &str, password: &str) -> Result<(Uuid, String)> {
    let req = LoginRequest {
        username_or_email: username.to_string(),
        password: password.to_string(),
    };

    let resp = client
        .post(format!("{}/auth/login", API_BASE))
        .json(&req)
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Login failed");
    }

    let json: Value = resp.json().await?;
    
    let token = json
        .get("data")
        .and_then(|d| d.get("token"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .context("Failed to extract token")?;

    let user_resp = client
        .get(format!("{}/users/me", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    let user_json: Value = user_resp.json().await?;
    let user_id = user_json
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .context("Failed to extract user ID")?;

    Ok((user_id, token))
}

async fn update_profile(
    client: &Client,
    token: &str,
    req: &UpdateProfileRequest,
) -> Result<()> {
    let resp = client
        .put(format!("{}/users/me", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .json(req)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        anyhow::bail!("Update failed")
    }
}

async fn follow_user(client: &Client, token: &str, user_id: Uuid) -> Result<()> {
    let resp = client
        .post(format!("{}/users/{}/follow", API_BASE, user_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        anyhow::bail!("Follow failed")
    }
}

async fn create_post(client: &Client, token: &str, req: &CreatePostRequest) -> Result<Uuid> {
    let resp = client
        .post(format!("{}/timeline/posts", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .json(req)
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Create post failed");
    }

    let json: Value = resp.json().await?;
    let post_id = json
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .context("Failed to extract post ID")?;

    Ok(post_id)
}

async fn create_forum(client: &Client, token: &str, req: &CreateForumRequest) -> Result<Uuid> {
    let resp = client
        .post(format!("{}/forums", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .json(req)
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Create forum failed");
    }

    let json: Value = resp.json().await?;
    let forum_id = json
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .context("Failed to extract forum ID")?;

    Ok(forum_id)
}

async fn join_forum(client: &Client, token: &str, forum_id: Uuid) -> Result<()> {
    let resp = client
        .post(format!("{}/forums/{}/join", API_BASE, forum_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        anyhow::bail!("Join forum failed")
    }
}

async fn send_message(client: &Client, token: &str, req: &SendMessageRequest) -> Result<()> {
    let resp = client
        .post(format!("{}/messages", API_BASE))
        .header("Authorization", format!("Bearer {}", token))
        .json(req)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        anyhow::bail!("Send message failed")
    }
}