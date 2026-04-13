use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::Path;
use std::time::Instant;

mod models;
mod scenarios;
mod utils;

use models::TestResults;
use scenarios::auth::AuthScenarios;
use scenarios::forums::ForumScenarios;
use scenarios::groups::GroupScenarios;
use scenarios::messages::MessageScenarios;
use scenarios::posts::PostScenarios;
use scenarios::users::UserScenarios;
use utils::logging::{log_section, log_test_result};
use utils::{ensure_data_dir, load_test_users, save_test_results};

#[derive(Parser, Debug)]
#[command(name = "Jojuhu API Flow Tests")]
#[command(about = "Comprehensive API testing for Jojuhu backend")]
struct Args {
    /// API base URL
    #[arg(short, long, default_value = "http://localhost:8080")]
    url: String,

    /// Skip user generation (assume test_users.json exists)
    #[arg(short, long)]
    skip_generate: bool,

    /// Run specific scenario (auth, users, posts, forums, messages, groups, all)
    #[arg(short, long, default_value = "all")]
    scenario: String,

    /// Number of users to register
    #[arg(short, long, default_value_t = 20)]
    users: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    print_banner();

    ensure_data_dir()?;

    // Generate test users if needed
    if !args.skip_generate && !Path::new("data/test_users.json").exists() {
        log_section("GENERATING TEST USERS");
        generate_test_users(&args.url).await?;
    }

    // Check if test users exist
    if !Path::new("data/test_users.json").exists() {
        eprintln!("{}", "❌ No test users found. Run with --skip-generate=false or generate users first.".red());
        std::process::exit(1);
    }

    // Run tests based on scenario
    let mut results = TestResults::new();

    match args.scenario.as_str() {
        "auth" => {
            let users = run_auth_scenario(&args.url, &mut results).await?;
            results.created_users = users;
        }
        "users" => run_users_scenario(&args.url, &mut results).await?,
        "posts" => run_posts_scenario(&args.url, &mut results).await?,
        "forums" => run_forums_scenario(&args.url, &mut results).await?,
        "messages" => run_messages_scenario(&args.url, &mut results).await?,
        "groups" => run_groups_scenario(&args.url, &mut results).await?,
        "all" => run_all_scenarios(&args.url, &mut results).await?,
        _ => {
            eprintln!("{}", format!("❌ Unknown scenario: {}", args.scenario).red());
            std::process::exit(1);
        }
    }

    results.complete();

    // Save results
    save_test_results("data/test_results.json", &results)?;

    // Print summary
    print_summary(&results);

