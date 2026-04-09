import 'package:json_annotation/json_annotation.dart';

part 'privacy_settings.g.dart';

@JsonSerializable()
class PrivacySettings {
  @JsonKey(name: 'id')
  final String id;

  @JsonKey(name: 'user_id')
  final String userId;

  @JsonKey(name: 'profile_visibility')
  final String profileVisibility;

  @JsonKey(name: 'show_email')
  final bool showEmail;

  @JsonKey(name: 'show_phone')
  final bool showPhone;

  @JsonKey(name: 'allow_mentions')
  final bool allowMentions;

  @JsonKey(name: 'allow_tags')
  final bool allowTags;

  @JsonKey(name: 'show_online_status')
  final bool showOnlineStatus;

  @JsonKey(name: 'show_activity')
  final bool showActivity;

  @JsonKey(name: 'allow_search_engines')
  final bool allowSearchEngines;

  @JsonKey(name: 'data_processing_consent')
  final bool dataProcessingConsent;

  @JsonKey(name: 'marketing_emails_consent')
  final bool marketingEmailsConsent;

  @JsonKey(name: 'created_at')
  final DateTime createdAt;

  @JsonKey(name: 'updated_at')
  final DateTime updatedAt;

  PrivacySettings({
    required this.id,
    required this.userId,
    required this.profileVisibility,
    required this.showEmail,
    required this.showPhone,
    required this.allowMentions,
    required this.allowTags,
    required this.showOnlineStatus,
    required this.showActivity,
    required this.allowSearchEngines,
    required this.dataProcessingConsent,
    required this.marketingEmailsConsent,
    required this.createdAt,
    required this.updatedAt,
  });

  factory PrivacySettings.fromJson(Map<String, dynamic> json) =>
      _$PrivacySettingsFromJson(json);

  Map<String, dynamic> toJson() => _$PrivacySettingsToJson(this);

  PrivacySettings copyWith({
    String? profileVisibility,
    bool? showEmail,
    bool? showPhone,
    bool? allowMentions,
    bool? allowTags,
    bool? showOnlineStatus,
    bool? showActivity,
    bool? allowSearchEngines,
    bool? dataProcessingConsent,
    bool? marketingEmailsConsent,
  }) {
    return PrivacySettings(
      id: id,
      userId: userId,
      profileVisibility: profileVisibility ?? this.profileVisibility,
      showEmail: showEmail ?? this.showEmail,
      showPhone: showPhone ?? this.showPhone,
      allowMentions: allowMentions ?? this.allowMentions,
      allowTags: allowTags ?? this.allowTags,
      showOnlineStatus: showOnlineStatus ?? this.showOnlineStatus,
      showActivity: showActivity ?? this.showActivity,
      allowSearchEngines: allowSearchEngines ?? this.allowSearchEngines,
      dataProcessingConsent:
          dataProcessingConsent ?? this.dataProcessingConsent,
      marketingEmailsConsent:
          marketingEmailsConsent ?? this.marketingEmailsConsent,
      createdAt: createdAt,
      updatedAt: updatedAt,
    );
  }
}

@JsonSerializable()
class UpdatePrivacySettingsRequest {
  @JsonKey(name: 'profile_visibility')
  final String? profileVisibility;

  @JsonKey(name: 'show_email')
  final bool? showEmail;

  @JsonKey(name: 'show_phone')
  final bool? showPhone;

  @JsonKey(name: 'allow_mentions')
  final bool? allowMentions;

  @JsonKey(name: 'allow_tags')
  final bool? allowTags;

  @JsonKey(name: 'show_online_status')
  final bool? showOnlineStatus;

  @JsonKey(name: 'show_activity')
  final bool? showActivity;

  @JsonKey(name: 'allow_search_engines')
  final bool? allowSearchEngines;

  @JsonKey(name: 'data_processing_consent')
  final bool? dataProcessingConsent;

  @JsonKey(name: 'marketing_emails_consent')
  final bool? marketingEmailsConsent;

  UpdatePrivacySettingsRequest({
    this.profileVisibility,
    this.showEmail,
    this.showPhone,
    this.allowMentions,
    this.allowTags,
    this.showOnlineStatus,
    this.showActivity,
    this.allowSearchEngines,
    this.dataProcessingConsent,
    this.marketingEmailsConsent,
  });

  factory UpdatePrivacySettingsRequest.fromJson(Map<String, dynamic> json) =>
      _$UpdatePrivacySettingsRequestFromJson(json);

  Map<String, dynamic> toJson() => _$UpdatePrivacySettingsRequestToJson(this);
}