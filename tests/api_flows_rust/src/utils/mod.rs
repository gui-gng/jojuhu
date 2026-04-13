pub mod api_client;
pub mod logging;
pub mod scaling;

use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::models::{TestResults, TestUsersData};

pub fn load_test_users<P: AsRef<Path>>(path: P) -> Result<TestUsersData> {
    let content = fs::read_to_string(path)?;
    let data: TestUsersData = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn save_test_results<P: AsRef<Path>>(path: P, results: &TestResults) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn ensure_data_dir() -> Result<()> {
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
    }
    Ok(())
}
