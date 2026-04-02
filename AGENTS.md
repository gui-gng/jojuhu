# AGENTS.md - AI Coding Agent Instructions

This document provides guidelines for AI agents working on the Social Network Backend project.

## Project Overview

This is a Rust-based backend API for a social network platform, built with Actix-web. It follows Clean Architecture principles with modular domain-driven design.

## Project Structure

```
jojuhu/
├── backend/              # Rust API (Actix-web)
├── frontend/             # Flutter mobile/web app
├── website/              # Astro marketing site
├── k8s/                  # Kubernetes deployment configs
├── tests/                # Test suites
│   └── api_flows/        # API integration tests
├── docs/                 # Documentation
│   ├── versions/         # Release notes per version
│   │   ├── README.md     # Version index
│   │   ├── v0.1.0.md     # MVP (Complete)
│   │   ├── v0.2.0.md     # In Progress (85%)
│   │   ├── v0.3.0.md     # Planned
│   │   └── v0.4.0.md     # Production (v1.0.0)
│   ├── ARCHITECTURE.md   # Technical architecture
│   ├── ROADMAP.md        # Product roadmap
│   ├── QUICKSTART.md     # Getting started guide
│   └── KUBERNETES_DEPLOYMENT.md  # K8s deployment docs
└── AGENTS.md            # This file
```

## Version Management

The project uses semantic versioning with detailed release notes in `docs/versions/`:

- **v0.1.0** ✅ COMPLETE - MVP with core features
- **v0.2.0** 🚧 IN PROGRESS (85%) - Real-time features, notifications
- **v0.3.0** 📋 PLANNED - Groups, moderation, analytics
- **v1.0.0** 📋 PLANNED - Production launch, mobile apps

Always check `docs/versions/README.md` for current status and `docs/versions/vX.Y.Z.md` for specific version details.

## Build/Lint/Test Commands

```bash
# Build the project (run from backend/ directory)
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run all tests
cargo test

# Run a specific test by name
cargo test test_validate_username_valid

# Run tests in a specific file
cargo test --test handler_tests

# Run tests with output
cargo test -- --nocapture

# Run Clippy lints
cargo clippy -- -D warnings

# Run Clippy with all targets and features
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
- Group imports in order: std lib, external crates, internal modules
- Use `use crate::` for internal imports
- Alphabetically sort within groups
- Separate groups with blank lines

Example:
```rust
use std::fmt;

use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{ApiResponse, PaginationParams};
```

### Formatting
- Use `cargo fmt` default settings
- Max line length: follow Rust defaults (100 chars)
- 4 spaces for indentation
- No trailing whitespace

### Types and Naming
- **Structs/Enums**: PascalCase (e.g., `MessageResponse`, `AppError`)
- **Functions/Variables**: snake_case (e.g., `send_message`, `user_id`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `MAX_MESSAGE_LENGTH`)
- **Traits**: PascalCase with clear purpose
- **Modules**: snake_case (e.g., `handlers.rs`, `repository.rs`)
- **Generic parameters**: Single uppercase letters (e.g., `T`, `K`, `V`)

### Error Handling
- Use the centralized `AppError` enum for all errors
- Implement `From` traits for automatic conversion (e.g., `From<sqlx::Error>`)
- Map database errors to appropriate HTTP status codes:
  - `RowNotFound` → 404 Not Found
  - Constraint violations → 409 Conflict
  - Other DB errors → 500 Internal Server Error
- Use `thiserror` for error definitions
- Return `Result<T, AppError>` from handlers and services
- Create error messages with `format!("...", value)`

### Architecture Patterns

Each domain module follows this structure:
```
modules/{name}/
├── mod.rs        # Module config, dependency injection, re-exports
├── models.rs     # Data models, DTOs, and database row types
├── repository.rs # Database access layer with SQLx queries
├── service.rs    # Business logic layer
├── handlers.rs   # HTTP request handlers
└── routes.rs     # Route definitions
```

Guidelines:
- **Handlers**: HTTP layer only, delegate to services, return `Result<HttpResponse, AppError>`
- **Services**: Business logic, no HTTP or DB details, validate inputs, sanitize data
- **Repositories**: Database access with raw SQL queries, use `sqlx::query_as!()`
- **Models**: Use `#[derive(Debug, Serialize, Deserialize)]` for DTOs
- Use dependency injection via constructors (e.g., `MessageService::new(repository)`)

