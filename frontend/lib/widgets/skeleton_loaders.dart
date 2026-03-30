import 'package:flutter/material.dart';
import 'package:shimmer/shimmer.dart';

/// Skeleton loader for feed posts
class PostSkeleton extends StatelessWidget {
  const PostSkeleton({super.key});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Shimmer.fromColors(
          baseColor: Colors.grey[300]!,
          highlightColor: Colors.grey[100]!,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Author header skeleton
              Row(
                children: [
                  Container(
                    width: 40,
                    height: 40,
                    decoration: const BoxDecoration(
                      color: Colors.white,
                      shape: BoxShape.circle,
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Container(
                          width: 120,
                          height: 16,
                          color: Colors.white,
                        ),
                        const SizedBox(height: 4),
                        Container(
                          width: 80,
                          height: 12,
                          color: Colors.white,
                        ),
                      ],
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 16),
              // Content skeleton
              Container(
                width: double.infinity,
                height: 12,
                color: Colors.white,
              ),
              const SizedBox(height: 8),
              Container(
                width: double.infinity,
                height: 12,
                color: Colors.white,
              ),
              const SizedBox(height: 8),
              Container(
                width: 200,
                height: 12,
                color: Colors.white,
              ),
              const SizedBox(height: 16),
              // Actions skeleton
              Row(
                children: [
                  Expanded(
                    child: Container(
                      height: 32,
                      color: Colors.white,
                    ),
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Container(
                      height: 32,
                      color: Colors.white,
                    ),
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Container(
                      height: 32,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
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
      itemCount: itemCount,
      itemBuilder: (context, index) {
        return const PostSkeleton();
      },
    );
  }
}