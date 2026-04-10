import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'jojuhu_theme.dart';

class ThemeProvider extends ChangeNotifier {
  ThemeData get theme => JojuhuTheme.lightTheme;
}

class AppTheme {
  static ThemeData get light => JojuhuTheme.lightTheme;

  static void initScreenUtil(BuildContext context) {
    ScreenUtil.init(context, designSize: const Size(375, 812));
  }
}