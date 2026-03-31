# Jojuhu Product Roadmap

**Product Vision:** A modern social network platform connecting people through shared interests, real-time conversations, and meaningful communities.

**Current Version:** v0.1.0 ✅ COMPLETE  
**Release Date:** March 27, 2026  
**Target Launch:** v1.0.0 (Production)  
**Status:** 🎉 MVP Production Ready!

---

## Release v0.1.0 - Foundation (MVP) 🚀
**Status:** ✅ COMPLETE  
**Completion Date:** March 27, 2026  
**Goal:** Core social networking functionality - users can register, post, follow, and message

### Features

#### Authentication & User Management
- [x] User registration with email/username
- [x] Login with JWT tokens
- [x] Secure password handling (bcrypt/argon2)
- [x] Token refresh mechanism
- [x] Password reset via email
- [x] Email verification

#### User Profiles
- [x] View user profile page
- [x] Edit profile (display name, bio, avatar)
- [x] Profile privacy settings (public/private)
- [x] User stats (posts count, followers, following)

#### Social Graph (Follow System)
- [x] Follow/unfollow users
- [x] View followers list
- [x] View following list
- [x] Mutual friends indicator

#### Feed & Posts
- [x] Create text posts
- [x] Like/unlike posts
- [x] Comment on posts
- [x] Delete own posts/comments
- [x] Edit posts (within time limit)
- [x] Personal feed (posts from followed users)
- [x] Global explore feed
- [x] Chronological sorting option (newest, oldest, popular)

#### Forums (Communities)
- [x] Create forums
- [x] Join/leave forums
- [x] Create topics
- [x] Reply to topics
- [x] Forum moderation (pin/lock topics)
- [x] Forum discovery/search

#### Messaging
- [x] Direct messaging between users
- [x] Conversation list
- [x] Message history with pagination
- [x] Mark messages as read
- [x] Delete conversations
- [x] Block users

#### Media Upload
- [x] Avatar upload (MinIO storage)
- [x] Post image attachments (up to 4 images)
- [x] Image validation (type, size)
- [x] Image deletion

#### Infrastructure
- [x] PostgreSQL database
- [x] Redis caching
- [x] MinIO object storage
- [x] OpenTelemetry observability
- [x] Docker Compose setup
- [x] CORS configuration
- [x] Rate limiting per user (not just IP)
- [x] Database backups strategy (daily at 2 AM, 7-day retention)

#### Frontend
- [x] Flutter web app
- [x] Authentication screens (login, register)
- [x] Feed/explore view
- [x] Forums interface (list, detail, topics, replies)
- [x] Messaging interface (conversations, chat)
- [x] Profile screens (view, edit, tabs: Posts/Forums/About)
- [x] Mobile responsive design
- [x] Error handling & retry logic
- [x] Offline mode basics (cached data, SharedPreferences)

**Definition of Done:**
- [x] Users can register, create profile, post content, follow others
- [x] Basic messaging works between users
- [x] Forums are functional with topics/replies
- [x] Media uploads work (avatars, post images)
- [x] No critical bugs or security issues

### v0.1.0 Achievement Summary 🎉

**Backend (Rust/Actix-web):**
- ✅ 114 tests passing (100% success rate)
- ✅ 40+ API endpoints implemented
- ✅ Clippy clean with zero warnings
- ✅ Database backups automated (daily at 2 AM)
- ✅ Rate limiting per user (100 req/min)
- ✅ Email verification and password reset
- ✅ Chronological sorting (newest/oldest/popular)

**Frontend (Flutter):**
- ✅ All profile screens implemented
- ✅ Mobile responsive design
- ✅ Error handling with retry logic
- ✅ Offline mode basics
- ✅ Avatar upload and image handling

**Infrastructure:**
- ✅ Docker Compose production ready
- ✅ PostgreSQL with migrations
- ✅ MinIO for media storage
- ✅ Automated backup scripts
- ✅ Comprehensive API documentation

---

## Release v0.2.0 - Engagement & Polish ✨
**Status:** 🚧 IN PROGRESS (85% Complete)  
**Target:** April 5, 2026  
**Goal:** Increase user engagement with notifications, real-time features, and improved UX

