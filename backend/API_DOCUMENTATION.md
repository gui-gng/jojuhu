# Social Network API Documentation

> **For App Developers** - Complete API reference for building the jojuhu-app application.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

The API uses **JWT Bearer Token** authentication for protected endpoints.

### Getting a Token

1. Register or Login via `/auth/register` or `/auth/login`
2. Store the `token` from the response
3. Include it in the `Authorization` header for protected endpoints:

```http
Authorization: Bearer <your_jwt_token>
```

### Public vs Protected Endpoints

| Type | Endpoints | Auth Required |
|------|-----------|---------------|
| Public | `/api/v1/auth/*`, `/health` | No |
| Protected | All others | Yes |

## Response Format

All API responses follow a consistent structure:

### Success Response

```json
{
  "success": true,
  "data": { ... },
  "message": null
}
```

### Error Response

```json
{
  "success": false,
  "error": "Error message description"
}
```

### HTTP Status Codes

| Code | Meaning |
|------|---------|
| 200 | Success |
| 201 | Created |
| 204 | No Content (successful delete) |
| 400 | Bad Request (validation error) |
| 401 | Unauthorized (invalid/missing token) |
| 403 | Forbidden (no permission) |
| 404 | Not Found |
| 409 | Conflict (duplicate data) |
| 500 | Internal Server Error |

---

## Authentication Endpoints

### Register

Create a new user account.

```http
POST /api/v1/auth/register
Content-Type: application/json
```

**Request Body:**

```json
{
  "username": "string (3-32 chars, required)",
  "email": "string (valid email, required)",
  "password": "string (min 8 chars, required)",
  "display_name": "string (optional)"
}
```

**Response (201 Created):**

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "username": "string",
      "email": "string",
      "display_name": "string | null",
      "bio": "string | null",
      "avatar_url": "string | null",
      "created_at": "2024-01-15T10:30:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIs..."
  }
}
```

**Errors:**
- `400` - Validation error (username too short, password too weak)
- `409` - Username or email already exists

---

### Login

Authenticate and get a JWT token.

```http
POST /api/v1/auth/login
Content-Type: application/json
```

**Request Body:**

```json
{
  "username_or_email": "string (required)",
  "password": "string (required)"
}
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "username": "string",
      "email": "string",
      "display_name": "string | null",
      "bio": "string | null",
      "avatar_url": "string | null",
      "created_at": "2024-01-15T10:30:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIs..."
  }
}
```

**Errors:**
- `401` - Invalid credentials

---

### Get Current User

Get the currently authenticated user's profile.

```http
GET /api/v1/me
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "username": "string",
    "email": "string",
    "display_name": "string | null",
    "bio": "string | null",
    "avatar_url": "string | null",
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

---

## Messages Endpoints

Direct messaging between users.

### Send Message

Send a direct message to another user.

```http
POST /api/v1/messages
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "recipient_id": "uuid (required)",
  "content": "string (required)"
}
```

**Response (201 Created):**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "sender_id": "uuid",
    "sender_username": "string",
    "recipient_id": "uuid",
    "recipient_username": "string",
    "content": "string",
    "is_read": false,
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

---

### List Conversations

Get a list of all conversations for the current user.

```http
GET /api/v1/messages/conversations
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "user_id": "uuid",
      "username": "string",
      "display_name": "string | null",
      "avatar_url": "string | null",
      "last_message": "string",
      "last_message_at": "2024-01-15T10:30:00Z",
      "unread_count": 3
    }
  ]
}
```

---

### Get Conversation

Get messages between the current user and another user.

```http
GET /api/v1/messages/conversations/{user_id}?page=1&per_page=20
Authorization: Bearer <token>
```

**Query Parameters:**

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number (min: 1) |
| per_page | integer | 20 | Items per page (1-100) |

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "username": "string",
      "display_name": "string | null",
      "avatar_url": "string | null"
    },
    "messages": [
      {
        "id": "uuid",
        "sender_id": "uuid",
        "sender_username": "string",
        "recipient_id": "uuid",
        "recipient_username": "string",
        "content": "string",
        "is_read": true,
        "created_at": "2024-01-15T10:30:00Z"
      }
    ]
  }
}
```

---

### Mark Message as Read

Mark a specific message as read.

```http
PUT /api/v1/messages/{message_id}/read
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "message": "Marked as read"
  }
}
```

---

### Delete Message

Delete a message (only the sender can delete).

```http
DELETE /api/v1/messages/{message_id}
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

## Timeline Endpoints

Posts, likes, comments, and social feed.

### Get Feed

Get the social feed (posts from followed users and public posts).

```http
GET /api/v1/timeline/feed?page=1&per_page=20
Authorization: Bearer <token>
```

