# AGENTS.md

Monorepo for Jojuhu social network platform.

## Directories

| Directory | Tech | Run Commands |
|-----------|------|--------------|
| `backend/` | Rust (Actix-web) | `cargo build`, `cargo run`, `cargo test` |
| `frontend/` | Flutter | `flutter pub get`, `flutter run` |
| `website/` | Astro (pnpm) | `pnpm dev`, `pnpm build` |
| `infrastructure/` | Docker Compose | `docker-compose up -d` |
| `seeder/` | Rust | `cargo run` |
| `k8s/ | Kubernetes manifests | `make deploy` |
| `tests/api_flows/` | Python tests | `./run_all_tests.py` |

## Frontend (Flutter)

**Design System:**
- Theme: `lib/theme/jojuhu_theme.dart` - Colors (Sol/Lua/Encontro palette), Material 3
- Typography: `lib/theme/typography.dart` - Playfair Display + Inter fonts
- Tokens: `lib/theme/tokens.dart` - Spacing, radius, durations, elevations
- Animations: `lib/utils/animations.dart` - Reusable animation presets

**Widgets (`lib/widgets/`):**
- `jojuhu_avatar.dart` - Avatar with caching, sizes, badge support
- `jojuhu_button.dart` - Button variants (primary/secondary/outline/text/danger)
- `jojuhu_card.dart` - Card container with elevation options
- `jojuhu_text_field.dart` - Input with focus animations, validation states
- `jojuhu_image_viewer.dart` - Full-screen zoomable images
- `jojuhu_shimmer.dart` - Loading skeleton components
- `jojuhu_post_card.dart` - Social post card component
- `jojuhu_widgets.dart` - Barrel export for all widgets

**Key Packages:**
- `cached_network_image` - Image caching (replace all `Image.network()`)
- `flutter_screenutil` - Responsive sizing (`.w`, `.h`, `.sp`)
- `google_fonts` - Playfair Display + Inter typography
- `flutter_animate` - Declarative animations
- `photo_view` - Image zoom/pan

**Usage Pattern:**
```dart
// Responsive sizing
Container(width: 100.w, height: 50.h)

// Use design tokens
padding: EdgeInsets.all(JojuhuSpacing.lg)
borderRadius: JojuhuRadius.smRadius

// Import all widgets
import 'package:jojuhu/widgets/jojuhu_widgets.dart';```

## Backend Commands

```bash
cd backend

# Build/run
cargo build
cargo build --release
cargo run                    # Runs on localhost:8080

# Testing
cargo test                   # All tests
cargo test --test unit_tests # Unit tests only
cargo test --test integration_tests # Integration tests only
cargo test test_name         # Specific test
cargo test -- --nocapture    # With output

# Linting
cargo clippy -- -D warnings
cargo fmt

# Database migrations (requires DATABASE_URL)
cargo sqlx migrate run
```

## Environment Setup

Backend requires `.env` in `backend/` directory. Copy from `backend/.env.example`.

Required for build:
- `DATABASE_URL` - Required for SQLx compile-time query verification

Required for runtime:
- `JWT_SECRET` (min 32 chars)
- `DATABASE_URL`
- `REDIS_URL`
- `MINIO_*` (for file uploads)

## Development Workflow

```bash
# Start infrastructure services (PostgreSQL, Redis, MinIO, etc.)
./start.sh                  # From repo root (creates .env files if missing)
# OR
cd infrastructure && docker-compose up -d

# Run backend
cd backend
cargo run

# Health check
curl http://localhost:8080/health
```

Infrastructure services (from `infrastructure/docker-compose.yml`):
- PostgreSQL: `localhost:5432`
- Redis: `localhost:6379`
- MinIO API: `localhost:9000`
- MinIO Console: `localhost:9001`
- Grafana: `localhost:3000`
- Prometheus: `localhost:9090`
- Jaeger: `localhost:16686`

## API Flow Tests (Python)

Integration tests for API endpoints in `tests/api_flows/`.

```bash
cd tests/api_flows
pip install requests

# Run all tests
./run_all_tests.py

# Run individually (order matters!)
python3 test_onboarding.py  # MUST run first - creates users/tokens
python3 test_posts.py       # Requires onboarding
python3 test_forums.py      # Requires onboarding
python3 test_messages.py    # Requires onboarding

# Custom endpoint
API_BASE_URL=http://localhost:8080 ./run_all_tests.py
```

## Backend Architecture

Domain modules in `backend/src/modules/{name}/`:
- `handlers.rs` - HTTP layer, delegates to services
- `service.rs` - Business logic, no HTTP/DB details
- `repository.rs` - Database access with SQLx
- `models.rs` - DTOs and row types
- `routes.rs` - Route definitions
- `mod.rs` - Module config and DI

Key patterns:
- SQLx compile-time checked queries (`sqlx::query_as!()`)
- Centralized `AppError` enum for HTTP errors
- `ApiResponse<T>` wrapper for responses
- Extract user from `AuthenticatedUser` middleware in protected routes

## Commit Prefixes

```
feat(backend/scope): description
fix(frontend/scope): description
chore(infrastructure): description
```

## Kubernetes Deployment

```bash
make all        # Full setup (registry, cluster, ingress, build, deploy)
make status     # Check pods/services
make logs       # Backend logs
make destroy    # Delete cluster

# Port-forward for local access
kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080
```

Add to `/etc/hosts`: `127.0.0.1 jojuhu.local app.jojuhu.local`