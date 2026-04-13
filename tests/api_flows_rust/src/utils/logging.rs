use colored::*;

pub fn log_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

pub fn log_error(message: &str) {
    println!("{} {}", "✗".red(), message);
}

pub fn log_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

pub fn log_warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message);
}

pub fn log_section(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!("{}", title.cyan().bold());
    println!("{}\n", "=".repeat(70));
}

pub fn log_test_start(name: &str) {
    println!("\n{}", "─".repeat(70));
    println!("🧪 Testing: {}", name.yellow());
    println!("{}", "─".repeat(70));
}

pub fn log_test_result(passed: usize, failed: usize) {
    let total = passed + failed;
    if failed == 0 {
        println!("\n{} {}/{} tests passed\n", "✓".green(), passed, total);
    } else {
        println!(
            "\n{} {}/{} tests passed, {} failed\n",
            "⚠".yellow(),
            passed,
            total,
            failed
        );
    }
}