**Query Parameters:**

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number |
| per_page | integer | 20 | Items per page (1-100) |

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "author": {
        "id": "uuid",
        "username": "string",
        "display_name": "string | null",
        "avatar_url": "string | null"
      },
      "content": "string",
      "media_urls": ["string", "string"],
      "likes_count": 42,
      "comments_count": 5,
      "shares_count": 3,
      "is_public": true,
      "created_at": "2024-01-15T10:30:00Z",
      "is_liked": false
    }
  ]
}
```

---

### Create Post

Create a new post.

```http
POST /api/v1/timeline/posts
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "content": "string (required)",
  "media_urls": ["string"],
  "is_public": true
}
```

**Response (201 Created):**

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "author": {
      "id": "uuid",
      "username": "string",
      "display_name": "string | null",
      "avatar_url": "string | null"
    },
    "content": "string",
    "media_urls": ["string"],
    "likes_count": 0,
    "comments_count": 0,
    "shares_count": 0,
    "is_public": true,
    "created_at": "2024-01-15T10:30:00Z",
    "is_liked": false
  }
}
```

---

### Get Post

Get a single post by ID.

```http
GET /api/v1/timeline/posts/{post_id}
Authorization: Bearer <token>
```

**Response (200 OK):** Same as Create Post response

---

### Update Post

Update an existing post (only author can update).

```http
PUT /api/v1/timeline/posts/{post_id}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "content": "string (optional)",
  "is_public": false
}
```

**Response (200 OK):** Same as Create Post response

---

### Delete Post

Delete a post (only author can delete).

```http
DELETE /api/v1/timeline/posts/{post_id}
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

### Get User Posts

Get all posts by a specific user.

```http
GET /api/v1/timeline/users/{user_id}/posts?page=1&per_page=20
Authorization: Bearer <token>
```

**Response (200 OK):** Array of PostResponse

---

### Like Post

Like a post.

```http
POST /api/v1/timeline/posts/{post_id}/like
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "liked": true
  }
}
```

---

### Unlike Post

Remove like from a post.

```http
DELETE /api/v1/timeline/posts/{post_id}/like
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "liked": false
  }
}
```

---

### Get Comments

Get comments on a post.

```http
GET /api/v1/timeline/posts/{post_id}/comments?page=1&per_page=20
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "author": {
        "id": "uuid",
        "username": "string",
        "display_name": "string | null",
        "avatar_url": "string | null"
      },
      "content": "string",
      "parent_comment_id": "uuid | null",
      "likes_count": 5,
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

---

### Add Comment

Add a comment to a post.

```http
POST /api/v1/timeline/posts/{post_id}/comments
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "content": "string (required)",
  "parent_comment_id": "uuid (optional, for replies)"
}
```

**Response (201 Created):** Same as comment object in Get Comments

---

### Delete Comment

Delete a comment (only author can delete).

```http
DELETE /api/v1/timeline/posts/{post_id}/comments/{comment_id}
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

## Forums Endpoints

Community forums with topics and replies.

### List Forums

Get all public forums.

```http
GET /api/v1/forums?page=1&per_page=20
```

**Query Parameters:**

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| page | integer | 1 | Page number |
| per_page | integer | 20 | Items per page (1-100) |

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "string",
      "slug": "string",
      "description": "string | null",
      "icon_url": "string | null",
      "cover_image_url": "string | null",
      "creator": {
        "id": "uuid",
        "username": "string",
        "display_name": "string | null"
      },
      "is_public": true,
      "members_count": 150,
      "topics_count": 45,
      "is_member": false,
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

---

### Create Forum

Create a new forum.

```http
POST /api/v1/forums
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "name": "string (required)",
  "description": "string (optional)",
  "is_public": true
}
```

**Response (201 Created):** Same as forum object in List Forums

---

### Get Forum

Get a single forum by ID.

```http
GET /api/v1/forums/{forum_id}
```

**Response (200 OK):** Same as forum object in List Forums

---

### Update Forum

Update forum details (only creator/moderators).

```http
PUT /api/v1/forums/{forum_id}
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "name": "string (optional)",
  "description": "string (optional)",
  "is_public": false
}
```

**Response (200 OK):** Same as forum object

---

### Delete Forum

Delete a forum (only creator).

```http
DELETE /api/v1/forums/{forum_id}
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

### Join Forum

Join a forum as a member.

```http
POST /api/v1/forums/{forum_id}/join
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "joined": true
  }
}
```

---

### Leave Forum

Leave a forum.

```http
POST /api/v1/forums/{forum_id}/leave
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "joined": false
  }
}
```

---

### List Topics

