import 'package:json_annotation/json_annotation.dart';

part 'push_token.g.dart';

@JsonSerializable()
class PushNotificationToken {
  @JsonKey(name: 'id')
  final String id;

  @JsonKey(name: 'device_type')
  final String deviceType;

  @JsonKey(name: 'device_name')
  final String? deviceName;

  @JsonKey(name: 'is_active')
  final bool isActive;

  @JsonKey(name: 'created_at')
  final DateTime createdAt;

  PushNotificationToken({
    required this.id,
    required this.deviceType,
    this.deviceName,
    required this.isActive,
    required this.createdAt,
  });

  factory PushNotificationToken.fromJson(Map<String, dynamic> json) =>
      _$PushNotificationTokenFromJson(json);

  Map<String, dynamic> toJson() => _$PushNotificationTokenToJson(this);
}

@JsonSerializable()
class RegisterPushTokenRequest {
  @JsonKey(name: 'device_token')
  final String deviceToken;

  @JsonKey(name: 'device_type')
  final String deviceType;

  @JsonKey(name: 'device_name')
  final String? deviceName;

  RegisterPushTokenRequest({
    required this.deviceToken,
    required this.deviceType,
    this.deviceName,
  });

  factory RegisterPushTokenRequest.fromJson(Map<String, dynamic> json) =>
      _$RegisterPushTokenRequestFromJson(json);

  Map<String, dynamic> toJson() => _$RegisterPushTokenRequestToJson(this);
}

@JsonSerializable()
class DeactivatePushTokenRequest {
  @JsonKey(name: 'device_token')
  final String deviceToken;

  DeactivatePushTokenRequest({required this.deviceToken});

  factory DeactivatePushTokenRequest.fromJson(Map<String, dynamic> json) =>
      _$DeactivatePushTokenRequestFromJson(json);

  Map<String, dynamic> toJson() => _$DeactivatePushTokenRequestToJson(this);
}