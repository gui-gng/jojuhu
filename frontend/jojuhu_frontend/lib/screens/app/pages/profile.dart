import 'package:flutter/material.dart';

class ProfileScreen extends StatefulWidget {
  const ProfileScreen({super.key});

  @override
  State<ProfileScreen> createState() => _ProfileScreen();
}

class _ProfileScreen extends State<ProfileScreen> {
  int currentPageIndex = 0;

  @override
  Widget build(BuildContext context) {
    final ThemeData theme = Theme.of(context);
    const txtTitle = Text('Explore');
    const txtCenter = Text('Oi');

    return Scaffold(
      body: Card(
        shadowColor: Colors.transparent,
        margin: const EdgeInsets.all(8.0),
        child: SizedBox.expand(
          child: Center(
              child: Text('Home page', style: theme.textTheme.titleLarge)),
        ),
      ),
    );
  }
}
