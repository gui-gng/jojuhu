# Social Network Backend

A Rust-based backend API for a social network platform, built with Actix-web.

## Features

- **Messages**: Direct messaging between users with conversation threading
- **Timeline**: Posts, likes, comments with social feed
- **Forums**: Community forums with topics and replies
- **Authentication**: JWT-based authentication with secure password hashing

## Architecture

This project follows a modular, domain-driven architecture inspired by Clean Architecture principles:

```
src/
├── auth/              # Authentication logic (login, register, JWT)
│   ├── handlers.rs    # Auth HTTP handlers
│   ├── models.rs      # Auth data models
│   └── mod.rs         # Auth service functions
├── config/            # Configuration management
│   └── mod.rs         # Settings and environment config
├── errors/            # Error handling types
│   └── mod.rs         # AppError enum and conversions
├── middleware/        # Authentication and logging middleware
│   ├── auth.rs        # JWT validation middleware
│   └── logging.rs     # Request logging middleware
├── models/            # Shared data models
│   ├── user.rs        # User model
│   └── mod.rs         # Common models (ApiResponse, Pagination)
├── modules/           # Domain modules
│   ├── messages/      # Direct messaging module
│   │   ├── handlers.rs
│   │   ├── models.rs
│   │   ├── repository.rs
│   │   ├── routes.rs
│   │   └── service.rs
│   ├── timeline/      # Posts and social feed module
│   │   ├── handlers.rs
│   │   ├── models.rs
│   │   ├── repository.rs
│   │   ├── routes.rs
│   │   └── service.rs
│   └── forums/        # Community forums module
│       ├── handlers.rs
│       ├── models.rs
│       ├── repository.rs
│       ├── routes.rs
│       └── service.rs
├── routes/            # Centralized route configuration
│   └── mod.rs         # All route definitions
├── utils/             # Utilities
│   ├── auth.rs        # JWT token functions
│   ├── validators.rs  # Input validation
│   └── mod.rs         # Password hashing
├── lib.rs             # Library exports
└── main.rs            # Application entry point
```

### Module Structure

Each domain module (messages, timeline, forums) follows a consistent structure:

- **`models.rs`** - Data models and DTOs (Data Transfer Objects)
- **`repository.rs`** - Database access layer (raw SQL queries)
- **`service.rs`** - Business logic layer
- **`handlers.rs`** - HTTP request handlers (Actix-web)
- **`routes.rs`** - Route definitions
- **`mod.rs`** - Module configuration and dependency injection

### Key Design Decisions

1. **Separation of Concerns**: Each layer has a single responsibility
   - Handlers: HTTP layer only, no business logic
   - Services: Business logic, no HTTP or database details
   - Repositories: Database access only

2. **Dependency Injection**: Services receive repositories via constructors, enabling testability

3. **Error Handling**: Centralized `AppError` type with automatic HTTP status mapping

4. **Authentication**: Extracted into `auth/` module for reusability

5. **Route Configuration**: Centralized in `routes/` module for better overview

## Prerequisites

- Rust 1.75+
- PostgreSQL 14+
- Cargo

## Getting Started

1. **Clone the repository**

2. **Set up environment variables**

   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Set up the database**

   ```bash
   # Create database
   createdb social_network_rust

   # Migrations run automatically on startup
   ```

4. **Run the application**

   ```bash
   cargo run
   ```

5. **Run tests**

   ```bash
   # Run all tests
   cargo test
   
   # Run unit tests only
   cargo test --test unit_tests
   
   # Run integration tests only
   cargo test --test integration_tests
   ```

   See [tests/README.md](tests/README.md) for more details.

## API Documentation

### Authentication

| Method | Endpoint                | Description       |
| ------ | ----------------------- | ----------------- |
| POST   | `/api/v1/auth/register` | Register new user |
| POST   | `/api/v1/auth/login`    | Login user        |
| GET    | `/api/v1/me`            | Get current user  |

### Messages

| Method | Endpoint                                   | Description        |
| ------ | ------------------------------------------ | ------------------ |
| POST   | `/api/v1/messages`                         | Send message       |
| GET    | `/api/v1/messages/conversations`           | List conversations |
| GET    | `/api/v1/messages/conversations/{user_id}` | Get conversation   |
| PUT    | `/api/v1/messages/{message_id}/read`       | Mark as read       |
| DELETE | `/api/v1/messages/{message_id}`            | Delete message     |

