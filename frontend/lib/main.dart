import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:jojuhu/screens/app/home_screen.dart';
import 'package:jojuhu/screens/auth/login_screen.dart';
import 'package:jojuhu/services/api_service.dart';
import 'package:jojuhu/theme/theme_provider.dart';

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
    return ChangeNotifierProvider(
      create: (_) => ThemeProvider(),
      child: Consumer<ThemeProvider>(
        builder: (context, themeProvider, child) {
          return MaterialApp(
            title: 'Jojuhu',
            debugShowCheckedModeBanner: false,
            theme: themeProvider.theme,
            home: FutureBuilder<bool>(
              future: _checkAuth(),
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const Scaffold(
                    body: Center(
                      child: CircularProgressIndicator(),
                    ),
                  );
                }
                
                if (snapshot.hasData && snapshot.data == true) {
                  return const HomeScreen();
                } else {
                  return const LoginScreen();
                }
              },
            ),
          );
        },
      ),
    );
  }
}