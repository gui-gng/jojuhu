import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import '../theme/jojuhu_theme.dart';
import '../theme/tokens.dart';
import '../utils/animations.dart';import 'jojuhu_avatar.dart';

/// Jojuhu Post Card Widget
/// Standardized social media post card with animations

class JojuhuPostCard extends StatelessWidget {
  final String postId;
  final String authorName;
  final String? authorAvatar;
  final String? authorId;
  final String content;
  final List<String>? images;
  final int likeCount;
  final int commentCount;
  final int? shareCount;
  final bool isLiked;
  final DateTime createdAt;
  final VoidCallback? onTap;
  final VoidCallback? onLike;
  final VoidCallback? onComment;
  final VoidCallback? onShare;
  final VoidCallback? onAuthorTap;
  final bool showActions;

  const JojuhuPostCard({
    super.key,
    required this.postId,
    required this.authorName,
    this.authorAvatar,
    this.authorId,
    required this.content,
    this.images,
    this.likeCount = 0,
    this.commentCount = 0,
    this.shareCount,
    this.isLiked = false,
    required this.createdAt,
    this.onTap,
    this.onLike,
    this.onComment,
    this.onShare,
    this.onAuthorTap,
    this.showActions = true,
  });

  @override
  Widget build(BuildContext context) {
    return JojuhuCard(
      onTap: onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header with author info
          _buildHeader(context),
          SizedBox(height: JojuhuSpacing.md.h),

          // Content
          if (content.isNotEmpty) ...[Text(
            content,
            style: Theme.of(context).textTheme.bodyLarge,
            maxLines: 10,
            overflow: TextOverflow.ellipsis,
          ),
          SizedBox(height: JojuhuSpacing.md.h),
          ],

          // Images
          if (images != null && images!.isNotEmpty) ...[JojuhuImageGrid(imageUrls: images!), SizedBox(height: JojuhuSpacing.md.h),
          ],

          // Actions
          if (showActions) _buildActions(context),
        ],
      ),
    );
  }

  Widget _buildHeader(BuildContext context) {
    return GestureDetector(
      onTap: onAuthorTap,
      child: Row(
        children: [
          JojuhuAvatar(
            imageUrl: authorAvatar,
            initials: authorName,
            size: JojuhuAvatarSize.md,
          ),
          SizedBox(width: JojuhuSpacing.md.w),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  authorName,
                  style: Theme.of(context).textTheme.titleMedium,
                ),
                SizedBox(height: JojuhuSpacing.xs.h / 2),
                Text(
                  _formatTimeAgo(createdAt),
                  style: Theme.of(context).textTheme.bodySmall,
                ),
              ],
            ),
          ),
          // More options button
          IconButton(
            icon: const Icon(Icons.more_horiz),
            color: JojuhuColors.textoCinza,
            onPressed: () {
              // TODO: Show options menu
            },
          ),
        ],
      ),
    );
  }

  Widget _buildActions(BuildContext context) {
    return Row(
      children: [
        _ActionButton(
          icon: isLiked ? Icons.favorite : Icons.favorite_border,
          label: _formatCount(likeCount),
          isActive: isLiked,
          color: isLiked ? JojuhuColors.error : JojuhuColors.textoCinza,
          onTap: onLike,
        ),
        SizedBox(width: JojuhuSpacing.xl.w),
        _ActionButton(
          icon: Icons.mode_comment_outlined,
          label: _formatCount(commentCount),
          onTap: onComment,
        ),
        if (shareCount != null) ...[SizedBox(width: JojuhuSpacing.xl.w),
          _ActionButton(
            icon: Icons.share_outlined,
            label: _formatCount(shareCount!),
            onTap: onShare,
          ),
        ],
        const Spacer(),
        // Bookmark button
        IconButton(
          icon: const Icon(Icons.bookmark_border),
          color: JojuhuColors.textoCinza,
          onPressed: () {
            // TODO: Implement bookmark
          },
        ),
      ],
    );
  }

  String _formatTimeAgo(DateTime dateTime) {
    final now = DateTime.now();
    final difference = now.difference(dateTime);

    if (difference.inDays > 365) {
      return '${(difference.inDays / 365).floor()}y ago';
    } else if (difference.inDays > 30) {
      return '${(difference.inDays / 30).floor()}mo ago';
    } else if (difference.inDays > 0) {
      return '${difference.inDays}d ago';
    } else if (difference.inHours > 0) {
      return '${difference.inHours}h ago';
    } else if (difference.inMinutes > 0) {
      return '${difference.inMinutes}m ago';
    } else {
      return 'Just now';
    }
  }

  String _formatCount(int count) {
    if (count >= 1000000) {
      return '${(count / 1000000).toStringAsFixed(1)}M';
    } else if (count >= 1000) {
      return '${(count / 1000).toStringAsFixed(1)}K';
    } else {
      return count.toString();
    }
  }
}

class _ActionButton extends StatelessWidget {
  final IconData icon;
  final String label;
  final bool isActive;
  final Color? color;
  final VoidCallback? onTap;

