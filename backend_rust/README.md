# Social Network Backend

A Rust-based backend API for a social network platform, built with Actix-web.

## Features

- **Messages**: Direct messaging between users with conversation threading
- **Timeline**: Posts, likes, comments with social feed
- **Forums**: Community forums with topics and replies

## Architecture

This project follows a modular, domain-driven architecture:

```
src/
├── config/          # Configuration management
├── errors/          # Error handling types
├── middleware/      # Authentication and logging middleware
├── models/          # Shared data models
├── modules/         # Domain modules
│   ├── messages/    # Direct messaging module
│   ├── timeline/    # Posts and social feed module
│   └── forums/      # Community forums module
├── utils/           # Utilities (auth, validators)
└── main.rs          # Application entry point
```

Each module contains:

- `models.rs` - Data models and DTOs
- `repository.rs` - Database access layer
- `service.rs` - Business logic
- `handlers.rs` - HTTP request handlers
- `routes.rs` - Route definitions

## Prerequisites

- Rust 1.75+
- PostgreSQL 14+

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
   cargo test
   ```

## API Documentation

### Authentication

| Method | Endpoint                | Description       |
| ------ | ----------------------- | ----------------- |
| POST   | `/api/v1/auth/register` | Register new user |
| POST   | `/api/v1/auth/login`    | Login user        |
| GET    | `/api/v1/auth/me`       | Get current user  |

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
