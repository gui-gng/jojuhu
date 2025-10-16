import 'package:flutter/material.dart';

class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    const txtTitle = Text('Home');
    const txtCenter = Text('Oi');

    return Scaffold(
      appBar: AppBar(title: txtTitle),
      body: Center(
        child: txtCenter,
      ),
    );
  }
}
