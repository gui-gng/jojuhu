# Docker Deployment Guide

## Quick Start (Local Development)

```bash
# Build and start with docker-compose
docker-compose up --build

# The API will be available at http://localhost:8080
```

## Production Deployment

### 1. Set Environment Variables

Create a `.env` file:

```bash
cp .env.production.example .env
# Edit .env with your production values
```

Required variables:
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secure random string (min 32 chars)

### 2. Using Docker Compose (Production)

```bash
# Start the application
docker-compose -f docker-compose.prod.yml up -d

# View logs
docker-compose -f docker-compose.prod.yml logs -f

# Stop
docker-compose -f docker-compose.prod.yml down
```

### 3. Using Docker Only

```bash
# Build image
docker build -t social-network-api .

# Run container
docker run -d \
  --name social-network \
  -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host:5432/db \
  -e JWT_SECRET=your-secret-key \
  -e RUST_LOG=info \
  social-network-api
```

## Database Migrations

Migrations run **automatically** on application startup via the built-in SQLx migration system. No manual action required.

To create new migrations during development:

```bash
# Install sqlx-cli locally (requires Rust)
cargo install sqlx-cli --no-default-features --features postgres

# Create a new migration
sqlx migrate add <migration_name>
```

## Health Check

```bash
curl http://localhost:8080/health
```

Response:
```json
{
  "status": "healthy",
  "service": "social_network"
}
```

## Image Size Optimization

The Dockerfile uses a multi-stage build:
- **Builder stage**: ~2GB (includes Rust toolchain and build deps)
- **Runtime stage**: ~150MB (debian-slim with runtime deps)

## Security Features

- Non-root user (`appuser`)
- Minimal runtime dependencies
- Health checks configured
- No sensitive data in image layers