### Timeline

| Method | Endpoint                                                 | Description    |
| ------ | -------------------------------------------------------- | -------------- |
| POST   | `/api/v1/timeline/posts`                                 | Create post    |
| GET    | `/api/v1/timeline/feed`                                  | Get feed       |
| GET    | `/api/v1/timeline/posts/{post_id}`                       | Get post       |
| PUT    | `/api/v1/timeline/posts/{post_id}`                       | Update post    |
| DELETE | `/api/v1/timeline/posts/{post_id}`                       | Delete post    |
| POST   | `/api/v1/timeline/posts/{post_id}/like`                  | Like post      |
| DELETE | `/api/v1/timeline/posts/{post_id}/like`                  | Unlike post    |
| GET    | `/api/v1/timeline/posts/{post_id}/comments`              | Get comments   |
| POST   | `/api/v1/timeline/posts/{post_id}/comments`              | Add comment    |
| DELETE | `/api/v1/timeline/posts/{post_id}/comments/{comment_id}` | Delete comment |

### Forums

| Method | Endpoint                                                         | Description  |
| ------ | ---------------------------------------------------------------- | ------------ |
| GET    | `/api/v1/forums`                                                 | List forums  |
| POST   | `/api/v1/forums`                                                 | Create forum |
| GET    | `/api/v1/forums/{forum_id}`                                      | Get forum    |
| PUT    | `/api/v1/forums/{forum_id}`                                      | Update forum |
| DELETE | `/api/v1/forums/{forum_id}`                                      | Delete forum |
| POST   | `/api/v1/forums/{forum_id}/join`                                 | Join forum   |
| POST   | `/api/v1/forums/{forum_id}/leave`                                | Leave forum  |
| GET    | `/api/v1/forums/{forum_id}/topics`                               | List topics  |
| POST   | `/api/v1/forums/{forum_id}/topics`                               | Create topic |
| GET    | `/api/v1/forums/{forum_id}/topics/{topic_id}`                    | Get topic    |
| DELETE | `/api/v1/forums/{forum_id}/topics/{topic_id}`                    | Delete topic |
| POST   | `/api/v1/forums/{forum_id}/topics/{topic_id}/lock`               | Lock topic   |
| POST   | `/api/v1/forums/{forum_id}/topics/{topic_id}/unlock`             | Unlock topic |
| POST   | `/api/v1/forums/{forum_id}/topics/{topic_id}/pin`                | Pin topic    |
| POST   | `/api/v1/forums/{forum_id}/topics/{topic_id}/unpin`              | Unpin topic  |
| GET    | `/api/v1/forums/{forum_id}/topics/{topic_id}/replies`            | Get replies  |
| POST   | `/api/v1/forums/{forum_id}/topics/{topic_id}/replies`            | Create reply |
| DELETE | `/api/v1/forums/{forum_id}/topics/{topic_id}/replies/{reply_id}` | Delete reply |

## Authentication

All protected endpoints require a Bearer token in the Authorization header:

```
Authorization: Bearer <token>
```

The token is obtained from the `/api/v1/auth/login` or `/api/v1/auth/register` endpoints.

## Tech Stack

- **Web Framework**: Actix-web
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT with bcrypt
- **Serialization**: Serde
- **Validation**: Validator
- **Logging**: Tracing
- **Migrations**: SQLx migrate
- **Testing**: Built-in Rust testing + tokio-test

## Project Structure for New Features

To add a new feature module:

1. Create a new directory under `src/modules/`
2. Add the standard files: `mod.rs`, `models.rs`, `repository.rs`, `service.rs`, `handlers.rs`, `routes.rs`
3. Register the module in `src/modules/mod.rs`
4. Add routes in `src/routes/mod.rs`
5. Write tests in `tests/unit/` for service logic

Example module structure:

```rust
// src/modules/myfeature/mod.rs
pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;

use actix_web::web;
use sqlx::PgPool;

use self::repository::MyFeatureRepository;
use self::service::MyFeatureService;

pub fn configure_module(cfg: &mut web::ServiceConfig, pool: PgPool) {
    let repository = MyFeatureRepository::new(pool);
    let service = MyFeatureService::new(repository);
    
    cfg.app_data(web::Data::new(service))
        .configure(routes::configure_routes);
}
```

## License

[Your License Here]