### Features

#### Real-Time Features
- [x] WebSocket server infrastructure (actix-ws)
- [x] Session management for connections
- [x] Real-time notification delivery via WebSocket
- [ ] Live new post notifications (pending integration)
- [ ] Live message updates (pending integration)
- [ ] Typing indicators in chat (pending integration)
- [ ] Online/offline status indicators (pending integration)
- [ ] Live comment updates on posts (pending integration)

#### Notification System
- [x] In-app notification center infrastructure
- [x] Notification database schema
- [x] Notification types:
  - [x] New followers
  - [x] Likes on posts
  - [x] Comments on posts
  - [x] New messages
  - [x] Forum topic replies
  - [x] Mentions (@username)
- [x] Notification CRUD API endpoints
- [x] Real-time notification delivery via WebSocket
- [ ] Push notifications (mobile) - Future
- [ ] Email notifications (optional) - Future

#### Stories/Ephemeral Content
- [x] 24-hour disappearing stories
- [x] Image/video stories
- [x] Story viewers list
- [x] Story reactions

#### UI/UX Improvements
- [x] Dark mode toggle
- [x] Infinite scroll with skeleton loaders
- [x] Pull-to-refresh on mobile
- [ ] Image lightbox/gallery view
- [ ] Loading states and empty states
- [ ] Swipe gestures (mobile)

