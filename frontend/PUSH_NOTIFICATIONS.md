# Push Notification Setup Guide

## Overview
This guide explains how to enable Firebase Cloud Messaging (FCM) push notifications in the Jojuhu app.

## Current Status
✅ Backend ready (push token registration endpoints)
✅ Service skeleton created (`lib/services/push_notification_service.dart`)
⚠️ Firebase configuration needed
⚠️ Platform-specific setup needed

## Prerequisites
1. Firebase project created
2. Flutter project configured with Firebase
3. Platform-specific requirements met

## Setup Steps

### 1. Add Dependencies

Update `pubspec.yaml`:

```yaml
dependencies:
  flutter:
    sdk: flutter
  # ... existing dependencies ...
  
  # Push notifications
  firebase_core: ^2.24.2
  firebase_messaging: ^14.7.0
  flutter_local_notifications: ^16.0.0
  
dev_dependencies:
  # ... existing dependencies ...
```

### 2. Create Firebase Project

1. Go to [Firebase Console](https://console.firebase.google.com)
2. Create or select project
3. Go to Project Settings > General
4. Add apps for each platform (Android, iOS, Web)

### 3. Android Setup

**Download google-services.json:**
1. In Firebase Console, add Android app
2. Package name: `com.jojuhu.app`
3. Download `google-services.json`
4. Place in `android/app/google-services.json`

**Update android/build.gradle:**
```gradle
dependencies {
    classpath 'com.android.tools.build:gradle:7.4.2'
    classpath 'com.google.gms:google-services:4.3.15'  // Add this
}
```

**Update android/app/build.gradle:**
```gradle
plugins {
    id "com.android.application"
    id "kotlin-android"
    id "dev.flutter.flutter-gradle-plugin"
    id "com.google.gms.google-services"  // Add this
}

android {
    // ... existing config ...
}

dependencies {
    implementation "org.jetbrains.kotlin:kotlin-stdlib-jdk7:$kotlin_version"
    implementation 'com.google.firebase:firebase-messaging:23.3.1'  // Add this
}
```

**Update android/app/src/main/AndroidManifest.xml:**
```xml
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <uses-permission android:name="android.permission.INTERNET"/>
    <uses-permission android:name="android.permission.RECEIVE_BOOT_COMPLETED"/>
    <uses-permission android:name="android.permission.VIBRATE"/>
    <uses-permission android:name="android.permission.WAKE_LOCK"/>
    
    <application
        android:label="Jojuhu"
        android:name="${applicationName}">
        
        <!-- Add these receivers -->
        <receiver android:name="com.dexterous.flutterlocalnotifications.ScheduledNotificationReceiver"
            android:exported="false" />
        <receiver android:name="com.dexterous.flutterlocalnotifications.ScheduledNotificationBootReceiver"
            android:exported="false">
            <intent-filter>
                <action android:name="android.intent.action.BOOT_COMPLETED"/>
                <action android:name="android.intent.action.QUICKBOOT_POWERON"/>
                <action android:name="android.intent.action.MY_PACKAGE_REPLACED"/>
            </intent-filter>
        </receiver>
        
        <!-- Firebase Messaging -->
        <service
            android:name="com.google.firebase.messaging.FirebaseMessagingService"
            android:exported="false">
            <intent-filter>
                <action android:name="com.google.firebase.MESSAGING_EVENT" />
            </intent-filter>
        </service>
        
        <activity
            android:name=".MainActivity"
            android:launchMode="singleTop"
            android:theme="@style/LaunchTheme"
            android:configChanges="orientation|keyboardHidden|keyboard|screenSize|smallestScreenSize|locale|layoutDirection|fontScale|screenLayout|density|uiMode"
            android:hardwareAccelerated="true"
            android:windowSoftInputMode="adjustResize">
            <meta-data
                android:name="io.flutter.embedding.android.NormalTheme"
                android:resource="@style/NormalTheme"
                />
            <intent-filter>
                <action android:name="android.intent.action.MAIN"/>
                <category android:name="android.intent.category.LAUNCHER"/>
            </intent-filter>
        </activity>
        
        <meta-data
            android:name="flutterEmbedding"
            android:value="2" />
        
        <!-- Default notification channel -->
        <meta-data
            android:name="com.google.firebase.messaging.default_notification_channel_id"
            android:value="default_channel"/>
    </application>
</manifest>
```

### 4. iOS Setup

**Download GoogleService-Info.plist:**
1. In Firebase Console, add iOS app
2. Bundle ID: `com.jojuhu.app`
3. Download `GoogleService-Info.plist`
4. Place in `ios/Runner/GoogleService-Info.plist`

**Update ios/Runner/Info.plist:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- ... existing keys ... -->
    
    <key>UIBackgroundModes</key>
    <array>
        <string>fetch</string>
        <string>remote-notification</string>
    </array>
    
    <key>FirebaseAppDelegateProxyEnabled</key>
    <false/>
</dict>
</plist>
```

**Update ios/Runner/AppDelegate.swift:**
```swift
import UIKit
import Flutter

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    
    // Request notification permission
    if #available(iOS 10.0, *) {
        UNUserNotificationCenter.current().requestAuthorization(
            options: [.alert, .badge, .sound],
            completionHandler: { granted, error in
                if granted {
                    DispatchQueue.main.async {
                        application.registerForRemoteNotifications()
                    }
                }
            }
        )
    } else {
        let settings = UIUserNotificationSettings(
            types: [.alert, .badge, .sound],
            categories: nil
        )
        application.registerUserNotificationSettings(settings)
        application.registerForRemoteNotifications()
    }
    
    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
