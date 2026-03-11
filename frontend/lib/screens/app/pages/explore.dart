import 'package:flutter/material.dart';
import 'package:jojuhu/models/post.dart';
import 'package:jojuhu/services/api_service.dart';
import 'post_detail_screen.dart';

class ExploreScreen extends StatefulWidget {
  const ExploreScreen({super.key});

  @override
  State<ExploreScreen> createState() => _ExploreScreenState();
}

class _ExploreScreenState extends State<ExploreScreen> {
  List<Post> _posts = [];
  bool _isLoading = true;
  bool _isLoadingMore = false;
  String? _error;
  int _currentPage = 1;
  bool _hasMore = true;
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _loadFeed();
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

  Future<void> _loadFeed() async {
    setState(() {
      _isLoading = true;
      _error = null;
      _currentPage = 1;
    });

    final result = await ApiService.getFeed(page: 1);

    setState(() {
      _isLoading = false;
      if (result.success && result.data != null) {
        _posts = result.data!;
        _hasMore = result.data!.length >= 20;
      } else {
        _error = result.error ?? 'Failed to load feed';
      }
    });
  }

  Future<void> _loadMore() async {
    setState(() {
      _isLoadingMore = true;
    });

    final result = await ApiService.getFeed(page: _currentPage + 1);

    setState(() {
      _isLoadingMore = false;
      if (result.success && result.data != null) {
        _posts.addAll(result.data!);
        _currentPage++;
        _hasMore = result.data!.length >= 20;
      }
    });
  }

  Future<void> _toggleLike(Post post, int index) async {
    final result = post.isLiked
        ? await ApiService.unlikePost(post.id)
        : await ApiService.likePost(post.id);

    if (result.success && result.data != null) {
      setState(() {
        _posts[index] = Post(
          id: post.id,
          author: post.author,
          content: post.content,
          mediaUrls: post.mediaUrls,
          likesCount: post.isLiked ? post.likesCount - 1 : post.likesCount + 1,
          commentsCount: post.commentsCount,
          sharesCount: post.sharesCount,
          isPublic: post.isPublic,
          createdAt: post.createdAt,
          isLiked: !post.isLiked,
        );
      });
    }
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

  void _createPost() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => _CreatePostSheet(
        onPostCreated: () {
          _loadFeed();
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Explore'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _loadFeed,
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: _loadFeed,
        child: _buildBody(),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _createPost,
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
              onPressed: _loadFeed,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_posts.isEmpty) {
      return const Center(
        child: Text('No posts yet. Be the first to post!'),
      );
    }

    return ListView.builder(
      controller: _scrollController,
      itemCount: _posts.length + (_hasMore ? 1 : 0),
      itemBuilder: (context, index) {
        if (index == _posts.length) {
          return const Center(
            child: Padding(
              padding: EdgeInsets.all(16),
              child: CircularProgressIndicator(),
            ),
          );
        }

        final post = _posts[index];
        return _PostCard(
          post: post,
          onLike: () => _toggleLike(post, index),
          onComment: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => PostDetailScreen(post: post),
              ),
            ).then((_) => _loadFeed());
          },
          formatTime: _formatTime,
        );
      },
    );
  }
}

class _PostCard extends StatelessWidget {
  final Post post;
  final VoidCallback onLike;
  final VoidCallback onComment;
  final String Function(DateTime) formatTime;

  const _PostCard({
    required this.post,
    required this.onLike,
    required this.onComment,
    required this.formatTime,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Author header
          ListTile(
            leading: CircleAvatar(
              backgroundImage: post.author.avatarUrl != null
                  ? NetworkImage(post.author.avatarUrl!)
                  : null,
              child: post.author.avatarUrl == null
                  ? Text(post.author.username[0].toUpperCase())
                  : null,
            ),
            title: Text(
              post.author.displayName ?? post.author.username,
              style: const TextStyle(fontWeight: FontWeight.bold),
            ),
            subtitle: Text('@${post.author.username} · ${formatTime(post.createdAt)}'),
            trailing: IconButton(
              icon: const Icon(Icons.more_vert),
              onPressed: () {},
            ),
          ),

          // Content
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Text(post.content),
          ),

          // Media
          if (post.mediaUrls.isNotEmpty)
            SizedBox(
              height: 200,
              child: ListView.builder(
                scrollDirection: Axis.horizontal,
                itemCount: post.mediaUrls.length,
                itemBuilder: (context, index) {
                  return Padding(
                    padding: const EdgeInsets.all(8),
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(8),
                      child: Image.network(
                        post.mediaUrls[index],
                        height: 200,
                        fit: BoxFit.cover,
                      ),
                    ),
                  );
                },
              ),
            ),

          // Actions
          Padding(
            padding: const EdgeInsets.all(8),
            child: Row(
              children: [
                _ActionButton(
                  icon: post.isLiked ? Icons.favorite : Icons.favorite_border,
                  color: post.isLiked ? Colors.red : null,
                  count: post.likesCount,
                  onTap: onLike,
                ),
                _ActionButton(
                  icon: Icons.comment_outlined,
                  count: post.commentsCount,
                  onTap: onComment,
                ),
                _ActionButton(
                  icon: Icons.share_outlined,
                  count: post.sharesCount,
                  onTap: () {},
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _ActionButton extends StatelessWidget {
  final IconData icon;
  final Color? color;
  final int count;
  final VoidCallback onTap;

  const _ActionButton({
    required this.icon,
    this.color,
    required this.count,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: InkWell(
        onTap: onTap,
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(icon, size: 20, color: color),
              const SizedBox(width: 4),
              Text(
                count.toString(),
                style: TextStyle(color: color),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _CreatePostSheet extends StatefulWidget {
  final VoidCallback onPostCreated;

  const _CreatePostSheet({required this.onPostCreated});

  @override
  State<_CreatePostSheet> createState() => _CreatePostSheetState();
}

class _CreatePostSheetState extends State<_CreatePostSheet> {
  final _controller = TextEditingController();
  bool _isPublic = true;
  bool _isLoading = false;

  Future<void> _submit() async {
    final content = _controller.text.trim();
    if (content.isEmpty) return;

    setState(() {
      _isLoading = true;
    });

    final result = await ApiService.createPost(
      content: content,
      isPublic: _isPublic,
    );

    setState(() {
      _isLoading = false;
    });

    if (result.success) {
      if (mounted) {
        Navigator.pop(context);
        widget.onPostCreated();
      }
    } else {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(result.error ?? 'Failed to create post')),
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
                    : const Text('Post'),
              ),
            ],
          ),
          TextField(
            controller: _controller,
            maxLines: 5,
            decoration: const InputDecoration(
              hintText: 'What\'s on your mind?',
              border: InputBorder.none,
            ),
            autofocus: true,
          ),
          Row(
            children: [
              IconButton(
                icon: const Icon(Icons.image),
                onPressed: () {
                  // TODO: Implement image upload
                },
              ),
              const Spacer(),
              Row(
                children: [
                  const Text('Public'),
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
            ],
          ),
          const SizedBox(height: 16),
        ],
      ),
    );
  }
}
