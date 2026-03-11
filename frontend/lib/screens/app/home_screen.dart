import 'package:flutter/material.dart';
import 'pages/explore.dart';
import 'pages/forum.dart';
import 'pages/messages/messages_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  int currentPageIndex = 0;

  final List<Widget> _pages = const [
    ExploreScreen(),
    MessagesScreen(),
    ForumScreen(),
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
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      bottomNavigationBar: NavigationBar(
        onDestinationSelected: (int index) {
          setState(() {
            currentPageIndex = index;
          });
        },
        selectedIndex: currentPageIndex,
        destinations: _destinations,
      ),
      body: _pages[currentPageIndex],
    );
  }
}
