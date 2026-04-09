import 'package:json_annotation/json_annotation.dart';

part 'account_deletion_request.g.dart';

@JsonSerializable()
class AccountDeletionRequest {
  @JsonKey(name: 'id')
  final String id;

  @JsonKey(name: 'user_id')
  final String userId;

  @JsonKey(name: 'reason')
  final String? reason;

  @JsonKey(name: 'status')
  final String status;

  @JsonKey(name: 'requested_at')
  final DateTime requestedAt;

  @JsonKey(name: 'scheduled_deletion_at')
  final DateTime scheduledDeletionAt;

  @JsonKey(name: 'completed_at')
  final DateTime? completedAt;

  AccountDeletionRequest({
    required this.id,
    required this.userId,
    this.reason,
    required this.status,
    required this.requestedAt,
    required this.scheduledDeletionAt,
    this.completedAt,
  });

  factory AccountDeletionRequest.fromJson(Map<String, dynamic> json) =>
      _$AccountDeletionRequestFromJson(json);

  Map<String, dynamic> toJson() => _$AccountDeletionRequestToJson(this);
}

@JsonSerializable()
class RequestAccountDeletionRequest {
  @JsonKey(name: 'reason')
  final String? reason;

  RequestAccountDeletionRequest({this.reason});

  factory RequestAccountDeletionRequest.fromJson(Map<String, dynamic> json) =>
      _$RequestAccountDeletionRequestFromJson(json);

  Map<String, dynamic> toJson() =>
      _$RequestAccountDeletionRequestToJson(this);
}