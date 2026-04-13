use anyhow::Result;
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::faker::name::en::FirstName;
use fake::Fake;
use rand::Rng;
use std::fs;
use std::path::Path;

use jojuhu_api_flow_tests::models::{TestUser, TestUsersData};

const QNT_USERS: usize = 500;

fn generate_unique_username(base: String, index: usize) -> String {
    format!(
        "{}_{}_{:03}",
        base.to_lowercase(),
        generate_random_suffix(),
        index
    )
}

fn generate_random_suffix() -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>()
        .to_lowercase()
}

pub fn generate_test_users(count: usize) -> Vec<TestUser> {
    let mut users = Vec::with_capacity(count);
    let mut used_usernames: Vec<String> = Vec::new();

    for i in 0..count {
        let first_name: String = FirstName().fake();
        let base_username = first_name.to_lowercase().replace(" ", "_");

        // Ensure unique username
        let mut username = generate_unique_username(base_username.clone(), i);
        while used_usernames.contains(&username) {
            username = generate_unique_username(base_username.clone(), i + 100);
        }
        used_usernames.push(username.clone());

        let email: String = FreeEmail().fake();
        let password: String = Password(12..20).fake();
        let display_name = format!("{} TestUser", first_name);

        users.push(TestUser::new(username, email, password, display_name));
    }

    users
}

fn main() -> Result<()> {
    println!("🚀 Jojuhu API Test Users Generator\n");

    let api_base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    let output_dir = Path::new("data");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("📁 Created data directory");
    }

    println!("📝 Generating {} test users...", QNT_USERS);
    let users = generate_test_users(QNT_USERS);

    let users_data = TestUsersData::new(api_base_url.clone(), users);

    let output_path = output_dir.join("test_users.json");
    let json = serde_json::to_string_pretty(&users_data)?;
    fs::write(&output_path, json)?;

    println!("✅ Successfully generated {} test users", QNT_USERS);
    println!("📄 Saved to: {}", output_path.display());
    println!("\n📋 Sample users:");

    // Display first 5 users as sample
    for (i, user) in users_data.users.iter().take(5).enumerate() {
        println!(
            "  {}. {} ({}) - {}",
            i + 1,
            user.username,
            user.email,
            "*".repeat(user.password.len())
        );
    }

    println!("\n💡 Use these users for API testing:");
    println!("   cargo run --bin api-flow-tests");
    println!("\n🔧 Environment:");
    println!("   API_BASE_URL={}", api_base_url);

    Ok(())
}
