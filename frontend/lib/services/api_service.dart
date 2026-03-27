import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../models/api_response.dart';
import '../models/user.dart';
import '../models/user_profile.dart';
import '../models/message.dart';
import '../models/post.dart';
import '../models/forum.dart';
import '../models/upload.dart';

class ApiService {
  static const String baseUrl = 'http://localhost:8080';
  static const String apiPrefix = '/api/v1';
  
  static const storage = FlutterSecureStorage();

  // ============================================================================
  // TOKEN MANAGEMENT
  // ============================================================================
  
  static Future<void> storeToken(String token) async {
    await storage.write(key: 'auth_token', value: token);
  }

  static Future<String?> getToken() async {
    return await storage.read(key: 'auth_token');
  }

  static Future<void> deleteToken() async {
    await storage.delete(key: 'auth_token');
  }

  static Future<Map<String, String>> _getHeaders({bool requiresAuth = true}) async {
    final headers = {'Content-Type': 'application/json'};
    if (requiresAuth) {
      final token = await getToken();
      if (token != null) {
        headers['Authorization'] = 'Bearer $token';
      }
    }
    return headers;
  }

  // ============================================================================
  // HELPER METHODS
  // ============================================================================

  static ApiResponse<T> _handleResponse<T>(
    http.Response response,
    T Function(dynamic)? parser,
  ) {
    final body = jsonDecode(response.body);
    
    if (response.statusCode >= 200 && response.statusCode < 300) {
      return ApiResponse<T>(
        success: true,
        data: parser != null && body['data'] != null ? parser(body['data']) : null,
        message: body['message'],
        statusCode: response.statusCode,
      );
    } else {
      return ApiResponse<T>(
        success: false,
        error: body['error'] ?? 'Unknown error',
        statusCode: response.statusCode,
      );
    }
  }

  static ApiResponse<T> _handleEmptyResponse<T>(http.Response response) {
    if (response.statusCode == 204 || response.statusCode == 200) {
      return ApiResponse<T>(success: true, statusCode: response.statusCode);
    } else {
      final body = jsonDecode(response.body);
      return ApiResponse<T>(
        success: false,
        error: body['error'] ?? 'Unknown error',
        statusCode: response.statusCode,
      );
    }
  }

  // ============================================================================
  // AUTHENTICATION ENDPOINTS
  // ============================================================================

  /// POST /api/v1/auth/register
  /// Creates a new user account
  static Future<ApiResponse<Map<String, dynamic>>> register({
    required String username,
    required String email,
    required String password,
    String? displayName,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/auth/register'),
      headers: await _getHeaders(requiresAuth: false),
      body: jsonEncode({
        'username': username,
        'email': email,
        'password': password,
        if (displayName != null) 'display_name': displayName,
      }),
    );

    final result = _handleResponse<Map<String, dynamic>>(response, (data) => data);
    
    if (result.success && result.data != null) {
      final token = result.data!['token'];
      if (token != null) {
        await storeToken(token);
      }
    }
    
    return result;
  }

  /// POST /api/v1/auth/login
  /// Authenticates and returns JWT token
  static Future<ApiResponse<Map<String, dynamic>>> login({
    required String usernameOrEmail,
    required String password,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/auth/login'),
      headers: await _getHeaders(requiresAuth: false),
      body: jsonEncode({
        'username_or_email': usernameOrEmail,
        'password': password,
      }),
    );

    final result = _handleResponse<Map<String, dynamic>>(response, (data) => data);
    
    if (result.success && result.data != null) {
      final token = result.data!['token'];
      if (token != null) {
        await storeToken(token);
      }
    }
    
    return result;
  }

