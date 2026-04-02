# Technical Architecture Overview

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Client Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Flutter    │  │   Flutter    │  │    Web       │      │
│  │    iOS       │  │   Android    │  │   Browser    │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
└─────────┼────────────────┼────────────────┼────────────────┘
          │                │                │
          └────────────────┴────────────────┘
                           │
                    ┌──────▼──────┐
                    │  Cloudflare │  (Optional: CDN/DDoS)
                    │     CDN     │
                    └──────┬──────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                      API Gateway                             │
│                     (Actix-web)                              │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐             │
│  │    Auth    │  │   Posts    │  │   Forums   │             │
│  │  Module    │  │  Module    │  │  Module    │             │
│  └────────────┘  └────────────┘  └────────────┘             │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐             │
│  │  Messages  │  │   Users    │  │   Upload   │             │
│  │  Module    │  │  Module    │  │  Module    │             │
│  └────────────┘  └────────────┘  └────────────┘             │
│                                                              │
│  Middleware: Auth, Rate Limiting, CORS, Security Headers    │
└──────────────────────────┬──────────────────────────────────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
    │  PostgreSQL │ │    Redis    │ │    MinIO    │
    │  (Primary)  │ │   (Cache)   │ │  (Storage)  │
    └─────────────┘ └─────────────┘ └─────────────┘
```

## Technology Stack

### Backend
| Component | Technology | Purpose |
|-----------|-----------|---------|
| Language | Rust | Performance, safety, concurrency |
| Framework | Actix-web | HTTP web framework |
| Database | PostgreSQL 16 | Primary data store |
| Cache | Redis 7 | Sessions, rate limiting, feeds |
| Storage | MinIO | S3-compatible object storage |
| Auth | JWT (jsonwebtoken) | Stateless authentication |
| ORM/DB | SQLx | Compile-time checked SQL |
| Validation | Validator | Input validation |
| Logging | Tracing | Structured logging |
| Metrics | OpenTelemetry | Observability |

### Frontend
| Component | Technology | Purpose |
|-----------|-----------|---------|
| Framework | Flutter | Cross-platform UI |
| Language | Dart | Flutter language |
| State | setState/Provider | State management (v0.1) |
| HTTP | http package | API communication |
| Storage | flutter_secure_storage | Token storage |
| Images | image_picker | Media selection |

### Infrastructure
| Component | Technology | Purpose |
|-----------|-----------|---------|
| Container | Docker | Containerization |
| Orchestration | Docker Compose | Local development |
| Monitoring | Prometheus | Metrics collection |
| Tracing | Jaeger | Distributed tracing |
| Dashboards | Grafana | Visualization |
| Logs | OpenTelemetry Collector | Log aggregation |

## Module Structure

```
backend/
├── src/
│   ├── main.rs                 # Entry point, CORS, middleware
│   ├── config/                 # Configuration management
│   ├── errors.rs              # Error handling
│   ├── middleware/            # Auth, security, rate limiting
│   ├── models/                # Shared models
│   ├── routes/                # Route definitions
│   │   ├── mod.rs            # Health, route aggregation
│   │   └── v1/               # API v1 routes
│   │       ├── auth/         # Authentication
│   │       ├── users/        # User profiles, follows
│   │       ├── posts/        # Posts, comments, likes
│   │       ├── messages/     # Direct messaging
│   │       ├── forums/       # Forums, topics, replies
│   │       └── upload/       # File uploads
│   ├── services/             # Business logic layer
│   ├── repositories/         # Database access layer
│   └── utils/                # Utilities (auth, validation)
├── tests/                    # Integration tests
├── migrations/               # Database migrations
└── Cargo.toml

frontend/
├── lib/
│   ├── main.dart             # Entry point
│   ├── models/               # Data models
│   ├── services/             # API service layer
│   ├── screens/              # UI screens
│   │   ├── auth/            # Login, register
│   │   └── app/             # Main app screens
│   │       ├── pages/       # Explore, Forums, Messages
│   │       └── home_screen.dart
│   └── widgets/              # Reusable widgets
├── web/                      # Web-specific files
└── pubspec.yaml
```

## Data Flow

### Authentication Flow
```
1. Client POST /api/v1/auth/register
2. Backend validates input, hashes password
3. Backend creates user record in PostgreSQL
4. Backend generates JWT access token + refresh token
5. Client stores tokens (secure storage)
6. Client includes JWT in Authorization header for protected routes
7. Backend middleware validates JWT signature and expiration
```

### Post Creation Flow
```
1. Client POST /api/v1/posts with content and optional media
2. If media included:
   a. Client uploads images to MinIO via presigned URL
   b. MinIO returns object keys
3. Client sends post data with media URLs
4. Backend validates input, sanitizes content
5. Backend inserts post into PostgreSQL
6. Backend invalidates user's followers' feed cache (Redis)
7. Backend returns created post
```

### Feed Loading Flow
```
1. Client GET /api/v1/timeline/following?page=1
2. Backend extracts user_id from JWT
3. Backend queries Redis for cached feed
4. If cache miss:
   a. Query PostgreSQL for posts from followed users
   b. Join with users table for author info
   c. Order by created_at DESC with pagination
   d. Store result in Redis (TTL: 5 minutes)
5. Backend returns paginated posts
6. Client displays posts with infinite scroll
```

### Real-Time Messages (v0.2.0)
```
1. Client establishes WebSocket connection to /ws
2. Backend authenticates WebSocket with JWT
3. Client sends message via WebSocket or HTTP POST
4. Backend stores message in PostgreSQL
5. Backend publishes message to Redis Pub/Sub
6. WebSocket server (separate service) receives via Redis
7. WebSocket pushes message to recipient if online
8. Client displays message immediately (no refresh)
```

## Database Schema Overview

### Core Tables
```sql
-- Users
users (id, username, email, password_hash, display_name, 
       bio, avatar_url, is_verified, created_at)

