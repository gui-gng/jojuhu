import 'package:flutter/material.dart';
import 'package:jojuhu/models/post.dart';
import 'package:jojuhu/models/user_profile.dart';
import 'package:jojuhu/services/api_service.dart';
import 'package:jojuhu/theme/jojuhu_theme.dart';
import 'edit_profile_screen.dart';
import 'follow_list_screen.dart';
import 'post_detail_screen.dart';
import 'privacy_settings_screen.dart';
import 'scheduled_posts_screen.dart';
import 'groups_screen.dart';

class ProfileScreen extends StatefulWidget {
  final String? userId;
  final bool isMyProfile;

  const ProfileScreen({
    super.key,
    this.userId,
    this.isMyProfile = false,
  });

  @override
  State<ProfileScreen> createState() => _ProfileScreenState();
}

class _ProfileScreenState extends State<ProfileScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  UserProfile? _profile;
  MyProfile? _myProfile;
  List<Post> _posts = [];
  bool _isLoading = true;
  bool _isLoadingPosts = true;
  String? _error;
  int _currentTab = 0;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
    _tabController.addListener(() {
      setState(() {
        _currentTab = _tabController.index;
      });
    });
    _loadProfile();
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  Future<void> _loadProfile() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      if (widget.isMyProfile || widget.userId == null) {
        // Load my profile
        final result = await ApiService.getMyProfile();
        if (result.success && result.data != null) {
          setState(() {
            _myProfile = result.data;
          });
          // Also load posts
          await _loadUserPosts(result.data!.id);
        } else {
          setState(() {
            _error = result.error ?? 'Failed to load profile';
          });
        }
      } else {
        // Load other user's profile
        final result = await ApiService.getUserProfile(widget.userId!);
        if (result.success && result.data != null) {
          setState(() {
            _profile = result.data;
          });
          // Also load posts
          await _loadUserPosts(widget.userId!);
        } else {
          setState(() {
            _error = result.error ?? 'Failed to load profile';
          });
        }
      }
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<void> _loadUserPosts(String userId) async {
    setState(() {
      _isLoadingPosts = true;
    });

    try {
      final result = await ApiService.getUserPosts(userId: userId);
      if (result.success && result.data != null) {
        setState(() {
          _posts = result.data!;
        });
      }
    } finally {
      setState(() {
        _isLoadingPosts = false;
      });
    }
  }

  Future<void> _toggleFollow() async {
    if (_profile == null) return;

    final isFollowing = _profile!.isFollowing;
    
    setState(() {
      _profile = UserProfile(
        id: _profile!.id,
        username: _profile!.username,
        displayName: _profile!.displayName,
        bio: _profile!.bio,
        avatarUrl: _profile!.avatarUrl,
        createdAt: _profile!.createdAt,
        followersCount: isFollowing
            ? _profile!.followersCount - 1
            : _profile!.followersCount + 1,
        followingCount: _profile!.followingCount,
        postsCount: _profile!.postsCount,
        isFollowing: !isFollowing,
      );
    });

    try {
      if (isFollowing) {
        await ApiService.unfollowUser(_profile!.id);
      } else {
        await ApiService.followUser(_profile!.id);
      }
    } catch (e) {
      // Revert on error
      setState(() {
        _profile = UserProfile(
          id: _profile!.id,
          username: _profile!.username,
          displayName: _profile!.displayName,
          bio: _profile!.bio,
          avatarUrl: _profile!.avatarUrl,
          createdAt: _profile!.createdAt,
          followersCount: isFollowing
              ? _profile!.followersCount + 1
              : _profile!.followersCount - 1,
          followingCount: _profile!.followingCount,
          postsCount: _profile!.postsCount,
          isFollowing: isFollowing,
        );
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    
    if (_isLoading) {
      return const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      );
    }

    if (_error != null) {
      return Scaffold(
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(_error!, style: const TextStyle(color: Colors.red)),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: _loadProfile,
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    final isMyProfile = widget.isMyProfile || widget.userId == null;
    final dynamic profile = _myProfile ?? _profile;

    if (profile == null) {
      return const Scaffold(
        body: Center(child: Text('Profile not found')),
      );
    }

    return Scaffold(
      body: NestedScrollView(
        headerSliverBuilder: (context, innerBoxIsScrolled) {
          return [
            SliverAppBar(
              expandedHeight: 200,
              floating: false,
              pinned: true,
              flexibleSpace: FlexibleSpaceBar(
                title: Text(
                  profile.displayNameOrUsername,
                  style: const TextStyle(color: Colors.white),
                ),
                background: Container(
                  color: theme.primaryColor,
                ),
              ),
              actions: [
                if (isMyProfile)
                  IconButton(
                    icon: const Icon(Icons.edit),
                    onPressed: () async {
                      final result = await Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) => const EditProfileScreen(),
                        ),
                      );
                      if (result == true) {
                        _loadProfile();
                      }
                    },
                  ),
              ],
            ),
            SliverToBoxAdapter(
              child: _buildProfileHeader(profile, isMyProfile),
            ),
            SliverPersistentHeader(
              delegate: _SliverAppBarDelegate(
                TabBar(
                  controller: _tabController,
                  tabs: const [
                    Tab(text: 'Posts'),
                    Tab(text: 'Forums'),
                    Tab(text: 'About'),
                  ],
                ),
              ),
              pinned: true,
            ),
          ];
        },
        body: TabBarView(
          controller: _tabController,
          children: [
            _buildPostsTab(),
            _buildForumsTab(),
            _buildAboutTab(profile),
          ],
        ),
      ),
    );
  }

  Widget _buildProfileHeader(dynamic profile, bool isMyProfile) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        children: [
          Row(
            children: [
              Container(
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  border: Border.all(
                    color: JojuhuColors.sol,
                    width: 3,
                  ),
                ),
                child: CircleAvatar(
                  radius: 50,
                  backgroundColor: JojuhuColors.lua.withOpacity(0.1),
                  backgroundImage: profile.avatarUrl != null
                      ? NetworkImage(profile.avatarUrl!)
                      : null,
                  child: profile.avatarUrl == null
                      ? Text(
                          profile.username[0].toUpperCase(),
                          style: const TextStyle(
                            fontSize: 40,
                            color: JojuhuColors.lua,
                            fontWeight: FontWeight.bold,
                          ),
                        )
                      : null,
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      profile.displayNameOrUsername,
                      style: const TextStyle(
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    Text(
                      '@${profile.username}',
                      style: TextStyle(
                        fontSize: 16,
                        color: Colors.grey[600],
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          if (profile.bio != null) ...[
            Text(
              profile.bio!,
              style: const TextStyle(fontSize: 16),
            ),
            const SizedBox(height: 16),
          ],
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            children: [
              _buildStatColumn('Posts', profile.postsCount.toString()),
              GestureDetector(
                onTap: () => Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => FollowListScreen(
                      userId: profile.id,
                      username: profile.username,
                      initialTab: 0,
                    ),
                  ),
                ),
                child: _buildStatColumn('Followers', profile.followersCount.toString()),
              ),
              GestureDetector(
                onTap: () => Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => FollowListScreen(
                      userId: profile.id,
                      username: profile.username,
                      initialTab: 1,
                    ),
                  ),
                ),
                child: _buildStatColumn('Following', profile.followingCount.toString()),
              ),
            ],
          ),
          const SizedBox(height: 16),
          if (!isMyProfile)
            SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _toggleFollow,
                style: ElevatedButton.styleFrom(
                  backgroundColor: profile.isFollowing
                      ? JojuhuColors.fundo
                      : JojuhuColors.sol,
                  foregroundColor: profile.isFollowing
                      ? JojuhuColors.textoEscuro
                      : JojuhuColors.textoClaro,
                  side: profile.isFollowing
                      ? const BorderSide(color: JojuhuColors.textoCinza)
                      : null,
                ),
                child: Text(profile.isFollowing ? 'Following' : 'Follow'),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildStatColumn(String label, String value) {
    return Column(
      children: [
        Text(
          value,
          style: const TextStyle(
            fontSize: 20,
            fontWeight: FontWeight.bold,
          ),
        ),
        Text(
          label,
          style: TextStyle(
            fontSize: 14,
            color: Colors.grey[600],
          ),
        ),
      ],
    );
  }

  Widget _buildPostsTab() {
    if (_isLoadingPosts) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_posts.isEmpty) {
      return const Center(
        child: Text('No posts yet'),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(8),
      itemCount: _posts.length,
      itemBuilder: (context, index) {
        final post = _posts[index];
        return ListTile(
          title: Text(post.content.substring(0, post.content.length > 100 ? 100 : post.content.length)),
          subtitle: Text('${post.likesCount} likes · ${post.commentsCount} comments'),
          onTap: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => PostDetailScreen(post: post),
              ),
            );
          },
        );
      },
    );
  }

  Widget _buildForumsTab() {
    return const Center(
      child: Text('Forums joined will appear here'),
    );
  }

  Widget _buildAboutTab(dynamic profile) {
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        _buildInfoRow(Icons.calendar_today, 'Joined', _formatDate(profile.createdAt)),
        if (profile is MyProfile)
          _buildInfoRow(Icons.email, 'Email', profile.email),
        if (widget.isMyProfile) ...[
          const Divider(height: 32),
          const Text(
            'Settings',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 8),
          ListTile(
            leading: const Icon(Icons.privacy_tip),
            title: const Text('Privacy Settings'),
            subtitle: const Text('Manage your privacy and data'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const PrivacySettingsScreen(),
                ),
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.schedule),
            title: const Text('Scheduled Posts'),
            subtitle: const Text('View and manage scheduled posts'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const ScheduledPostsScreen(),
                ),
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.group),
            title: const Text('Groups'),
            subtitle: const Text('Browse and manage groups'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const GroupsScreen(),
                ),
              );
            },
          ),
        ],
      ],
    );
  }

  Widget _buildInfoRow(IconData icon, String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Icon(icon, size: 20, color: Colors.grey[600]),
          const SizedBox(width: 8),
          Text(
            '$label: ',
            style: TextStyle(
              color: Colors.grey[600],
              fontWeight: FontWeight.w500,
            ),
          ),
          Text(value),
        ],
      ),
    );
  }

  String _formatDate(DateTime date) {
    final months = [
      'January', 'February', 'March', 'April', 'May', 'June',
      'July', 'August', 'September', 'October', 'November', 'December'
    ];
    return '${months[date.month - 1]} ${date.year}';
  }
}

class _SliverAppBarDelegate extends SliverPersistentHeaderDelegate {
  final TabBar _tabBar;

  _SliverAppBarDelegate(this._tabBar);

  @override
  double get minExtent => _tabBar.preferredSize.height;

  @override
  double get maxExtent => _tabBar.preferredSize.height;

  @override
  Widget build(BuildContext context, double shrinkOffset, bool overlapsContent) {
    return Container(
      color: Theme.of(context).scaffoldBackgroundColor,
      child: _tabBar,
    );
  }

  @override
  bool shouldRebuild(_SliverAppBarDelegate oldDelegate) {
    return false;
  }
}
