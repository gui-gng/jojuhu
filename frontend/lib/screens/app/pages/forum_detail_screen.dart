import 'package:flutter/material.dart';
import 'package:jojuhu/models/forum.dart';
import 'package:jojuhu/services/api_service.dart';
import 'topic_detail_screen.dart';

class ForumDetailScreen extends StatefulWidget {
  final Forum forum;

  const ForumDetailScreen({super.key, required this.forum});

  @override
  State<ForumDetailScreen> createState() => _ForumDetailScreenState();
}

class _ForumDetailScreenState extends State<ForumDetailScreen> {
  late Forum _forum;
  List<Topic> _topics = [];
  bool _isLoading = true;
  bool _isLoadingMore = false;
  String? _error;
  int _currentPage = 1;
  bool _hasMore = true;
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _forum = widget.forum;
    _loadTopics();
    _scrollController.addListener(_onScroll);
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 200) {
      if (!_isLoadingMore && _hasMore) {
        _loadMore();
      }
    }
  }

  Future<void> _loadTopics() async {
    setState(() {
      _isLoading = true;
      _error = null;
      _currentPage = 1;
    });

    final result = await ApiService.getTopics(
      forumId: _forum.id,
      page: 1,
    );

    setState(() {
      _isLoading = false;
      if (result.success && result.data != null) {
        _topics = result.data!;
        _hasMore = result.data!.length >= 20;
      } else {
        _error = result.error ?? 'Failed to load topics';
      }
    });
  }

  Future<void> _loadMore() async {
    setState(() {
      _isLoadingMore = true;
    });

    final result = await ApiService.getTopics(
      forumId: _forum.id,
      page: _currentPage + 1,
    );

    setState(() {
      _isLoadingMore = false;
      if (result.success && result.data != null) {
        _topics.addAll(result.data!);
        _currentPage++;
        _hasMore = result.data!.length >= 20;
      }
    });
  }

  Future<void> _toggleMembership() async {
    if (_forum.isMember) {
      final result = await ApiService.leaveForum(_forum.id);
      if (result.success && result.data != null) {
        setState(() {
          _forum = Forum(
            id: _forum.id,
            name: _forum.name,
            slug: _forum.slug,
            description: _forum.description,
            iconUrl: _forum.iconUrl,
            coverImageUrl: _forum.coverImageUrl,
            creator: _forum.creator,
            isPublic: _forum.isPublic,
            membersCount: _forum.membersCount - 1,
            topicsCount: _forum.topicsCount,
            isMember: false,
            createdAt: _forum.createdAt,
          );
        });
      }
    } else {
      final result = await ApiService.joinForum(_forum.id);
      if (result.success && result.data != null) {
        setState(() {
          _forum = Forum(
            id: _forum.id,
            name: _forum.name,
            slug: _forum.slug,
            description: _forum.description,
            iconUrl: _forum.iconUrl,
            coverImageUrl: _forum.coverImageUrl,
            creator: _forum.creator,
            isPublic: _forum.isPublic,
            membersCount: _forum.membersCount + 1,
            topicsCount: _forum.topicsCount,
            isMember: true,
            createdAt: _forum.createdAt,
          );
        });
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

  void _createTopic() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => _CreateTopicSheet(
        forumId: _forum.id,
        onTopicCreated: () {
          _loadTopics();
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: CustomScrollView(
        controller: _scrollController,
        slivers: [
          // App bar with forum info
          SliverAppBar(
            expandedHeight: 200,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              title: Text(_forum.name),
              background: _forum.coverImageUrl != null
                  ? Image.network(
                      _forum.coverImageUrl!,
                      fit: BoxFit.cover,
                    )
                  : Container(
                      color: Theme.of(context).primaryColor.withOpacity(0.2),
                    ),
            ),
            actions: [
              IconButton(
                icon: const Icon(Icons.refresh),
                onPressed: _loadTopics,
              ),
            ],
          ),

          // Forum info
          SliverToBoxAdapter(
            child: _buildForumInfo(),
          ),

          // Topics header
          const SliverToBoxAdapter(
            child: Padding(
              padding: EdgeInsets.all(16),
              child: Text(
                'Topics',
                style: TextStyle(
                  fontWeight: FontWeight.bold,
                  fontSize: 20,
                ),
              ),
            ),
          ),

          // Topics list
          _buildTopicsList(),

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
      floatingActionButton: _forum.isMember
          ? FloatingActionButton(
              onPressed: _createTopic,
              child: const Icon(Icons.add),
            )
          : null,
    );
  }

  Widget _buildForumInfo() {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (_forum.description != null) ...[
            Text(
              _forum.description!,
              style: TextStyle(color: Colors.grey[700]),
            ),
            const SizedBox(height: 16),
          ],
          Row(
            children: [
              Icon(Icons.person, size: 16, color: Colors.grey[600]),
              const SizedBox(width: 4),
              Text(
                'Created by ${_forum.creator.displayName ?? _forum.creator.username}',
                style: TextStyle(color: Colors.grey[600]),
              ),
            ],
          ),
          const SizedBox(height: 8),
          Row(
            children: [
              Icon(Icons.people, size: 16, color: Colors.grey[600]),
              const SizedBox(width: 4),
              Text(
                '${_forum.membersCount} members',
                style: TextStyle(color: Colors.grey[600]),
              ),
              const SizedBox(width: 16),
              Icon(Icons.chat_bubble_outline, size: 16, color: Colors.grey[600]),
              const SizedBox(width: 4),
              Text(
                '${_forum.topicsCount} topics',
                style: TextStyle(color: Colors.grey[600]),
              ),
            ],
          ),
          const SizedBox(height: 16),
          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: _toggleMembership,
              style: ElevatedButton.styleFrom(
                backgroundColor: _forum.isMember
                    ? Colors.grey[200]
                    : Theme.of(context).primaryColor,
                foregroundColor: _forum.isMember
                    ? Colors.black87
                    : Colors.white,
              ),
              child: Text(_forum.isMember ? 'Leave Forum' : 'Join Forum'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTopicsList() {
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
                onPressed: _loadTopics,
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    if (_topics.isEmpty) {
      return const SliverFillRemaining(
        child: Center(
          child: Text('No topics yet. Create one!'),
        ),
      );
    }

    return SliverList(
      delegate: SliverChildBuilderDelegate(
        (context, index) {
          final topic = _topics[index];
          return _TopicCard(
            topic: topic,
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => TopicDetailScreen(
                    forum: _forum,
                    topic: topic,
                  ),
                ),
              ).then((_) => _loadTopics());
            },
            formatTime: _formatTime,
          );
        },
        childCount: _topics.length,
      ),
    );
  }
}

class _TopicCard extends StatelessWidget {
  final Topic topic;
  final VoidCallback onTap;
  final String Function(DateTime) formatTime;

  const _TopicCard({
    required this.topic,
    required this.onTap,
    required this.formatTime,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
      child: InkWell(
        onTap: onTap,
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  if (topic.isPinned)
                    const Padding(
                      padding: EdgeInsets.only(right: 8),
                      child: Icon(Icons.push_pin, size: 16, color: Colors.orange),
                    ),
                  if (topic.isLocked)
                    const Padding(
                      padding: EdgeInsets.only(right: 8),
                      child: Icon(Icons.lock, size: 16, color: Colors.red),
                    ),
                  Expanded(
                    child: Text(
                      topic.title,
                      style: const TextStyle(
                        fontWeight: FontWeight.bold,
                        fontSize: 16,
                      ),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 8),
              Text(
                topic.content,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: TextStyle(color: Colors.grey[700]),
              ),
              const SizedBox(height: 8),
              Row(
                children: [
                  CircleAvatar(
                    radius: 12,
                    child: Text(topic.author.username[0].toUpperCase()),
                  ),
                  const SizedBox(width: 8),
                  Text(
                    topic.author.displayName ?? topic.author.username,
                    style: const TextStyle(fontWeight: FontWeight.w500),
                  ),
                  const SizedBox(width: 8),
                  Text(
                    '· ${formatTime(topic.createdAt)}',
                    style: TextStyle(color: Colors.grey[600], fontSize: 12),
                  ),
                  const Spacer(),
                  Icon(Icons.remove_red_eye_outlined, size: 16, color: Colors.grey[600]),
                  const SizedBox(width: 4),
                  Text(
                    topic.viewsCount.toString(),
                    style: TextStyle(color: Colors.grey[600], fontSize: 12),
                  ),
                  const SizedBox(width: 16),
                  Icon(Icons.chat_bubble_outline, size: 16, color: Colors.grey[600]),
                  const SizedBox(width: 4),
                  Text(
                    topic.repliesCount.toString(),
                    style: TextStyle(color: Colors.grey[600], fontSize: 12),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _CreateTopicSheet extends StatefulWidget {
  final String forumId;
  final VoidCallback onTopicCreated;

  const _CreateTopicSheet({
    required this.forumId,
    required this.onTopicCreated,
  });

  @override
  State<_CreateTopicSheet> createState() => _CreateTopicSheetState();
}

class _CreateTopicSheetState extends State<_CreateTopicSheet> {
  final _titleController = TextEditingController();
  final _contentController = TextEditingController();
  bool _isLoading = false;

  Future<void> _submit() async {
    final title = _titleController.text.trim();
    final content = _contentController.text.trim();

    if (title.isEmpty || content.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please fill in all fields')),
      );
      return;
    }

    setState(() {
      _isLoading = true;
    });

    final result = await ApiService.createTopic(
      forumId: widget.forumId,
      title: title,
      content: content,
    );

    setState(() {
      _isLoading = false;
    });

    if (result.success) {
      if (mounted) {
        Navigator.pop(context);
        widget.onTopicCreated();
      }
    } else {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(result.error ?? 'Failed to create topic')),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.only(
        bottom: MediaQuery.of(context).viewInsets.bottom,
        left: 16,
        right: 16,
        top: 16,
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('Cancel'),
              ),
              ElevatedButton(
                onPressed: _isLoading ? null : _submit,
                child: _isLoading
                    ? const SizedBox(
                        width: 16,
                        height: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      )
                    : const Text('Create'),
              ),
            ],
          ),
          TextField(
            controller: _titleController,
            decoration: const InputDecoration(
              labelText: 'Topic Title',
              hintText: 'Enter a descriptive title',
            ),
            autofocus: true,
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _contentController,
            maxLines: 5,
            decoration: const InputDecoration(
              labelText: 'Content',
              hintText: 'Write your topic content here...',
            ),
          ),
          const SizedBox(height: 16),
        ],
      ),
    );
  }
}
