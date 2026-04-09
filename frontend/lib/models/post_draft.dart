import 'package:json_annotation/json_annotation.dart';

part 'post_draft.g.dart';

@JsonSerializable()
class PostDraft {
  @JsonKey(name: 'id')
  final String id;

  @JsonKey(name: 'user_id')
  final String userId;

  @JsonKey(name: 'content')
  final String? content;

  @JsonKey(name: 'media_urls')
  final List<String>? mediaUrls;

  @JsonKey(name: 'visibility')
  final String? visibility;

  @JsonKey(name: 'updated_at')
  final DateTime updatedAt;

  PostDraft({
    required this.id,
    required this.userId,
    this.content,
    this.mediaUrls,
    this.visibility,
    required this.updatedAt,
  });

  factory PostDraft.fromJson(Map<String, dynamic> json) =>
      _$PostDraftFromJson(json);

  Map<String, dynamic> toJson() => _$PostDraftToJson(this);
}

@JsonSerializable()
class SaveDraftRequest {
  @JsonKey(name: 'content')
  final String? content;

  @JsonKey(name: 'media_urls')
  final List<String>? mediaUrls;

  @JsonKey(name: 'visibility')
  final String? visibility;

  SaveDraftRequest({
    this.content,
    this.mediaUrls,
    this.visibility,
  });

  factory SaveDraftRequest.fromJson(Map<String, dynamic> json) =>
      _$SaveDraftRequestFromJson(json);

  Map<String, dynamic> toJson() => _$SaveDraftRequestToJson(this);
}