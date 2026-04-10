import 'package:flutter/material.dart';
import 'package:shimmer/shimmer.dart';
import 'jojuhu_shimmer.dart';

/// Skeleton loader for feed posts
/// Maintained for backward compatibility
/// Prefer using JojuhuPostCardShimmer from jojuhu_shimmer.dart
class PostSkeleton extends StatelessWidget {
  const PostSkeleton({super.key});

  @override
  Widget build(BuildContext context) {
    return const JojuhuPostCardShimmer();
  }
}

/// Skeleton loader list for feed
class FeedSkeletonList extends StatelessWidget {
  final int itemCount;

  const FeedSkeletonList({
    super.key,
    this.itemCount = 5,
  });

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      physics: const NeverScrollableScrollPhysics(),
      shrinkWrap: true,
      padding: EdgeInsets.zero,
      itemCount: itemCount,
      itemBuilder: (context, index) {
        return const Padding(
          padding: EdgeInsets.only(bottom: 12),
          child: JojuhuPostCardShimmer(),
        );
      },
    );
  }
}