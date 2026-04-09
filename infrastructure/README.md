# Jojuhu Infrastructure

This directory contains all infrastructure services for the Jojuhu social network platform.

## Services

### Core Services

- **PostgreSQL** (`postgres:5432`) - Primary database
- **Redis** (`redis:6379`) - Caching and session storage
- **MinIO** (`minio:9000`) - S3-compatible object storage for uploads

### Observability Stack

- **OpenTelemetry Collector** (`otel-collector:4317/4318`) - Traces and metrics collection
- **Jaeger** (`jaeger:16686`) - Distributed tracing UI
- **Prometheus** (`prometheus:9090`) - Metrics storage and querying
- **Grafana** (`grafana:3000`) - Dashboards and visualization

## Quick Start

1. **Start all infrastructure services:**

```bash
cd infrastructure
docker-compose up -d
```

2. **Check service status:**

```bash
docker-compose ps
```

3. **View logs:**

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f postgres
```

4. **Stop all services:**

```bash
docker-compose down
```

5. **Stop and remove all data (clean slate):**

```bash
docker-compose down -v
```

## Environment Variables

Create a `.env` file in the `infrastructure` directory:

```bash
# Database
POSTGRES_USER=jojuhu
POSTGRES_PASSWORD=jojuhu_secret
POSTGRES_DB=jojuhu_backend_db
POSTGRES_PORT=5432

# Redis
REDIS_PORT=6379

# MinIO
MINIO_USER=minioadmin
MINIO_PASSWORD=minioadmin
MINIO_API_PORT=9000
MINIO_CONSOLE_PORT=9001

# OpenTelemetry
OTEL_GRPC_PORT=4317
OTEL_HTTP_PORT=4318
OTEL_PROM_PORT=8889
OTEL_HEALTH_PORT=13133

# Jaeger
JAEGER_UI_PORT=16686

# Prometheus
PROMETHEUS_PORT=9090

# Grafana
GRAFANA_USER=admin
GRAFANA_PASSWORD=admin
GRAFANA_PORT=3000
GRAFANA_ROOT_URL=http://localhost:3000
```

## Service URLs

After starting the infrastructure:

| Service | URL | Description |
|---------|-----|-------------|
| PostgreSQL | `postgres://localhost:5432` | Database connection |
| Redis | `redis://localhost:6379` | Cache connection |
| MinIO Console | http://localhost:9001 | Object storage UI |
| MinIO API | http://localhost:9000 | S3-compatible API |
| Grafana | http://localhost:3000 | Dashboards (admin/admin) |
| Prometheus | http://localhost:9090 | Metrics query |
| Jaeger UI | http://localhost:16686 | Trace visualization |

## Configuration

### PostgreSQL

- Configuration: `postgres/postgresql.conf`
- Initialization scripts: `postgres/init/`
- Data volume: `postgres_data`

### Redis

- Configuration: `redis/redis.conf`
- Data volume: `redis_data`

### OpenTelemetry

- Configuration: `opentelemetry/otel-collector-config.yaml`
- Receives traces via OTLP (gRPC: 4317, HTTP: 4318)
- Exports to Jaeger for traces, Prometheus for metrics

### Prometheus

- Configuration: `prometheus/prometheus.yml`
- Rules: `prometheus/rules/`
- Data volume: `prometheus_data`
- Retention: 15 days

### Grafana

- Provisioning: `grafana/provisioning/`
- Dashboards: `grafana/dashboards/`
- Data volume: `grafana_data`

## Database Initialization

The PostgreSQL container automatically runs SQL scripts from `postgres/init/` on first startup:

- `01-init.sql` - Creates extensions, users, and grants privileges

## MinIO Buckets

On startup, the `minio-createbucket` service creates:

- `jojuhu-uploads` - Public bucket for user uploads

## Application Integration

### Backend Environment Variables

```bash
# Database
DATABASE_URL=postgres://jojuhu:jojuhu_secret@localhost:5432/jojuhu_backend_db

# Redis (if using)
REDIS_URL=redis://localhost:6379

# Object Storage
AWS_ENDPOINT_URL=http://localhost:9000
AWS_ACCESS_KEY_ID=minioadmin
AWS_SECRET_ACCESS_KEY=minioadmin
AWS_BUCKET_NAME=jojuhu-uploads

# Observability
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
OTEL_SERVICE_NAME=jojuhu-backend
OTEL_SERVICE_VERSION=0.1.0
```

## Monitoring

### Accessing Dashboards

1. **Grafana** (http://localhost:3000)
   - Login: admin/admin
   - Pre-configured with Prometheus and Jaeger datasources
   - Dashboard: "Jojuhu Application Overview"

2. **Jaeger** (http://localhost:16686)
   - Search traces by service, operation, tags
   - View trace timelines and dependencies

3. **Prometheus** (http://localhost:9090)
   - Query metrics using PromQL
   - View targets and service discovery

### Useful Queries

**Prometheus:**

```promql
# Request rate
rate(http_requests_total[5m])

# P95 latency
histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))

# Error rate
rate(http_requests_total{status_code=~"5.."}[5m])
```

## Troubleshooting

### PostgreSQL Connection Issues

```bash
# Check PostgreSQL is running
docker-compose ps postgres

# View PostgreSQL logs
docker-compose logs postgres

# Connect to PostgreSQL
docker-compose exec postgres psql -U jojuhu -d jojuhu_backend_db
```

### Reset All Data

```bash
# Stop and remove volumes
docker-compose down -v

# Restart
docker-compose up -d
```

### Port Conflicts

If ports are already in use, change them in `.env`:

```bash
POSTGRES_PORT=5433  # Instead of 5432
REDIS_PORT=6380     # Instead of 6379
```

## Security Notes

⚠️ **Development Only**: Default passwords are for local development only. Change them for production:

- PostgreSQL: `POSTGRES_PASSWORD`
- MinIO: `MINIO_PASSWORD`
- Grafana: `GRAFANA_PASSWORD`

## Maintenance

### Backup PostgreSQL

```bash
docker-compose exec postgres pg_dump -U jojuhu jojuhu_backend_db > backup.sql
```

### Restore PostgreSQL

```bash
docker-compose exec -T postgres psql -U jojuhu -d jojuhu_backend_db < backup.sql
```

### Update Images

```bash
docker-compose pull
docker-compose up -d
```

## Architecture

```
┌─────────────────┐
│  Jojuhu Backend │
└────────┬────────┘
         │
    ┌────┴────┬──────────┬──────────┐
    ▼         ▼          ▼          ▼
┌───────┐ ┌────────┐ ┌────────┐ ┌──────────┐
│PostgreSQL│ │ Redis  │ │ MinIO  │ │ OpenTelemetry│
└───────┘ └────────┘ └────────┘ └────┬─────┘
                                     │
                    ┌────────────────┼────────────────┐
                    ▼                ▼                ▼
              ┌──────────┐     ┌──────────┐    ┌──────────┐
              │  Jaeger  │     │Prometheus│    │  Grafana │
              └──────────┘     └──────────┘    └──────────┘
```
