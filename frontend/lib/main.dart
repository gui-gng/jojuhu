import 'package:flutter/material.dart';
import 'package:jojuhu/screens/app/home_screen.dart';
import 'package:jojuhu/screens/auth/login_screen.dart';
import 'package:jojuhu/services/api_service.dart';
import 'package:jojuhu/theme/jojuhu_theme.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  Future<bool> _checkAuth() async {
    final token = await ApiService.getToken();
    return token != null;
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Jojuhu',
      debugShowCheckedModeBanner: false,
      theme: JojuhuTheme.lightTheme,
      home: FutureBuilder<bool>(
        future: _checkAuth(),
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Scaffold(
              backgroundColor: JojuhuTheme.lightTheme.scaffoldBackgroundColor,
              body: const Center(
                child: CircularProgressIndicator(),
              ),
            );
          }
          
          // If user is authenticated, show home screen
          // Otherwise, show login screen
          if (snapshot.hasData && snapshot.data == true) {
            return const HomeScreen();
          } else {
            return const LoginScreen();
          }
        },
      ),
    );
  }
}
