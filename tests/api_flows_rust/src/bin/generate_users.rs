use anyhow::Result;
use clap::Parser;
use fake::faker::internet::en::{FreeEmail, Password};
use fake::faker::name::en::FirstName;
use fake::Fake;
use rand::Rng;
use std::fs;
use std::path::Path;

use jojuhu_api_flow_tests::models::{TestUser, TestUsersData};

#[derive(Parser, Debug)]
#[command(name = "generate-users")]
#[command(about = "Generate test users for Jojuhu API testing")]
struct Args {
    /// Number of users to generate
    #[arg(short, long, default_value_t = 50)]
    count: usize,

    /// API base URL
    #[arg(short = 'a', long, default_value = "http://localhost:8080")]
    url: String,

    /// Output file path
    #[arg(short, long, default_value = "data/test_users.json")]
    output: String,
}

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
    let args = Args::parse();

    println!("🚀 Jojuhu API Test Users Generator\n");

    let output_dir = Path::new(&args.output).parent().unwrap_or(Path::new("."));
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("📁 Created output directory: {}", output_dir.display());
    }

    println!("📝 Generating {} test users...", args.count);
    let users = generate_test_users(args.count);

    let users_data = TestUsersData::new(args.url.clone(), users);

    let output_path = Path::new(&args.output);
    let json = serde_json::to_string_pretty(&users_data)?;
    fs::write(&output_path, json)?;

    println!(
        "✅ Successfully generated {} test users",
        users_data.user_count
    );
    println!("📄 Saved to: {}", output_path.display());
    println!("\n📋 Sample users:");

    // Display first 5 users as sample (or fewer if less than 5)
    let sample_count = std::cmp::min(5, users_data.users.len());
    for (i, user) in users_data.users.iter().take(sample_count).enumerate() {
        println!(
            "  {}. {} ({}) - {}",
            i + 1,
            user.username,
            user.email,
            "*".repeat(user.password.len().min(20))
        );
    }

    if users_data.users.len() > sample_count {
        println!("  ... and {} more", users_data.users.len() - sample_count);
    }

    println!("\n💡 Use these users for API testing:");
    println!("   cargo run --bin api-flow-tests");
    println!("\n🔧 Environment:");
    println!("   API_BASE_URL={}", args.url);

    Ok(())
}
