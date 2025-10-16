import 'package:flutter/material.dart';
import 'package:jojuhu_frontend/screens/app/home_screen.dart';
import 'screens/auth/login_screen.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    const home = HomeScreen();

    return MaterialApp(
      title: 'Jojuhu',
      theme: ThemeData(
        primarySwatch: Colors.deepOrange,
      ),
      home: home,
    );
  }
}
