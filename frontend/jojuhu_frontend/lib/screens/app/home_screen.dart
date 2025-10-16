import 'package:flutter/material.dart';
import 'package:jojuhu_frontend/screens/app/pages/explore.dart';
import 'package:jojuhu_frontend/screens/app/pages/forum.dart';
import 'package:jojuhu_frontend/screens/app/pages/messages.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreen();
}

class _HomeScreen extends State<HomeScreen> {
  int currentPageIndex = 0;

  @override
  Widget build(BuildContext context) {
    final ThemeData theme = Theme.of(context);

    const exploreScreen = ExploreScreen();
    const messageScreen = MessagesScreen();
    const forumScreen = ForumScreen();

    return Scaffold(
      bottomNavigationBar: NavigationBar(
        onDestinationSelected: (int index) {
          setState(() {
            currentPageIndex = index;
          });
        },
        indicatorColor: Colors.amber,
        selectedIndex: currentPageIndex,
        destinations: const <Widget>[
          NavigationDestination(
            selectedIcon:
                Badge(label: Text('2'), child: Icon(Icons.south_america_sharp)),
            icon: Badge(label: Text('2'), child: Icon(Icons.public)),
            label: 'Explore',
          ),
          NavigationDestination(
            icon: Badge(label: Text('2'), child: Icon(Icons.messenger_sharp)),
            label: 'Messages',
          ),
          NavigationDestination(
            icon: Badge(label: Text('2'), child: Icon(Icons.description)),
            label: 'Forum',
          ),
        ],
      ),
      body: <Widget>[
        exploreScreen,
        messageScreen,
        forumScreen
      ][currentPageIndex],
    );
  }
}
