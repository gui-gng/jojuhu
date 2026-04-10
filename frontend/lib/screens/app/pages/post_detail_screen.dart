import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:jojuhu/models/post.dart';
import 'package:jojuhu/services/api_service.dart';
import 'package:jojuhu/theme/jojuhu_theme.dart';

class PostDetailScreen extends StatefulWidget {
  final Post post;

  const PostDetailScreen({super.key, required this.post});

  @override
  State<PostDetailScreen> createState() => _PostDetailScreenState();
}

class _PostDetailScreenState extends State<PostDetailScreen> {
  late Post _post;
  List<Comment> _comments = [];
  bool _isLoading = true;
  bool _isLoadingMore = false;
  String? _error;
  int _currentPage = 1;
  bool _hasMore = true;
  final TextEditingController _commentController = TextEditingController();
  final ScrollController _scrollController = ScrollController();
  bool _isSubmitting = false;

  @override
  void initState() {
    super.initState();
    _post = widget.post;
    _loadComments();
    _scrollController.addListener(_onScroll);
  }

  @override
  void dispose() {
    _commentController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 200) {
      if (!_isLoadingMore && _hasMore) {
        _loadMoreComments();
      }
    }
  }

  Future<void> _loadComments() async {
    setState(() {
      _isLoading = true;
      _error = null;
      _currentPage = 1;
    });

    final result = await ApiService.getComments(
      postId: _post.id,
      page: 1,
    );

    setState(() {
      _isLoading = false;
      if (result.success && result.data != null) {
        _comments = result.data!;
        _hasMore = result.data!.length >= 20;
      } else {
        _error = result.error ?? 'Failed to load comments';
      }
    });
  }

  Future<void> _loadMoreComments() async {
    setState(() {
      _isLoadingMore = true;
    });

    final result = await ApiService.getComments(
      postId: _post.id,
      page: _currentPage + 1,
    );

    setState(() {
      _isLoadingMore = false;
      if (result.success && result.data != null) {
        _comments.addAll(result.data!);
        _currentPage++;
        _hasMore = result.data!.length >= 20;
      }
    });
  }

  Future<void> _toggleLike() async {
    final result = _post.isLiked
        ? await ApiService.unlikePost(_post.id)
        : await ApiService.likePost(_post.id);

    if (result.success && result.data != null) {
      setState(() {
        _post = Post(
          id: _post.id,
          author: _post.author,
          content: _post.content,
          mediaUrls: _post.mediaUrls,
          likesCount: _post.isLiked ? _post.likesCount - 1 : _post.likesCount + 1,
          commentsCount: _post.commentsCount,
          sharesCount: _post.sharesCount,
          isPublic: _post.isPublic,
          createdAt: _post.createdAt,
          isLiked: !_post.isLiked,
        );
      });
    }
  }

  Future<void> _submitComment() async {
    final content = _commentController.text.trim();
    if (content.isEmpty) return;

    setState(() {
      _isSubmitting = true;
    });

    final result = await ApiService.addComment(
      postId: _post.id,
      content: content,
    );

    setState(() {
      _isSubmitting = false;
    });

    if (result.success && result.data != null) {
      _commentController.clear();
      setState(() {
        _comments.insert(0, result.data!);
        _post = Post(
          id: _post.id,
          author: _post.author,
          content: _post.content,
          mediaUrls: _post.mediaUrls,
          likesCount: _post.likesCount,
          commentsCount: _post.commentsCount + 1,
          sharesCount: _post.sharesCount,
          isPublic: _post.isPublic,
          createdAt: _post.createdAt,
          isLiked: _post.isLiked,
        );
      });
    } else {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(result.error ?? 'Failed to add comment')),
        );
      }
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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Post'),
      ),
      body: Column(
        children: [
          Expanded(
            child: CustomScrollView(
              controller: _scrollController,
              slivers: [
                // Post content
                SliverToBoxAdapter(
                  child: _buildPostHeader(),
                ),

                // Comments header
                SliverToBoxAdapter(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Text(
                      'Comments (${_post.commentsCount})',
                      style: const TextStyle(
                        fontWeight: FontWeight.bold,
                        fontSize: 18,
                      ),
                    ),
                  ),
                ),

                // Comments list
                _buildCommentsList(),

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

          // Comment input
          _buildCommentInput(),
        ],
      ),
    );
  }

  Widget _buildPostHeader() {
    return Card(
      margin: const EdgeInsets.all(8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ListTile(
            leading: CircleAvatar(
              backgroundImage: _post.author.avatarUrl != null
                  ? NetworkImage(_post.author.avatarUrl!)
                  : null,
              child: _post.author.avatarUrl == null
                  ? Text(_post.author.username[0].toUpperCase())
                  : null,
            ),
            title: Text(
              _post.author.displayName ?? _post.author.username,
              style: const TextStyle(fontWeight: FontWeight.bold),
            ),
            subtitle: Text('@${_post.author.username} · ${_formatTime(_post.createdAt)}'),
          ),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Text(
              _post.content,
              style: const TextStyle(fontSize: 16),
            ),
          ),
          if (_post.mediaUrls.isNotEmpty)
            SizedBox(
              height: 200,
              child: ListView.builder(
                scrollDirection: Axis.horizontal,
                itemCount: _post.mediaUrls.length,
                itemBuilder: (context, index) {
                  return Padding(
                    padding: const EdgeInsets.all(8),
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(8),
                      child: CachedNetworkImage(
                        imageUrl: _post.mediaUrls[index],
                        height: 200,
                        fit: BoxFit.cover,
                        placeholder: (context, url) => Container(
                          height: 200,
                          color: JojuhuColors.fundo,
                          child: const Center(
                            child: CircularProgressIndicator(
                              color: JojuhuColors.sol,
                            ),
                          ),
                        ),
                        errorWidget: (context, url, error) => Container(
                          height: 200,
                          color: JojuhuColors.fundo,
                          child: const Icon(Icons.image_not_supported),
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),
          Padding(
            padding: const EdgeInsets.all(8),
            child: Row(
              children: [
                _ActionButton(
                  icon: _post.isLiked ? Icons.favorite : Icons.favorite_border,
                  color: _post.isLiked ? Colors.red : null,
                  count: _post.likesCount,
                  onTap: _toggleLike,
                ),
                _ActionButton(
                  icon: Icons.comment_outlined,
                  count: _post.commentsCount,
                  onTap: () {},
                ),
                _ActionButton(
                  icon: Icons.share_outlined,
                  count: _post.sharesCount,
                  onTap: () {},
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCommentsList() {
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
                onPressed: _loadComments,
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    if (_comments.isEmpty) {
      return const SliverFillRemaining(
        child: Center(
          child: Text('No comments yet. Be the first to comment!'),
        ),
      );
    }

    return SliverList(
      delegate: SliverChildBuilderDelegate(
        (context, index) {
          final comment = _comments[index];
          return _CommentCard(
            comment: comment,
            formatTime: _formatTime,
          );
        },
        childCount: _comments.length,
      ),
    );
  }

  Widget _buildCommentInput() {
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
                controller: _commentController,
                decoration: const InputDecoration(
                  hintText: 'Add a comment...',
                  border: InputBorder.none,
                ),
                textInputAction: TextInputAction.send,
                onSubmitted: (_) => _submitComment(),
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
              onPressed: _isSubmitting ? null : _submitComment,
            ),
          ],
        ),
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

class _CommentCard extends StatelessWidget {
  final Comment comment;
  final String Function(DateTime) formatTime;

  const _CommentCard({
    required this.comment,
    required this.formatTime,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          CircleAvatar(
            radius: 20,
            backgroundImage: comment.author.avatarUrl != null
                ? NetworkImage(comment.author.avatarUrl!)
                : null,
            child: comment.author.avatarUrl == null
                ? Text(comment.author.username[0].toUpperCase())
                : null,
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Text(
                      comment.author.displayName ?? comment.author.username,
                      style: const TextStyle(fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(width: 8),
                    Text(
                      '@${comment.author.username}',
                      style: TextStyle(color: Colors.grey[600], fontSize: 12),
                    ),
                    const SizedBox(width: 8),
                    Text(
                      formatTime(comment.createdAt),
                      style: TextStyle(color: Colors.grey[600], fontSize: 12),
                    ),
                  ],
                ),
                const SizedBox(height: 4),
                Text(comment.content),
                const SizedBox(height: 4),
                Row(
                  children: [
                    Icon(Icons.favorite_border, size: 16, color: Colors.grey[600]),
                    const SizedBox(width: 4),
                    Text(
                      comment.likesCount.toString(),
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
        ],
      ),
    );
  }
}
