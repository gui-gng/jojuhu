import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:shimmer/shimmer.dart';
import '../theme/jojuhu_theme.dart';
import '../theme/tokens.dart';

/// Jojuhu Avatar Widget
/// Standardized avatar with caching, sizes, and optional badge

enum JojuhuAvatarSize { xs, sm, md, lg, xl }

class JojuhuAvatar extends StatelessWidget {
  final String? imageUrl;
  final String? initials;
  final JojuhuAvatarSize size;
  final VoidCallback? onTap;
  final Widget? badge;
  final Color? backgroundColor;
  final Color? textColor;
  final bool showOnlineStatus;
  final bool isOnline;

  const JojuhuAvatar({
    super.key,
    this.imageUrl,
    this.initials,
    this.size = JojuhuAvatarSize.md,
    this.onTap,
    this.badge,
    this.backgroundColor,
    this.textColor,
    this.showOnlineStatus = false,
    this.isOnline = false,
  });

  double get _size {
    switch (size) {
      case JojuhuAvatarSize.xs:
        return 24.r;
      case JojuhuAvatarSize.sm:
        return JojuhuInsets.avatarSm.r;
      case JojuhuAvatarSize.md:
        return JojuhuInsets.avatarMd.r;
      case JojuhuAvatarSize.lg:
        return JojuhuInsets.avatarLg.r;
      case JojuhuAvatarSize.xl:
        return JojuhuInsets.avatarXl.r;
    }
  }

  double get _fontSize {
    switch (size) {
      case JojuhuAvatarSize.xs:
        return 10.sp;
      case JojuhuAvatarSize.sm:
        return 12.sp;
      case JojuhuAvatarSize.md:
        return 14.sp;
      case JojuhuAvatarSize.lg:
        return 18.sp;
      case JojuhuAvatarSize.xl:
        return 24.sp;
    }
  }

  @override
  Widget build(BuildContext context) {
    final avatarColors = [
      JojuhuColors.sol,
      JojuhuColors.lua,
      JojuhuColors.encontro,
      JojuhuColors.success,
      JojuhuColors.info,
    ];

    final bgColor = backgroundColor ??
        (initials != null
            ? avatarColors[initials!.hashCode % avatarColors.length]
            : JojuhuColors.textoCinza);

    Widget avatar;

    if (imageUrl != null && imageUrl!.isNotEmpty) {
      avatar = ClipOval(
        child: CachedNetworkImage(
          imageUrl: imageUrl!,
          width: _size,
          height: _size,
          fit: BoxFit.cover,
          placeholder: (context, url) => Shimmer.fromColors(
            baseColor: JojuhuColors.borderLight,
            highlightColor: JojuhuColors.fundo,
            child: Container(
              width: _size,
              height: _size,
              decoration: const BoxDecoration(
                color: JojuhuColors.fundoBranco,
                shape: BoxShape.circle,
              ),
            ),
          ),
          errorWidget: (context, url, error) => _buildInitials(bgColor),
        ),
      );
    } else if (initials != null && initials!.isNotEmpty) {
      avatar = _buildInitials(bgColor);
    } else {
      avatar = Container(
        width: _size,
        height: _size,
        decoration: BoxDecoration(
          color: bgColor,
          shape: BoxShape.circle,
        ),
        child: Icon(
          Icons.person,
          size: _size * 0.5,
          color: textColor ?? JojuhuColors.textoClaro,
        ),
      );
    }

    if (showOnlineStatus || badge != null) {
      avatar = Stack(
        clipBehavior: Clip.none,
        children: [
          avatar,
          if (showOnlineStatus)
            Positioned(
              right: 0,
              bottom: 0,
              child: Container(
                width: _size * 0.3,
                height: _size * 0.3,
                decoration: BoxDecoration(
                  color: isOnline ? JojuhuColors.success : JojuhuColors.textoCinza,
                  shape: BoxShape.circle,
                  border: Border.all(
                    color: JojuhuColors.fundoBranco,
                    width: 2,
                  ),
                ),
              ),
            ),
          if (badge != null)
            Positioned(
              right: -4,
              top: -4,
              child: badge!,
            ),
        ],
      );
    }

    if (onTap != null) {
      return GestureDetector(
        onTap: onTap,
        child: avatar,
      );
    }

    return avatar;
  }

  Widget _buildInitials(Color bgColor) {
    return Container(
      width: _size,
      height: _size,
      decoration: BoxDecoration(
        color: bgColor,
        shape: BoxShape.circle,
      ),
      alignment: Alignment.center,
      child: Text(
        initials!.substring(0, initials!.length > 2 ? 2 : initials!.length).toUpperCase(),
        style: TextStyle(
          color: textColor ?? JojuhuColors.textoClaro,
          fontSize: _fontSize,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }
}

class JojuhuAvatarSkeleton extends StatelessWidget {
  final JojuhuAvatarSize size;

  const JojuhuAvatarSkeleton({
    super.key,
    this.size = JojuhuAvatarSize.md,
  });

  double get _size {
    switch (size) {
      case JojuhuAvatarSize.xs:
        return 24.r;
      case JojuhuAvatarSize.sm:
        return JojuhuInsets.avatarSm.r;
      case JojuhuAvatarSize.md:
        return JojuhuInsets.avatarMd.r;
      case JojuhuAvatarSize.lg:
        return JojuhuInsets.avatarLg.r;
      case JojuhuAvatarSize.xl:
        return JojuhuInsets.avatarXl.r;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Shimmer.fromColors(
      baseColor: JojuhuColors.borderLight,
      highlightColor: JojuhuColors.fundo,
      child: Container(
        width: _size,
        height: _size,
        decoration: const BoxDecoration(
          color: JojuhuColors.fundoBranco,
          shape: BoxShape.circle,
        ),
      ),
    );
  }
}