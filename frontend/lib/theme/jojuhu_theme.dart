import 'package:flutter/material.dart';

/// Jojuhu Color Palette
/// Based on the Meeting of Waters (Encontro das Águas) from Amazonas, Brazil
/// 
/// Sol (Sun) / Rio Solimões - Orange (#FF6B35)
/// Lua (Moon) / Rio Negro - Deep Blue (#004E89)  
/// Encontro (Meeting) - Purple (#9B59B6)
/// Fundo (Background) - Off White (#F5F5F0)

class JojuhuColors {
  // Primary Colors
  static const Color sol = Color(0xFFFF6B35);      // Rio Solimões - Orange
  static const Color lua = Color(0xFF004E89);      // Rio Negro - Deep Blue
  static const Color encontro = Color(0xFF9B59B6); // Purple - Mixture
  
  // Background Colors
  static const Color fundo = Color(0xFFF5F5F0);    // Off white
  static const Color fundoBranco = Color(0xFFFFFFFF); // Pure white
  
  // Text Colors
  static const Color textoEscuro = Color(0xFF1A1A1A);
  static const Color textoCinza = Color(0xFF666666);
  static const Color textoClaro = Color(0xFFFFFFFF);
  
  // Accent Colors
  static const Color success = Color(0xFF27AE60);
  static const Color error = Color(0xFFE74C3C);
  static const Color warning = Color(0xFFF39C12);
  
  // Gradient for special elements
  static LinearGradient get encontroGradient => const LinearGradient(
    colors: [sol, encontro, lua],
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
  );
  
  static LinearGradient get solLuaGradient => const LinearGradient(
    colors: [sol, lua],
    begin: Alignment.centerLeft,
    end: Alignment.centerRight,
  );
}

/// Jojuhu Theme Data
class JojuhuTheme {
  static ThemeData get darkTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      
      // Primary Colors
      primaryColor: JojuhuColors.sol,
      colorScheme: const ColorScheme.dark(
        primary: JojuhuColors.sol,
        secondary: JojuhuColors.lua,
        tertiary: JojuhuColors.encontro,
        surface: Color(0xFF1E1E1E),
        background: Color(0xFF121212),
        onPrimary: JojuhuColors.textoClaro,
        onSecondary: JojuhuColors.textoClaro,
        onSurface: JojuhuColors.textoClaro,
        onBackground: JojuhuColors.textoClaro,
      ),
      
      // Scaffold Background
      scaffoldBackgroundColor: const Color(0xFF121212),
      
      // App Bar Theme
      appBarTheme: const AppBarTheme(
        backgroundColor: Color(0xFF1E1E1E),
        foregroundColor: JojuhuColors.textoClaro,
        elevation: 0,
        centerTitle: true,
      ),
      
      // Bottom Navigation Bar
      bottomNavigationBarTheme: const BottomNavigationBarThemeData(
        backgroundColor: Color(0xFF1E1E1E),
        selectedItemColor: JojuhuColors.sol,
        unselectedItemColor: Color(0xFF9E9E9E),
        type: BottomNavigationBarType.fixed,
        elevation: 8,
      ),
      
