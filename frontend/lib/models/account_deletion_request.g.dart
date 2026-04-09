// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'account_deletion_request.dart';

AccountDeletionRequest _$AccountDeletionRequestFromJson(
        Map<String, dynamic> json) =>
    AccountDeletionRequest(
      id: json['id'] as String,
      userId: json['user_id'] as String,
      reason: json['reason'] as String?,
      status: json['status'] as String,
      requestedAt: DateTime.parse(json['requested_at'] as String),
      scheduledDeletionAt:
          DateTime.parse(json['scheduled_deletion_at'] as String),
      completedAt: json['completed_at'] != null
          ? DateTime.parse(json['completed_at'] as String)
          : null,
    );

Map<String, dynamic> _$AccountDeletionRequestToJson(
        AccountDeletionRequest instance) =>
    <String, dynamic>{
      'id': instance.id,
      'user_id': instance.userId,
      'reason': instance.reason,
      'status': instance.status,
      'requested_at': instance.requestedAt.toIso8601String(),
      'scheduled_deletion_at': instance.scheduledDeletionAt.toIso8601String(),
      'completed_at': instance.completedAt?.toIso8601String(),
    };

RequestAccountDeletionRequest _$RequestAccountDeletionRequestFromJson(
        Map<String, dynamic> json) =>
    RequestAccountDeletionRequest(
      reason: json['reason'] as String?,
    );

Map<String, dynamic> _$RequestAccountDeletionRequestToJson(
        RequestAccountDeletionRequest instance) =>
    <String, dynamic>{
      'reason': instance.reason,
    };