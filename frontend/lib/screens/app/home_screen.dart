import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:jojuhu/theme/theme_provider.dart';
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
  Widget build(BuildContext context) {
    final themeProvider = Provider.of<ThemeProvider>(context);
    
    return Scaffold(
      appBar: AppBar(
        title: const Text('Jojuhu'),
        actions: [
          // Theme toggle button
          IconButton(
            icon: Icon(
              themeProvider.isDarkMode ? Icons.light_mode : Icons.dark_mode,
            ),
            onPressed: () {
              themeProvider.toggleTheme();
            },
            tooltip: themeProvider.isDarkMode ? 'Switch to Light Mode' : 'Switch to Dark Mode',
          ),
        ],
      ),
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