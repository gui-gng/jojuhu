// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'data_export_request.dart';

DataExportRequest _$DataExportRequestFromJson(Map<String, dynamic> json) =>
    DataExportRequest(
      id: json['id'] as String,
      userId: json['user_id'] as String,
      status: json['status'] as String,
      filePath: json['file_path'] as String?,
      fileSize: json['file_size'] as int?,
      createdAt: DateTime.parse(json['created_at'] as String),
      completedAt: json['completed_at'] != null
          ? DateTime.parse(json['completed_at'] as String)
          : null,
      expiresAt: json['expires_at'] != null
          ? DateTime.parse(json['expires_at'] as String)
          : null,
    );

Map<String, dynamic> _$DataExportRequestToJson(DataExportRequest instance) =>
    <String, dynamic>{
      'id': instance.id,
      'user_id': instance.userId,
      'status': instance.status,
      'file_path': instance.filePath,
      'file_size': instance.fileSize,
      'created_at': instance.createdAt.toIso8601String(),
      'completed_at': instance.completedAt?.toIso8601String(),
      'expires_at': instance.expiresAt?.toIso8601String(),
    };