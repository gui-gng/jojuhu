import 'package:flutter/material.dart';
import 'package:jojuhu/models/user_profile.dart';
import 'package:jojuhu/services/api_service.dart';
import 'profile_screen.dart';

class FollowListScreen extends StatefulWidget {
  final String userId;
  final String username;
  final int initialTab;

  const FollowListScreen({
    super.key,
    required this.userId,
    required this.username,
    this.initialTab = 0,
  });

  @override
  State<FollowListScreen> createState() => _FollowListScreenState();
}

class _FollowListScreenState extends State<FollowListScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  List<UserInfo> _followers = [];
  List<UserInfo> _following = [];
  bool _isLoadingFollowers = true;
  bool _isLoadingFollowing = true;
  String? _followersError;
  String? _followingError;
  int _followersPage = 1;
  int _followingPage = 1;
  bool _hasMoreFollowers = true;
  bool _hasMoreFollowing = true;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(
      length: 2,
      vsync: this,
      initialIndex: widget.initialTab,
    );
    _loadFollowers();
    _loadFollowing();
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  Future<void> _loadFollowers() async {
    setState(() {
      _isLoadingFollowers = true;
      _followersError = null;
    });

    final result = await ApiService.getFollowers(
      userId: widget.userId,
      page: _followersPage,
    );

    if (result.success && result.data != null) {
      setState(() {
        if (_followersPage == 1) {
          _followers = result.data!.users;
        } else {
          _followers.addAll(result.data!.users);
        }
        _hasMoreFollowers = result.data!.hasMore;
      });
    } else {
      setState(() {
        _followersError = result.error ?? 'Failed to load followers';
      });
    }

    setState(() {
      _isLoadingFollowers = false;
    });
  }

  Future<void> _loadFollowing() async {
    setState(() {
      _isLoadingFollowing = true;
      _followingError = null;
    });

    final result = await ApiService.getFollowing(
      userId: widget.userId,
      page: _followingPage,
    );

    if (result.success && result.data != null) {
      setState(() {
        if (_followingPage == 1) {
          _following = result.data!.users;
        } else {
          _following.addAll(result.data!.users);
        }
        _hasMoreFollowing = result.data!.hasMore;
      });
    } else {
      setState(() {
        _followingError = result.error ?? 'Failed to load following';
      });
    }

    setState(() {
      _isLoadingFollowing = false;
    });
  }

  Future<void> _toggleFollow(UserInfo user, int index, bool isFollowersList) async {
    final isFollowing = user.isFollowing;
    
    // Optimistic update
    if (isFollowersList) {
      setState(() {
        _followers[index] = UserInfo(
          id: user.id,
          username: user.username,
          displayName: user.displayName,
          avatarUrl: user.avatarUrl,
          isFollowing: !isFollowing,
        );
      });
    } else {
      setState(() {
        _following[index] = UserInfo(
          id: user.id,
          username: user.username,
          displayName: user.displayName,
          avatarUrl: user.avatarUrl,
          isFollowing: !isFollowing,
        );
      });
    }

    try {
      if (isFollowing) {
        await ApiService.unfollowUser(user.id);
      } else {
        await ApiService.followUser(user.id);
      }
    } catch (e) {
      // Revert on error
      if (isFollowersList) {
        setState(() {
          _followers[index] = user;
        });
      } else {
        setState(() {
          _following[index] = user;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.username),
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: 'Followers'),
            Tab(text: 'Following'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          _buildFollowersList(),
          _buildFollowingList(),
        ],
      ),
    );
  }

  Widget _buildFollowersList() {
    if (_isLoadingFollowers && _followers.isEmpty) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_followersError != null && _followers.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(_followersError!, style: const TextStyle(color: Colors.red)),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadFollowers,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_followers.isEmpty) {
      return const Center(
        child: Text('No followers yet'),
      );
    }

    return ListView.builder(
      itemCount: _followers.length + (_hasMoreFollowers ? 1 : 0),
      itemBuilder: (context, index) {
        if (index == _followers.length) {
          return Center(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: _isLoadingFollowers
                  ? const CircularProgressIndicator()
                  : TextButton(
                      onPressed: () {
                        setState(() {
                          _followersPage++;
                        });
                        _loadFollowers();
                      },
                      child: const Text('Load More'),
                    ),
            ),
          );
        }

        final user = _followers[index];
        return _buildUserListTile(user, index, true);
      },
    );
  }

  Widget _buildFollowingList() {
    if (_isLoadingFollowing && _following.isEmpty) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_followingError != null && _following.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(_followingError!, style: const TextStyle(color: Colors.red)),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadFollowing,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_following.isEmpty) {
      return const Center(
        child: Text('Not following anyone yet'),
      );
    }

    return ListView.builder(
      itemCount: _following.length + (_hasMoreFollowing ? 1 : 0),
      itemBuilder: (context, index) {
        if (index == _following.length) {
          return Center(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: _isLoadingFollowing
                  ? const CircularProgressIndicator()
                  : TextButton(
                      onPressed: () {
                        setState(() {
                          _followingPage++;
                        });
                        _loadFollowing();
                      },
                      child: const Text('Load More'),
                    ),
            ),
          );
        }

        final user = _following[index];
        return _buildUserListTile(user, index, false);
      },
    );
  }

  Widget _buildUserListTile(UserInfo user, int index, bool isFollowersList) {
    return ListTile(
      leading: CircleAvatar(
        backgroundImage: user.avatarUrl != null
            ? NetworkImage(user.avatarUrl!)
            : null,
        child: user.avatarUrl == null
            ? Text(user.username[0].toUpperCase())
            : null,
      ),
      title: Text(
        user.displayNameOrUsername,
        style: const TextStyle(fontWeight: FontWeight.bold),
      ),
      subtitle: Text('@${user.username}'),
      trailing: ElevatedButton(
        onPressed: () => _toggleFollow(user, index, isFollowersList),
        style: ElevatedButton.styleFrom(
          backgroundColor: user.isFollowing
              ? Colors.grey[200]
              : Theme.of(context).primaryColor,
          foregroundColor: user.isFollowing
              ? Colors.black87
              : Colors.white,
          elevation: 0,
        ),
        child: Text(user.isFollowing ? 'Following' : 'Follow'),
      ),
      onTap: () {
        Navigator.push(
          context,
          MaterialPageRoute(
            builder: (context) => ProfileScreen(userId: user.id),
          ),
        );
      },
    );
  }
}
