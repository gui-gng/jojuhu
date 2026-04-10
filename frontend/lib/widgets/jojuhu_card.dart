import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import '../theme/jojuhu_theme.dart';
import '../theme/tokens.dart';

/// Jojuhu Card Widget
/// Standardized card container with elevation options

enum JojuhuCardElevation { none, low, medium, high }

class JojuhuCard extends StatelessWidget {
  final Widget child;
  final VoidCallback? onTap;
  final VoidCallback? onLongPress;
  final JojuhuCardElevation elevation;
  final Color? color;
  final EdgeInsetsGeometry? padding;
  final EdgeInsetsGeometry? margin;
  final double? borderRadius;
  final double? width;
  final double? height;

  const JojuhuCard({
    super.key,
    required this.child,
    this.onTap,
    this.onLongPress,
    this.elevation = JojuhuCardElevation.low,
    this.color,
    this.padding,
    this.margin,
    this.borderRadius,
    this.width,
    this.height,
  });

  double get _elevation {
    switch (elevation) {
      case JojuhuCardElevation.none:
        return JojuhuElevation.none;
      case JojuhuCardElevation.low:
        return JojuhuElevation.low;
      case JojuhuCardElevation.medium:
        return JojuhuElevation.medium;
      case JojuhuCardElevation.high:
        return JojuhuElevation.high;
    }
  }

  @override
  Widget build(BuildContext context) {
    final card = Container(
      width: width,
      height: height,
      margin: margin,
      child: Card(
        color: color ?? JojuhuColors.fundoBranco,
        elevation: _elevation,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(borderRadius ?? JojuhuRadius.sm),
        ),
        child: Padding(
          padding: padding ?? JojuhuSpacing.paddingCard,
          child: child,
        ),
      ),
    );

    if (onTap != null || onLongPress != null) {
      return InkWell(
        onTap: onTap,
        onLongPress: onLongPress,
        borderRadius: BorderRadius.circular(borderRadius ?? JojuhuRadius.sm),
        child: card,
      );
    }

    return card;
  }
}

/// Specialized card for list items
class JojuhuListTileCard extends StatelessWidget {
  final Widget? leading;
  final String title;
  final String? subtitle;
  final Widget? trailing;
  final VoidCallback? onTap;
  final VoidCallback? onLongPress;
  final EdgeInsetsGeometry? padding;

  const JojuhuListTileCard({
    super.key,
    this.leading,
    required this.title,
    this.subtitle,
    this.trailing,
    this.onTap,
    this.onLongPress,
    this.padding,
  });

  @override
  Widget build(BuildContext context) {
    return JojuhuCard(
      onTap: onTap,
      onLongPress: onLongPress,
      padding: padding ?? EdgeInsets.zero,
      child: ListTile(
        leading: leading,
        title: Text(
          title,
          style: Theme.of(context).textTheme.titleMedium,
        ),
        subtitle: subtitle != null
            ? Text(
                subtitle!,
                style: Theme.of(context).textTheme.bodySmall,
              )
            : null,
        trailing: trailing,
        contentPadding: padding ?? const EdgeInsets.symmetric(
          horizontal: JojuhuSpacing.lg,
          vertical: JojuhuSpacing.xs,
        ),
      ),
    );
  }
}

/// Loading skeleton for cards
class JojuhuCardSkeleton extends StatelessWidget {
  final double? width;
  final double? height;

  const JojuhuCardSkeleton({
    super.key,
    this.width,
    this.height,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: width,
      height: height ?? 120.h,
      margin: const EdgeInsets.symmetric(vertical: JojuhuSpacing.sm),
      decoration: BoxDecoration(
        color: JojuhuColors.fundoBranco,
        borderRadius: JojuhuRadius.smRadius,
        border: Border.all(color: JojuhuColors.border),
      ),
    );
  }
}