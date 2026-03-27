# Quick Start Guide

Get Jojuhu running locally in 5 minutes!

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) & Docker Compose
- [Flutter](https://docs.flutter.dev/get-started/install) (for frontend development)
- [Rust](https://www.rust-lang.org/tools/install) (for backend development)
- [Git](https://git-scm.com/downloads)

## 1. Clone the Repository

```bash
git clone https://github.com/yourusername/jojuhu.git
cd jojuhu
```

## 2. Start Infrastructure Services

```bash
./start.sh
```

This will:
- ✅ Create Docker network
- ✅ Start PostgreSQL, Redis, MinIO, and observability stack
- ✅ Set up environment files
- ✅ Show service URLs

**Services will be available at:**
- Backend API: http://localhost:8080
- PostgreSQL: localhost:5432
- Redis: localhost:6379
- MinIO Console: http://localhost:9001 (minioadmin/minioadmin)
- Grafana: http://localhost:3000 (admin/admin)
- Jaeger: http://localhost:16686

## 3. Run the Backend

In a new terminal:

```bash
cd backend

# Copy environment file
cp .env.example .env

# Build and run
cargo build --release
cargo run
```

The backend will start at `http://localhost:8080`

**Test it's working:**
```bash
curl http://localhost:8080/health
```

Should return:
```json
{
  "status": "healthy",
  "service": "jojuhu_backend",
  "timestamp": "2026-03-26T10:00:00Z"
}
```

## 4. Run the Frontend

In another terminal:

```bash
cd frontend

# Get dependencies
flutter pub get

# Run on Chrome
flutter run -d chrome
```

The Flutter app will open in Chrome at `http://localhost:<port>` (usually 3000, 5000, or similar)

**Note:** The Flutter web port changes each time. If you get CORS errors, the backend now automatically supports common development ports (3000, 5000, 8080, 4200).

## 5. Create Your First Account

1. Open the Flutter web app in Chrome
2. Click "Register" 
3. Create an account with:
   - Username: your_username
   - Email: your@email.com
   - Password: (min 8 characters)
4. You should be redirected to the home screen!

## Common Issues

### Port Already in Use

If you see "port is already allocated":

```bash
# Find what's using the port
lsof -i :8080

# Kill the process or change ports in docker-compose.yml
```

### CORS Errors

If you see CORS errors in Chrome console:

1. Check the backend is running: `curl http://localhost:8080/health`
2. Check Flutter is running on a supported port (3000, 5000, 8080, 4200)
3. If using a different port, set:
   ```bash
   cd backend
   ALLOWED_ORIGINS=http://localhost:YOUR_PORT cargo run
   ```

### Database Connection Errors

```bash
# Reset database (WARNING: deletes all data)
docker-compose down -v
docker-compose up -d postgres

# Wait for PostgreSQL to be healthy, then run backend
```

### Flutter Build Errors

```bash
cd frontend

# Clean build
flutter clean
flutter pub get

# Verify setup
flutter doctor

# Run again
flutter run -d chrome
```

## Development Workflow

### Daily Development

```bash
# 1. Start infrastructure (if not running)
docker-compose up -d

# 2. Terminal 1 - Backend
cd backend
cargo run

# 3. Terminal 2 - Frontend
cd frontend
flutter run -d chrome

# 4. Open browser and develop!
```

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f postgres
```

### Run Tests

**Backend:**
```bash
cd backend
cargo test
```

**Frontend:**
```bash
cd frontend
flutter test
```

### Database Migrations

When schema changes:

```bash
cd backend

# Install sqlx-cli if not already installed
cargo install sqlx-cli

# Create new migration
sqlx migrate add description_of_change

# Run migrations
sqlx migrate run

# Revert last migration (careful!)
sqlx migrate revert
```

### Reset Everything

```bash
# Stop all services
docker-compose down

# Remove all data (volumes)
docker-compose down -v

# Start fresh
./start.sh
```

## Environment Variables

### Backend (.env)

```bash
# Database
DATABASE_URL=postgres://jojuhu:jojuhu_secret@localhost:5432/jojuhu_backend

# Server
HOST=127.0.0.1
PORT=8080

# JWT
JWT_SECRET=your-super-secret-key-change-in-production
JWT_EXPIRATION_HOURS=24

# CORS (comma-separated)
ALLOWED_ORIGINS=http://localhost:3000,http://localhost:5000

# Environment
RUST_LOG=info
APP_ENVIRONMENT=development
```

### Frontend

No environment file needed for local development. The frontend uses `http://localhost:8080` by default.

To change API URL:

```bash
# Set environment variable before running
export API_URL=http://your-api-url:8080
flutter run -d chrome
```

## Next Steps

Now that Jojuhu is running:

1. **Read the [Roadmap](ROADMAP.md)** - See what's planned
2. **Check [Architecture](ARCHITECTURE.md)** - Understand the system
3. **Review v0.1.0 Checklist** - See what features are being built
4. **Pick a Task** - Start contributing!

## Troubleshooting

### Backend won't start

- Check PostgreSQL is running: `docker-compose ps postgres`
- Check DATABASE_URL is correct
- Check port 8080 isn't in use

### Frontend shows blank screen

- Check backend is running
- Open Chrome DevTools (F12) → Console
- Look for red errors
- Check Network tab for failed requests

### Can't register/login

- Check backend logs for errors
- Verify PostgreSQL has users table
- Check JWT_SECRET is set

### Images don't upload

- Check MinIO is running: `docker-compose ps minio`
- Check MinIO console at http://localhost:9001
- Verify MinIO bucket exists: `jojuhu-uploads`

## Getting Help

- 📖 [Architecture Guide](ARCHITECTURE.md)
- 📋 [Release Checklist](v0.1.0-checklist.md)
- 🗺️ [Product Roadmap](ROADMAP.md)
- 🐛 [Create an Issue](https://github.com/yourusername/jojuhu/issues)

## Production Deployment

**⚠️ Not yet ready for production!**

See [Architecture Guide](ARCHITECTURE.md#production-deployment) for production setup details.

Key requirements before production:
- [ ] Security audit
- [ ] HTTPS/TLS setup
- [ ] Proper secrets management
- [ ] Database backups
- [ ] Monitoring & alerting
- [ ] Load testing

---

**Happy coding! 🚀**
