#!/bin/bash

# Jojuhu Project Startup Script
# This script sets up the environment and starts all services

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_NAME="jojuhu"
NETWORK_NAME="network-jojuhu"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is installed
check_docker() {
    log_info "Checking Docker installation..."
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
    
    log_success "Docker and Docker Compose are installed"
}

# Check if Docker daemon is running
check_docker_daemon() {
    log_info "Checking Docker daemon..."
    if ! docker info &> /dev/null; then
        log_error "Docker daemon is not running. Please start Docker first."
        exit 1
    fi
    log_success "Docker daemon is running"
}

# Create Docker network if it doesn't exist
create_network() {
    log_info "Checking Docker network: $NETWORK_NAME"
    
    if docker network ls | grep -q "$NETWORK_NAME"; then
        log_success "Network '$NETWORK_NAME' already exists"
    else
        log_info "Creating Docker network: $NETWORK_NAME"
        docker network create "$NETWORK_NAME"
        log_success "Network '$NETWORK_NAME' created successfully"
    fi
}

# Create environment files if they don't exist
setup_environment() {
    log_info "Setting up environment files..."
    
    # Root .env file
    if [ ! -f "$SCRIPT_DIR/.env" ]; then
        if [ -f "$SCRIPT_DIR/.env.example" ]; then
            cp "$SCRIPT_DIR/.env.example" "$SCRIPT_DIR/.env"
            log_success "Created root .env file from .env.example"
        else
            log_warning "No .env.example file found in root"
        fi
    else
        log_info "Root .env file already exists"
    fi
    
    # Infrastructure .env file
    if [ ! -f "$SCRIPT_DIR/infrastructure/.env" ]; then
        if [ -f "$SCRIPT_DIR/infrastructure/.env.example" ]; then
            cp "$SCRIPT_DIR/infrastructure/.env.example" "$SCRIPT_DIR/infrastructure/.env"
            log_success "Created infrastructure .env file"
        else
            log_warning "No .env.example file found in infrastructure/"
        fi
    else
        log_info "Infrastructure .env file already exists"
    fi
    
    # Backend .env file
    if [ ! -f "$SCRIPT_DIR/backend/.env" ]; then
        if [ -f "$SCRIPT_DIR/backend/.env.example" ]; then
            cp "$SCRIPT_DIR/backend/.env.example" "$SCRIPT_DIR/backend/.env"
            log_success "Created backend .env file"
        else
            log_warning "No .env.example file found in backend/"
        fi
    else
        log_info "Backend .env file already exists"
    fi
}

# Create necessary directories
create_directories() {
    log_info "Creating necessary directories..."
    
    # Create data directories
    mkdir -p "$SCRIPT_DIR/data/postgres"
    mkdir -p "$SCRIPT_DIR/data/redis"
    mkdir -p "$SCRIPT_DIR/data/minio"
    
    log_success "Directories created"
}

# Pull latest images
pull_images() {
    log_info "Pulling latest Docker images..."
    cd "$SCRIPT_DIR"
    docker-compose pull
    log_success "Images pulled successfully"
}

# Start services
start_services() {
    log_info "Starting all services..."
    cd "$SCRIPT_DIR"
    
    # Build and start services
    docker-compose up -d --build
    
    log_success "All services started successfully"
}

# Wait for services to be healthy
wait_for_services() {
    log_info "Waiting for services to be healthy..."
    
    # Wait for PostgreSQL
    log_info "Waiting for PostgreSQL..."
    until docker-compose ps postgres | grep -q "healthy"; do
        sleep 2
    done
    log_success "PostgreSQL is healthy"
    
    # Wait for Backend
    log_info "Waiting for Backend API..."
    sleep 5
    until curl -sf http://localhost:8080/health > /dev/null 2>&1; do
        sleep 2
    done
    log_success "Backend API is healthy"
}

# Display service URLs
show_urls() {
    echo ""
    echo "=========================================="
    echo "  Jojuhu Services are running!"
    echo "=========================================="
    echo ""
    echo "Application Services:"
    echo "  Backend API:    http://localhost:8080"
    echo "  Health Check:   http://localhost:8080/health"
    echo ""
    echo "Infrastructure Services:"
    echo "  PostgreSQL:     localhost:5432"
    echo "  Redis:          localhost:6379"
    echo "  MinIO Console:  http://localhost:9001 (minioadmin/minioadmin)"
    echo "  MinIO API:      http://localhost:9000"
    echo ""
    echo "Observability:"
    echo "  Grafana:        http://localhost:3000 (admin/admin)"
    echo "  Prometheus:     http://localhost:9090"
    echo "  Jaeger UI:      http://localhost:16686"
    echo ""
    echo "Useful Commands:"
    echo "  View logs:      docker-compose logs -f [service]"
    echo "  Stop services:  docker-compose down"
    echo "  Full cleanup:   docker-compose down -v"
    echo ""
    echo "=========================================="
}

# Main execution
main() {
    echo "=========================================="
    echo "  Starting Jojuhu Project"
    echo "=========================================="
    echo ""
    
    # Pre-flight checks
    check_docker
    check_docker_daemon
    
    # Setup
    create_network
    setup_environment
    create_directories
    
    # Start services
    pull_images
    start_services
    
    # Wait and verify
    wait_for_services
    
    # Show information
    show_urls
}

# Handle script interruption
trap 'log_error "Script interrupted"; exit 1' INT TERM

# Run main function
main "$@"
