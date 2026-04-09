// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'push_token.dart';

PushNotificationToken _$PushNotificationTokenFromJson(
        Map<String, dynamic> json) =>
    PushNotificationToken(
      id: json['id'] as String,
      deviceType: json['device_type'] as String,
      deviceName: json['device_name'] as String?,
      isActive: json['is_active'] as bool,
      createdAt: DateTime.parse(json['created_at'] as String),
    );

Map<String, dynamic> _$PushNotificationTokenToJson(
        PushNotificationToken instance) =>
    <String, dynamic>{
      'id': instance.id,
      'device_type': instance.deviceType,
      'device_name': instance.deviceName,
      'is_active': instance.isActive,
      'created_at': instance.createdAt.toIso8601String(),
    };

RegisterPushTokenRequest _$RegisterPushTokenRequestFromJson(
        Map<String, dynamic> json) =>
    RegisterPushTokenRequest(
      deviceToken: json['device_token'] as String,
      deviceType: json['device_type'] as String,
      deviceName: json['device_name'] as String?,
    );

Map<String, dynamic> _$RegisterPushTokenRequestToJson(
        RegisterPushTokenRequest instance) =>
    <String, dynamic>{
      'device_token': instance.deviceToken,
      'device_type': instance.deviceType,
      'device_name': instance.deviceName,
    };

DeactivatePushTokenRequest _$DeactivatePushTokenRequestFromJson(
        Map<String, dynamic> json) =>
    DeactivatePushTokenRequest(
      deviceToken: json['device_token'] as String,
    );

Map<String, dynamic> _$DeactivatePushTokenRequestToJson(
        DeactivatePushTokenRequest instance) =>
    <String, dynamic>{
      'device_token': instance.deviceToken,
    };