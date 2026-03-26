-- Initialize database extensions and initial setup
-- This script runs automatically when the PostgreSQL container starts

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";

-- Create application user if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'jojuhu_app') THEN
        CREATE USER jojuhu_app WITH PASSWORD 'jojuhu_app_secret';
    END IF;
END
$$;

-- Grant privileges to application user
GRANT ALL PRIVILEGES ON DATABASE jojuhu_backend TO jojuhu_app;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO jojuhu_app;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO jojuhu_app;

-- Create indexes for common queries (will be created by migrations, but these help with initial setup)
COMMENT ON DATABASE jojuhu_backend IS 'Jojuhu Social Network Backend Database';