-- Social Graph
follow_relationships (id, follower_id, following_id, created_at)

-- Posts
posts (id, user_id, content, media_urls[], is_public, 
       likes_count, comments_count, created_at, updated_at)

post_likes (id, post_id, user_id, created_at)
comments (id, post_id, user_id, content, parent_id, 
          likes_count, created_at)

-- Messages
conversations (id, user1_id, user2_id, last_message_at)
messages (id, conversation_id, sender_id, content, 
          is_read, created_at)

-- Forums
forums (id, name, slug, description, creator_id, is_public,
        members_count, topics_count, created_at)
forum_members (id, forum_id, user_id, role, joined_at)
topics (id, forum_id, user_id, title, content, is_pinned,
        is_locked, views_count, replies_count, created_at)
replies (id, topic_id, user_id, content, parent_id, 
         likes_count, created_at)
```

### Indexes
- `users`: username (unique), email (unique)
- `follow_relationships`: (follower_id, following_id) (unique)
- `posts`: user_id, created_at (for feed queries)
- `messages`: conversation_id, created_at
- `forums`: slug (unique)
- `topics`: forum_id, created_at, is_pinned

## Security Architecture

### Authentication
- **JWT Tokens:** Access token (15 min), Refresh token (7 days)
- **Password Hashing:** Argon2id (memory-hard, resistant to GPU attacks)
- **Token Storage:** Client-side secure storage (Keychain/Keystore)

### Authorization
- **Middleware:** Extracts user from JWT, attaches to request
- **Ownership Checks:** Users can only modify own content
- **Role-Based:** Forum moderators, admins (v0.3.0)

### Input Security
- **Validation:** Strict input validation on all endpoints
- **Sanitization:** HTML sanitization to prevent XSS
- **Rate Limiting:** Per-user and per-IP limits
- **Size Limits:** Request body, file uploads

### Infrastructure Security
- **HTTPS Only:** TLS 1.3 in production
- **Security Headers:** CSP, HSTS, X-Frame-Options, etc.
- **CORS:** Whitelist origins only
- **SQL Injection:** SQLx compile-time checked queries

## Scaling Strategy

### Current (v0.1.0 - 1,000 users)
- Single server deployment
- PostgreSQL on same machine
- Redis for caching
- Vertical scaling (bigger server)

### Growth (v0.3.0 - 10,000 users)
- Separate database server
- Redis cluster for sessions
- CDN for static assets
- Horizontal scaling (multiple API servers)
- Load balancer (nginx/traefik)

### Scale (v1.0.0 - 100,000+ users)
- Kubernetes orchestration
- Read replicas for PostgreSQL
- Database sharding (if needed)
- Microservices for heavy workloads
- Global CDN (CloudFlare/AWS CloudFront)
- Message queue (RabbitMQ/AWS SQS) for async tasks

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| API Response (p50) | < 100ms | API endpoints |
| API Response (p95) | < 300ms | API endpoints |
| API Response (p99) | < 500ms | API endpoints |
| Page Load | < 2s | Frontend initial load |
| Time to Interactive | < 3s | Frontend ready |
| Database Query | < 50ms | Single query |
| Image Load | < 1s | Optimized images |
| WebSocket Latency | < 100ms | Real-time messages |

## Monitoring & Observability

### Metrics (Prometheus)
- Request rate, latency, error rate (RED method)
- Database connection pool stats
- Cache hit/miss rates
- Active WebSocket connections
- User engagement metrics

### Tracing (Jaeger)
- Request flow across services
- Database query performance
- External API calls
- User action tracking

### Logging
- Structured JSON logs
- Log levels: error, warn, info, debug
- Correlation IDs for request tracing
- Sensitive data redaction

### Alerts
- High error rate (> 1%)
- High latency (p95 > 500ms)
- Database connection issues
- Disk space < 20%
- Memory usage > 80%

## Development Workflow

### Local Development
```bash
# Start infrastructure
docker-compose up -d

# Run backend
cd backend && cargo run

# Run frontend (another terminal)
cd frontend && flutter run -d chrome
```

### Testing
- **Backend:** Unit tests + Integration tests with test database
- **Frontend:** Widget tests + Integration tests
- **E2E:** Manual testing + Cypress/Playwright (future)

### Deployment
- **Development:** Docker Compose locally
- **Staging:** Docker Compose on VPS
- **Production:** Kubernetes or managed container service

### CI/CD Pipeline (Future)
1. Code commit triggers build
2. Run tests (unit, integration)
3. Security scanning (dependencies)
4. Build Docker images
5. Deploy to staging
6. Integration tests
7. Deploy to production (blue-green)

## Cost Estimates (Monthly)

### Development (v0.1.0)
- VPS (2 vCPU, 4GB RAM): $20
- Storage (50GB): $5
- Bandwidth: $5
- **Total: ~$30/month**

### Production v1.0.0 (10,000 users)
- Load Balancer: $15
- API Servers (2x): $40
- Database Server: $50
- Redis/Cache: $20
- Storage (500GB): $25
- CDN: $20
- Monitoring: $15
- **Total: ~$185/month**

### Scale (100,000+ users)
- Kubernetes cluster: $500+
- Managed PostgreSQL: $200+
- Managed Redis: $100+
- Storage (5TB+): $150+
- CDN: $100+
- **Total: $1,000+/month**

---

*Architecture decisions should be revisited as user base grows*
