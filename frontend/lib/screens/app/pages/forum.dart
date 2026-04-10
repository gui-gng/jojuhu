import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:jojuhu/models/forum.dart';
import 'package:jojuhu/services/api_service.dart';
import 'package:jojuhu/theme/jojuhu_theme.dart';
import 'forum_detail_screen.dart';

class ForumScreen extends StatefulWidget {
  const ForumScreen({super.key});

  @override
  State<ForumScreen> createState() => _ForumScreenState();
}

class _ForumScreenState extends State<ForumScreen> {
  List<Forum> _forums = [];
  bool _isLoading = true;
  bool _isLoadingMore = false;
  String? _error;
  int _currentPage = 1;
  bool _hasMore = true;
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _loadForums();
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

  Future<void> _loadForums() async {
    setState(() {
      _isLoading = true;
      _error = null;
      _currentPage = 1;
    });

    final result = await ApiService.getForums(page: 1);

    setState(() {
      _isLoading = false;
      if (result.success && result.data != null) {
        _forums = result.data!;
        _hasMore = result.data!.length >= 20;
      } else {
        _error = result.error ?? 'Failed to load forums';
      }
    });
  }

  Future<void> _loadMore() async {
    setState(() {
      _isLoadingMore = true;
    });

    final result = await ApiService.getForums(page: _currentPage + 1);

    setState(() {
      _isLoadingMore = false;
      if (result.success && result.data != null) {
        _forums.addAll(result.data!);
        _currentPage++;
        _hasMore = result.data!.length >= 20;
      }
    });
  }

  Future<void> _joinForum(Forum forum, int index) async {
    final result = await ApiService.joinForum(forum.id);

    if (result.success && result.data != null) {
      setState(() {
        _forums[index] = Forum(
          id: forum.id,
          name: forum.name,
          slug: forum.slug,
          description: forum.description,
          iconUrl: forum.iconUrl,
          coverImageUrl: forum.coverImageUrl,
          creator: forum.creator,
          isPublic: forum.isPublic,
          membersCount: forum.membersCount + 1,
          topicsCount: forum.topicsCount,
          isMember: true,
          createdAt: forum.createdAt,
        );
      });
    }
  }

  Future<void> _leaveForum(Forum forum, int index) async {
    final result = await ApiService.leaveForum(forum.id);

    if (result.success && result.data != null) {
      setState(() {
        _forums[index] = Forum(
          id: forum.id,
          name: forum.name,
          slug: forum.slug,
          description: forum.description,
          iconUrl: forum.iconUrl,
          coverImageUrl: forum.coverImageUrl,
          creator: forum.creator,
          isPublic: forum.isPublic,
          membersCount: forum.membersCount - 1,
          topicsCount: forum.topicsCount,
          isMember: false,
          createdAt: forum.createdAt,
        );
      });
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

  void _createForum() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => _CreateForumSheet(
        onForumCreated: () {
          _loadForums();
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Forums'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _loadForums,
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: _loadForums,
        child: _buildBody(),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _createForum,
        child: const Icon(Icons.add),
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
              onPressed: _loadForums,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_forums.isEmpty) {
      return const Center(
        child: Text('No forums yet. Create one!'),
      );
    }

    return ListView.builder(
      controller: _scrollController,
      itemCount: _forums.length + (_hasMore ? 1 : 0),
      itemBuilder: (context, index) {
        if (index == _forums.length) {
          return const Center(
            child: Padding(
              padding: EdgeInsets.all(16),
              child: CircularProgressIndicator(),
            ),
          );
        }

        final forum = _forums[index];
        return _ForumCard(
          forum: forum,
          onTap: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => ForumDetailScreen(forum: forum),
              ),
            ).then((_) => _loadForums());
          },
          onJoin: () => _joinForum(forum, index),
          onLeave: () => _leaveForum(forum, index),
          formatTime: _formatTime,
        );
      },
    );
  }
}

class _ForumCard extends StatelessWidget {
  final Forum forum;
  final VoidCallback onTap;
  final VoidCallback onJoin;
  final VoidCallback onLeave;
  final String Function(DateTime) formatTime;

