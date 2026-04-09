// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'privacy_settings.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PrivacySettings _$PrivacySettingsFromJson(Map<String, dynamic> json) =>
    PrivacySettings(
      id: json['id'] as String,
      userId: json['user_id'] as String,
      profileVisibility: json['profile_visibility'] as String,
      showEmail: json['show_email'] as bool,
      showPhone: json['show_phone'] as bool,
      allowMentions: json['allow_mentions'] as bool,
      allowTags: json['allow_tags'] as bool,
      showOnlineStatus: json['show_online_status'] as bool,
      showActivity: json['show_activity'] as bool,
      allowSearchEngines: json['allow_search_engines'] as bool,
      dataProcessingConsent: json['data_processing_consent'] as bool,
      marketingEmailsConsent: json['marketing_emails_consent'] as bool,
      createdAt: DateTime.parse(json['created_at'] as String),
      updatedAt: DateTime.parse(json['updated_at'] as String),
    );

Map<String, dynamic> _$PrivacySettingsToJson(PrivacySettings instance) =>
    <String, dynamic>{
      'id': instance.id,
      'user_id': instance.userId,
      'profile_visibility': instance.profileVisibility,
      'show_email': instance.showEmail,
      'show_phone': instance.showPhone,
      'allow_mentions': instance.allowMentions,
      'allow_tags': instance.allowTags,
      'show_online_status': instance.showOnlineStatus,
      'show_activity': instance.showActivity,
      'allow_search_engines': instance.allowSearchEngines,
      'data_processing_consent': instance.dataProcessingConsent,
      'marketing_emails_consent': instance.marketingEmailsConsent,
      'created_at': instance.createdAt.toIso8601String(),
      'updated_at': instance.updatedAt.toIso8601String(),
    };

UpdatePrivacySettingsRequest _$UpdatePrivacySettingsRequestFromJson(
        Map<String, dynamic> json) =>
    UpdatePrivacySettingsRequest(
      profileVisibility: json['profile_visibility'] as String?,
      showEmail: json['show_email'] as bool?,
      showPhone: json['show_phone'] as bool?,
      allowMentions: json['allow_mentions'] as bool?,
      allowTags: json['allow_tags'] as bool?,
      showOnlineStatus: json['show_online_status'] as bool?,
      showActivity: json['show_activity'] as bool?,
      allowSearchEngines: json['allow_search_engines'] as bool?,
      dataProcessingConsent: json['data_processing_consent'] as bool?,
      marketingEmailsConsent: json['marketing_emails_consent'] as bool?,
    );

Map<String, dynamic> _$UpdatePrivacySettingsRequestToJson(
        UpdatePrivacySettingsRequest instance) =>
    <String, dynamic>{
      'profile_visibility': instance.profileVisibility,
      'show_email': instance.showEmail,
      'show_phone': instance.showPhone,
      'allow_mentions': instance.allowMentions,
      'allow_tags': instance.allowTags,
      'show_online_status': instance.showOnlineStatus,
      'show_activity': instance.showActivity,
      'allow_search_engines': instance.allowSearchEngines,
      'data_processing_consent': instance.dataProcessingConsent,
      'marketing_emails_consent': instance.marketingEmailsConsent,
    };