  const _ActionButton({
    required this.icon,
    required this.label,
    this.isActive = false,
    this.color,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: JojuhuRadius.smRadius,
      child: Padding(
        padding: EdgeInsets.symmetric(
          horizontal: JojuhuSpacing.sm.w,
          vertical: JojuhuSpacing.xs.h,
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              icon,
              size: JojuhuInsets.iconSm,
              color: color ?? JojuhuColors.textoCinza,
            ),
            if (label.isNotEmpty) ...[SizedBox(width: JojuhuSpacing.xs.w),
              Text(
                label,
                style: Theme.of(context).textTheme.labelMedium?.copyWith(
                      color: color ?? JojuhuColors.textoCinza,
                    ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}

/// Image grid for post images
class JojuhuImageGrid extends StatelessWidget {
  final List<String> imageUrls;

  const JojuhuImageGrid({
    super.key,
    required this.imageUrls,
  });

  @override
  Widget build(BuildContext context) {
    if (imageUrls.isEmpty) return const SizedBox.shrink();

    if (imageUrls.length == 1) {
      return GestureDetector(
        onTap: () => JojuhuImageViewer.show(
          context: context,
          imageUrl: imageUrls.first,
        ),
        child: ClipRRect(
          borderRadius: JojuhuRadius.smRadius,
          child: CachedNetworkImage(
            imageUrl: imageUrls.first,
            width: double.infinity,
            fit: BoxFit.cover,
            placeholder: (context, url) => Container(
              height: 200.h,
              color: JojuhuColors.fundo,
            ),
            errorWidget: (context, url, error) => Container(
              height: 200.h,
              color: JojuhuColors.fundo,
              child: const Icon(Icons.image_not_supported),
            ),
          ),
        ),
      );
    }

    if (imageUrls.length == 2) {
      return Row(
        children: [
          Expanded(
            child: GestureDetector(
              onTap: () => JojuhuImageViewer.show(
                context: context,
                imageUrl: imageUrls[0],
              ),
              child: _buildImage(imageUrls[0], 120),
            ),
          ),
          SizedBox(width: JojuhuSpacing.xs.w),
          Expanded(
            child: GestureDetector(
              onTap: () => JojuhuImageViewer.show(
                context: context,
                imageUrl: imageUrls[1],
              ),
              child: _buildImage(imageUrls[1], 120),
            ),
          ),
        ],
      );
    }

    return Column(
      children: [
        Row(
          children: [
            Expanded(
              child: GestureDetector(
                onTap: () => JojuhuImageViewer.show(
                  context: context,
                  imageUrl: imageUrls[0],
                ),
                child: _buildImage(imageUrls[0], 120),
              ),
            ),
            SizedBox(width: JojuhuSpacing.xs.w),
            Expanded(
              child: GestureDetector(
                onTap: () => JojuhuImageViewer.show(
                  context: context,
                  imageUrl: imageUrls[1],
                ),
                child: _buildImage(imageUrls[1], 120),
              ),
            ),
          ],
        ),
        if (imageUrls.length > 2) ...[SizedBox(height: JojuhuSpacing.xs.h),
          Row(
            children: [
              Expanded(
                child: GestureDetector(
                  onTap: () => JojuhuImageViewer.show(
                    context: context,
                    imageUrl: imageUrls[2],
                  ),
                  child: _buildImage(
                    imageUrls[2],
                    120,
                    overlay: imageUrls.length > 3
                        ? '+${imageUrls.length - 3}'
                        : null,
                  ),
                ),
              ),
              if (imageUrls.length > 3) ...[SizedBox(width: JojuhuSpacing.xs.w),Expanded(
                  child: GestureDetector(
                    onTap: () => JojuhuImageViewer.show(
                      context: context,
                      imageUrl: imageUrls[3],
                    ),
                    child: _buildImage(imageUrls[3], 120),
                  ),
                ),
              ],
            ],
          ),
        ],
      ],
    );
  }

  Widget _buildImage(String url, double height, {String? overlay}) {
    return Stack(
      children: [
        ClipRRect(
          borderRadius: JojuhuRadius.smRadius,
          child: CachedNetworkImage(
            imageUrl: url,
            width: double.infinity,
            height: height.h,
            fit: BoxFit.cover,
            placeholder: (context, url) => Container(
              height: height.h,
              color: JojuhuColors.fundo,
            ),
            errorWidget: (context, url, error) => Container(
              height: height.h,
              color: JojuhuColors.fundo,
              child: const Icon(Icons.image_not_supported),
            ),
          ),
        ),
        if (overlay != null)
          Positioned.fill(
            child: Container(
              decoration: BoxDecoration(
                color: JojuhuColors.overlay,
                borderRadius: JojuhuRadius.smRadius,
              ),
              child: Center(
                child: Text(
                  overlay,
                  style: const TextStyle(
                    color: JojuhuColors.textoClaro,
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
          ),
      ],
    );
  }
}