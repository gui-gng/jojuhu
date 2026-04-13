use anyhow::Result;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

/// Helper to calculate dynamic size based on available items
pub fn dynamic_size(available: usize, percentage: f64, min: usize, max: usize) -> usize {
    let calculated = (available as f64 * percentage / 100.0).ceil() as usize;
    calculated.clamp(min, max)
}

/// Calculate number of posts per user based on total user count
pub fn posts_per_user(total_users: usize) -> usize {
    match total_users {
        0..=5 => 3,  // Few users: more posts each
        6..=20 => 2, // Medium: standard
        _ => 1,      // Many users: fewer posts each to avoid overload
    }
}

/// Calculate number of items to create based on creator count
pub fn items_to_create(creators: usize, items_per_creator: usize, max_total: usize) -> usize {
    let total = creators * items_per_creator;
    total.min(max_total)
}

/// Get random delay between actions (in milliseconds)
pub fn random_delay(min_ms: u64, max_ms: u64) -> Duration {
    use rand::thread_rng;
    let mut rng = thread_rng();
    Duration::from_millis(rng.gen_range(min_ms..=max_ms))
}

/// Select random subset from a collection
pub fn select_random_subset<T: Clone>(
    items: &[T],
    percentage: f64,
    min: usize,
    max: usize,
) -> Vec<T> {
    use rand::thread_rng;
    let mut rng = thread_rng();
    let count = dynamic_size(items.len(), percentage, min, max);

    let mut selected: Vec<T> = items.choose_multiple(&mut rng, count).cloned().collect();

    selected
}

/// Shuffle a vector randomly
pub fn shuffle<T>(items: &mut [T]) {
    use rand::thread_rng;
    let mut rng = thread_rng();
    items.shuffle(&mut rng);
}
