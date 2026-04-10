import 'package:flutter/material.dart';
import 'package:shimmer/shimmer.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import '../theme/jojuhu_theme.dart';
import '../theme/tokens.dart';

/// Jojuhu Shimmer Widgets
/// Loading placeholder widgets with animation

class JojuhuShimmer extends StatelessWidget {
  final double width;
  final double height;
  final double borderRadius;

  const JojuhuShimmer({
    super.key,
    required this.width,
    required this.height,
    this.borderRadius = JojuhuRadius.sm,
  });

  @override
  Widget build(BuildContext context) {
    return Shimmer.fromColors(
      baseColor: JojuhuColors.borderLight,
      highlightColor: JojuhuColors.fundo,
      child: Container(
        width: width.w,
        height: height.h,
        decoration: BoxDecoration(
          color: JojuhuColors.fundoBranco,
          borderRadius: BorderRadius.circular(borderRadius),
        ),
      ),
    );
  }
}

/// Shimmer for text lines
class JojuhuTextShimmer extends StatelessWidget {
  final double width;
  final double height;
  final double borderRadius;

  const JojuhuTextShimmer({
    super.key,
    this.width = double.infinity,
    this.height = 14,
    this.borderRadius = JojuhuRadius.xs,
  });

  @override
  Widget build(BuildContext context) {
    return JojuhuShimmer(
      width: width == double.infinity ? 200 : width,
      height: height,
      borderRadius: borderRadius,
    );
  }
}

/// Shimmer for circular avatars
class JojuhuCircleShimmer extends StatelessWidget {
  final double size;

  const JojuhuCircleShimmer({
    super.key,
    required this.size,
  });

  @override
  Widget build(BuildContext context) {
    return Shimmer.fromColors(
      baseColor: JojuhuColors.borderLight,
      highlightColor: JojuhuColors.fundo,
      child: Container(
        width: size.r,
        height: size.r,
        decoration: const BoxDecoration(
          color: JojuhuColors.fundoBranco,
          shape: BoxShape.circle,
        ),
      ),
    );
  }
}

/// Post card shimmer
class JojuhuPostCardShimmer extends StatelessWidget {
  const JojuhuPostCardShimmer({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(JojuhuSpacing.lg),
      decoration: BoxDecoration(
        color: JojuhuColors.fundoBranco,
        borderRadius: JojuhuRadius.smRadius,
        border: Border.all(color: JojuhuColors.border),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              JojuhuCircleShimmer(size: JojuhuInsets.avatarMd),
              SizedBox(width: JojuhuSpacing.md.w),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    JojuhuTextShimmer(width: 120, height: 14),
                    SizedBox(height: JojuhuSpacing.xs.h),
                    JojuhuTextShimmer(width: 80, height: 12),
                  ],
                ),
              ),
            ],
          ),
          SizedBox(height: JojuhuSpacing.md.h),
          JojuhuTextShimmer(width: double.infinity, height: 14),
          SizedBox(height: JojuhuSpacing.xs.h),
          JojuhuTextShimmer(width: double.infinity, height: 14),
          SizedBox(height: JojuhuSpacing.xs.h),
          JojuhuTextShimmer(width: 200, height: 14),
          SizedBox(height: JojuhuSpacing.md.h),
          JojuhuShimmer(
            width: double.infinity,
            height: 150,
            borderRadius: JojuhuRadius.sm,
          ),
          SizedBox(height: JojuhuSpacing.md.h),
          Row(
            children: [
              JojuhuTextShimmer(width: 60, height: 12),
              SizedBox(width: JojuhuSpacing.lg.w),
              JojuhuTextShimmer(width: 60, height: 12),
              SizedBox(width: JojuhuSpacing.lg.w),
              JojuhuTextShimmer(width: 60, height: 12),
            ],
          ),
        ],
      ),
    );
  }
}

///List tile shimmer
class JojuhuListTileShimmer extends StatelessWidget {
  final bool hasLeading;
  final bool hasTrailing;
  final int subtitleLines;

  const JojuhuListTileShimmer({
    super.key,
    this.hasLeading = true,
    this.hasTrailing = false,
    this.subtitleLines = 1,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(
        horizontal: JojuhuSpacing.lg,
        vertical: JojuhuSpacing.md,
      ),
      child: Row(
        children: [
          if (hasLeading) ...[
            JojuhuCircleShimmer(size: JojuhuInsets.avatarMd),
            SizedBox(width: JojuhuSpacing.md.w),
          ],
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                JojuhuTextShimmer(width: 150, height: 14),
                if (subtitleLines > 0) ...[SizedBox(height: JojuhuSpacing.xs.h),
                  ...List.generate(
                    subtitleLines,
                    (index) => Padding(
                      padding: EdgeInsets.only(top: index > 0 ? JojuhuSpacing.xs.h : 0),
                      child: JojuhuTextShimmer(
                        width: index == subtitleLines - 1 ? 100 : double.infinity,
                        height: 12,
                      ),
                    ),
                  ),
                ],
              ],
            ),
          ),
          if (hasTrailing) ...[SizedBox(width: JojuhuSpacing.md.w),
            JojuhuCircleShimmer(size: JojuhuInsets.iconSm),
          ],
        ],
      ),
    );
  }
}

/// Grid shimmer for image or card grids
class JojuhuGridShimmer extends StatelessWidget {
  final int itemCount;
  final int crossAxisCount;
  final double itemHeight;
  final double spacing;

  const JojuhuGridShimmer({
    super.key,
    this.itemCount = 6,
    this.crossAxisCount = 2,
    this.itemHeight = 180,
    this.spacing = JojuhuSpacing.md,
  });

  @override
  Widget build(BuildContext context) {
    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: crossAxisCount,
        mainAxisSpacing: spacing.h,
        crossAxisSpacing: spacing.w,
        mainAxisExtent: itemHeight.h,
      ),
      itemCount: itemCount,
      itemBuilder: (context, index) => JojuhuShimmer(
        width: double.infinity,
        height: itemHeight,
        borderRadius: JojuhuRadius.sm,
      ),
    );
  }
}

/// Full page loading shimmer
class JojuhuPageShimmer extends StatelessWidget {
  final int itemCount;

  const JojuhuPageShimmer({
    super.key,
    this.itemCount = 5,
  });

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Padding(
        padding: JojuhuSpacing.paddingScreen,
        child: Column(
          children: List.generate(
            itemCount,
            (index) => const Padding(
              padding: EdgeInsets.only(bottom: JojuhuSpacing.md),
              child: JojuhuPostCardShimmer(),
            ),
          ),
        ),
      ),
    );
  }
}