  /// GET /api/v1/me
  /// Gets the currently authenticated user's profile
  static Future<ApiResponse<User>> getCurrentUser() async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/me'),
      headers: await _getHeaders(),
    );

    return _handleResponse<User>(response, (data) => User.fromJson(data));
  }

  // ============================================================================
  // MESSAGES ENDPOINTS
  // ============================================================================

  /// POST /api/v1/messages
  /// Sends a direct message to another user
  static Future<ApiResponse<Message>> sendMessage({
    required String recipientId,
    required String content,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/messages'),
      headers: await _getHeaders(),
      body: jsonEncode({
        'recipient_id': recipientId,
        'content': content,
      }),
    );

    return _handleResponse<Message>(response, (data) => Message.fromJson(data));
  }

  /// GET /api/v1/messages/conversations
  /// Gets a list of all conversations for the current user
  static Future<ApiResponse<List<Conversation>>> getConversations() async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/messages/conversations'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Conversation>>(
      response,
      (data) => (data as List).map((c) => Conversation.fromJson(c)).toList(),
    );
  }

  /// GET /api/v1/messages/conversations/{user_id}
  /// Gets messages between the current user and another user
  static Future<ApiResponse<ConversationDetail>> getConversation({
    required String userId,
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/messages/conversations/$userId?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<ConversationDetail>(
      response,
      (data) => ConversationDetail.fromJson(data),
    );
  }

  /// PUT /api/v1/messages/{message_id}/read
  /// Marks a specific message as read
  static Future<ApiResponse<void>> markMessageAsRead(String messageId) async {
    final response = await http.put(
      Uri.parse('$baseUrl$apiPrefix/messages/$messageId/read'),
      headers: await _getHeaders(),
    );

    return _handleResponse<void>(response, null);
  }

  /// DELETE /api/v1/messages/{message_id}
  /// Deletes a message (only sender can delete)
  static Future<ApiResponse<void>> deleteMessage(String messageId) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/messages/$messageId'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  // ============================================================================
  // TIMELINE / POSTS ENDPOINTS
  // ============================================================================

  /// GET /api/v1/timeline/feed
  /// Gets the social feed (posts from followed users and public posts)
  static Future<ApiResponse<List<Post>>> getFeed({
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/timeline/feed?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Post>>(
      response,
      (data) => (data as List).map((p) => Post.fromJson(p)).toList(),
    );
  }

  /// POST /api/v1/timeline/posts
  /// Creates a new post
  static Future<ApiResponse<Post>> createPost({
    required String content,
    List<String>? mediaUrls,
    bool isPublic = true,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts'),
      headers: await _getHeaders(),
      body: jsonEncode({
        'content': content,
        if (mediaUrls != null) 'media_urls': mediaUrls,
        'is_public': isPublic,
      }),
    );

    return _handleResponse<Post>(response, (data) => Post.fromJson(data));
  }

  /// GET /api/v1/timeline/posts/{post_id}
  /// Gets a single post by ID
  static Future<ApiResponse<Post>> getPost(String postId) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId'),
      headers: await _getHeaders(),
    );

    return _handleResponse<Post>(response, (data) => Post.fromJson(data));
  }

  /// PUT /api/v1/timeline/posts/{post_id}
  /// Updates an existing post (only author can update)
  static Future<ApiResponse<Post>> updatePost({
    required String postId,
    String? content,
    bool? isPublic,
  }) async {
    final body = <String, dynamic>{};
    if (content != null) body['content'] = content;
    if (isPublic != null) body['is_public'] = isPublic;

    final response = await http.put(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId'),
      headers: await _getHeaders(),
      body: jsonEncode(body),
    );

    return _handleResponse<Post>(response, (data) => Post.fromJson(data));
  }

  /// DELETE /api/v1/timeline/posts/{post_id}
  /// Deletes a post (only author can delete)
  static Future<ApiResponse<void>> deletePost(String postId) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// GET /api/v1/timeline/users/{user_id}/posts
  /// Gets all posts by a specific user
  static Future<ApiResponse<List<Post>>> getUserPosts({
    required String userId,
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/timeline/users/$userId/posts?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Post>>(
      response,
      (data) => (data as List).map((p) => Post.fromJson(p)).toList(),
    );
  }

  /// POST /api/v1/timeline/posts/{post_id}/like
  /// Likes a post
  static Future<ApiResponse<bool>> likePost(String postId) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId/like'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['liked']);
  }

  /// DELETE /api/v1/timeline/posts/{post_id}/like
  /// Removes like from a post
  static Future<ApiResponse<bool>> unlikePost(String postId) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId/like'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['liked']);
  }

  /// GET /api/v1/timeline/posts/{post_id}/comments
  /// Gets comments on a post
  static Future<ApiResponse<List<Comment>>> getComments({
    required String postId,
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId/comments?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Comment>>(
      response,
      (data) => (data as List).map((c) => Comment.fromJson(c)).toList(),
    );
  }

  /// POST /api/v1/timeline/posts/{post_id}/comments
  /// Adds a comment to a post
  static Future<ApiResponse<Comment>> addComment({
    required String postId,
    required String content,
    String? parentCommentId,
  }) async {
    final body = {
      'content': content,
      if (parentCommentId != null) 'parent_comment_id': parentCommentId,
    };

    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId/comments'),
      headers: await _getHeaders(),
      body: jsonEncode(body),
    );

    return _handleResponse<Comment>(response, (data) => Comment.fromJson(data));
  }

  /// DELETE /api/v1/timeline/posts/{post_id}/comments/{comment_id}
  /// Deletes a comment (only author can delete)
  static Future<ApiResponse<void>> deleteComment({
    required String postId,
    required String commentId,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/timeline/posts/$postId/comments/$commentId'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  // ============================================================================
  // FORUMS ENDPOINTS
  // ============================================================================

  /// GET /api/v1/forums
  /// Gets all public forums
  static Future<ApiResponse<List<Forum>>> getForums({
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/forums?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Forum>>(
      response,
      (data) => (data as List).map((f) => Forum.fromJson(f)).toList(),
    );
  }

  /// POST /api/v1/forums
  /// Creates a new forum
  static Future<ApiResponse<Forum>> createForum({
    required String name,
    String? description,
    bool isPublic = true,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums'),
      headers: await _getHeaders(),
      body: jsonEncode({
        'name': name,
        if (description != null) 'description': description,
        'is_public': isPublic,
      }),
    );

    return _handleResponse<Forum>(response, (data) => Forum.fromJson(data));
  }

  /// GET /api/v1/forums/{forum_id}
  /// Gets a single forum by ID
  static Future<ApiResponse<Forum>> getForum(String forumId) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId'),
      headers: await _getHeaders(),
    );

    return _handleResponse<Forum>(response, (data) => Forum.fromJson(data));
  }

  /// PUT /api/v1/forums/{forum_id}
  /// Updates forum details (only creator/moderators)
  static Future<ApiResponse<Forum>> updateForum({
    required String forumId,
    String? name,
    String? description,
    bool? isPublic,
  }) async {
    final body = <String, dynamic>{};
    if (name != null) body['name'] = name;
    if (description != null) body['description'] = description;
    if (isPublic != null) body['is_public'] = isPublic;

    final response = await http.put(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId'),
      headers: await _getHeaders(),
      body: jsonEncode(body),
    );

    return _handleResponse<Forum>(response, (data) => Forum.fromJson(data));
  }

  /// DELETE /api/v1/forums/{forum_id}
  /// Deletes a forum (only creator)
  static Future<ApiResponse<void>> deleteForum(String forumId) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// POST /api/v1/forums/{forum_id}/join
  /// Joins a forum as a member
  static Future<ApiResponse<bool>> joinForum(String forumId) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/join'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['joined']);
  }

  /// POST /api/v1/forums/{forum_id}/leave
  /// Leaves a forum
  static Future<ApiResponse<bool>> leaveForum(String forumId) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/leave'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['joined']);
  }

  /// GET /api/v1/forums/{forum_id}/topics
  /// Gets all topics in a forum
  static Future<ApiResponse<List<Topic>>> getTopics({
    required String forumId,
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Topic>>(
      response,
      (data) => (data as List).map((t) => Topic.fromJson(t)).toList(),
    );
  }

  /// POST /api/v1/forums/{forum_id}/topics
  /// Creates a new topic in a forum
  static Future<ApiResponse<Topic>> createTopic({
    required String forumId,
    required String title,
    required String content,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics'),
      headers: await _getHeaders(),
      body: jsonEncode({
        'title': title,
        'content': content,
      }),
    );

    return _handleResponse<Topic>(response, (data) => Topic.fromJson(data));
  }

  /// GET /api/v1/forums/{forum_id}/topics/{topic_id}
  /// Gets a single topic by ID
  static Future<ApiResponse<Topic>> getTopic({
    required String forumId,
    required String topicId,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId'),
      headers: await _getHeaders(),
    );

    return _handleResponse<Topic>(response, (data) => Topic.fromJson(data));
  }

  /// DELETE /api/v1/forums/{forum_id}/topics/{topic_id}
  /// Deletes a topic (author, moderator, or admin only)
  static Future<ApiResponse<void>> deleteTopic({
    required String forumId,
    required String topicId,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// POST /api/v1/forums/{forum_id}/topics/{topic_id}/lock
  /// Locks a topic to prevent new replies
  static Future<ApiResponse<bool>> lockTopic({
    required String forumId,
    required String topicId,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId/lock'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['locked']);
  }

  /// POST /api/v1/forums/{forum_id}/topics/{topic_id}/unlock
  /// Unlocks a previously locked topic
  static Future<ApiResponse<bool>> unlockTopic({
    required String forumId,
    required String topicId,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId/unlock'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['locked']);
  }

  /// POST /api/v1/forums/{forum_id}/topics/{topic_id}/pin
  /// Pins a topic to the top of the forum
  static Future<ApiResponse<bool>> pinTopic({
    required String forumId,
    required String topicId,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId/pin'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['pinned']);
  }

  /// POST /api/v1/forums/{forum_id}/topics/{topic_id}/unpin
  /// Unpins a topic
  static Future<ApiResponse<bool>> unpinTopic({
    required String forumId,
    required String topicId,
  }) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId/unpin'),
      headers: await _getHeaders(),
    );

    return _handleResponse<bool>(response, (data) => data['pinned']);
  }

  /// GET /api/v1/forums/{forum_id}/topics/{topic_id}/replies
  /// Gets all replies in a topic
  static Future<ApiResponse<List<Reply>>> getReplies({
    required String forumId,
    required String topicId,
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId/replies?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Reply>>(
      response,
      (data) => (data as List).map((r) => Reply.fromJson(r)).toList(),
    );
  }

  /// POST /api/v1/forums/{forum_id}/topics/{topic_id}/replies
  /// Adds a reply to a topic
  static Future<ApiResponse<Reply>> createReply({
    required String forumId,
    required String topicId,
    required String content,
    String? parentReplyId,
  }) async {
    final body = {
      'content': content,
      if (parentReplyId != null) 'parent_reply_id': parentReplyId,
    };

    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId/replies'),
      headers: await _getHeaders(),
      body: jsonEncode(body),
    );

    return _handleResponse<Reply>(response, (data) => Reply.fromJson(data));
  }

  /// DELETE /api/v1/forums/{forum_id}/topics/{topic_id}/replies/{reply_id}
  /// Deletes a reply
  static Future<ApiResponse<void>> deleteReply({
    required String forumId,
    required String topicId,
    required String replyId,
  }) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/forums/$forumId/topics/$topicId/replies/$replyId'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  // ============================================================================
  // HEALTH CHECK
  // ============================================================================

  /// GET /health
  /// Checks if the API is running
  static Future<ApiResponse<Map<String, dynamic>>> healthCheck() async {
    final response = await http.get(
      Uri.parse('$baseUrl/health'),
      headers: await _getHeaders(requiresAuth: false),
    );

    return _handleResponse<Map<String, dynamic>>(response, (data) => data);
  }

  // ============================================================================
  // USER PROFILE
  // ============================================================================

  /// GET /api/v1/users/me
  /// Gets the current user's profile
  static Future<ApiResponse<MyProfile>> getMyProfile() async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/users/me'),
      headers: await _getHeaders(),
    );

    return _handleResponse<MyProfile>(response, (data) => MyProfile.fromJson(data));
  }

  /// GET /api/v1/users/{id}
  /// Gets a user's profile by ID
  static Future<ApiResponse<UserProfile>> getUserProfile(String userId) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/users/$userId'),
      headers: await _getHeaders(),
    );

    return _handleResponse<UserProfile>(response, (data) => UserProfile.fromJson(data));
  }

  /// PUT /api/v1/users/me
  /// Updates the current user's profile
  static Future<ApiResponse<void>> updateProfile({
    String? displayName,
    String? bio,
  }) async {
    final body = {
      if (displayName != null) 'display_name': displayName,
      if (bio != null) 'bio': bio,
    };

    final response = await http.put(
      Uri.parse('$baseUrl$apiPrefix/users/me'),
      headers: await _getHeaders(),
      body: jsonEncode(body),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// PUT /api/v1/users/me/avatar
  /// Updates the current user's avatar
  static Future<ApiResponse<void>> updateAvatar(String avatarUrl) async {
    final response = await http.put(
      Uri.parse('$baseUrl$apiPrefix/users/me/avatar'),
      headers: await _getHeaders(),
      body: jsonEncode({'avatar_url': avatarUrl}),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// POST /api/v1/users/{id}/follow
  /// Follows a user
  static Future<ApiResponse<void>> followUser(String userId) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/users/$userId/follow'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// DELETE /api/v1/users/{id}/follow
  /// Unfollows a user
  static Future<ApiResponse<void>> unfollowUser(String userId) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/users/$userId/follow'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// GET /api/v1/users/{id}/followers
  /// Gets a user's followers
  static Future<ApiResponse<UsersListResponse>> getFollowers({
    required String userId,
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/users/$userId/followers?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<UsersListResponse>(response, (data) => UsersListResponse.fromJson(data));
  }

  /// GET /api/v1/users/{id}/following
  /// Gets a user's following list
  static Future<ApiResponse<UsersListResponse>> getFollowing({
    required String userId,
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/users/$userId/following?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<UsersListResponse>(response, (data) => UsersListResponse.fromJson(data));
  }

  // ============================================================================
  // FEED
  // ============================================================================

  /// GET /api/v1/timeline/following
  /// Gets the personalized following feed
  static Future<ApiResponse<List<Post>>> getFollowingFeed({
    int page = 1,
    int perPage = 20,
  }) async {
    final response = await http.get(
      Uri.parse('$baseUrl$apiPrefix/timeline/following?page=$page&per_page=$perPage'),
      headers: await _getHeaders(),
    );

    return _handleResponse<List<Post>>(
      response,
      (data) => (data as List).map((p) => Post.fromJson(p)).toList(),
    );
  }

  // ============================================================================
  // UPLOAD
  // ============================================================================

  /// POST /api/v1/upload/presigned-url
  /// Generates a presigned URL for uploading a file
  static Future<ApiResponse<PresignedUrlResponse>> generatePresignedUrl(
    PresignedUrlRequest request,
  ) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/upload/presigned-url'),
      headers: await _getHeaders(),
      body: jsonEncode(request.toJson()),
    );

    return _handleResponse<PresignedUrlResponse>(
      response,
      (data) => PresignedUrlResponse.fromJson(data),
    );
  }

  /// POST /api/v1/upload/confirm-avatar
  /// Confirms an avatar upload
  static Future<ApiResponse<void>> confirmAvatarUpload(String key) async {
    final response = await http.post(
      Uri.parse('$baseUrl$apiPrefix/upload/confirm-avatar'),
      headers: await _getHeaders(),
      body: jsonEncode({'key': key}),
    );

    return _handleEmptyResponse<void>(response);
  }

  /// Uploads a file directly to MinIO using a presigned URL
  static Future<bool> uploadToPresignedUrl({
    required String presignedUrl,
    required List<int> fileBytes,
    required String contentType,
  }) async {
    try {
      final response = await http.put(
        Uri.parse(presignedUrl),
        headers: {'Content-Type': contentType},
        body: fileBytes,
      );

      return response.statusCode == 200;
    } catch (e) {
      return false;
    }
  }

  /// DELETE /api/v1/upload/files/{key}
  /// Deletes a file from storage
  static Future<ApiResponse<void>> deleteFile(String key) async {
    final response = await http.delete(
      Uri.parse('$baseUrl$apiPrefix/upload/files/$key'),
      headers: await _getHeaders(),
    );

    return _handleEmptyResponse<void>(response);
  }
}
