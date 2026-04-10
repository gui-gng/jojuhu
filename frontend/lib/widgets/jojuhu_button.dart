import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import '../theme/jojuhu_theme.dart';
import '../theme/tokens.dart';

/// Jojuhu Button Widget
/// Standardized button with multiple variants and states

enum JojuhuButtonVariant { primary, secondary, outline, text, danger }
enum JojuhuButtonSize { small, medium, large }

class JojuhuButton extends StatelessWidget {
  final String text;
  final VoidCallback? onPressed;
  final JojuhuButtonVariant variant;
  final JojuhuButtonSize size;
  final bool isLoading;
  final bool isFullWidth;
  final IconData? icon;
  final IconData? trailingIcon;
  final bool isDisabled;

  const JojuhuButton({
    super.key,
    required this.text,
    this.onPressed,
    this.variant = JojuhuButtonVariant.primary,
    this.size = JojuhuButtonSize.medium,
    this.isLoading = false,
    this.isFullWidth = false,
    this.icon,
    this.trailingIcon,
    this.isDisabled = false,
  });

  double get _height {
    switch (size) {
      case JojuhuButtonSize.small:
        return JojuhuInsets.buttonHeightSm.h;
      case JojuhuButtonSize.medium:
        return JojuhuInsets.buttonHeight.h;
      case JojuhuButtonSize.large:
        return JojuhuInsets.buttonHeightLg.h;
    }
  }

  double get _paddingH {
    switch (size) {
      case JojuhuButtonSize.small:
        return JojuhuSpacing.md.w;
      case JojuhuButtonSize.medium:
        return JojuhuSpacing.lg.w;
      case JojuhuButtonSize.large:
        return JojuhuSpacing.xl.w;
    }
  }

  double get _fontSize {
    switch (size) {
      case JojuhuButtonSize.small:
        return 12.sp;
      case JojuhuButtonSize.medium:
        return 14.sp;
      case JojuhuButtonSize.large:
        return 16.sp;
    }
  }

  double get _iconSize {
    switch (size) {
      case JojuhuButtonSize.small:
        return 16.r;
      case JojuhuButtonSize.medium:
        return 20.r;
      case JojuhuButtonSize.large:
        return 24.r;
    }
  }

  @override
  Widget build(BuildContext context) {
    final isEffectiveDisabled = isDisabled || isLoading;

    Widget buttonChild = Row(
      mainAxisSize: isFullWidth ? MainAxisSize.max : MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        if (isLoading)
          SizedBox(
            width: _iconSize,
            height: _iconSize,
            child: CircularProgressIndicator(
              strokeWidth: 2,
              valueColor: AlwaysStoppedAnimation<Color>(
                _getTextColor(isEffectiveDisabled),
              ),
            ),
          )
        else if (icon != null) ...[
          Icon(icon, size: _iconSize),
          SizedBox(width: JojuhuSpacing.sm.w),
        ],
        Text(
          text,
          style: TextStyle(
            fontSize: _fontSize,
            fontWeight: FontWeight.w600,
            color: _getTextColor(isEffectiveDisabled),
          ),
        ),
        if (trailingIcon != null && !isLoading) ...[SizedBox(width: JojuhuSpacing.sm.w),
          Icon(trailingIcon, size: _iconSize),
        ],
      ],
    );