#### Content Features
- [x] Repost/share database schema
- [x] Quote posts (repost with comment)
- [ ] Hashtags (#tag) with auto-linking
- [ ] Hashtag search and trending hashtags
- [ ] Rich text formatting (bold, italics, links)
- [ ] Notification preferences/settings

#### Feed Algorithm
- [ ] Algorithmic feed (not just chronological)
- [ ] "For You" page (personalized recommendations)
- [ ] "Following" page (chronological)
- [ ] Trending posts
- [ ] Suggested users to follow
- [ ] Content ranking factors (engagement, recency, relevance)
- [ ] Hashtag search and trending hashtags
- [ ] Rich text formatting (bold, italics, links)
- [ ] Link previews (OpenGraph)
- [ ] Polls in posts

#### Stories/Ephemeral Content
- [ ] 24-hour disappearing stories
- [ ] Image/video stories
- [ ] Story viewers list
- [ ] Story reactions

#### Search & Discovery
- [x] Full-text search (PostgreSQL) - Already implemented
- [ ] Search filters (users, posts, forums)
- [ ] Advanced search (date range, media only)
- [ ] Trending topics/forums
- [ ] User recommendations
- [ ] "Who to follow" suggestions

#### UI/UX Improvements
- [x] Dark mode
- [x] Infinite scroll with skeleton loaders
- [ ] Image lightbox/gallery view
- [x] Pull-to-refresh on mobile
- [ ] Swipe gestures (mobile)
- [ ] Keyboard shortcuts
- [ ] Accessibility improvements (WCAG 2.1)
- [ ] Loading states and empty states

#### Performance
- [ ] Image lazy loading
- [ ] Virtual scrolling for long lists
- [x] Database query optimization
- [x] Redis caching for feeds
- [ ] CDN for static assets
- [ ] API response compression

#### Stories/Ephemeral Content
- [x] 24-hour disappearing stories
- [x] Image/video stories
- [x] Story viewers list
- [x] Story reactions

### v0.2.0 Achievement Summary 🚀

**Completed Infrastructure:**
- ✅ WebSocket server (actix-ws) with real-time notification delivery
- ✅ Notification system (database + API + WebSocket integration)
- ✅ Redis caching module
- ✅ Chronological sorting (newest/oldest/popular)
- ✅ Dark mode frontend
- ✅ Infinite scroll + skeleton loaders
- ✅ Stories/ephemeral content (24-hour expiration)
- ✅ Repost functionality (with quote posts)

**Completed Integration:**
- ✅ WebSocket routes for real-time connections
- ✅ Notification triggers in services (follows, likes, comments, messages)
- ✅ Real-time notification delivery via WebSocket
- ✅ Stories API endpoints

**Definition of Done:**
- [x] Users receive real-time notifications
- [x] Dark mode works across app
- [x] Infinite scroll with skeleton loaders
- [x] Stories feature (24-hour ephemeral content)
- [x] Repost functionality (with quote text)
- [x] All tests passing (114 tests)

---

## Release v0.3.0 - Community & Growth 🌱
**Status:** Planned  
**Goal:** Scale the platform with groups, moderation, and growth features

### Features

#### Groups (Private Communities)
- [ ] Create private groups (invite-only)
- [ ] Group admin roles
- [ ] Group permissions (who can post)
- [ ] Group file sharing
- [ ] Group events/calendar
- [ ] Group announcements

#### Enhanced Forums
- [ ] Forum categories
- [ ] Forum analytics (views, engagement)
- [ ] Forum rules/guidelines
- [ ] Forum moderators
- [ ] Forum bans/timeouts
- [ ] Sticky topics
- [ ] Topic tags

#### Content Moderation
- [ ] Report content (posts, comments, users)
- [ ] Admin moderation dashboard
- [ ] Automated content filtering (spam, profanity)
- [ ] Shadow banning
- [ ] Content warnings/sensitive content
- [ ] Appeal system for bans
- [ ] Terms of Service & Community Guidelines

#### Verification & Trust
- [ ] Verified badge system
- [ ] Identity verification (optional)
- [ ] Report fake accounts
- [ ] Account suspension/reinstatement

#### Analytics & Insights
- [ ] User analytics dashboard
  - [ ] Post performance (views, engagement)
  - [ ] Follower growth
  - [ ] Best posting times
- [ ] Forum analytics
- [ ] Platform admin analytics

#### API & Developer Tools
- [ ] Public API (read-only initially)
- [ ] API rate limiting
- [ ] API documentation
- [ ] Webhook support
- [ ] Third-party integrations

#### Internationalization
- [ ] Multi-language support (i18n)
- [ ] RTL (Right-to-Left) language support
- [ ] Timezone handling
- [ ] Date/time localization

**Definition of Done:**
- Platform handles spam/abuse effectively
- Groups are actively used
- Analytics help users grow their audience
- API is documented and usable
- Platform is ready for international users

---

## Release v1.0.0 - Production Launch 🎯
**Status:** Planned  
**Goal:** Production-ready platform with enterprise features and monetization

### Features

#### Enterprise/Admin Features
- [ ] Admin dashboard
  - [ ] User management
  - [ ] Content moderation queue
  - [ ] Platform analytics
  - [ ] Feature flags
- [ ] Multi-tenant support (optional)
- [ ] Custom branding (white-label)
- [ ] SSO integration (OAuth2/SAML)

#### Monetization (Optional)
- [ ] Premium subscriptions
  - [ ] Verified badge
  - [ ] Analytics insights
  - [ ] Priority support
  - [ ] Custom themes
- [ ] Advertising system (if applicable)
- [ ] Tipping/donations between users
- [ ] Paid groups/forums

#### Advanced Features
- [ ] Live streaming
- [ ] Voice messages
- [ ] Video posts
- [ ] Collaborative posts
- [ ] Scheduled posts
- [ ] Drafts auto-save

#### Mobile Apps
- [ ] iOS app (Flutter)
- [ ] Android app (Flutter)
- [ ] App store optimization
- [ ] Push notification deep linking
- [ ] Mobile-specific features (camera, contacts)

#### Security & Compliance
- [ ] GDPR compliance
  - [ ] Data export
  - [ ] Account deletion (right to be forgotten)
  - [ ] Privacy controls
- [ ] CCPA compliance
- [ ] SOC 2 compliance (if needed)
- [ ] Security audit
- [ ] Penetration testing
- [ ] Bug bounty program

#### Infrastructure & DevOps
- [ ] Kubernetes deployment
- [ ] Auto-scaling
- [ ] Multi-region deployment
- [ ] Disaster recovery plan
- [ ] 99.9% uptime SLA
- [ ] Blue-green deployments
- [ ] Database sharding (if needed)
- [ ] CDN global distribution

**Definition of Done:**
- Platform is stable and scalable
- Security audit passed
- Legal compliance (GDPR, etc.)
- Mobile apps in app stores
- Ready for public launch and marketing

---

## Post v1.0.0 - Future Roadmap 🔮

### AI & ML Features
- [ ] Content recommendation AI
- [ ] Spam detection ML models
- [ ] Automated content moderation
- [ ] Smart replies/suggestions
- [ ] Image recognition (content safety)
- [ ] Sentiment analysis

### Advanced Social Features
- [ ] Audio rooms (like Clubhouse/Twitter Spaces)
- [ ] Collaborative documents
- [ ] Events with RSVPs
- [ ] Marketplace (buy/sell)
- [ ] Fundraising/crowdfunding
- [ ] Job board

### Platform Expansion
- [ ] Browser extension
- [ ] Desktop app (Electron)
- [ ] Smart TV app
- [ ] Wearable support (Apple Watch)
- [ ] VR/AR integration

### Enterprise Features
- [ ] Team collaboration tools
- [ ] Project management integration
- [ ] Custom workflows
- [ ] Advanced permissions/roles
- [ ] Audit logs

---

## Technical Debt & Maintenance

### Ongoing Tasks
- [ ] Dependency updates (weekly)
- [ ] Security patches (immediate)
- [ ] Performance monitoring
- [ ] User feedback collection
- [ ] Bug fixes (continuous)
- [ ] Documentation updates

### Refactoring Goals
- [ ] Backend test coverage > 80%
- [ ] Frontend test coverage > 70%
- [ ] API versioning strategy
- [ ] Database migration tooling
- [ ] Code splitting (frontend)
- [ ] Microservices architecture (if needed)

---

## Success Metrics

### v0.1.0 Success Criteria
- [x] 114 backend tests passing (100% success rate)
- [x] 40+ API endpoints implemented and tested
- [x] Zero clippy warnings (cargo clippy -- -D warnings)
- [x] Docker Compose production deployment ready
- [x] Automated database backups configured
- [x] All v0.1.0 features implemented
- [x] Zero critical security vulnerabilities
- [x] Comprehensive documentation (API docs, README, RELEASE notes)

### v0.2.0 Success Criteria
- [ ] 100+ registered users
- [ ] 50+ daily active users (DAU)
- [ ] User retention > 30% (7-day)
- [ ] Average session duration > 5 minutes
- [ ] < 0.5% error rate

### v0.3.0 Success Criteria
- [ ] 1,000+ registered users
- [ ] 300+ daily active users
- [ ] User retention > 40% (7-day)
- [ ] 100+ active forums/groups
- [ ] Platform moderation queue < 24 hours

### v1.0.0 Success Criteria
- [ ] 10,000+ registered users
- [ ] 2,000+ daily active users
- [ ] 99.9% uptime
- [ ] < 200ms API response time (p95)
- [ ] Security audit passed
- [ ] Ready for monetization

---

## Decision Log

### Architecture Decisions
- **Backend:** Rust (Actix-web) for performance and safety
- **Frontend:** Flutter for cross-platform (web + mobile)
- **Database:** PostgreSQL for relational data
- **Cache:** Redis for sessions and feeds
- **Storage:** MinIO (S3-compatible) for media
- **Observability:** OpenTelemetry + Grafana stack

### Product Decisions
- **Prioritize web first, mobile apps later:** Faster iteration
- **Forums before Groups:** Simpler to implement, test community features
- **Algorithmic feed v0.2.0:** Start with chronological to build user base
- **No anonymous posting:** Accountability from day one

---

## Notes

- **Current Version:** v0.1.0 COMPLETE ✅
- **Next Version:** v0.2.0 - Real-time features and notifications
- **Current Blockers:** None
- **Risks:** User acquisition, content moderation at scale
- **Dependencies:** None (self-hosted infrastructure)
- **Team Size:** Currently solo developer
- **Achievement:** 114 tests passing, 40+ API endpoints, production-ready

---

*Last Updated: March 27, 2026*  
*v0.1.0 COMPLETE - Now focusing on v0.2.0 features*
