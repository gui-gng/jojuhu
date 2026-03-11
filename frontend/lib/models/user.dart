class User {
  final String id;
  final String username;
  final String email;
  final String? displayName;
  final String? bio;
  final String? avatarUrl;
  final DateTime createdAt;

  User({
    required this.id,
    required this.username,
    required this.email,
    this.displayName,
    this.bio,
    this.avatarUrl,
    required this.createdAt,
  });

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      id: json['id'],
      username: json['username'],
      email: json['email'],
      displayName: json['display_name'],
      bio: json['bio'],
      avatarUrl: json['avatar_url'],
      createdAt: DateTime.parse(json['created_at']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'username': username,
      'email': email,
      'display_name': displayName,
      'bio': bio,
      'avatar_url': avatarUrl,
      'created_at': createdAt.toIso8601String(),
    };
  }
}

class UserProfile {
  final String id;
  final String username;
  final String? displayName;
  final String? avatarUrl;

  UserProfile({
    required this.id,
    required this.username,
    this.displayName,
    this.avatarUrl,
  });

  factory UserProfile.fromJson(Map<String, dynamic> json) {
    return UserProfile(
      id: json['id'],
      username: json['username'],
      displayName: json['display_name'],
      avatarUrl: json['avatar_url'],
    );
  }
}