```

**Enable Push Notifications capability:**
1. Open `ios/Runner.xcworkspace` in Xcode
2. Select Runner project
3. Go to Signing & Capabilities
4. Click + Capability
5. Add "Push Notifications"

### 5. Web Setup

**Add Firebase config to web/index.html:**
```html
<head>
  <!-- ... existing head content ... -->
  
  <!-- Firebase SDKs -->
  <script src="https://www.gstatic.com/firebasejs/9.22.0/firebase-app.js"></script>
  <script src="https://www.gstatic.com/firebasejs/9.22.0/firebase-messaging.js"></script>
  
  <script>
    // Your web app's Firebase configuration
    var firebaseConfig = {
      apiKey: "YOUR_API_KEY",
      authDomain: "YOUR_PROJECT.firebaseapp.com",
      projectId: "YOUR_PROJECT",
      storageBucket: "YOUR_PROJECT.appspot.com",
      messagingSenderId: "YOUR_SENDER_ID",
      appId: "YOUR_APP_ID"
    };
    
    // Initialize Firebase
    firebase.initializeApp(firebaseConfig);
  </script>
</head>
```

### 6. Initialize in main.dart

Update `lib/main.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:jojuhu/services/push_notification_service.dart';
import 'app.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize Firebase
  await Firebase.initializeApp();
  
  // Initialize push notifications
  await PushNotificationService().initialize();
  
  runApp(const MyApp());
}
```

### 7. Implement Notification Handling

Update notification tap handling in `PushNotificationService`:

```dart
void _handleNotificationTap(Map<String, dynamic> data) {
  final type = data['type'] as String?;
  final id = data['id'] as String?;
  
  switch (type) {
    case 'message':
      // Navigate to messages screen
      navigatorKey.currentState?.pushNamed('/messages', arguments: {'conversationId': id});
      break;
    case 'comment':
      // Navigate to post detail
      navigatorKey.currentState?.pushNamed('/post', arguments: {'postId': id});
      break;
    case 'follow':
      // Navigate to user profile
      navigatorKey.currentState?.pushNamed('/profile', arguments: {'userId': id});
      break;
    default:
      // Navigate to home
      navigatorKey.currentState?.pushNamed('/');
  }
}
```

### 8. Configure Backend (if not done)

Backend already has endpoints:
- `POST /api/v1/push-tokens` - Register device token
- `POST /api/v1/push-tokens/deactivate` - Deactivate device token

## Testing

### Android
1. Run on physical device (emulators have limitations)
2. App should request notification permission
3. Send test notification from Firebase Console:
   - Go to Cloud Messaging
   - Click "Send your first message"
   - Enter title and body
   - Select app
   - Send

### iOS
1. Run on physical device (simulator doesn't receive push)
2. Accept notification permission
3. Send test from Firebase Console

### Web
1. Run on localhost or HTTPS
2. Browser should request notification permission
3. Test from Firebase Console

## Troubleshooting

### Common Issues

1. **No notifications on Android:**
   - Check `google-services.json` placement
   - Verify Gradle dependencies
   - Check notification permission granted

2. **No notifications on iOS:**
   - Verify `GoogleService-Info.plist` placement
   - Check Push Notifications capability enabled
   - Verify Apple Push Notification service (APNs) configured in Firebase
   - Must test on physical device

3. **No notifications on Web:**
   - Must serve over HTTPS
   - Check browser notification permissions
   - Verify Firebase config in index.html

4. **Notifications not received in foreground:**
   - Implement `FirebaseMessaging.onMessage` handler
   - Use `flutter_local_notifications` for foreground display

## Resources

- [Firebase Flutter Setup](https://firebase.google.com/docs/flutter/setup)
- [Firebase Cloud Messaging Flutter](https://firebase.google.com/docs/cloud-messaging/flutter/client)
- [flutter_local_notifications](https://pub.dev/packages/flutter_local_notifications)

## Current Implementation Status

✅ Backend API ready
✅ Push notification service skeleton created
✅ Device token registration code written
⚠️ Firebase configuration pending
⚠️ Platform-specific setup pending
⚠️ Notification handling needs testing

## Next Steps

1. Create Firebase project
2. Configure each platform (Android, iOS, Web)
3. Test push notifications
4. Integrate with app navigation
5. Handle different notification types
6. Test background/foreground scenarios