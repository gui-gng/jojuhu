/// Request to generate a presigned upload URL
class PresignedUrlRequest {
  final String fileName;
  final String contentType;
  final UploadType uploadType;

  PresignedUrlRequest({
    required this.fileName,
    required this.contentType,
    required this.uploadType,
  });

  Map<String, dynamic> toJson() {
    return {
      'file_name': fileName,
      'content_type': contentType,
      'upload_type': uploadType.value,
    };
  }
}

enum UploadType {
  avatar('avatar'),
  postImage('post_image');

  final String value;
  const UploadType(this.value);
}

/// Response with presigned URL
class PresignedUrlResponse {
  final String uploadUrl;
  final String fileUrl;
  final String key;
  final int expiresIn;

  PresignedUrlResponse({
    required this.uploadUrl,
    required this.fileUrl,
    required this.key,
    required this.expiresIn,
  });

  factory PresignedUrlResponse.fromJson(Map<String, dynamic> json) {
    return PresignedUrlResponse(
      uploadUrl: json['upload_url'],
      fileUrl: json['file_url'],
      key: json['key'],
      expiresIn: json['expires_in'] ?? 300,
    );
  }
}

/// Request to confirm upload completion
class ConfirmUploadRequest {
  final String key;

  ConfirmUploadRequest({required this.key});

  Map<String, dynamic> toJson() {
    return {'key': key};
  }
}
