import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'jojuhu_theme.dart';

/// Jojuhu Typography System
/// Uses Playfair Display for headings and Inter for body text

class JojuhuTypography {
  static TextTheme get textTheme {
    return GoogleFonts.interTextTheme(
      _interTextTheme,
    );
  }

  static TextTheme get _interTextTheme {
    return TextTheme(
      displayLarge: GoogleFonts.playfairDisplay(
        fontSize: 57,
        fontWeight: FontWeight.w700,
        color: JojuhuColors.textoEscuro,
        letterSpacing: -0.25,
      ),
      displayMedium: GoogleFonts.playfairDisplay(
        fontSize: 45,
        fontWeight: FontWeight.w700,
        color: JojuhuColors.textoEscuro,
      ),
      displaySmall: GoogleFonts.playfairDisplay(
        fontSize: 36,
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
      ),
      headlineLarge: GoogleFonts.playfairDisplay(
        fontSize: 32,
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
      ),
      headlineMedium: GoogleFonts.playfairDisplay(
        fontSize: 28,
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
      ),
      headlineSmall: GoogleFonts.playfairDisplay(
        fontSize: 24,
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
      ),
      titleLarge: GoogleFonts.inter(
        fontSize: 22,
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
        letterSpacing: 0,
      ),
      titleMedium: GoogleFonts.inter(
        fontSize: 16,
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
        letterSpacing: 0.15,
      ),
      titleSmall: GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
        letterSpacing: 0.1,
      ),
      bodyLarge: GoogleFonts.inter(
        fontSize: 16,
        fontWeight: FontWeight.w400,
        color: JojuhuColors.textoEscuro,
        letterSpacing: 0.5,
      ),
      bodyMedium: GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w400,
        color: JojuhuColors.textoCinza,
        letterSpacing: 0.25,
      ),
      bodySmall: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w400,
        color: JojuhuColors.textoCinza,
        letterSpacing: 0.4,
      ),
      labelLarge: GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w500,
        color: JojuhuColors.textoEscuro,
        letterSpacing: 0.1,
      ),
      labelMedium: GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w500,
        color: JojuhuColors.textoCinza,
        letterSpacing: 0.5,
      ),
      labelSmall: GoogleFonts.inter(
        fontSize: 11,
        fontWeight: FontWeight.w500,
        color: JojuhuColors.textoCinza,
        letterSpacing: 0.5,
      ),
    );
  }

  static TextStyle get headingPlayfair => GoogleFonts.playfairDisplay(
        fontWeight: FontWeight.w600,
        color: JojuhuColors.textoEscuro,
      );

  static TextStyle get bodyInter => GoogleFonts.inter(
        fontWeight: FontWeight.w400,
        color: JojuhuColors.textoCinza,
      );

  static TextStyle get button => GoogleFonts.inter(
        fontSize: 14,
        fontWeight: FontWeight.w600,
        letterSpacing: 0.5,
      );

  static TextStyle get caption => GoogleFonts.inter(
        fontSize: 12,
        fontWeight: FontWeight.w400,
        color: JojuhuColors.textoCinza,
        letterSpacing: 0.4,
      );

  static TextStyle get overline => GoogleFonts.inter(
        fontSize: 10,
        fontWeight: FontWeight.w500,
        color: JojuhuColors.textoCinza,
        letterSpacing: 1.5,
      );
}