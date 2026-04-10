import 'package:flutter/material.dart';
import 'tokens.dart';
import 'typography.dart';

/// Jojuhu Color Palette
/// Based on the Meeting of Waters (Encontro das Águas) from Amazonas, Brazil
///
/// Sol (Sun) / Rio Solimões - Orange (#FF6B35)
/// Lua (Moon) / Rio Negro - Deep Blue (#004E89)
/// Encontro (Meeting) - Purple (#9B59B6)
/// Fundo (Background) - Off White (#F5F5F0)

class JojuhuColors {
  // Primary Colors
  static const Color sol = Color(0xFFFF6B35); // Rio Solimões - Orange
  static const Color lua = Color(0xFF004E89); // Rio Negro - Deep Blue
  static const Color encontro = Color(0xFF9B59B6); // Purple - Mixture

  // Background Colors
  static const Color fundo = Color(0xFFF5F5F0); // Off white
  static const Color fundoBranco = Color(0xFFFFFFFF); // Pure white

  // Text Colors
  static const Color textoEscuro = Color(0xFF1A1A1A);
  static const Color textoCinza = Color(0xFF666666);
  static const Color textoClaro = Color(0xFFFFFFFF);

  // Accent Colors
  static const Color success = Color(0xFF27AE60);
  static const Color error = Color(0xFFE74C3C);
  static const Color warning = Color(0xFFF39C12);
  static const Color info = Color(0xFF3498DB);

  // Semantic Colors
  static const Color border = Color(0xFFE0E0E0);
  static const Color borderLight = Color(0xFFF0F0F0);
  static const Color overlay = Color(0x80000000);

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

  static LinearGradient get heroGradient => LinearGradient(
        colors: [lua.withOpacity(0.9), encontro.withOpacity(0.8)],
        begin: Alignment.topLeft,
        end: Alignment.bottomRight,
      );
}

