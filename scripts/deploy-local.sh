#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
K8S_DIR="$(dirname "$SCRIPT_DIR")/k8s"
CLUSTER_NAME="jojuhu"
REGISTRY_NAME="jojuhu-registry"
REGISTRY_PORT="5000"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

check_dependencies() {
    log_info "Checking dependencies..."
    
    local missing=0
    
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl is not installed"
        missing=1
    fi
    
    if ! command -v docker &> /dev/null; then
        log_error "docker is not installed"
        missing=1
    fi
    
    if ! command -v kind &> /dev/null; then
        log_error "kind is not installed"
        missing=1
    fi
    
    if [ $missing -eq 1 ]; then
        log_error "Please install missing dependencies and try again"
        exit 1
    fi
    
    log_success "All dependencies are installed"
}

create_registry() {
    log_info "Creating local registry..."
    
    if docker ps -a | grep -q "${REGISTRY_NAME}"; then
        log_warning "Registry '${REGISTRY_NAME}' already exists"
    else
        docker run -d --name "${REGISTRY_NAME}" -p "${REGISTRY_PORT}:5000" --network bridge registry:2
        log_success "Registry created"
    fi
}

create_cluster() {
    log_info "Creating Kind cluster..."
    
    if kind get clusters 2>/dev/null | grep -q "${CLUSTER_NAME}"; then
        log_warning "Cluster '${CLUSTER_NAME}' already exists. Deleting it..."
        kind delete cluster --name "${CLUSTER_NAME}"
    fi
    
    cat > /tmp/kind-config.yaml << 'EOF'
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraPortMappings:
  - containerPort: 80
    hostPort: 80
    protocol: TCP
  - containerPort: 443
    hostPort: 443
    protocol: TCP
- role: worker
containerdConfigPatches:
- |-
  [plugins."io.containerd.grpc.v1.cri".registry.mirrors."localhost:5000"]
    endpoint = ["http://jojuhu-registry:5000"]
EOF
    
    kind create cluster --name "${CLUSTER_NAME}" --config /tmp/kind-config.yaml --image kindest/node:v1.29.2
    
    log_info "Connecting registry to kind network..."
    docker network connect kind "${REGISTRY_NAME}" 2>/dev/null || true
    
    log_success "Cluster created"
}

install_nginx_ingress() {
    log_info "Installing NGINX Ingress Controller..."
    
    kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
    
    log_info "Waiting for NGINX Ingress to be ready..."
    kubectl wait --namespace ingress-nginx \
        --for=condition=ready pod \
        --selector=app.kubernetes.io/component=controller \
        --timeout=90s
    
    log_success "NGINX Ingress Controller installed"
}

build_images() {
    log_info "Building Docker images..."
    
    log_info "Building backend image..."
    docker build -t localhost:${REGISTRY_PORT}/jojuhu-backend:latest ./backend
    
    log_info "Building app image..."
    docker build -t localhost:${REGISTRY_PORT}/jojuhu-app:latest ./jojuhu-app
    
    log_success "Images built"
}

push_images() {
    log_info "Pushing images to local registry..."
    
    docker push localhost:${REGISTRY_PORT}/jojuhu-backend:latest
    docker push localhost:${REGISTRY_PORT}/jojuhu-app:latest
    
    log_success "Images pushed to registry"
}

deploy() {
    log_info "Deploying to Kubernetes..."
    
    local k8s_dir="$K8S_DIR"
    
    log_info "Creating namespace..."
    kubectl apply -f "$k8s_dir/base/namespace.yml"
    
    log_info "Applying ConfigMaps and Secrets..."
    kubectl apply -f "$k8s_dir/base/configmap.yml"
    kubectl apply -f "$k8s_dir/base/secret.yml"
    
    log_info "Deploying PostgreSQL..."
    kubectl apply -f "$k8s_dir/services/postgres.yml"
    
    log_info "Deploying Redis..."
    kubectl apply -f "$k8s_dir/services/redis.yml"
    
    log_info "Deploying MinIO..."
    kubectl apply -f "$k8s_dir/services/minio.yml"
    
    log_info "Waiting for databases to be ready..."
    kubectl rollout status statefulset/postgres -n jojuhu --timeout=120s
    kubectl rollout status deployment/redis -n jojuhu --timeout=60s
    kubectl rollout status deployment/minio -n jojuhu --timeout=60s
    
    log_info "Deploying Backend..."
    kubectl apply -f "$k8s_dir/services/backend.yml"
    
    log_info "Deploying App..."
    kubectl apply -f "$k8s_dir/services/app.yml"
    
    log_info "Deploying Ingress..."
    kubectl apply -f "$k8s_dir/services/ingress.yml"
    
    log_info "Deploying Monitoring..."
    kubectl apply -f "$k8s_dir/monitoring/prometheus.yml"
    kubectl apply -f "$k8s_dir/monitoring/grafana.yml"
    kubectl apply -f "$k8s_dir/monitoring/jaeger.yml"
    
    log_info "Waiting for deployments to be ready..."
    kubectl rollout status deployment/jojuhu-backend -n jojuhu --timeout=120s
    kubectl rollout status deployment/jojuhu-app -n jojuhu --timeout=60s
    
    log_success "Deployment completed"
}

show_status() {
    log_info "Current status:"
    echo ""
    kubectl get pods -n jojuhu
    echo ""
    kubectl get svc -n jojuhu
    echo ""
    kubectl get ingress -n jojuhu
    echo ""
    log_success "Don't forget to add '127.0.0.1 jojuhu.local' to your /etc/hosts file"
    log_info "Access the application at: http://jojuhu.local"
    log_info "Grafana: http://jojuhu.local/grafana (admin/admin)"
    log_info "Jaeger: http://jojuhu.local/jaeger"
}

destroy() {
    log_warning "Destroying cluster..."
    kind delete cluster --name "${CLUSTER_NAME}"
    
    log_warning "Removing registry..."
    docker rm -f "${REGISTRY_NAME}" 2>/dev/null || true
    
    log_success "Cluster destroyed"
}

case "${1:-}" in
    setup)
        check_dependencies
        create_cluster
        install_nginx_ingress
        ;;
    build)
        build_images
        ;;
    push)
        push_images
        ;;
    deploy)
        deploy
        ;;
    ingress)
        install_nginx_ingress
        ;;
    all)
        check_dependencies
        create_registry
        create_cluster
        install_nginx_ingress
        build_images
        push_images
        deploy
        show_status
        ;;
    status)
        show_status
        ;;
    destroy)
        destroy
        ;;
    registry)
        create_registry
        ;;
    *)
        echo "Usage: $0 {setup|build|push|deploy|all|status|destroy|registry|ingress}"
        echo ""
        echo "Commands:"
        echo "  setup   - Create Kind cluster"
        echo "  build   - Build Docker images"
        echo "  push    - Push images to local registry"
        echo "  deploy  - Deploy to Kubernetes"
        echo "  ingress - Install NGINX Ingress Controller"
        echo "  all     - Complete setup and deploy"
        echo "  status  - Show deployment status"
        echo "  destroy - Delete cluster"
        echo "  registry- Create local registry only"
        exit 1
        ;;
esac