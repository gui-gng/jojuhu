# Flutter SDK for building
FROM ghcr.io/cirruslabs/flutter:3.24.5 AS builder

# Build argument for API URL (default empty for relative URLs)
ARG API_URL=""

WORKDIR /app

# Copy pubspec files first for better caching
COPY pubspec.yaml ./
COPY pubspec.lock* ./

# Get dependencies
RUN flutter pub get

# Copy source code
COPY lib ./lib
COPY web ./web
COPY assets ./assets
COPY analysis_options.yaml* ./

# Build for web with API URL
# If API_URL is empty, it will use relative URLs (works with nginx proxy)
RUN flutter build web --release --dart-define=API_URL=${API_URL}

# Runtime stage with nginx
FROM nginx:alpine AS runtime

# Copy built web files
COPY --from=builder /app/build/web /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Create non-root user
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup && \
    chown -R appuser:appgroup /usr/share/nginx/html && \
    chown -R appuser:appgroup /var/cache/nginx && \
    chown -R appuser:appgroup /var/log/nginx && \
    touch /var/run/nginx.pid && \
    chown -R appuser:appgroup /var/run/nginx.pid

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/ || exit 1

# Switch to non-root user
USER appuser

# Start nginx
CMD ["nginx", "-g", "daemon off;"]