    if results.failed_tests > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn print_banner() {
    println!("\n{}", "═".repeat(70).cyan());
    println!("{}", "  JOJUHU API FLOW TESTS".cyan().bold());
    println!("{}", "  Comprehensive API Testing Suite".cyan());
    println!("{}", "═".repeat(70).cyan());
    println!();
}

async fn generate_test_users(api_url: &str) -> Result<()> {
    use fake::faker::internet::en::{FreeEmail, Password, Username};
    use fake::faker::name::en::FirstName;
    use fake::Fake;
    use rand::Rng;

    println!("{}", "📝 Generating 50 test users...".yellow());

    let mut users = Vec::new();
    let mut used_usernames: Vec<String> = Vec::new();

    for i in 0..50 {
        let first_name: String = FirstName().fake();
        let base_username = first_name.to_lowercase().replace(" ", "_");

        let rng = rand::thread_rng();
        let suffix: String = rng
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(6)
            .map(char::from)
            .collect::<String>()
            .to_lowercase();

        let username = format!("{}_{}_{:03}", base_username, suffix, i);

        if used_usernames.contains(&username) {
            continue;
        }
        used_usernames.push(username.clone());

        let email: String = FreeEmail().fake();
        let password: String = Password(12..20).fake();
        let display_name = format!("{} TestUser", first_name);

        users.push(models::TestUser::new(
            username,
            email,
            password,
            display_name,
        ));
    }

    let users_data = models::TestUsersData::new(api_url.to_string(), users);
    let json = serde_json::to_string_pretty(&users_data)?;
    std::fs::write("data/test_users.json", json)?;

    println!("{}", format!("✅ Generated {} test users", users_data.users.len()).green());
    println!("{}", "📄 Saved to: data/test_users.json".blue());

    Ok(())
}

async fn run_auth_scenario(base_url: &str, results: &mut TestResults) -> Result<Vec<models::TestUser>> {
    log_section("AUTHENTICATION SCENARIOS");
    let auth = AuthScenarios::new(base_url.to_string());
    auth.run_all(results).await
}

async fn run_users_scenario(base_url: &str, results: &mut TestResults) -> Result<()> {
    log_section("USER MANAGEMENT SCENARIOS");

    let mut users_data = load_test_users("data/test_users.json")?;

    // Filter only users with tokens (authenticated)
    let authenticated_users: Vec<models::TestUser> = users_data
        .users
        .into_iter()
        .filter(|u| u.token.is_some())
        .collect();

    if authenticated_users.is_empty() {
        println!("{}", "⚠ No authenticated users found. Run auth scenario first.".yellow());
        return Ok(());
    }

    let mut users = authenticated_users;
    let user_scenarios = UserScenarios::new(base_url.to_string());
    user_scenarios.run_all(&mut users, results).await
}

async fn run_posts_scenario(base_url: &str, results: &mut TestResults) -> Result<()> {
    log_section("POSTS/TIMELINE SCENARIOS");

    let users_data = load_test_users("data/test_users.json")?;
    let users: Vec<models::TestUser> = users_data
        .users
        .into_iter()
        .filter(|u| u.token.is_some())
        .collect();

    if users.is_empty() {
        println!("{}", "⚠ No authenticated users found. Run auth scenario first.".yellow());
        return Ok(());
    }

    let post_scenarios = PostScenarios::new(base_url.to_string());
    let (posts, comments) = post_scenarios.run_all(&users, results).await?;

    results.created_posts = posts;
    results.created_comments = comments;

    Ok(())
}

async fn run_forums_scenario(base_url: &str, results: &mut TestResults) -> Result<()> {
    log_section("FORUMS SCENARIOS");

    let users_data = load_test_users("data/test_users.json")?;
    let users: Vec<models::TestUser> = users_data
        .users
        .into_iter()
        .filter(|u| u.token.is_some())
        .collect();

    if users.is_empty() {
        println!("{}", "⚠ No authenticated users found. Run auth scenario first.".yellow());
        return Ok(());
    }

    let forum_scenarios = ForumScenarios::new(base_url.to_string());
    let (forums, topics) = forum_scenarios.run_all(&users, results).await?;

    results.created_forums = forums;
    results.created_topics = topics;

    Ok(())
}

async fn run_messages_scenario(base_url: &str, results: &mut TestResults) -> Result<()> {
    log_section("MESSAGES SCENARIOS");

    let users_data = load_test_users("data/test_users.json")?;
    let users: Vec<models::TestUser> = users_data
        .users
        .into_iter()
        .filter(|u| u.token.is_some())
        .collect();

    if users.is_empty() {
        println!("{}", "⚠ No authenticated users found. Run auth scenario first.".yellow());
        return Ok(());
    }

    let message_scenarios = MessageScenarios::new(base_url.to_string());
    let messages = message_scenarios.run_all(&users, results).await?;

    results.created_messages = messages;

    Ok(())
}

async fn run_groups_scenario(base_url: &str, results: &mut TestResults) -> Result<()> {
    log_section("GROUPS SCENARIOS");

    let users_data = load_test_users("data/test_users.json")?;
    let users: Vec<models::TestUser> = users_data
        .users
        .into_iter()
        .filter(|u| u.token.is_some())
        .collect();

    if users.is_empty() {
        println!("{}", "⚠ No authenticated users found. Run auth scenario first.".yellow());
        return Ok(());
    }

    let group_scenarios = GroupScenarios::new(base_url.to_string());
    let groups = group_scenarios.run_all(&users, results).await?;

    results.created_groups = groups;

    Ok(())
}

async fn run_all_scenarios(base_url: &str, results: &mut TestResults) -> Result<()> {
    let start = Instant::now();

    // 1. Authentication (creates users)
    let mut users = run_auth_scenario(base_url, results).await?;

    // Save updated users with tokens
    let users_data = models::TestUsersData::new(base_url.to_string(), users.clone());
    let json = serde_json::to_string_pretty(&users_data)?;
    std::fs::write("data/test_users.json", json)?;

    results.created_users = users.clone();

    // 2. User Management
    let user_scenarios = UserScenarios::new(base_url.to_string());
    user_scenarios.run_all(&mut users, results).await?;

    // 3. Posts/Timeline
    let post_scenarios = PostScenarios::new(base_url.to_string());
    let (posts, comments) = post_scenarios.run_all(&users, results).await?;
    results.created_posts = posts;
    results.created_comments = comments;

    // 4. Forums
    let forum_scenarios = ForumScenarios::new(base_url.to_string());
    let (forums, topics) = forum_scenarios.run_all(&users, results).await?;
    results.created_forums = forums;
    results.created_topics = topics;

    // 5. Messages
    let message_scenarios = MessageScenarios::new(base_url.to_string());
    let messages = message_scenarios.run_all(&users, results).await?;
    results.created_messages = messages;

    // 6. Groups
    let group_scenarios = GroupScenarios::new(base_url.to_string());
    let groups = group_scenarios.run_all(&users, results).await?;
    results.created_groups = groups;

    let duration = start.elapsed();
    println!("\n{}", format!("⏱ Total execution time: {:?}", duration).cyan());

    Ok(())
}

fn print_summary(results: &TestResults) {
    println!("\n{}", "═".repeat(70).cyan());
    println!("{}", "  TEST SUMMARY".cyan().bold());
    println!("{}", "═".repeat(70).cyan());

    let total = results.total_tests;
    let passed = results.passed_tests;
    let failed = results.failed_tests;

    println!("\n  Total Tests:    {}", total);
    println!("  ✅ Passed:       {}", passed.to_string().green());
    println!("  ❌ Failed:       {}", failed.to_string().red());

    if failed == 0 {
        println!("\n  {}", "🎉 All tests passed!".green().bold());
    } else {
        println!("\n  {}", format!("⚠️  {} tests failed", failed).yellow().bold());
    }

    println!("\n  Created Entities:");
    println!("    👤 Users:      {}", results.created_users.len());
    println!("    📝 Posts:      {}", results.created_posts.len());
    println!("    💬 Comments:   {}", results.created_comments.len());
    println!("    🏛️  Forums:     {}", results.created_forums.len());
    println!("    📌 Topics:     {}", results.created_topics.len());
    println!("    💌 Messages:   {}", results.created_messages.len());
    println!("    👥 Groups:     {}", results.created_groups.len());

    println!("\n  {}", "📄 Results saved to: data/test_results.json".blue());
    println!("{}", "═".repeat(70).cyan());
}
