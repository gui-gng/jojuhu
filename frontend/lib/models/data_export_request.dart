import 'package:json_annotation/json_annotation.dart';

part 'data_export_request.g.dart';

@JsonSerializable()
class DataExportRequest {
  @JsonKey(name: 'id')
  final String id;

  @JsonKey(name: 'user_id')
  final String userId;

  @JsonKey(name: 'status')
  final String status;

  @JsonKey(name: 'file_path')
  final String? filePath;

  @JsonKey(name: 'file_size')
  final int? fileSize;

  @JsonKey(name: 'created_at')
  final DateTime createdAt;

  @JsonKey(name: 'completed_at')
  final DateTime? completedAt;

  @JsonKey(name: 'expires_at')
  final DateTime? expiresAt;

  DataExportRequest({
    required this.id,
    required this.userId,
    required this.status,
    this.filePath,
    this.fileSize,
    required this.createdAt,
    this.completedAt,
    this.expiresAt,
  });

  factory DataExportRequest.fromJson(Map<String, dynamic> json) =>
      _$DataExportRequestFromJson(json);

  Map<String, dynamic> toJson() => _$DataExportRequestToJson(this);
}