    switch (variant) {
      case JojuhuButtonVariant.primary:
        return SizedBox(
          height: _height,
          width: isFullWidth ? double.infinity : null,
          child: ElevatedButton(
            onPressed: isEffectiveDisabled ? null : onPressed,
            style: ElevatedButton.styleFrom(
              backgroundColor: JojuhuColors.sol,
              foregroundColor: JojuhuColors.textoClaro,
              disabledBackgroundColor: JojuhuColors.border,
              disabledForegroundColor: JojuhuColors.textoCinza,
              elevation: 0,
              padding: EdgeInsets.symmetric(horizontal: _paddingH),
              shape: RoundedRectangleBorder(
                borderRadius: JojuhuRadius.smRadius,
              ),
            ),
            child: buttonChild,
          ),
        );

      case JojuhuButtonVariant.secondary:
        return SizedBox(
          height: _height,
          width: isFullWidth ? double.infinity : null,
          child: ElevatedButton(
            onPressed: isEffectiveDisabled ? null : onPressed,
            style: ElevatedButton.styleFrom(
              backgroundColor: JojuhuColors.lua,
              foregroundColor: JojuhuColors.textoClaro,
              disabledBackgroundColor: JojuhuColors.border,
              disabledForegroundColor: JojuhuColors.textoCinza,
              elevation: 0,
              padding: EdgeInsets.symmetric(horizontal: _paddingH),
              shape: RoundedRectangleBorder(
                borderRadius: JojuhuRadius.smRadius,
              ),
            ),
            child: buttonChild,
          ),
        );

      case JojuhuButtonVariant.outline:
        return SizedBox(
          height: _height,
          width: isFullWidth ? double.infinity : null,
          child: OutlinedButton(
            onPressed: isEffectiveDisabled ? null : onPressed,
            style: OutlinedButton.styleFrom(
              foregroundColor: JojuhuColors.lua,
              disabledForegroundColor: JojuhuColors.textoCinza,
              side: BorderSide(
                color: isEffectiveDisabled
                    ? JojuhuColors.border
                    : JojuhuColors.lua,
              ),
              padding: EdgeInsets.symmetric(horizontal: _paddingH),
              shape: RoundedRectangleBorder(
                borderRadius: JojuhuRadius.smRadius,
              ),
            ),
            child: buttonChild,
          ),
        );

      case JojuhuButtonVariant.text:
        return SizedBox(
          height: _height,
          width: isFullWidth ? double.infinity : null,
          child: TextButton(
            onPressed: isEffectiveDisabled ? null : onPressed,
            style: TextButton.styleFrom(
              foregroundColor: JojuhuColors.lua,
              disabledForegroundColor: JojuhuColors.textoCinza,
              padding: EdgeInsets.symmetric(horizontal: _paddingH),
            ),
            child: buttonChild,
          ),
        );

      case JojuhuButtonVariant.danger:
        return SizedBox(
          height: _height,
          width: isFullWidth ? double.infinity : null,
          child: ElevatedButton(
            onPressed: isEffectiveDisabled ? null : onPressed,
            style: ElevatedButton.styleFrom(
              backgroundColor: JojuhuColors.error,
              foregroundColor: JojuhuColors.textoClaro,
              disabledBackgroundColor: JojuhuColors.border,
              disabledForegroundColor: JojuhuColors.textoCinza,
              elevation: 0,
              padding: EdgeInsets.symmetric(horizontal: _paddingH),
              shape: RoundedRectangleBorder(
                borderRadius: JojuhuRadius.smRadius,
              ),
            ),
            child: buttonChild,
          ),
        );
    }
  }

  Color _getTextColor(bool isDisabled) {
    if (isDisabled) {
      return JojuhuColors.textoCinza;
    }
    switch (variant) {
      case JojuhuButtonVariant.primary:
      case JojuhuButtonVariant.secondary:
      case JojuhuButtonVariant.danger:
        return JojuhuColors.textoClaro;
      case JojuhuButtonVariant.outline:
      case JojuhuButtonVariant.text:
        return JojuhuColors.lua;
    }
  }
}

/// Icon Button variant
class JojuhuIconButton extends StatelessWidget {
  final IconData icon;
  final VoidCallback? onPressed;
  final Color? backgroundColor;
  final Color? iconColor;
  final double? size;
  final bool isLoading;

  const JojuhuIconButton({
    super.key,
    required this.icon,
    this.onPressed,
    this.backgroundColor,
    this.iconColor,
    this.size,
    this.isLoading = false,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: backgroundColor ?? Colors.transparent,
      borderRadius: JojuhuRadius.smRadius,
      child: InkWell(
        onTap: isLoading ? null : onPressed,
        borderRadius: JojuhuRadius.smRadius,
        child: Padding(
          padding: const EdgeInsets.all(JojuhuSpacing.sm),
          child: isLoading
              ? SizedBox(
                  width: size ?? JojuhuInsets.iconMd,
                  height: size ?? JojuhuInsets.iconMd,
                  child: CircularProgressIndicator(
                    strokeWidth: 2,
                    valueColor: AlwaysStoppedAnimation<Color>(
                      iconColor ?? JojuhuColors.sol,
                    ),
                  ),
                )
              : Icon(
                  icon,
                  size: size ?? JojuhuInsets.iconMd,
                  color: iconColor ?? JojuhuColors.textoEscuro,
                ),
        ),
      ),
    );
  }
}