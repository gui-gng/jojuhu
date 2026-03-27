import 'dart:io';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:jojuhu/models/upload.dart';
import 'package:jojuhu/services/api_service.dart';

class ImageUploadWidget extends StatefulWidget {
  final Function(String url) onImageUploaded;
  final UploadType uploadType;
  final int maxImages;
  final List<String> currentImages;

  const ImageUploadWidget({
    super.key,
    required this.onImageUploaded,
    this.uploadType = UploadType.postImage,
    this.maxImages = 4,
    this.currentImages = const [],
  });

  @override
  State<ImageUploadWidget> createState() => _ImageUploadWidgetState();
}

class _ImageUploadWidgetState extends State<ImageUploadWidget> {
  final ImagePicker _picker = ImagePicker();
  bool _isUploading = false;
  double _uploadProgress = 0;

  Future<void> _pickAndUploadImage() async {
    if (widget.currentImages.length >= widget.maxImages) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Maximum ${widget.maxImages} images allowed')),
      );
      return;
    }

    final XFile? image = await _picker.pickImage(
      source: ImageSource.gallery,
      maxWidth: 1200,
      maxHeight: 1200,
      imageQuality: 85,
    );

    if (image == null) return;

    setState(() {
      _isUploading = true;
      _uploadProgress = 0;
    });

    try {
      // Read file
      final file = File(image.path);
      final bytes = await file.readAsBytes();
      final fileName = image.path.split('/').last;
      final contentType = 'image/${fileName.split('.').last}';

      // Generate presigned URL
      final presignedResult = await ApiService.generatePresignedUrl(
        PresignedUrlRequest(
          fileName: fileName,
          contentType: contentType,
          uploadType: widget.uploadType,
        ),
      );

      if (!presignedResult.success || presignedResult.data == null) {
        throw Exception(presignedResult.error ?? 'Failed to generate upload URL');
      }

      final presignedData = presignedResult.data!;

      // Update progress
      setState(() {
        _uploadProgress = 0.5;
      });

      // Upload to MinIO
      final uploadSuccess = await ApiService.uploadToPresignedUrl(
        presignedUrl: presignedData.uploadUrl,
        fileBytes: bytes,
        contentType: contentType,
      );

      if (!uploadSuccess) {
        throw Exception('Failed to upload image');
      }

      setState(() {
        _uploadProgress = 1.0;
      });

      // Notify parent
      widget.onImageUploaded(presignedData.fileUrl);

      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Image uploaded successfully')),
      );
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Upload failed: $e')),
      );
    } finally {
      setState(() {
        _isUploading = false;
        _uploadProgress = 0;
      });
    }
  }

  Future<void> _deleteImage(String url, int index) async {
    // Extract key from URL
    final uri = Uri.parse(url);
    final pathSegments = uri.pathSegments;
    if (pathSegments.length >= 2) {
      final key = pathSegments.sublist(1).join('/'); // Skip bucket name
      
      try {
        await ApiService.deleteFile(key);
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Image deleted')),
        );
      } catch (e) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Failed to delete: $e')),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.currentImages.isNotEmpty)
          SizedBox(
            height: 100,
            child: ListView.builder(
              scrollDirection: Axis.horizontal,
              itemCount: widget.currentImages.length,
              itemBuilder: (context, index) {
                return Stack(
                  children: [
                    Container(
                      margin: const EdgeInsets.only(right: 8),
                      width: 100,
                      height: 100,
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(8),
                        image: DecorationImage(
                          image: NetworkImage(widget.currentImages[index]),
                          fit: BoxFit.cover,
                        ),
                      ),
                    ),
                    Positioned(
                      top: 4,
                      right: 12,
                      child: GestureDetector(
                        onTap: () => _deleteImage(widget.currentImages[index], index),
                        child: Container(
                          padding: const EdgeInsets.all(4),
                          decoration: BoxDecoration(
                            color: Colors.black54,
                            borderRadius: BorderRadius.circular(12),
                          ),
                          child: const Icon(
                            Icons.close,
                            color: Colors.white,
                            size: 16,
                          ),
                        ),
                      ),
                    ),
                  ],
                );
              },
            ),
          ),
        if (widget.currentImages.isNotEmpty) const SizedBox(height: 8),
        if (_isUploading)
          LinearProgressIndicator(value: _uploadProgress)
        else if (widget.currentImages.length < widget.maxImages)
          OutlinedButton.icon(
            onPressed: _pickAndUploadImage,
            icon: const Icon(Icons.image),
            label: Text('Add Image (${widget.currentImages.length}/${widget.maxImages})'),
          ),
      ],
    );
  }
}