  const _ForumCard({
    required this.forum,
    required this.onTap,
    required this.onJoin,
    required this.onLeave,
    required this.formatTime,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      clipBehavior: Clip.antiAlias,
      child: InkWell(
        onTap: onTap,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Cover image
            if (forum.coverImageUrl != null)
              CachedNetworkImage(
                imageUrl: forum.coverImageUrl!,
                height: 120,
                width: double.infinity,
                fit: BoxFit.cover,
                placeholder: (context, url) => Container(
                  height: 120,
                  color: JojuhuColors.fundo,
                ),
                errorWidget: (context, url, error) => Container(
                  height: 120,
                  color: JojuhuColors.fundo,
                  child: const Icon(Icons.image_not_supported),
                ),
              ),

            Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Icon
                  if (forum.iconUrl != null)
                    ClipRRect(
                      borderRadius: BorderRadius.circular(8),
                      child: CachedNetworkImage(
                        imageUrl: forum.iconUrl!,
                        width: 60,
                        height: 60,
                        fit: BoxFit.cover,
                        placeholder: (context, url) => Container(
                          width: 60,
                          height: 60,
                          color: JojuhuColors.fundo,
                        ),
                        errorWidget: (context, url, error) => Container(
                          width: 60,
                          height: 60,
                          color: JojuhuColors.fundo,
                          child: const Icon(Icons.forum, size: 30),
                        ),
                      ),
                    )
                  else
                    Container(
                      width: 60,
                      height: 60,
                      decoration: BoxDecoration(
                        color: Theme.of(context).primaryColor.withOpacity(0.1),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Icon(
                        Icons.forum,
                        size: 30,
                        color: Theme.of(context).primaryColor,
                      ),
                    ),

                  const SizedBox(width: 16),

                  // Info
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          forum.name,
                          style: const TextStyle(
                            fontWeight: FontWeight.bold,
                            fontSize: 18,
                          ),
                        ),
                        if (forum.description != null) ...[
                          const SizedBox(height: 4),
                          Text(
                            forum.description!,
                            maxLines: 2,
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(color: Colors.grey[600]),
                          ),
                        ],
                        const SizedBox(height: 8),
                        Row(
                          children: [
                            Icon(Icons.people, size: 16, color: Colors.grey[600]),
                            const SizedBox(width: 4),
                            Text(
                              '${forum.membersCount} members',
                              style: TextStyle(color: Colors.grey[600], fontSize: 12),
                            ),
                            const SizedBox(width: 16),
                            Icon(Icons.chat_bubble_outline, size: 16, color: Colors.grey[600]),
                            const SizedBox(width: 4),
                            Text(
                              '${forum.topicsCount} topics',
                              style: TextStyle(color: Colors.grey[600], fontSize: 12),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),

                  // Join/Leave button
                  ElevatedButton(
                    onPressed: forum.isMember ? onLeave : onJoin,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: forum.isMember
                          ? Colors.grey[200]
                          : Theme.of(context).primaryColor,
                      foregroundColor: forum.isMember
                          ? Colors.black87
                          : Colors.white,
                      elevation: 0,
                    ),
                    child: Text(forum.isMember ? 'Joined' : 'Join'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _CreateForumSheet extends StatefulWidget {
  final VoidCallback onForumCreated;

  const _CreateForumSheet({required this.onForumCreated});

  @override
  State<_CreateForumSheet> createState() => _CreateForumSheetState();
}

class _CreateForumSheetState extends State<_CreateForumSheet> {
  final _nameController = TextEditingController();
  final _descriptionController = TextEditingController();
  bool _isPublic = true;
  bool _isLoading = false;

  Future<void> _submit() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) return;

    setState(() {
      _isLoading = true;
    });

    final result = await ApiService.createForum(
      name: name,
      description: _descriptionController.text.trim(),
      isPublic: _isPublic,
    );

    setState(() {
      _isLoading = false;
    });

    if (result.success) {
      if (mounted) {
        Navigator.pop(context);
        widget.onForumCreated();
      }
    } else {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(result.error ?? 'Failed to create forum')),
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
            controller: _nameController,
            decoration: const InputDecoration(
              labelText: 'Forum Name',
              hintText: 'Enter forum name',
            ),
            autofocus: true,
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _descriptionController,
            maxLines: 3,
            decoration: const InputDecoration(
              labelText: 'Description (Optional)',
              hintText: 'Describe your forum',
            ),
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              const Text('Public Forum'),
              const Spacer(),
              Switch(
                value: _isPublic,
                onChanged: (value) {
                  setState(() {
                    _isPublic = value;
                  });
                },
              ),
            ],
          ),
          const SizedBox(height: 16),
        ],
      ),
    );
  }
}