/// Jojuhu Theme Data
class JojuhuTheme {
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
        error: JojuhuColors.error,
        onPrimary: JojuhuColors.textoClaro,
        onSecondary: JojuhuColors.textoClaro,
        onSurface: JojuhuColors.textoEscuro,
        onError: JojuhuColors.textoClaro,
      ),

      // Scaffold Background
      scaffoldBackgroundColor: JojuhuColors.fundo,

      // Typography
      textTheme: JojuhuTypography.textTheme,

      // App Bar Theme
      appBarTheme: const AppBarTheme(
        backgroundColor: JojuhuColors.lua,
        foregroundColor: JojuhuColors.textoClaro,
        elevation: 0,
        centerTitle: true,
        titleTextStyle: TextStyle(
          color: JojuhuColors.textoClaro,
          fontSize: 18,
          fontWeight: FontWeight.w600,
        ),
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
        labelTextStyle: WidgetStateProperty.all(
          const TextStyle(
            color: JojuhuColors.textoEscuro,
            fontSize: 12,
            fontWeight: FontWeight.w500,
          ),
        ),
        iconTheme: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) {
            return const IconThemeData(color: JojuhuColors.sol, size: 24);
          }
          return const IconThemeData(color: JojuhuColors.textoCinza, size: 24);
        }),
      ),

      // Card Theme
      cardTheme: CardThemeData(
        color: JojuhuColors.fundoBranco,
        elevation: JojuhuElevation.low,
        shape: RoundedRectangleBorder(
          borderRadius: JojuhuRadius.smRadius,
        ),
        margin: EdgeInsets.zero,
      ),

      // Input Decoration
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: JojuhuColors.fundoBranco,
        border: OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: BorderSide.none,
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.border),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.sol, width: 2),
        ),
        errorBorder: OutlineInputBorder(
          borderRadius: JojuhuRadius.smRadius,
          borderSide: const BorderSide(color: JojuhuColors.error),
        ),
        contentPadding: const EdgeInsets.symmetric(
          horizontal: JojuhuSpacing.lg,
          vertical: JojuhuSpacing.md,
        ),
      ),

      // Elevated Button
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: JojuhuColors.sol,
          foregroundColor: JojuhuColors.textoClaro,
          elevation: 0,
          shadowColor: Colors.transparent,
          minimumSize: const Size(0, JojuhuInsets.buttonHeight),
          padding: JojuhuSpacing.paddingButton,
          shape: RoundedRectangleBorder(
            borderRadius: JojuhuRadius.smRadius,
          ),
        ).copyWith(
          textStyle: WidgetStateProperty.all(JojuhuTypography.button),
        ),
      ),

      // Text Button
      textButtonTheme: TextButtonThemeData(
        style: TextButton.styleFrom(
          foregroundColor: JojuhuColors.lua,
          minimumSize: const Size(0, JojuhuInsets.buttonHeight),
          padding: JojuhuSpacing.paddingButton,
        ).copyWith(
          textStyle: WidgetStateProperty.all(JojuhuTypography.button),
        ),
      ),

      // Outlined Button
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: JojuhuColors.lua,
          side: const BorderSide(color: JojuhuColors.lua),
          minimumSize: const Size(0, JojuhuInsets.buttonHeight),
          padding: JojuhuSpacing.paddingButton,
          shape: RoundedRectangleBorder(
            borderRadius: JojuhuRadius.smRadius,
          ),
        ).copyWith(
          textStyle: WidgetStateProperty.all(JojuhuTypography.button),
        ),
      ),

      // Floating Action Button
      floatingActionButtonTheme: const FloatingActionButtonThemeData(
        backgroundColor: JojuhuColors.sol,
        foregroundColor: JojuhuColors.textoClaro,
        elevation: JojuhuElevation.medium,
        shape: CircleBorder(),
      ),

      // Tab Bar
      tabBarTheme: const TabBarThemeData(
        labelColor: JojuhuColors.sol,
        unselectedLabelColor: JojuhuColors.textoCinza,
        indicatorColor: JojuhuColors.sol,
        indicatorSize: TabBarIndicatorSize.label,
      ),

      // Divider
      dividerTheme: const DividerThemeData(
        color: JojuhuColors.border,
        thickness: 1,
        space: JojuhuSpacing.lg,
      ),

      // Progress Indicator
      progressIndicatorTheme: const ProgressIndicatorThemeData(
        color: JojuhuColors.sol,
        linearTrackColor: JojuhuColors.borderLight,
      ),

      // Snack Bar
      snackBarTheme: SnackBarThemeData(
        backgroundColor: JojuhuColors.textoEscuro,
        contentTextStyle: const TextStyle(color: JojuhuColors.textoClaro),
        shape: RoundedRectangleBorder(
          borderRadius: JojuhuRadius.xsRadius,
        ),
        behavior: SnackBarBehavior.floating,
      ),

      // Chip Theme
      chipTheme: ChipThemeData(
        backgroundColor: JojuhuColors.fundo,
        selectedColor: JojuhuColors.sol.withOpacity(0.2),
        labelStyle: JojuhuTypography.textTheme.bodySmall,
        shape: RoundedRectangleBorder(
          borderRadius: JojuhuRadius.xsRadius,
          side: const BorderSide(color: JojuhuColors.border),
        ),
      ),

      // Dialog Theme
      dialogTheme: DialogThemeData(
        backgroundColor: JojuhuColors.fundoBranco,
        shape: RoundedRectangleBorder(
          borderRadius: JojuhuRadius.mdRadius,
        ),
        titleTextStyle: JojuhuTypography.textTheme.titleLarge,
      ),

      // Bottom Sheet Theme
      bottomSheetTheme: const BottomSheetThemeData(
        backgroundColor: JojuhuColors.fundoBranco,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(
            top: Radius.circular(JojuhuRadius.md),
          ),
        ),
      ),
    );
  }
}