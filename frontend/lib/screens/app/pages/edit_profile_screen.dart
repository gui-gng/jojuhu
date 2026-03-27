import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:jojuhu/models/upload.dart';
import 'package:jojuhu/models/user_profile.dart';
import 'package:jojuhu/services/api_service.dart';

class EditProfileScreen extends StatefulWidget {
  const EditProfileScreen({super.key});

  @override
  State<EditProfileScreen> createState() => _EditProfileScreenState();
}

class _EditProfileScreenState extends State<EditProfileScreen> {
  final _formKey = GlobalKey<FormState>();
  final _displayNameController = TextEditingController();
  final _bioController = TextEditingController();
  
  MyProfile? _profile;
  bool _isLoading = true;
  bool _isSaving = false;
  String? _error;
  
  // Avatar upload
  File? _selectedAvatar;
  String? _currentAvatarUrl;
  bool _isUploadingAvatar = false;

  @override
  void initState() {
    super.initState();
    _loadProfile();
  }

  @override
  void dispose() {
    _displayNameController.dispose();
    _bioController.dispose();
    super.dispose();
  }

  Future<void> _loadProfile() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    final result = await ApiService.getMyProfile();
    
    if (result.success && result.data != null) {
      setState(() {
        _profile = result.data;
        _currentAvatarUrl = result.data!.avatarUrl;
        _displayNameController.text = result.data!.displayName ?? '';
        _bioController.text = result.data!.bio ?? '';
      });
    } else {
      setState(() {
        _error = result.error ?? 'Failed to load profile';
      });
    }

    setState(() {
      _isLoading = false;
    });
  }

  Future<void> _pickAvatar() async {
    final ImagePicker picker = ImagePicker();
    final XFile? image = await picker.pickImage(
      source: ImageSource.gallery,
      maxWidth: 512,
      maxHeight: 512,
      imageQuality: 85,
    );

    if (image != null) {
      setState(() {
        _selectedAvatar = File(image.path);
      });
    }
  }

  Future<void> _uploadAvatar() async {
    if (_selectedAvatar == null) return;

    setState(() {
      _isUploadingAvatar = true;
    });

    try {
      // Read file bytes
      final bytes = await _selectedAvatar!.readAsBytes();
      
      // Get file info
      final fileName = _selectedAvatar!.path.split('/').last;
      final contentType = 'image/${fileName.split('.').last}';

      // Generate presigned URL
      final presignedResult = await ApiService.generatePresignedUrl(
        PresignedUrlRequest(
          fileName: fileName,
          contentType: contentType,
          uploadType: UploadType.avatar,
        ),
      );

      if (!presignedResult.success || presignedResult.data == null) {
        throw Exception(presignedResult.error ?? 'Failed to generate upload URL');
      }

      final presignedData = presignedResult.data!;

      // Upload to MinIO
      final uploadSuccess = await ApiService.uploadToPresignedUrl(
        presignedUrl: presignedData.uploadUrl,
        fileBytes: bytes,
        contentType: contentType,
      );

      if (!uploadSuccess) {
        throw Exception('Failed to upload image');
      }

      // Confirm avatar upload
      final confirmResult = await ApiService.confirmAvatarUpload(presignedData.key);
      
      if (!confirmResult.success) {
        throw Exception(confirmResult.error ?? 'Failed to confirm upload');
      }

      setState(() {
        _currentAvatarUrl = presignedData.fileUrl;
        _selectedAvatar = null;
      });

      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Avatar updated successfully')),
      );
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Failed to upload avatar: $e')),
      );
    } finally {
      setState(() {
        _isUploadingAvatar = false;
      });
    }
  }

  Future<void> _saveProfile() async {
    if (!_formKey.currentState!.validate()) return;

    setState(() {
      _isSaving = true;
    });

    // Upload avatar first if selected
    if (_selectedAvatar != null) {
      await _uploadAvatar();
    }

    // Update profile
    final displayName = _displayNameController.text.trim();
    final bio = _bioController.text.trim();

    final result = await ApiService.updateProfile(
      displayName: displayName.isNotEmpty ? displayName : null,
      bio: bio.isNotEmpty ? bio : null,
    );

    setState(() {
      _isSaving = false;
    });

    if (result.success) {
      Navigator.pop(context, true);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(result.error ?? 'Failed to save profile')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      );
    }

    if (_error != null) {
      return Scaffold(
        appBar: AppBar(title: const Text('Edit Profile')),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(_error!, style: const TextStyle(color: Colors.red)),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: _loadProfile,
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Edit Profile'),
        actions: [
          if (_isSaving || _isUploadingAvatar)
            const Center(
              child: Padding(
                padding: EdgeInsets.all(16),
                child: SizedBox(
                  width: 20,
                  height: 20,
                  child: CircularProgressIndicator(strokeWidth: 2),
                ),
              ),
            )
          else
            TextButton(
              onPressed: _saveProfile,
              child: const Text('Save'),
            ),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Form(
          key: _formKey,
          child: Column(
            children: [
              // Avatar
              GestureDetector(
                onTap: _pickAvatar,
                child: Stack(
                  alignment: Alignment.bottomRight,
                  children: [
                    CircleAvatar(
                      radius: 60,
                      backgroundImage: _selectedAvatar != null
                          ? FileImage(_selectedAvatar!)
                          : _currentAvatarUrl != null
                              ? NetworkImage(_currentAvatarUrl!)
                              : null,
                      child: _selectedAvatar == null && _currentAvatarUrl == null
                          ? Text(
                              _profile?.username[0].toUpperCase() ?? '?',
                              style: const TextStyle(fontSize: 48),
                            )
                          : null,
                    ),
                    Container(
                      padding: const EdgeInsets.all(4),
                      decoration: BoxDecoration(
                        color: Theme.of(context).primaryColor,
                        shape: BoxShape.circle,
                      ),
                      child: const Icon(
                        Icons.camera_alt,
                        color: Colors.white,
                        size: 20,
                      ),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 8),
              TextButton(
                onPressed: _pickAvatar,
                child: const Text('Change Avatar'),
              ),
              if (_selectedAvatar != null) ...[
                TextButton(
                  onPressed: _isUploadingAvatar ? null : _uploadAvatar,
                  child: _isUploadingAvatar
                      ? const SizedBox(
                          width: 16,
                          height: 16,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Text('Upload Now'),
                ),
              ],
              const SizedBox(height: 24),
              
              // Display Name
              TextFormField(
                controller: _displayNameController,
                decoration: const InputDecoration(
                  labelText: 'Display Name',
                  hintText: 'How you want to be called',
                  prefixIcon: Icon(Icons.person),
                ),
                maxLength: 100,
              ),
              const SizedBox(height: 16),
              
              // Bio
              TextFormField(
                controller: _bioController,
                decoration: const InputDecoration(
                  labelText: 'Bio',
                  hintText: 'Tell us about yourself',
                  prefixIcon: Icon(Icons.info),
                  alignLabelWithHint: true,
                ),
                maxLines: 4,
                maxLength: 500,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
