
/// Extended user profile with stats
class UserProfile {
  final String id;
  final String username;
  final String? displayName;
  final String? bio;
  final String? avatarUrl;
  final DateTime createdAt;
  final int followersCount;
  final int followingCount;
  final int postsCount;
  final bool isFollowing;

  UserProfile({
    required this.id,
    required this.username,
    this.displayName,
    this.bio,
    this.avatarUrl,
    required this.createdAt,
    required this.followersCount,
    required this.followingCount,
    required this.postsCount,
    required this.isFollowing,
  });

  factory UserProfile.fromJson(Map<String, dynamic> json) {
    return UserProfile(
      id: json['id'],
      username: json['username'],
      displayName: json['display_name'],
      bio: json['bio'],
      avatarUrl: json['avatar_url'],
      createdAt: DateTime.parse(json['created_at']),
      followersCount: json['followers_count'] ?? 0,
      followingCount: json['following_count'] ?? 0,
      postsCount: json['posts_count'] ?? 0,
      isFollowing: json['is_following'] ?? false,
    );
  }

  String get displayNameOrUsername => displayName ?? username;
}

/// User profile for authenticated user (includes email)
class MyProfile {
  final String id;
  final String username;
  final String email;
  final String? displayName;
  final String? bio;
  final String? avatarUrl;
  final DateTime createdAt;
  final int followersCount;
  final int followingCount;
  final int postsCount;

  MyProfile({
    required this.id,
    required this.username,
    required this.email,
    this.displayName,
    this.bio,
    this.avatarUrl,
    required this.createdAt,
    required this.followersCount,
    required this.followingCount,
    required this.postsCount,
  });

  factory MyProfile.fromJson(Map<String, dynamic> json) {
    return MyProfile(
      id: json['id'],
      username: json['username'],
      email: json['email'],
      displayName: json['display_name'],
      bio: json['bio'],
      avatarUrl: json['avatar_url'],
      createdAt: DateTime.parse(json['created_at']),
      followersCount: json['followers_count'] ?? 0,
      followingCount: json['following_count'] ?? 0,
      postsCount: json['posts_count'] ?? 0,
    );
  }

  String get displayNameOrUsername => displayName ?? username;
}

/// User info for lists
class UserInfo {
  final String id;
  final String username;
  final String? displayName;
  final String? avatarUrl;
  final bool isFollowing;

  UserInfo({
    required this.id,
    required this.username,
    this.displayName,
    this.avatarUrl,
    required this.isFollowing,
  });

  factory UserInfo.fromJson(Map<String, dynamic> json) {
    return UserInfo(
      id: json['id'],
      username: json['username'],
      displayName: json['display_name'],
      avatarUrl: json['avatar_url'],
      isFollowing: json['is_following'] ?? false,
    );
  }

  String get displayNameOrUsername => displayName ?? username;
}

/// Users list response
class UsersListResponse {
  final List<UserInfo> users;
  final int total;
  final int page;
  final int perPage;
  final bool hasMore;

  UsersListResponse({
    required this.users,
    required this.total,
    required this.page,
    required this.perPage,
    required this.hasMore,
  });

  factory UsersListResponse.fromJson(Map<String, dynamic> json) {
    return UsersListResponse(
      users: (json['users'] as List)
          .map((u) => UserInfo.fromJson(u))
          .toList(),
      total: json['total'] ?? 0,
      page: json['page'] ?? 1,
      perPage: json['per_page'] ?? 20,
      hasMore: json['has_more'] ?? false,
    );
  }
}
