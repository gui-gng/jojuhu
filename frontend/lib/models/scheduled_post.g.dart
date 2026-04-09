// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'scheduled_post.dart';

ScheduledPost _$ScheduledPostFromJson(Map<String, dynamic> json) =>
    ScheduledPost(
      id: json['id'] as String,
      content: json['content'] as String,
      mediaUrls: (json['media_urls'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      scheduledFor: DateTime.parse(json['scheduled_for'] as String),
      status: json['status'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
    );

Map<String, dynamic> _$ScheduledPostToJson(ScheduledPost instance) =>
    <String, dynamic>{
      'id': instance.id,
      'content': instance.content,
      'media_urls': instance.mediaUrls,
      'scheduled_for': instance.scheduledFor.toIso8601String(),
      'status': instance.status,
      'created_at': instance.createdAt.toIso8601String(),
    };

CreateScheduledPostRequest _$CreateScheduledPostRequestFromJson(
        Map<String, dynamic> json) =>
    CreateScheduledPostRequest(
      content: json['content'] as String,
      mediaUrls: (json['media_urls'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      scheduledFor: DateTime.parse(json['scheduled_for'] as String),
    );

Map<String, dynamic> _$CreateScheduledPostRequestToJson(
        CreateScheduledPostRequest instance) =>
    <String, dynamic>{
      'content': instance.content,
      'media_urls': instance.mediaUrls,
      'scheduled_for': instance.scheduledFor.toIso8601String(),
    };

UpdateScheduledPostRequest _$UpdateScheduledPostRequestFromJson(
        Map<String, dynamic> json) =>
    UpdateScheduledPostRequest(
      content: json['content'] as String?,
      mediaUrls: (json['media_urls'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      scheduledFor: json['scheduled_for'] != null
          ? DateTime.parse(json['scheduled_for'] as String)
          : null,
    );

Map<String, dynamic> _$UpdateScheduledPostRequestToJson(
        UpdateScheduledPostRequest instance) =>
    <String, dynamic>{
      'content': instance.content,
      'media_urls': instance.mediaUrls,
      'scheduled_for': instance.scheduledFor?.toIso8601String(),
    };