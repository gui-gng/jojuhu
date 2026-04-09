import 'package:json_annotation/json_annotation.dart';

part 'scheduled_post.g.dart';

@JsonSerializable()
class ScheduledPost {
  @JsonKey(name: 'id')
  final String id;

  @JsonKey(name: 'content')
  final String content;

  @JsonKey(name: 'media_urls')
  final List<String>? mediaUrls;

  @JsonKey(name: 'scheduled_for')
  final DateTime scheduledFor;

  @JsonKey(name: 'status')
  final String status;

  @JsonKey(name: 'created_at')
  final DateTime createdAt;

  ScheduledPost({
    required this.id,
    required this.content,
    this.mediaUrls,
    required this.scheduledFor,
    required this.status,
    required this.createdAt,
  });

  factory ScheduledPost.fromJson(Map<String, dynamic> json) =>
      _$ScheduledPostFromJson(json);

  Map<String, dynamic> toJson() => _$ScheduledPostToJson(this);
}

@JsonSerializable()
class CreateScheduledPostRequest {
  @JsonKey(name: 'content')
  final String content;

  @JsonKey(name: 'media_urls')
  final List<String>? mediaUrls;

  @JsonKey(name: 'scheduled_for')
  final DateTime scheduledFor;

  CreateScheduledPostRequest({
    required this.content,
    this.mediaUrls,
    required this.scheduledFor,
  });

  factory CreateScheduledPostRequest.fromJson(Map<String, dynamic> json) =>
      _$CreateScheduledPostRequestFromJson(json);

  Map<String, dynamic> toJson() => _$CreateScheduledPostRequestToJson(this);
}

@JsonSerializable()
class UpdateScheduledPostRequest {
  @JsonKey(name: 'content')
  final String? content;

  @JsonKey(name: 'media_urls')
  final List<String>? mediaUrls;

  @JsonKey(name: 'scheduled_for')
  final DateTime? scheduledFor;

  UpdateScheduledPostRequest({
    this.content,
    this.mediaUrls,
    this.scheduledFor,
  });

  factory UpdateScheduledPostRequest.fromJson(Map<String, dynamic> json) =>
      _$UpdateScheduledPostRequestFromJson(json);

  Map<String, dynamic> toJson() => _$UpdateScheduledPostRequestToJson(this);
}