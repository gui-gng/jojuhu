#!/bin/bash
# Database Backup Script for Jojuhu
# Usage: ./scripts/backup-db.sh [backup-dir]

set -e

# Configuration
BACKUP_DIR="${1:-./backups}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="${BACKUP_DIR}/jojuhu_backup_${TIMESTAMP}.sql"

# Ensure backup directory exists
mkdir -p "$BACKUP_DIR"

# Get database connection from environment or use defaults
DB_HOST="${DATABASE_HOST:-localhost}"
DB_PORT="${DATABASE_PORT:-5432}"
DB_NAME="${DATABASE_NAME:-jojuhu_backend}"
DB_USER="${DATABASE_USER:-jojuhu}"
DB_PASSWORD="${DATABASE_PASSWORD:-jojuhu_secret}"

echo "Starting database backup..."
echo "Backup file: $BACKUP_FILE"

# Perform backup
PGPASSWORD="$DB_PASSWORD" pg_dump \
    -h "$DB_HOST" \
    -p "$DB_PORT" \
    -U "$DB_USER" \
    -d "$DB_NAME" \
    -F plain \
    -v \
    -f "$BACKUP_FILE"

# Compress backup
gzip "$BACKUP_FILE"
echo "Backup completed: ${BACKUP_FILE}.gz"

# Keep only last 7 days of backups
echo "Cleaning old backups..."
find "$BACKUP_DIR" -name "jojuhu_backup_*.sql.gz" -mtime +7 -delete

echo "Backup process completed successfully!"