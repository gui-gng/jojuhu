import 'package:flutter/material.dart';
import 'package:jojuhu/models/forum.dart';
import 'package:jojuhu/services/api_service.dart';

class TopicDetailScreen extends StatefulWidget {
  final Forum forum;
  final Topic topic;

  const TopicDetailScreen({
    super.key,
    required this.forum,
    required this.topic,
  });

  @override
  State<TopicDetailScreen> createState() => _TopicDetailScreenState();
}

class _TopicDetailScreenState extends State<TopicDetailScreen> {
  late Topic _topic;
  List<Reply> _replies = [];
  bool _isLoading = true;
  bool _isLoadingMore = false;
  String? _error;
  int _currentPage = 1;
  bool _hasMore = true;
  final TextEditingController _replyController = TextEditingController();
  final ScrollController _scrollController = ScrollController();
  bool _isSubmitting = false;

  @override
  void initState() {
    super.initState();
    _topic = widget.topic;
    _loadReplies();
    _scrollController.addListener(_onScroll);
  }

  @override
  void dispose() {
    _replyController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 200) {
      if (!_isLoadingMore && _hasMore) {
        _loadMoreReplies();
      }
    }
  }

  Future<void> _loadReplies() async {
    setState(() {
      _isLoading = true;
      _error = null;
      _currentPage = 1;
    });

    final result = await ApiService.getReplies(
      forumId: widget.forum.id,
      topicId: _topic.id,
      page: 1,
    );

    setState(() {
      _isLoading = false;
      if (result.success && result.data != null) {
        _replies = result.data!;
        _hasMore = result.data!.length >= 20;
      } else {
        _error = result.error ?? 'Failed to load replies';
      }
    });
  }

  Future<void> _loadMoreReplies() async {
    setState(() {
      _isLoadingMore = true;
    });

    final result = await ApiService.getReplies(
      forumId: widget.forum.id,
      topicId: _topic.id,
      page: _currentPage + 1,
    );

    setState(() {
      _isLoadingMore = false;
      if (result.success && result.data != null) {
        _replies.addAll(result.data!);
        _currentPage++;
        _hasMore = result.data!.length >= 20;
      }
    });
  }

  Future<void> _submitReply() async {
    if (_topic.isLocked) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('This topic is locked')),
      );
      return;
    }

    final content = _replyController.text.trim();
    if (content.isEmpty) return;

    setState(() {
      _isSubmitting = true;
    });

    final result = await ApiService.createReply(
      forumId: widget.forum.id,
      topicId: _topic.id,
      content: content,
    );

    setState(() {
      _isSubmitting = false;
    });

    if (result.success && result.data != null) {
      _replyController.clear();
      setState(() {
        _replies.add(result.data!);
        _topic = Topic(
          id: _topic.id,
          forumId: _topic.forumId,
          author: _topic.author,
          title: _topic.title,
          content: _topic.content,
          isPinned: _topic.isPinned,
          isLocked: _topic.isLocked,
          viewsCount: _topic.viewsCount,
          repliesCount: _topic.repliesCount + 1,
          createdAt: _topic.createdAt,
        );
      });
    } else {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(result.error ?? 'Failed to add reply')),
        );
      }
    }
  }

  String _formatTime(DateTime dateTime) {
    final now = DateTime.now();
    final diff = now.difference(dateTime);

    if (diff.inDays > 365) {
      return '${(diff.inDays / 365).floor()}y ago';
    } else if (diff.inDays > 30) {
      return '${(diff.inDays / 30).floor()}mo ago';
    } else if (diff.inDays > 0) {
      return '${diff.inDays}d ago';
    } else if (diff.inHours > 0) {
      return '${diff.inHours}h ago';
    } else if (diff.inMinutes > 0) {
      return '${diff.inMinutes}m ago';
    } else {
      return 'just now';
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Topic'),
      ),
      body: Column(
        children: [
          Expanded(
            child: CustomScrollView(
              controller: _scrollController,
              slivers: [
                // Topic content
                SliverToBoxAdapter(
                  child: _buildTopicHeader(),
                ),

                // Replies header
                SliverToBoxAdapter(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Text(
                      '${_topic.repliesCount} Replies',
                      style: const TextStyle(
                        fontWeight: FontWeight.bold,
                        fontSize: 18,
                      ),
                    ),
                  ),
                ),

                // Replies list
                _buildRepliesList(),

                // Loading indicator
                if (_isLoadingMore)
                  const SliverToBoxAdapter(
                    child: Center(
                      child: Padding(
                        padding: EdgeInsets.all(16),
                        child: CircularProgressIndicator(),
                      ),
                    ),
                  ),
              ],
            ),
          ),

          // Reply input (if not locked and user is member)
          if (!widget.topic.isLocked && widget.forum.isMember)
            _buildReplyInput(),
        ],
      ),
    );
  }

  Widget _buildTopicHeader() {
    return Card(
      margin: const EdgeInsets.all(8),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                if (_topic.isPinned)
                  const Padding(
                    padding: EdgeInsets.only(right: 8),
                    child: Icon(Icons.push_pin, size: 20, color: Colors.orange),
                  ),
                if (_topic.isLocked)
                  const Padding(
                    padding: EdgeInsets.only(right: 8),
                    child: Icon(Icons.lock, size: 20, color: Colors.red),
                  ),
                Expanded(
                  child: Text(
                    _topic.title,
                    style: const TextStyle(
                      fontWeight: FontWeight.bold,
                      fontSize: 20,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Row(
              children: [
                CircleAvatar(
                  radius: 20,
                  child: Text(_topic.author.username[0].toUpperCase()),
                ),
                const SizedBox(width: 12),
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      _topic.author.displayName ?? _topic.author.username,
                      style: const TextStyle(fontWeight: FontWeight.bold),
                    ),
                    Text(
                      '@${_topic.author.username} · ${_formatTime(_topic.createdAt)}',
                      style: TextStyle(color: Colors.grey[600]),
                    ),
                  ],
                ),
              ],
            ),
            const SizedBox(height: 16),
            Text(
              _topic.content,
              style: const TextStyle(fontSize: 16),
            ),
            const SizedBox(height: 16),
            Row(
              children: [
                Icon(Icons.remove_red_eye_outlined, size: 16, color: Colors.grey[600]),
                const SizedBox(width: 4),
                Text(
                  '${_topic.viewsCount} views',
                  style: TextStyle(color: Colors.grey[600]),
                ),
                const SizedBox(width: 16),
                Icon(Icons.chat_bubble_outline, size: 16, color: Colors.grey[600]),
                const SizedBox(width: 4),
                Text(
                  '${_topic.repliesCount} replies',
                  style: TextStyle(color: Colors.grey[600]),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildRepliesList() {
    if (_isLoading) {
      return const SliverFillRemaining(
        child: Center(child: CircularProgressIndicator()),
      );
    }

    if (_error != null) {
      return SliverFillRemaining(
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(_error!, style: const TextStyle(color: Colors.red)),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: _loadReplies,
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    if (_replies.isEmpty) {
      return const SliverFillRemaining(
        child: Center(
          child: Text('No replies yet. Be the first to reply!'),
        ),
      );
    }

    return SliverList(
      delegate: SliverChildBuilderDelegate(
        (context, index) {
          final reply = _replies[index];
          return _ReplyCard(
            reply: reply,
            formatTime: _formatTime,
          );
        },
        childCount: _replies.length,
      ),
    );
  }

  Widget _buildReplyInput() {
    return Container(
      padding: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        color: Colors.white,
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.05),
            blurRadius: 4,
            offset: const Offset(0, -2),
          ),
        ],
      ),
      child: SafeArea(
        child: Row(
          children: [
            Expanded(
              child: TextField(
                controller: _replyController,
                decoration: const InputDecoration(
                  hintText: 'Write a reply...',
                  border: InputBorder.none,
                ),
                textInputAction: TextInputAction.send,
                onSubmitted: (_) => _submitReply(),
              ),
            ),
            IconButton(
              icon: _isSubmitting
                  ? const SizedBox(
                      width: 20,
                      height: 20,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Icon(Icons.send),
              onPressed: _isSubmitting ? null : _submitReply,
            ),
          ],
        ),
      ),
    );
  }
}

class _ReplyCard extends StatelessWidget {
  final Reply reply;
  final String Function(DateTime) formatTime;

  const _ReplyCard({
    required this.reply,
    required this.formatTime,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                CircleAvatar(
                  radius: 16,
                  child: Text(reply.author.username[0].toUpperCase()),
                ),
                const SizedBox(width: 12),
                Text(
                  reply.author.displayName ?? reply.author.username,
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
                const SizedBox(width: 8),
                Text(
                  '@${reply.author.username}',
                  style: TextStyle(color: Colors.grey[600], fontSize: 12),
                ),
                const SizedBox(width: 8),
                Text(
                  '· ${formatTime(reply.createdAt)}',
                  style: TextStyle(color: Colors.grey[600], fontSize: 12),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(reply.content),
            const SizedBox(height: 8),
            Row(
              children: [
                Icon(Icons.favorite_border, size: 16, color: Colors.grey[600]),
                const SizedBox(width: 4),
                Text(
                  reply.likesCount.toString(),
                  style: TextStyle(color: Colors.grey[600], fontSize: 12),
                ),
                const SizedBox(width: 16),
                Icon(Icons.reply, size: 16, color: Colors.grey[600]),
                const SizedBox(width: 4),
                Text(
                  'Reply',
                  style: TextStyle(color: Colors.grey[600], fontSize: 12),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
