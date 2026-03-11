class ForumCreator {
  final String id;
  final String username;
  final String? displayName;

  ForumCreator({
    required this.id,
    required this.username,
    this.displayName,
  });

  factory ForumCreator.fromJson(Map<String, dynamic> json) {
    return ForumCreator(
      id: json['id'],
      username: json['username'],
      displayName: json['display_name'],
    );
  }
}

class Forum {
  final String id;
  final String name;
  final String slug;
  final String? description;
  final String? iconUrl;
  final String? coverImageUrl;
  final ForumCreator creator;
  final bool isPublic;
  final int membersCount;
  final int topicsCount;
  final bool isMember;
  final DateTime createdAt;

  Forum({
    required this.id,
    required this.name,
    required this.slug,
    this.description,
    this.iconUrl,
    this.coverImageUrl,
    required this.creator,
    required this.isPublic,
    required this.membersCount,
    required this.topicsCount,
    required this.isMember,
    required this.createdAt,
  });

  factory Forum.fromJson(Map<String, dynamic> json) {
    return Forum(
      id: json['id'],
      name: json['name'],
      slug: json['slug'],
      description: json['description'],
      iconUrl: json['icon_url'],
      coverImageUrl: json['cover_image_url'],
      creator: ForumCreator.fromJson(json['creator']),
      isPublic: json['is_public'],
      membersCount: json['members_count'],
      topicsCount: json['topics_count'],
      isMember: json['is_member'],
      createdAt: DateTime.parse(json['created_at']),
    );
  }
}

class TopicAuthor {
  final String id;
  final String username;
  final String? displayName;

  TopicAuthor({
    required this.id,
    required this.username,
    this.displayName,
  });

  factory TopicAuthor.fromJson(Map<String, dynamic> json) {
    return TopicAuthor(
      id: json['id'],
      username: json['username'],
      displayName: json['display_name'],
    );
  }
}

class Topic {
  final String id;
  final String forumId;
  final TopicAuthor author;
  final String title;
  final String content;
  final bool isPinned;
  final bool isLocked;
  final int viewsCount;
  final int repliesCount;
  final DateTime createdAt;

  Topic({
    required this.id,
    required this.forumId,
    required this.author,
    required this.title,
    required this.content,
    required this.isPinned,
    required this.isLocked,
    required this.viewsCount,
    required this.repliesCount,
    required this.createdAt,
  });

  factory Topic.fromJson(Map<String, dynamic> json) {
    return Topic(
      id: json['id'],
      forumId: json['forum_id'],
      author: TopicAuthor.fromJson(json['author']),
      title: json['title'],
      content: json['content'],
      isPinned: json['is_pinned'],
      isLocked: json['is_locked'],
      viewsCount: json['views_count'],
      repliesCount: json['replies_count'],
      createdAt: DateTime.parse(json['created_at']),
    );
  }
}

class Reply {
  final String id;
  final TopicAuthor author;
  final String content;
  final String? parentReplyId;
  final int likesCount;
  final DateTime createdAt;

  Reply({
    required this.id,
    required this.author,
    required this.content,
    this.parentReplyId,
    required this.likesCount,
    required this.createdAt,
  });

  factory Reply.fromJson(Map<String, dynamic> json) {
    return Reply(
      id: json['id'],
      author: TopicAuthor.fromJson(json['author']),
      content: json['content'],
      parentReplyId: json['parent_reply_id'],
      likesCount: json['likes_count'],
      createdAt: DateTime.parse(json['created_at']),
    );
  }
}
