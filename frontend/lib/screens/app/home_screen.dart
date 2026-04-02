import 'package:flutter/material.dart';
import 'pages/explore.dart';
import 'pages/forum.dart';
import 'pages/messages/messages_screen.dart';
import 'pages/profile_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  int currentPageIndex = 0;
  late PageController _pageController;

  final List<Widget> _pages = const [
    ExploreScreen(),
    MessagesScreen(),
    ForumScreen(),
    ProfileScreen(isMyProfile: true),
  ];

  final List<NavigationDestination> _destinations = const [
    NavigationDestination(
      selectedIcon: Icon(Icons.explore),
      icon: Icon(Icons.explore_outlined),
      label: 'Explore',
    ),
    NavigationDestination(
      selectedIcon: Icon(Icons.message),
      icon: Icon(Icons.message_outlined),
      label: 'Messages',
    ),
    NavigationDestination(
      selectedIcon: Icon(Icons.forum),
      icon: Icon(Icons.forum_outlined),
      label: 'Forum',
    ),
    NavigationDestination(
      selectedIcon: Icon(Icons.person),
      icon: Icon(Icons.person_outlined),
      label: 'Profile',
    ),
  ];

  @override
  void initState() {
    super.initState();
    _pageController = PageController(initialPage: currentPageIndex);
  }

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }

  void _onDestinationSelected(int index) {
    setState(() {
      currentPageIndex = index;
    });
    _pageController.animateToPage(
      index,
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeInOut,
    );
  }

  void _onPageChanged(int index) {
    setState(() {
      currentPageIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Jojuhu'),
      ),
      bottomNavigationBar: NavigationBar(
        onDestinationSelected: _onDestinationSelected,
        selectedIndex: currentPageIndex,
        destinations: _destinations,
      ),
      body: PageView(
        controller: _pageController,
        onPageChanged: _onPageChanged,
        children: _pages,
      ),
    );
  }
}