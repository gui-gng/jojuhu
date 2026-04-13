# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Placeholder for future features

## [0.2.0] - 2026-04-XX

### Added
#### Notifications System
- WebSocket server for real-time connections with session management
- Real-time notification delivery via WebSocket when notifications are created
- Notification API endpoints:
  - `GET /api/v1/notifications` - list user notifications
  - `GET /api/v1/notifications/count` - get unread count
  - `POST /api/v1/notifications/{id}/read` - mark as read
  - `POST /api/v1/notifications/read-all` - mark all as read
  - `DELETE /api/v1/notifications/{id}` - delete notification
- Notification triggers for:
  - New followers
  - Post likes
  - Post comments
  - New messages
  - Mentions (@username)
- WebSocket route at `/api/v1/ws` for real-time updates

#### Stories/Ephemeral Content
- Complete stories feature with 24-hour expiration
- Stories API endpoints:
  - `POST /api/v1/stories` - create new story
  - `GET /api/v1/stories/feed` - get stories feed
  - `GET /api/v1/stories/following` - get stories from followed users
  - `GET /api/v1/stories/me` - get current user's stories
  - `GET /api/v1/stories/user/{id}` - get user's stories
  - `DELETE /api/v1/stories/{id}` - delete story
  - `POST /api/v1/stories/{id}/view` - view story with optional reaction
  - `GET /api/v1/stories/{id}/viewers` - get story viewers (owner only)
- Support for image and video media types
- View count and viewer tracking
- Reaction support (like, heart, laugh, wow, sad, angry)

#### Repost/Quote Posts
- Repost functionality with optional quote text
- Repost API endpoints:
  - `POST /api/v1/timeline/reposts` - create repost with optional quote
  - `DELETE /api/v1/timeline/reposts/{post_id}` - remove repost
  - `GET /api/v1/timeline/reposts/me` - list current user's reposts
- Prevention of duplicate reposts
- Automatic reposts_count tracking on posts

#### Hashtags
- Hashtag search and trending functionality
- Hashtag API endpoints:
  - `GET /api/v1/hashtag/search?q=query` - search hashtags
  - `GET /api/v1/hashtag/trending` - get trending hashtags
  - `GET /api/v1/hashtag/{name}` - get hashtag stats
- Hashtag extraction from post content
- Usage count tracking

#### Algorithmic Feed
- Personalized "For You" feed with engagement scoring
- Ranking factors: likes (3x), comments (5x), shares (2x), recency decay
- Following boost: 2x multiplier for posts from followed users
- Trending posts endpoint (top posts from last 7 days)
- Suggested users to follow (based on mutual connections)
- Feed endpoints:
  - `GET /api/v1/timeline/for-you` - personalized feed
  - `GET /api/v1/timeline/trending` - trending posts
  - `GET /api/v1/timeline/suggestions/users` - user recommendations

#### Real-Time Infrastructure
- Redis cache module for feed caching
- WebSocket infrastructure for live updates
- Connection session management
- Feed caching with automatic cache invalidation on new posts

#### Frontend
- Dark mode support with theme toggle
- Infinite scroll with skeleton loaders
- Pull-to-refresh on mobile

#### Backend
- Hashtags utility with auto-linking support
- Mention detection in comments
- Chronological sorting for posts (newest/oldest/popular)

### Changed
- Enabled WebSocket and notification routes in main.rs
- Made timeline repository public for handler access
- Added `get_profile_by_id` and `find_by_username` to UserRepository
- Improved story responses to include user details (username, display_name, avatar)
- NotificationService now accepts WebSocketServer for real-time delivery

### Fixed
- Clippy warnings throughout codebase
- WebSocket handler code structure

## [0.1.0] - 2026-03-27

### Added
#### Authentication & User Management
- User registration with email/username
- Login with JWT tokens
- Secure password handling (bcrypt/argon2)
- Token refresh mechanism
- Password reset via email
- Email verification
- Per-user rate limiting (100 requests/minute)

#### User Profiles
- View user profile page
- Edit profile (display name, bio, avatar)
- Profile privacy settings (public/private)
- User stats (posts count, followers, following)
- Follow/unfollow users
- View followers/following lists
- Mutual friends indicator
- Block/unblock users

#### Feed & Posts
- Create text posts
- Like/unlike posts
- Comment on posts
- Delete own posts/comments
- Edit posts (within time limit)
- Personal feed (posts from followed users)
- Global explore feed
- Chronological sorting (newest, oldest, popular)

#### Forums (Communities)
- Create forums
- Join/leave forums
- Create topics
- Reply to topics
- Forum moderation (pin/lock topics)
- Forum discovery/search

#### Messaging
- Direct messaging between users
- Conversation list
- Message history with pagination
- Mark messages as read
- Delete conversations
- Block users

#### Media Upload
- Avatar upload (MinIO storage)
- Post image attachments (up to 4 images)
- Image validation (type, size)
- Image deletion

#### Infrastructure
- PostgreSQL database with migrations
- Redis caching
- MinIO object storage
- OpenTelemetry observability
- Docker Compose setup
- CORS configuration
- Database backups (daily at 2 AM, 7-day retention)
- Comprehensive API documentation (Swagger/OpenAPI)

#### Frontend
- Flutter web app
- Authentication screens (login, register)
- Feed/explore view
- Forums interface (list, detail, topics, replies)
- Messaging interface (conversations, chat)
- Profile screens (view, edit, tabs: Posts/Forums/About)
- Mobile responsive design
- Error handling & retry logic
- Offline mode basics (cached data)

### Security
- Rate limiting per user (not just IP)
- CORS restrictions for production
- Security headers (X-Content-Type-Options, X-Frame-Options, etc.)
- XSS sanitization with ammonia
- SQL injection prevention
- Input validation middleware
- Content length validation

### Testing
- 114 unit tests passing
- Repository tests
- Service tests
- Handler tests
- Security tests
- Validation tests

---

## Version History

| Version | Release Date | Description |
|---------|--------------|-------------|
| 0.2.0 | TBD | Engagement & Polish - Real-time features and notifications |
| 0.1.0 | 2026-03-27 | Foundation (MVP) - Core social networking functionality |

---

## Commit Convention

This project follows semantic commits with project-specific prefixes:

```
<type>(<project>/<scope>): <description>
```

### Project Prefixes
- `backend/` - Rust API and server-side code
- `jojuhu-app/` - Client-side UI code
- `infrastructure/` - Docker, CI/CD, deployment configs

### Commit Types
- `feat` - New features or functionality
- `fix` - Bug fixes
- `refactor` - Code refactoring without behavior changes
- `docs` - Documentation updates
- `test` - Adding or updating tests
- `chore` - Maintenance tasks, dependency updates
- `style` - Code style changes (formatting, linting)
- `perf` - Performance improvements