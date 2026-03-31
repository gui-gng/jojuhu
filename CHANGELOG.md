# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Stories feature infrastructure (ephemeral 24-hour content)

## [0.2.0] - 2026-04-XX

### Added
#### Notifications System
- WebSocket server for real-time connections with session management
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

#### Real-Time Infrastructure
- Redis cache module for feed caching
- WebSocket infrastructure for live updates
- Connection session management

#### Frontend
- Dark mode support with theme toggle
- Infinite scroll with skeleton loaders
- Pull-to-refresh on mobile

#### Backend
- Hashtags utility with auto-linking support
- Mention detection in comments
- Repost database schema and models
- Chronological sorting for posts (newest/oldest/popular)

### Changed
- Enabled WebSocket and notification routes in main.rs
- Made timeline repository public for handler access
- Added `get_profile_by_id` and `find_by_username` to UserRepository

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
- `frontend/` - Client-side UI code
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