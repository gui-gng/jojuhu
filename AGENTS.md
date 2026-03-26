# AGENTS.md - AI Coding Agent Instructions

This document provides guidelines for AI agents working on the Social Network Backend project.

## Project Overview

This is a Rust-based backend API for a social network platform, built with Actix-web. It follows Clean Architecture principles with modular domain-driven design.

## Build/Lint/Test Commands

```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run all tests
cargo test

# Run only unit tests
cargo test --test unit_tests

# Run only integration tests
cargo test --test integration_tests

# Run a specific test
cargo test test_validate_username_valid

# Run tests with output
cargo test -- --nocapture

# Run Clippy lints
cargo clippy -- -D warnings

# Run Clippy with all features
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check

# SQLx migrations (requires DATABASE_URL)
cargo sqlx migrate run

# Docker compose
docker-compose up -d
docker-compose -f docker-compose.prod.yml up -d
```

## Code Style Guidelines

### Imports
- Group imports: std lib, external crates, internal modules
- Use `use crate::` for internal imports
- Alphabetically sort within groups
- Example:
```rust
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;
```

### Formatting
- Use `cargo fmt` default settings
- Max line length: follow Rust defaults (100 chars)
- 4 spaces for indentation
- No trailing whitespace

### Types and Naming
- **Structs/Enums**: PascalCase (e.g., `MessageResponse`, `AppError`)
- **Functions/Variables**: snake_case (e.g., `send_message`, `user_id`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Traits**: PascalCase with clear purpose
- **Modules**: snake_case

### Error Handling
- Use the centralized `AppError` enum for all errors
- Implement `From` traits for automatic conversion (e.g., `From<sqlx::Error>`)
- Map database errors to appropriate HTTP status codes
- Use `thiserror` for error definitions
- Return `Result<T, AppError>` from handlers and services

### Architecture Patterns

Each domain module follows this structure:
```
modules/{name}/
├── mod.rs        # Module config, dependency injection
├── models.rs     # Data models and DTOs
├── repository.rs # Database access layer
├── service.rs    # Business logic layer
├── handlers.rs   # HTTP request handlers
└── routes.rs     # Route definitions
```

Guidelines:
- **Handlers**: HTTP layer only, delegate to services
- **Services**: Business logic, no HTTP or DB details
- **Repositories**: Database access with raw SQL queries
- **Models**: Use `#[derive(Debug, Serialize, Deserialize)]` for DTOs
- Use dependency injection via constructors

### Database
- Use SQLx for compile-time checked queries
- Use `sqlx::FromRow` for query results
- Prefer raw SQL in repositories over ORM abstractions
- Use UUIDs for primary keys (v4 for new records)
- Use chrono for datetime fields

### Testing
- Unit tests go in `tests/unit/{feature}_tests.rs`
- Integration tests go in `tests/integration_tests.rs`
- Use `#[actix_rt::test]` for async tests
- Mock external dependencies, test business logic in isolation
- Use descriptive test names: `test_validate_username_too_short`

### API Responses
- Use standardized JSON responses:
```rust
json!({
    "success": false,
    "error": message
})
```

### Authentication
- Use Bearer token in Authorization header
- JWT validation handled in middleware
- Extract user_id from authenticated requests

### Logging
- Use `tracing` for structured logging
- Use appropriate levels: trace, debug, info, warn, error

## Environment Setup

Required environment variables:
```bash
DATABASE_URL=postgres://user:password@localhost:5432/social_network_rust
JWT_SECRET=your_secret_key
JWT_EXPIRATION_HOURS=24
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

## Pre-Commit Checklist

- [ ] Code compiles: `cargo build`
- [ ] Tests pass: `cargo test`
- [ ] Clippy clean: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt`
- [ ] No hardcoded secrets or credentials
- [ ] Error handling implemented for all fallible operations
