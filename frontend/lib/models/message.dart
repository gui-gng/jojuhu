import 'user_profile.dart';

class Message {
  final String id;
  final String senderId;
  final String senderUsername;
  final String recipientId;
  final String recipientUsername;
  final String content;
  final bool isRead;
  final DateTime createdAt;

  Message({
    required this.id,
    required this.senderId,
    required this.senderUsername,
    required this.recipientId,
    required this.recipientUsername,
    required this.content,
    required this.isRead,
    required this.createdAt,
  });

  factory Message.fromJson(Map<String, dynamic> json) {
    return Message(
      id: json['id'],
      senderId: json['sender_id'],
      senderUsername: json['sender_username'],
      recipientId: json['recipient_id'],
      recipientUsername: json['recipient_username'],
      content: json['content'],
      isRead: json['is_read'],
      createdAt: DateTime.parse(json['created_at']),
    );
  }
}

class Conversation {
  final String userId;
  final String username;
  final String? displayName;
  final String? avatarUrl;
  final String lastMessage;
  final DateTime lastMessageAt;
  final int unreadCount;

  Conversation({
    required this.userId,
    required this.username,
    this.displayName,
    this.avatarUrl,
    required this.lastMessage,
    required this.lastMessageAt,
    required this.unreadCount,
  });

  factory Conversation.fromJson(Map<String, dynamic> json) {
    return Conversation(
      userId: json['user_id'],
      username: json['username'],
      displayName: json['display_name'],
      avatarUrl: json['avatar_url'],
      lastMessage: json['last_message'],
      lastMessageAt: DateTime.parse(json['last_message_at']),
      unreadCount: json['unread_count'],
    );
  }
}

class ConversationDetail {
  final UserProfile user;
  final List<Message> messages;

  ConversationDetail({
    required this.user,
    required this.messages,
  });

  factory ConversationDetail.fromJson(Map<String, dynamic> json) {
    return ConversationDetail(
      user: UserProfile.fromJson(json['user']),
      messages: (json['messages'] as List)
          .map((m) => Message.fromJson(m))
          .toList(),
    );
  }
}