      // Card Theme
      cardTheme: CardTheme(
        color: const Color(0xFF1E1E1E),
        elevation: 2,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
        ),
      ),
      
      // Input Decoration
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: const Color(0xFF2C2C2C),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide.none,
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Color(0xFF3E3E3E)),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: JojuhuColors.sol, width: 2),
        ),
        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
      ),
      
      // Elevated Button
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: JojuhuColors.sol,
          foregroundColor: JojuhuColors.textoClaro,
          elevation: 0,
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
      ),
      
      // Text Theme
      textTheme: const TextTheme(
        headlineLarge: TextStyle(
          color: JojuhuColors.textoClaro,
          fontWeight: FontWeight.bold,
        ),
        headlineMedium: TextStyle(
          color: JojuhuColors.textoClaro,
          fontWeight: FontWeight.bold,
        ),
        titleLarge: TextStyle(
          color: JojuhuColors.textoClaro,
          fontWeight: FontWeight.w600,
        ),
        bodyLarge: TextStyle(
          color: JojuhuColors.textoClaro,
        ),
        bodyMedium: TextStyle(
          color: Color(0xFF9E9E9E),
        ),
      ),
    );
  }
  
  static ThemeData get lightTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.light,
      
      // Primary Colors
      primaryColor: JojuhuColors.sol,
      colorScheme: const ColorScheme.light(
        primary: JojuhuColors.sol,
        secondary: JojuhuColors.lua,
        tertiary: JojuhuColors.encontro,
        surface: JojuhuColors.fundo,
        background: JojuhuColors.fundo,
        onPrimary: JojuhuColors.textoClaro,
        onSecondary: JojuhuColors.textoClaro,
        onSurface: JojuhuColors.textoEscuro,
        onBackground: JojuhuColors.textoEscuro,
      ),
      
      // Scaffold Background
      scaffoldBackgroundColor: JojuhuColors.fundo,
      
      // App Bar Theme
      appBarTheme: const AppBarTheme(
        backgroundColor: JojuhuColors.lua,
        foregroundColor: JojuhuColors.textoClaro,
        elevation: 0,
        centerTitle: true,
      ),
      
      // Bottom Navigation Bar
      bottomNavigationBarTheme: const BottomNavigationBarThemeData(
        backgroundColor: JojuhuColors.fundoBranco,
        selectedItemColor: JojuhuColors.sol,
        unselectedItemColor: JojuhuColors.textoCinza,
        type: BottomNavigationBarType.fixed,
        elevation: 8,
      ),
      
      // Navigation Bar (Material 3)
      navigationBarTheme: NavigationBarThemeData(
        backgroundColor: JojuhuColors.fundoBranco,
        indicatorColor: JojuhuColors.sol.withOpacity(0.2),
        labelTextStyle: MaterialStateProperty.all(
          const TextStyle(color: JojuhuColors.textoEscuro),
        ),
        iconTheme: MaterialStateProperty.all(
          const IconThemeData(color: JojuhuColors.textoCinza),
        ),
      ),
      
      // Card Theme
      cardTheme: CardTheme(
        color: JojuhuColors.fundoBranco,
        elevation: 2,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
        ),
      ),
      
      // Input Decoration
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: JojuhuColors.fundoBranco,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide.none,
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: Color(0xFFE0E0E0)),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: JojuhuColors.sol, width: 2),
        ),
        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
      ),
      
      // Elevated Button
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: JojuhuColors.sol,
          foregroundColor: JojuhuColors.textoClaro,
          elevation: 0,
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
      ),
      
      // Text Button
      textButtonTheme: TextButtonThemeData(
        style: TextButton.styleFrom(
          foregroundColor: JojuhuColors.lua,
        ),
      ),
      
      // Outlined Button
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: JojuhuColors.lua,
          side: const BorderSide(color: JojuhuColors.lua),
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
        ),
      ),
      
      // Floating Action Button
      floatingActionButtonTheme: const FloatingActionButtonThemeData(
        backgroundColor: JojuhuColors.sol,
        foregroundColor: JojuhuColors.textoClaro,
      ),
      
      // Tab Bar
      tabBarTheme: const TabBarTheme(
        labelColor: JojuhuColors.sol,
        unselectedLabelColor: JojuhuColors.textoCinza,
        indicatorColor: JojuhuColors.sol,
      ),
      
      // Divider
      dividerTheme: const DividerThemeData(
        color: Color(0xFFE0E0E0),
        thickness: 1,
      ),
      
      // Text Theme
      textTheme: const TextTheme(
        headlineLarge: TextStyle(
          color: JojuhuColors.textoEscuro,
          fontWeight: FontWeight.bold,
        ),
        headlineMedium: TextStyle(
          color: JojuhuColors.textoEscuro,
          fontWeight: FontWeight.bold,
        ),
        titleLarge: TextStyle(
          color: JojuhuColors.textoEscuro,
          fontWeight: FontWeight.w600,
        ),
        bodyLarge: TextStyle(
          color: JojuhuColors.textoEscuro,
        ),
        bodyMedium: TextStyle(
          color: JojuhuColors.textoCinza,
        ),
      ),
      
      // Progress Indicator
      progressIndicatorTheme: const ProgressIndicatorThemeData(
        color: JojuhuColors.sol,
      ),
      
      // Snack Bar
      snackBarTheme: SnackBarThemeData(
        backgroundColor: JojuhuColors.textoEscuro,
        contentTextStyle: const TextStyle(color: JojuhuColors.textoClaro),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(8),
        ),
        behavior: SnackBarBehavior.floating,
      ),
    );
  }
}
