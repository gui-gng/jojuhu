// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'post_draft.dart';

PostDraft _$PostDraftFromJson(Map<String, dynamic> json) => PostDraft(
      id: json['id'] as String,
      userId: json['user_id'] as String,
      content: json['content'] as String?,
      mediaUrls: (json['media_urls'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      visibility: json['visibility'] as String?,
      updatedAt: DateTime.parse(json['updated_at'] as String),
    );

Map<String, dynamic> _$PostDraftToJson(PostDraft instance) =>
    <String, dynamic>{
      'id': instance.id,
      'user_id': instance.userId,
      'content': instance.content,
      'media_urls': instance.mediaUrls,
      'visibility': instance.visibility,
      'updated_at': instance.updatedAt.toIso8601String(),
    };

SaveDraftRequest _$SaveDraftRequestFromJson(Map<String, dynamic> json) =>
    SaveDraftRequest(
      content: json['content'] as String?,
      mediaUrls: (json['media_urls'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      visibility: json['visibility'] as String?,
    );

Map<String, dynamic> _$SaveDraftRequestToJson(SaveDraftRequest instance) =>
    <String, dynamic>{
      'content': instance.content,
      'media_urls': instance.mediaUrls,
      'visibility': instance.visibility,
    };