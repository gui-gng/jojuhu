import 'package:flutter/material.dart';
import 'package:jojuhu/models/message.dart';
import 'package:jojuhu/services/api_service.dart';
import 'conversation_screen.dart';

class MessagesScreen extends StatefulWidget {
  const MessagesScreen({super.key});

  @override
  State<MessagesScreen> createState() => _MessagesScreenState();
}

class _MessagesScreenState extends State<MessagesScreen> {
  List<Conversation> _conversations = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadConversations();
  }

  Future<void> _loadConversations() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    final result = await ApiService.getConversations();

    setState(() {
      _isLoading = false;
      if (result.success && result.data != null) {
        _conversations = result.data!;
      } else {
        _error = result.error ?? 'Failed to load conversations';
      }
    });
  }

  String _formatTime(DateTime dateTime) {
    final now = DateTime.now();
    final diff = now.difference(dateTime);

    if (diff.inDays > 365) {
      return '${(diff.inDays / 365).floor()}y';
    } else if (diff.inDays > 30) {
      return '${(diff.inDays / 30).floor()}mo';
    } else if (diff.inDays > 0) {
      return '${diff.inDays}d';
    } else if (diff.inHours > 0) {
      return '${diff.inHours}h';
    } else if (diff.inMinutes > 0) {
      return '${diff.inMinutes}m';
    } else {
      return 'now';
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Messages'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _loadConversations,
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: _loadConversations,
        child: _buildBody(),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // TODO: Implement new message functionality
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('New message - coming soon')),
          );
        },
        child: const Icon(Icons.edit),
      ),
    );
  }

  Widget _buildBody() {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_error != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(_error!, style: const TextStyle(color: Colors.red)),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadConversations,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_conversations.isEmpty) {
      return const Center(
        child: Text('No conversations yet'),
      );
    }

    return ListView.builder(
      itemCount: _conversations.length,
      itemBuilder: (context, index) {
        final conversation = _conversations[index];
        return _ConversationTile(
          conversation: conversation,
          onTap: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => ConversationScreen(
                  userId: conversation.userId,
                  username: conversation.displayName ?? conversation.username,
                ),
              ),
            ).then((_) => _loadConversations());
          },
          formatTime: _formatTime,
        );
      },
    );
  }
}

class _ConversationTile extends StatelessWidget {
  final Conversation conversation;
  final VoidCallback onTap;
  final String Function(DateTime) formatTime;

  const _ConversationTile({
    required this.conversation,
    required this.onTap,
    required this.formatTime,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final hasUnread = conversation.unreadCount > 0;

    return ListTile(
      leading: CircleAvatar(
        backgroundImage: conversation.avatarUrl != null
            ? NetworkImage(conversation.avatarUrl!)
            : null,
        child: conversation.avatarUrl == null
            ? Text(conversation.username[0].toUpperCase())
            : null,
      ),
      title: Row(
        children: [
          Expanded(
            child: Text(
              conversation.displayName ?? conversation.username,
              style: TextStyle(
                fontWeight: hasUnread ? FontWeight.bold : FontWeight.normal,
              ),
            ),
          ),
          Text(
            formatTime(conversation.lastMessageAt),
            style: theme.textTheme.bodySmall?.copyWith(
              color: hasUnread ? theme.primaryColor : Colors.grey,
            ),
          ),
        ],
      ),
      subtitle: Row(
        children: [
          Expanded(
            child: Text(
              conversation.lastMessage,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(
                fontWeight: hasUnread ? FontWeight.bold : FontWeight.normal,
                color: hasUnread ? Colors.black87 : Colors.grey[600],
              ),
            ),
          ),
          if (hasUnread) ...[
            const SizedBox(width: 8),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
              decoration: BoxDecoration(
                color: theme.primaryColor,
                borderRadius: BorderRadius.circular(12),
              ),
              child: Text(
                conversation.unreadCount.toString(),
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
          ],
        ],
      ),
      onTap: onTap,
    );
  }
}