Get all topics in a forum.

```http
GET /api/v1/forums/{forum_id}/topics?page=1&per_page=20
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "forum_id": "uuid",
      "author": {
        "id": "uuid",
        "username": "string",
        "display_name": "string | null"
      },
      "title": "string",
      "content": "string",
      "is_pinned": false,
      "is_locked": false,
      "views_count": 100,
      "replies_count": 25,
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

---

### Create Topic

Create a new topic in a forum.

```http
POST /api/v1/forums/{forum_id}/topics
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "title": "string (required)",
  "content": "string (required)"
}
```

**Response (201 Created):** Same as topic object in List Topics

---

### Get Topic

Get a single topic by ID.

```http
GET /api/v1/forums/{forum_id}/topics/{topic_id}
```

**Response (200 OK):** Same as topic object

---

### Delete Topic

Delete a topic (author, moderator, or admin only).

```http
DELETE /api/v1/forums/{forum_id}/topics/{topic_id}
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

### Lock Topic

Lock a topic to prevent new replies (moderator/admin only).

```http
POST /api/v1/forums/{forum_id}/topics/{topic_id}/lock
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "locked": true
  }
}
```

---

### Unlock Topic

Unlock a previously locked topic.

```http
POST /api/v1/forums/{forum_id}/topics/{topic_id}/unlock
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "locked": false
  }
}
```

---

### Pin Topic

Pin a topic to the top of the forum (moderator/admin only).

```http
POST /api/v1/forums/{forum_id}/topics/{topic_id}/pin
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "pinned": true
  }
}
```

---

### Unpin Topic

Unpin a topic.

```http
POST /api/v1/forums/{forum_id}/topics/{topic_id}/unpin
Authorization: Bearer <token>
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": {
    "pinned": false
  }
}
```

---

### List Replies

Get all replies in a topic.

```http
GET /api/v1/forums/{forum_id}/topics/{topic_id}/replies?page=1&per_page=20
```

**Response (200 OK):**

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "author": {
        "id": "uuid",
        "username": "string",
        "display_name": "string | null"
      },
      "content": "string",
      "parent_reply_id": "uuid | null",
      "likes_count": 10,
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

---

### Create Reply

Add a reply to a topic.

```http
POST /api/v1/forums/{forum_id}/topics/{topic_id}/replies
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**

```json
{
  "content": "string (required)",
  "parent_reply_id": "uuid (optional, for nested replies)"
}
```

**Response (201 Created):** Same as reply object in List Replies

---

### Delete Reply

Delete a reply (author, moderator, or admin only).

```http
DELETE /api/v1/forums/{forum_id}/topics/{topic_id}/replies/{reply_id}
Authorization: Bearer <token>
```

**Response (204 No Content)**

---

## Health Check

Check if the API is running.

```http
GET /health
```

**Response (200 OK):**

```json
{
  "status": "healthy",
  "service": "social_network"
}
```

---

## Common Types Reference

### User

```typescript
interface User {
  id: string;           // UUID
  username: string;
  email: string;
  display_name: string | null;
  bio: string | null;
  avatar_url: string | null;
  created_at: string;   // ISO 8601 datetime
}
```

### Pagination

All paginated endpoints return an array of items and accept these query params:

| Param | Type | Default | Range |
|-------|------|---------|-------|
| page | integer | 1 | min: 1 |
| per_page | integer | 20 | 1-100 |

---

## Frontend Integration Tips

### 1. Store Token

After login/register, store the JWT token securely (e.g., in httpOnly cookies or secure localStorage).

### 2. Axios/Fetch Setup

Configure your HTTP client to automatically include the Authorization header:

```typescript
// Axios example
const api = axios.create({
  baseURL: 'http://localhost:8080/api/v1',
  headers: {
    'Authorization': `Bearer ${getToken()}`
  }
});
```

### 3. Handle Token Expiration

Implement an interceptor to handle 401 responses and redirect to login:

```typescript
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Token expired or invalid
      logout();
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);
```

### 4. UUID Handling

All IDs are UUIDs (e.g., `550e8400-e29b-41d4-a716-446655440000`). Treat them as strings.

### 5. Date Handling

All dates are in ISO 8601 format (UTC). Use `new Date(dateString)` to parse.

---

## Error Handling Examples

### Validation Error (400)

```json
{
  "success": false,
  "error": "Username must be between 3 and 32 characters"
}
```

### Authentication Error (401)

```json
{
  "success": false,
  "error": "Invalid credentials"
}
```

### Not Found (404)

```json
{
  "success": false,
  "error": "Record not found"
}
```

### Conflict (409)

```json
{
  "success": false,
  "error": "Username or email already exists"
}
```
