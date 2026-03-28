#!/bin/sh
# Cron-based backup script for Docker container

echo "Setting up backup cron job..."

# Create cron job
printenv | grep -E '^(PG|BACKUP)' > /etc/environment

echo "$BACKUP_SCHEDULE /backup-cron.sh run" > /etc/crontabs/root
echo "Backup cron scheduled: $BACKUP_SCHEDULE"

# Start cron in foreground
crond -f

# Alternative: run single backup and exit
# exec pg_dump -h "$PGHOST" -p "$PGPORT" -U "$PGUSER" -d "$PGDATABASE" -F plain | gzip > "/backups/jojuhu_$(date +%Y%m%d_%H%M%S).sql.gz"