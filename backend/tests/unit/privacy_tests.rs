//! Unit tests for privacy module

use jojuhu_backend::modules::privacy::models::{ProfileVisibility, UpdatePrivacySettingsRequest};
use uuid::Uuid;

#[test]
fn test_profile_visibility_default() {
    let visibility = ProfileVisibility::Public;
    assert!(matches!(visibility, ProfileVisibility::Public));

    let visibility = ProfileVisibility::FollowersOnly;
    assert!(matches!(visibility, ProfileVisibility::FollowersOnly));

    let visibility = ProfileVisibility::Private;
    assert!(matches!(visibility, ProfileVisibility::Private));
}

#[test]
fn test_update_privacy_settings_request() {
    let request = UpdatePrivacySettingsRequest {
        profile_visibility: Some("private".to_string()),
        show_email: Some(false),
        show_phone: Some(false),
        allow_mentions: Some(true),
        allow_tags: Some(true),
        show_online_status: Some(false),
        show_activity: Some(false),
        allow_search_engines: Some(false),
        data_processing_consent: Some(true),
        marketing_emails_consent: Some(false),
    };

    assert_eq!(request.profile_visibility, Some("private".to_string()));
    assert_eq!(request.show_email, Some(false));
    assert_eq!(request.allow_mentions, Some(true));
    assert_eq!(request.data_processing_consent, Some(true));
}

#[test]
fn test_update_privacy_settings_partial() {
    let request = UpdatePrivacySettingsRequest {
        profile_visibility: None,
        show_email: Some(true),
        show_phone: None,
        allow_mentions: None,
        allow_tags: Some(false),
        show_online_status: None,
        show_activity: None,
        allow_search_engines: None,
        data_processing_consent: None,
        marketing_emails_consent: None,
    };

    assert!(request.profile_visibility.is_none());
    assert_eq!(request.show_email, Some(true));
    assert_eq!(request.allow_tags, Some(false));
}

#[test]
fn test_privacy_settings_consent_flags() {
    let conservative_request = UpdatePrivacySettingsRequest {
        profile_visibility: Some("private".to_string()),
        show_email: Some(false),
        show_phone: Some(false),
        allow_mentions: Some(false),
        allow_tags: Some(false),
        show_online_status: Some(false),
        show_activity: Some(false),
        allow_search_engines: Some(false),
        data_processing_consent: Some(false),
        marketing_emails_consent: Some(false),
    };

    assert_eq!(conservative_request.show_email, Some(false));
    assert_eq!(conservative_request.data_processing_consent, Some(false));
    assert_eq!(conservative_request.marketing_emails_consent, Some(false));
}
