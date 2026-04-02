import 'package:flutter/material.dart';
import 'package:jojuhu/models/post.dart';
import 'package:jojuhu/services/api_service.dart';
import 'package:jojuhu/theme/jojuhu_theme.dart';
import 'package:jojuhu/widgets/skeleton_loaders.dart';
import 'post_detail_screen.dart';

class ExploreScreen extends StatefulWidget {
  const ExploreScreen({super.key});

  @override
  State<ExploreScreen> createState() => _ExploreScreenState();
}

class _ExploreScreenState extends State<ExploreScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  
  // For You feed (public/explore)
  List<Post> _forYouPosts = [];
  bool _isLoadingForYou = true;
  bool _isLoadingMoreForYou = false;
  String? _errorForYou;
  int _currentPageForYou = 1;
  bool _hasMoreForYou = true;
  
  // Following feed (personalized)
  List<Post> _followingPosts = [];
  bool _isLoadingFollowing = true;
  bool _isLoadingMoreFollowing = false;
  String? _errorFollowing;
  int _currentPageFollowing = 1;
  bool _hasMoreFollowing = true;
  
  final ScrollController _forYouScrollController = ScrollController();
  final ScrollController _followingScrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 2, vsync: this);
    _tabController.addListener(_onTabChanged);
    _loadForYouFeed();
    _forYouScrollController.addListener(_onForYouScroll);
    _followingScrollController.addListener(_onFollowingScroll);
  }

  @override
  void dispose() {
    _tabController.dispose();
    _forYouScrollController.dispose();
    _followingScrollController.dispose();
    super.dispose();
  }

  void _onTabChanged() {
    if (_tabController.index == 1 && _followingPosts.isEmpty) {
      _loadFollowingFeed();
    }
  }

  void _onForYouScroll() {
    if (_forYouScrollController.position.pixels >=
        _forYouScrollController.position.maxScrollExtent - 200) {
      if (!_isLoadingMoreForYou && _hasMoreForYou) {
        _loadMoreForYou();
      }
    }
  }

  void _onFollowingScroll() {
    if (_followingScrollController.position.pixels >=
        _followingScrollController.position.maxScrollExtent - 200) {
      if (!_isLoadingMoreFollowing && _hasMoreFollowing) {
        _loadMoreFollowing();
      }
    }
  }

  Future<void> _loadForYouFeed() async {
    setState(() {
      _isLoadingForYou = true;
      _errorForYou = null;
      _currentPageForYou = 1;
    });

    final result = await ApiService.getFeed(page: 1);

    setState(() {
      _isLoadingForYou = false;
      if (result.success && result.data != null) {
        _forYouPosts = result.data!;
        _hasMoreForYou = result.data!.length >= 20;
      } else {
        _errorForYou = result.error ?? 'Failed to load feed';
      }
    });
  }

  Future<void> _loadMoreForYou() async {
    setState(() {
      _isLoadingMoreForYou = true;
    });

    final result = await ApiService.getFeed(page: _currentPageForYou + 1);

    setState(() {
      _isLoadingMoreForYou = false;
      if (result.success && result.data != null) {
        _forYouPosts.addAll(result.data!);
        _currentPageForYou++;
        _hasMoreForYou = result.data!.length >= 20;
      }
    });
  }

  Future<void> _loadFollowingFeed() async {
    setState(() {
      _isLoadingFollowing = true;
      _errorFollowing = null;
      _currentPageFollowing = 1;
    });

    final result = await ApiService.getFollowingFeed(page: 1);

    setState(() {
      _isLoadingFollowing = false;
      if (result.success && result.data != null) {
        _followingPosts = result.data!;
        _hasMoreFollowing = result.data!.length >= 20;
      } else {
        _errorFollowing = result.error ?? 'Failed to load feed';
      }
    });
  }

  Future<void> _loadMoreFollowing() async {
    setState(() {
      _isLoadingMoreFollowing = true;
    });

    final result = await ApiService.getFollowingFeed(page: _currentPageFollowing + 1);

    setState(() {
      _isLoadingMoreFollowing = false;
      if (result.success && result.data != null) {
        _followingPosts.addAll(result.data!);
        _currentPageFollowing++;
        _hasMoreFollowing = result.data!.length >= 20;
      }
    });
  }

  Future<void> _toggleLike(Post post, int index, bool isForYou) async {
    final result = post.isLiked
        ? await ApiService.unlikePost(post.id)
        : await ApiService.likePost(post.id);

    if (result.success && result.data != null) {
      setState(() {
        final updatedPost = Post(
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
        
        if (isForYou) {
          _forYouPosts[index] = updatedPost;
        } else {
          _followingPosts[index] = updatedPost;
        }
      });
    }
  }

  Future<void> _deletePost(String postId, bool isForYou) async {
    final result = await ApiService.deletePost(postId);
    
    if (result.success) {
      setState(() {
        if (isForYou) {
          _forYouPosts.removeWhere((post) => post.id == postId);
        } else {
          _followingPosts.removeWhere((post) => post.id == postId);
        }
      });
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Post deleted')),
      );
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(result.error ?? 'Failed to delete post')),
      );
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
          _loadForYouFeed();
          if (_tabController.index == 1) {
            _loadFollowingFeed();
          }
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Jojuhu'),
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: 'For You'),
            Tab(text: 'Following'),
          ],
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () {
              if (_tabController.index == 0) {
                _loadForYouFeed();
              } else {
                _loadFollowingFeed();
              }
            },
          ),
        ],
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          _buildForYouTab(),
          _buildFollowingTab(),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _createPost,
        child: const Icon(Icons.add),
      ),
    );
  }

  Widget _buildForYouTab() {
    return RefreshIndicator(
      onRefresh: _loadForYouFeed,
      child: _buildFeedList(
        posts: _forYouPosts,
        isLoading: _isLoadingForYou,
        isLoadingMore: _isLoadingMoreForYou,
        error: _errorForYou,
        hasMore: _hasMoreForYou,
        scrollController: _forYouScrollController,
        isForYou: true,
      ),
    );
  }

  Widget _buildFollowingTab() {
    return RefreshIndicator(
      onRefresh: _loadFollowingFeed,
      child: _buildFeedList(
        posts: _followingPosts,
        isLoading: _isLoadingFollowing,
        isLoadingMore: _isLoadingMoreFollowing,
        error: _errorFollowing,
        hasMore: _hasMoreFollowing,
        scrollController: _followingScrollController,
        isForYou: false,
      ),
    );
  }

  Widget _buildFeedList({
    required List<Post> posts,
    required bool isLoading,
    required bool isLoadingMore,
    required String? error,
    required bool hasMore,
    required ScrollController scrollController,
    required bool isForYou,
  }) {
    if (isLoading) {
      return const FeedSkeletonList(itemCount: 5);
    }

    if (error != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(error, style: const TextStyle(color: Colors.red)),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: isForYou ? _loadForYouFeed : _loadFollowingFeed,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (posts.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(
              Icons.inbox_outlined,
              size: 64,
              color: JojuhuColors.textoCinza,
            ),
            const SizedBox(height: 16),
            Text(
              isForYou 
                  ? 'No posts yet. Be the first to post!'
                  : 'No posts from people you follow.\nFollow some users to see their posts here!',
              textAlign: TextAlign.center,
              style: const TextStyle(color: JojuhuColors.textoCinza),
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      controller: scrollController,
      itemCount: posts.length + (hasMore ? 1 : 0),
      itemBuilder: (context, index) {
        if (index == posts.length) {
          return const Center(
            child: Padding(
              padding: EdgeInsets.all(16),
              child: CircularProgressIndicator(),
            ),
          );
        }

        final post = posts[index];
        return _PostCard(
          post: post,
          onLike: () => _toggleLike(post, index, isForYou),
          onComment: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => PostDetailScreen(post: post),
              ),
            ).then((_) {
              if (isForYou) {
                _loadForYouFeed();
              } else {
                _loadFollowingFeed();
              }
            });
          },
          onDelete: () => _deletePost(post.id, isForYou),
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
  final VoidCallback onDelete;
  final String Function(DateTime) formatTime;

  const _PostCard({
    required this.post,
    required this.onLike,
    required this.onComment,
    required this.onDelete,
    required this.formatTime,
  });

  void _openImageViewer(BuildContext context, List<String> images, int initialIndex) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => _ImageViewer(
          images: images,
          initialIndex: initialIndex,
        ),
      ),
    );
  }

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
              backgroundColor: JojuhuColors.lua.withOpacity(0.1),
              backgroundImage: post.author.avatarUrl != null
                  ? NetworkImage(post.author.avatarUrl!)
                  : null,
              child: post.author.avatarUrl == null
                  ? Text(
                      post.author.username[0].toUpperCase(),
                      style: const TextStyle(
                        color: JojuhuColors.lua,
                        fontWeight: FontWeight.bold,
                      ),
                    )
                  : null,
            ),
            title: Text(
              post.author.displayName ?? post.author.username,
              style: const TextStyle(fontWeight: FontWeight.bold),
            ),
            subtitle: Text('@${post.author.username} · ${formatTime(post.createdAt)}'),
            trailing: PopupMenuButton<String>(
              onSelected: (value) {
                if (value == 'delete') {
                  showDialog(
                    context: context,
                    builder: (context) => AlertDialog(
                      title: const Text('Delete Post'),
                      content: const Text('Are you sure you want to delete this post?'),
                      actions: [
                        TextButton(
                          onPressed: () => Navigator.pop(context),
                          child: const Text('Cancel'),
                        ),
                        TextButton(
                          onPressed: () {
                            Navigator.pop(context);
                            onDelete();
                          },
                          style: TextButton.styleFrom(
                            foregroundColor: JojuhuColors.error,
                          ),
                          child: const Text('Delete'),
                        ),
                      ],
                    ),
                  );
                }
              },
              itemBuilder: (context) => [
                const PopupMenuItem(
                  value: 'delete',
                  child: Row(
                    children: [
                      Icon(Icons.delete_outline, color: JojuhuColors.error),
                      SizedBox(width: 8),
                      Text('Delete'),
                    ],
                  ),
                ),
              ],
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
                  return GestureDetector(
                    onTap: () => _openImageViewer(context, post.mediaUrls, index),
                    child: Padding(
                      padding: const EdgeInsets.all(8),
                      child: ClipRRect(
                        borderRadius: BorderRadius.circular(8),
                        child: Image.network(
                          post.mediaUrls[index],
                          height: 200,
                          fit: BoxFit.cover,
                        ),
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
                  color: post.isLiked ? JojuhuColors.error : null,
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

class _ImageViewer extends StatefulWidget {
  final List<String> images;
  final int initialIndex;

  const _ImageViewer({
    required this.images,
    required this.initialIndex,
  });

  @override
  State<_ImageViewer> createState() => _ImageViewerState();
}

class _ImageViewerState extends State<_ImageViewer> {
  late PageController _pageController;
  late int _currentIndex;

  @override
  void initState() {
    super.initState();
    _currentIndex = widget.initialIndex;
    _pageController = PageController(initialPage: widget.initialIndex);
  }

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.black,
        foregroundColor: Colors.white,
        title: Text('${_currentIndex + 1} / ${widget.images.length}'),
      ),
      body: PageView.builder(
        controller: _pageController,
        itemCount: widget.images.length,
        onPageChanged: (index) {
          setState(() {
            _currentIndex = index;
          });
        },
        itemBuilder: (context, index) {
          return InteractiveViewer(
            minScale: 0.5,
            maxScale: 4.0,
            child: Center(
              child: Image.network(
                widget.images[index],
                fit: BoxFit.contain,
                loadingBuilder: (context, child, loadingProgress) {
                  if (loadingProgress == null) return child;
                  return CircularProgressIndicator(
                    value: loadingProgress.expectedTotalBytes != null
                        ? loadingProgress.cumulativeBytesLoaded /
                            loadingProgress.expectedTotalBytes!
                        : null,
                    color: JojuhuColors.sol,
                  );
                },
              ),
            ),
          );
        },
      ),
    );
  }
}
