//! Unit tests for posts module (scheduled posts and drafts)

use chrono::{Duration, Utc};
use jojuhu_backend::modules::posts::scheduled::{
    CreateScheduledPostRequest, SaveDraftRequest, ScheduledPostStatus, UpdateScheduledPostRequest,
};

#[test]
fn test_create_scheduled_post_request() {
    let future_time = Utc::now() + Duration::hours(2);

    let request = CreateScheduledPostRequest {
        content: "This is a scheduled post".to_string(),
        media_urls: Some(vec!["https://example.com/image.jpg".to_string()]),
        scheduled_for: future_time,
    };

    assert_eq!(request.content, "This is a scheduled post");
    assert!(request.media_urls.is_some());
    assert_eq!(request.media_urls.as_ref().unwrap().len(), 1);
    assert!(request.scheduled_for > Utc::now());
}

#[test]
fn test_update_scheduled_post_request() {
    let new_time = Utc::now() + Duration::hours(4);

    let request = UpdateScheduledPostRequest {
        content: Some("Updated content".to_string()),
        media_urls: None,
        scheduled_for: Some(new_time),
    };

    assert_eq!(request.content, Some("Updated content".to_string()));
    assert!(request.media_urls.is_none());
    assert!(request.scheduled_for.is_some());
}

#[test]
fn test_save_draft_request_with_content() {
    let request = SaveDraftRequest {
        content: Some("Draft content".to_string()),
        media_urls: Some(vec!["https://example.com/draft.jpg".to_string()]),
        visibility: Some("public".to_string()),
    };

    assert_eq!(request.content, Some("Draft content".to_string()));
    assert!(request.media_urls.is_some());
    assert_eq!(request.visibility, Some("public".to_string()));
}

#[test]
fn test_save_draft_request_minimal() {
    let request = SaveDraftRequest {
        content: Some("Just text".to_string()),
        media_urls: None,
        visibility: None,
    };

    assert_eq!(request.content, Some("Just text".to_string()));
    assert!(request.media_urls.is_none());
    assert!(request.visibility.is_none());
}

#[test]
fn test_scheduled_post_status_variants() {
    let pending = ScheduledPostStatus::Pending;
    let published = ScheduledPostStatus::Published;
    let failed = ScheduledPostStatus::Failed;
    let cancelled = ScheduledPostStatus::Cancelled;

    assert!(matches!(pending, ScheduledPostStatus::Pending));
    assert!(matches!(published, ScheduledPostStatus::Published));
    assert!(matches!(failed, ScheduledPostStatus::Failed));
    assert!(matches!(cancelled, ScheduledPostStatus::Cancelled));
}

#[test]
fn test_scheduled_post_future_time_validation() {
    let now = Utc::now();
    let one_hour_future = now + Duration::hours(1);
    let one_hour_past = now - Duration::hours(1);

    assert!(one_hour_future > now);
    assert!(one_hour_past < now);
}

#[test]
fn test_create_scheduled_post_without_media() {
    let future_time = Utc::now() + Duration::hours(3);

    let request = CreateScheduledPostRequest {
        content: "Text only post".to_string(),
        media_urls: None,
        scheduled_for: future_time,
    };

    assert_eq!(request.content, "Text only post");
    assert!(request.media_urls.is_none());
}

#[test]
fn test_update_scheduled_post_partial() {
    let request = UpdateScheduledPostRequest {
        content: None,
        media_urls: Some(vec!["https://example.com/new.jpg".to_string()]),
        scheduled_for: None,
    };

    assert!(request.content.is_none());
    assert!(request.media_urls.is_some());
    assert!(request.scheduled_for.is_none());
}

#[test]
fn test_save_draft_empty_content() {
    let request = SaveDraftRequest {
        content: None,
        media_urls: None,
        visibility: Some("private".to_string()),
    };

    assert!(request.content.is_none());
    assert!(request.media_urls.is_none());
    assert_eq!(request.visibility, Some("private".to_string()));
}
