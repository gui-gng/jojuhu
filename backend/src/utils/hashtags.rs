use regex::Regex;
use std::collections::HashSet;

/// Extract hashtags from text
/// Returns a set of unique hashtag names (without # symbol)
pub fn extract_hashtags(text: &str) -> Vec<String> {
    lazy_static::lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(
            r"#([a-zA-Z0-9_]+)"
        ).unwrap();
    }

    let mut hashtags = HashSet::new();

    for cap in HASHTAG_REGEX.captures_iter(text) {
        if let Some(hashtag) = cap.get(1) {
            let tag = hashtag.as_str().to_lowercase();
            if !tag.is_empty() && tag.len() <= 100 {
                hashtags.insert(tag);
            }
        }
    }

    hashtags.into_iter().collect()
}

/// Convert hashtags in text to clickable links
/// Returns HTML string with linked hashtags
pub fn link_hashtags(text: &str) -> String {
    lazy_static::lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(
            r"#([a-zA-Z0-9_]+)"
        ).unwrap();
    }

    HASHTAG_REGEX
        .replace_all(text, |caps: &regex::Captures| {
            let hashtag = &caps[1];
            format!(
                r#"<a href="/hashtag/{}">#{}</a>"#,
                hashtag.to_lowercase(),
                hashtag
            )
        })
        .to_string()
}

/// Extract mentions from text (@username)
/// Returns a set of unique usernames (without @ symbol)
pub fn extract_mentions(text: &str) -> Vec<String> {
    lazy_static::lazy_static! {
        static ref MENTION_REGEX: Regex = Regex::new(
            r"@([a-zA-Z0-9_]+)"
        ).unwrap();
    }

    let mut mentions = HashSet::new();

    for cap in MENTION_REGEX.captures_iter(text) {
        if let Some(mention) = cap.get(1) {
            let username = mention.as_str().to_lowercase();
            if !username.is_empty() && username.len() <= 32 {
                mentions.insert(username);
            }
        }
    }

    mentions.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hashtags() {
        let text = "Check out #rust and #programming! #Rust";
        let hashtags = extract_hashtags(text);
        assert!(hashtags.contains(&"rust".to_string()));
        assert!(hashtags.contains(&"programming".to_string()));
        assert_eq!(hashtags.len(), 2); // Case insensitive, so only 2 unique
    }

    #[test]
    fn test_link_hashtags() {
        let text = "Check out #rust";
        let linked = link_hashtags(text);
        assert!(linked.contains("href=\"/hashtag/rust\""));
    }

    #[test]
    fn test_extract_mentions() {
        let text = "Hey @alice and @Bob!";
        let mentions = extract_mentions(text);
        assert!(mentions.contains(&"alice".to_string()));
        assert!(mentions.contains(&"bob".to_string()));
    }
}