### Database
- Use SQLx for compile-time checked queries with `sqlx::query_as!()`
- Use `sqlx::FromRow` for query result structs
- Prefer raw SQL in repositories over ORM abstractions
- Use UUIDs for primary keys (v4 for new records via `Uuid::new_v4()`)
- Use chrono for datetime fields (`DateTime<Utc>`)
- Use pagination with `get_offset()` and `get_limit()` methods

### Security
- Sanitize all user inputs with `sanitize_input()` from `middleware::security`
- Validate input safety with `validate_input_safety()` to prevent XSS/SQL injection
- Validate content length with `validate_content_length()`
- Use bcrypt/argon2 for password hashing
- All authenticated endpoints extract user via `AuthenticatedUser` middleware

### Testing
- Unit tests go in `tests/unit/{feature}_tests.rs`
- Integration tests go in `tests/integration_tests.rs`
- Use `#[actix_rt::test]` for async tests
- Use standard `#[test]` for synchronous tests
- Use descriptive test names: `test_{behavior}_{condition}` (e.g., `test_validate_username_too_short`)
- Mock external dependencies, test business logic in isolation
- Test error responses and status codes

### API Responses
- Use standardized `ApiResponse<T>` wrapper:
```rust
// Success
Ok(HttpResponse::Ok().json(ApiResponse::success(data)))

// Error (handled by AppError ResponseError trait)
Err(AppError::ValidationError("Invalid input".to_string()))
```
- HTTP status codes:
  - 200 OK for successful GET/PUT
  - 201 Created for successful POST
  - 204 No Content for successful DELETE
  - 400 Bad Request for validation errors
  - 401 Unauthorized for authentication errors
  - 403 Forbidden for authorization errors
  - 404 Not Found for missing resources
  - 409 Conflict for duplicate/constraint errors
  - 500 Internal Server Error for unexpected errors

### Authentication
- Use Bearer token in Authorization header
- JWT validation handled in `middleware::auth`
- Extract user_id from `AuthenticatedUser` in handlers
- Password hashing with argon2 (preferred) or bcrypt

### Logging
- Use `tracing` for structured logging
- Use appropriate levels: trace, debug, info, warn, error
- Log at service layer for business operations
- Log at handler layer for request/response info

## Environment Setup

Required environment variables:
```bash
DATABASE_URL=postgres://user:password@localhost:5432/social_network_rust
JWT_SECRET=your_secret_key_min_32_chars_long
JWT_EXPIRATION_HOURS=24
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

## Commit Guidelines

Always use semantic commits with project-specific prefixes.

### Commit Format
```
<type>(<project>/<scope>): <description>

[optional body]

[optional footer]
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

### Examples
```
feat(backend/auth): add JWT token refresh endpoint
fix(frontend/profile): resolve avatar upload validation bug
refactor(backend/messages): extract message validation logic
chore(infrastructure): update PostgreSQL to v15
docs(backend): add API documentation for user module
```

## Pre-Commit Checklist

- [ ] Code compiles: `cargo build`
- [ ] Tests pass: `cargo test`
- [ ] Clippy clean: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt`
- [ ] No hardcoded secrets or credentials
- [ ] Error handling implemented for all fallible operations
- [ ] Input validation and sanitization added for user inputs
- [ ] Documentation updated (if changing features)

## Documentation Guidelines

### Version Documentation
When implementing features:
1. Check `docs/versions/vX.Y.Z.md` for the target version
2. Update the feature status (✅ 🚧 📋)
3. Add technical details to version files

### Main Documentation Files
- **README.md** - Project overview and quick links
- **docs/ARCHITECTURE.md** - System design and technical decisions
- **docs/ROADMAP.md** - High-level product planning
- **docs/QUICKSTART.md** - Developer onboarding
- **docs/KUBERNETES_DEPLOYMENT.md** - Infrastructure setup

### API Documentation
- Update Bruno collections in `backend/bruno/`
- Document new endpoints with examples
- Include error response codes
