import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:photo_view/photo_view.dart';
import '../theme/jojuhu_theme.dart';
import '../theme/tokens.dart';

/// Jojuhu Image Viewer Widget
/// Full-screen image view with zoom capability

class JojuhuImageViewer extends StatelessWidget {
  final String imageUrl;
  final String? title;
  final Color? backgroundColor;

  const JojuhuImageViewer({
    super.key,
    required this.imageUrl,
    this.title,
    this.backgroundColor,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: backgroundColor ?? Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: title != null
            ? Text(
                title!,
                style: const TextStyle(color: JojuhuColors.textoClaro),
              )
            : null,
        leading: IconButton(
          icon: const Icon(Icons.close, color: JojuhuColors.textoClaro),
          onPressed: () => Navigator.of(context).pop(),
        ),
      ),
      body: PhotoView(
        imageProvider: CachedNetworkImageProvider(imageUrl),
        backgroundDecoration: BoxDecoration(
          color: backgroundColor ?? Colors.black,
        ),
        loadingBuilder: (context, event) => Center(
          child: CircularProgressIndicator(
            value: event?.expectedTotalBytes != null
                ? event!.cumulativeBytesLoaded / event.expectedTotalBytes!
                : null,
            color: JojuhuColors.sol,
          ),
        ),
        errorBuilder: (context, error, stackTrace) => Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Icon(
                Icons.broken_image,
                color: JojuhuColors.textoCinza,
                size: 64,
              ),
              SizedBox(height: JojuhuSpacing.lg.h),
              Text(
                'Failed to load image',
                style: TextStyle(
                  color: JojuhuColors.textoCinza,
                  fontSize: 14.sp,
                ),
              ),
            ],
          ),
        ),
        minScale: PhotoViewComputedScale.contained,
        maxScale: PhotoViewComputedScale.covered * 3,
        initialScale: PhotoViewComputedScale.contained,
      ),
    );
  }

  static Future<void> show({
    required BuildContext context,
    required String imageUrl,
    String? title,
  }) {
    return Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => JojuhuImageViewer(
          imageUrl: imageUrl,
          title: title,
        ),
      ),
    );
  }
}

/// Image thumbnail with tap-to-view functionality
class JojuhuImageThumbnail extends StatelessWidget {
  final String imageUrl;
  final double? width;
  final double? height;
  final BoxFit fit;
  final double borderRadius;
  final String? heroTag;

  const JojuhuImageThumbnail({
    super.key,
    required this.imageUrl,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
    this.borderRadius = JojuhuRadius.sm,
    this.heroTag,
  });

  @override
  Widget build(BuildContext context) {
    Widget image = ClipRRect(
      borderRadius: BorderRadius.circular(borderRadius),
      child: CachedNetworkImage(
        imageUrl: imageUrl,
        width: width?.w,
        height: height?.h,
        fit: fit,
        placeholder: (context, url) => Container(
          width: width?.w,
          height: height?.h,
          color: JojuhuColors.fundo,
          child: Center(
            child: SizedBox(
              width: 24.r,
              height: 24.r,
              child: const CircularProgressIndicator(
                strokeWidth: 2,
                color: JojuhuColors.sol,
              ),
            ),
          ),
        ),
        errorWidget: (context, url, error) => Container(
          width: width?.w,
          height: height?.h,
          color: JojuhuColors.fundo,
          child: Icon(
            Icons.image_not_supported,
            color: JojuhuColors.textoCinza,
            size: 32.r,
          ),
        ),
      ),
    );

    if (heroTag != null) {
      image = Hero(
        tag: heroTag!,
        child: image,
      );
    }

    return GestureDetector(
      onTap: () => show(context: context, imageUrl: imageUrl),
      child: image,
    );
  }
}

/// Image grid for displaying multiple images
class JojuhuImageGrid extends StatelessWidget {
  final List<String> imageUrls;
  final int maxImages;
  final double spacing;
  final double borderRadius;

  const JojuhuImageGrid({
    super.key,
    required this.imageUrls,
    this.maxImages = 4,
    this.spacing = JojuhuSpacing.sm,
    this.borderRadius = JojuhuRadius.sm,
  });

  @override
  Widget build(BuildContext context) {
    if (imageUrls.isEmpty) return const SizedBox.shrink();

    final displayImages = imageUrls.take(maxImages).toList();
    final remainingCount = imageUrls.length - maxImages;

    if (displayImages.length == 1) {
      return _buildSingleImage(context, displayImages.first);
    }

    if (displayImages.length == 2) {
      return _buildTwoImages(context, displayImages, spacing);
    }

    return _buildMultipleImages(
      context,
      displayImages,
      remainingCount,
      spacing,
    );
  }

  Widget _buildSingleImage(BuildContext context, String url) {
    return JojuhuImageThumbnail(
      imageUrl: url,
      width: double.infinity,// height will be calculated aspect ratio
      borderRadius: borderRadius,
      heroTag: 'image_${url}_0',
    );
  }

  Widget _buildTwoImages(BuildContext context, List<String> urls, double spacing) {
    return Row(
      children: [
        Expanded(
          child: JojuhuImageThumbnail(
            imageUrl: urls[0],
            height: 150,
            borderRadius: borderRadius,
            heroTag: 'image_${urls[0]}_0',
          ),
        ),
        SizedBox(width: spacing.w),
        Expanded(
          child: JojuhuImageThumbnail(
            imageUrl: urls[1],
            height: 150,
            borderRadius: borderRadius,
            heroTag: 'image_${urls[1]}_1',
          ),
        ),
      ],
    );
  }

  Widget _buildMultipleImages(
    BuildContext context,
    List<String> urls,
    int remainingCount,
    double spacing,
  ) {
    return Column(
      children: [
        Row(
          children: [
            Expanded(
              child: JojuhuImageThumbnail(
                imageUrl: urls[0],
                height: 120,
                borderRadius: borderRadius,
                heroTag: 'image_${urls[0]}_0',
              ),
            ),
            SizedBox(width: spacing.w),
            Expanded(
              child: JojuhuImageThumbnail(
                imageUrl: urls[1],
                height: 120,
                borderRadius: borderRadius,
                heroTag: 'image_${urls[1]}_1',
              ),
            ),
          ],
        ),
        SizedBox(height: spacing.h),
        Row(
          children: [
            Expanded(
              child: Stack(
                children: [
                  JojuhuImageThumbnail(
                    imageUrl: urls[2],
                    height: 120,
                    borderRadius: borderRadius,
                    heroTag: 'image_${urls[2]}_2',
                  ),
                  if (remainingCount > 0 && urls.length > 3)
                    Positioned.fill(
                      child: Container(
                        decoration: BoxDecoration(
                          color: JojuhuColors.overlay,
                          borderRadius: BorderRadius.circular(borderRadius),
                        ),
                        child: Center(
                          child: Text(
                            '+$remainingCount',
                            style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                                  color: JojuhuColors.textoClaro,
                                ),
                          ),
                        ),
                      ),
                    ),
                ],
              ),
            ),
            if (urls.length > 3) ...[SizedBox(width: spacing.w),
              Expanded(
                child: JojuhuImageThumbnail(
                  imageUrl: urls[3],
                  height: 120,
                  borderRadius: borderRadius,
                  heroTag: 'image_${urls[3]}_3',
                ),
              ),
            ] else
              const Spacer(),
          ],
        ),
      ],
    );
  }
}