#!/bin/sh
# Docker backup entrypoint with cron scheduling

BACKUP_DIR="/backups"
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-7}"

# Create backup function
perform_backup() {
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_FILE="${BACKUP_DIR}/jojuhu_backup_${TIMESTAMP}.sql.gz"
    
    echo "[$(date)] Starting backup to ${BACKUP_FILE}..."
    
    if pg_dump -h "$PGHOST" -p "$PGPORT" -U "$PGUSER" -d "$PGDATABASE" -F plain 2>/dev/null | gzip > "$BACKUP_FILE"; then
        echo "[$(date)] Backup completed: ${BACKUP_FILE}"
        
        # Clean old backups
        find "$BACKUP_DIR" -name "jojuhu_backup_*.sql.gz" -mtime +$RETENTION_DAYS -delete
        echo "[$(date)] Cleaned backups older than ${RETENTION_DAYS} days"
    else
        echo "[$(date)] ERROR: Backup failed!"
        rm -f "$BACKUP_FILE"
    fi
}

# Create backup directory
mkdir -p "$BACKUP_DIR"

echo "Database backup service started"
echo "Host: $PGHOST:$PGPORT"
echo "Database: $PGDATABASE"
echo "Retention: $RETENTION_DAYS days"
echo "Backup directory: $BACKUP_DIR"
echo ""

# Perform initial backup
perform_backup

# Run daily at 2 AM
while true; do
    CURRENT_HOUR=$(date +%H)
    if [ "$CURRENT_HOUR" = "02" ]; then
        perform_backup
        sleep 3600  # Sleep for 1 hour to avoid multiple backups
    else
        sleep 300  # Check every 5 minutes
    fi
done