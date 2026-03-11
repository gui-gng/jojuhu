class PostAuthor {
  final String id;
  final String username;
  final String? displayName;
  final String? avatarUrl;

  PostAuthor({
    required this.id,
    required this.username,
    this.displayName,
    this.avatarUrl,
  });

  factory PostAuthor.fromJson(Map<String, dynamic> json) {
    return PostAuthor(
      id: json['id'],
      username: json['username'],
      displayName: json['display_name'],
      avatarUrl: json['avatar_url'],
    );
  }
}

class Post {
  final String id;
  final PostAuthor author;
  final String content;
  final List<String> mediaUrls;
  final int likesCount;
  final int commentsCount;
  final int sharesCount;
  final bool isPublic;
  final DateTime createdAt;
  final bool isLiked;

  Post({
    required this.id,
    required this.author,
    required this.content,
    required this.mediaUrls,
    required this.likesCount,
    required this.commentsCount,
    required this.sharesCount,
    required this.isPublic,
    required this.createdAt,
    required this.isLiked,
  });

  factory Post.fromJson(Map<String, dynamic> json) {
    return Post(
      id: json['id'],
      author: PostAuthor.fromJson(json['author']),
      content: json['content'],
      mediaUrls: List<String>.from(json['media_urls'] ?? []),
      likesCount: json['likes_count'],
      commentsCount: json['comments_count'],
      sharesCount: json['shares_count'],
      isPublic: json['is_public'],
      createdAt: DateTime.parse(json['created_at']),
      isLiked: json['is_liked'],
    );
  }
}

class Comment {
  final String id;
  final PostAuthor author;
  final String content;
  final String? parentCommentId;
  final int likesCount;
  final DateTime createdAt;

  Comment({
    required this.id,
    required this.author,
    required this.content,
    this.parentCommentId,
    required this.likesCount,
    required this.createdAt,
  });

  factory Comment.fromJson(Map<String, dynamic> json) {
    return Comment(
      id: json['id'],
      author: PostAuthor.fromJson(json['author']),
      content: json['content'],
      parentCommentId: json['parent_comment_id'],
      likesCount: json['likes_count'],
      createdAt: DateTime.parse(json['created_at']),
    );
  }
}
