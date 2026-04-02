import 'package:flutter/material.dart';
import 'jojuhu_theme.dart';

class ThemeProvider extends ChangeNotifier {
  ThemeData get theme => JojuhuTheme.lightTheme;
}