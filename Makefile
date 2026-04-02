.PHONY: help setup build push deploy all status destroy clean registry ingress

K8S_DIR ?= k8s
REGISTRY ?= localhost:5000
BACKEND_IMAGE ?= $(REGISTRY)/jojuhu-backend:latest
FRONTEND_IMAGE ?= $(REGISTRY)/jojuhu-frontend:latest
WEBSITE_IMAGE ?= $(REGISTRY)/jojuhu-website:latest
CLUSTER_NAME ?= jojuhu
REGISTRY_NAME ?= jojuhu-registry

help:
	@echo "Jojuhu Kubernetes Deployment (Kind)"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  registry       - Create local Docker registry"
	@echo "  setup          - Create Kind cluster with local registry"
	@echo "  ingress        - Install NGINX Ingress Controller"
	@echo "  build          - Build all Docker images"
	@echo "  build-backend  - Build backend image only"
	@echo "  build-frontend - Build Flutter frontend (Mobile App)"
	@echo "  build-website  - Build Astro website (Marketing Site)"
	@echo "  push           - Push images to local registry"
	@echo "  deploy         - Deploy to Kubernetes"
	@echo "  all            - Complete setup and deploy"
	@echo "  status         - Show deployment status"
	@echo "  logs           - Show backend logs"
	@echo "  destroy        - Delete cluster"
	@echo "  clean          - Clean Docker images and volumes"
	@echo ""
	@echo "Services:"
	@echo "  jojuhu.local      - Website (Astro marketing site)"
	@echo "  app.jojuhu.local  - Flutter App (Mobile application)"
	@echo ""

registry:
	@echo "Creating local registry..."
	@if docker ps -a | grep -q "$(REGISTRY_NAME)"; then \
		echo "Registry already exists"; \
	else \
		docker run -d --name $(REGISTRY_NAME) -p 5000:5000 --network bridge registry:2; \
		echo "Registry created"; \
	fi

setup:
	@echo "Creating Kind cluster..."
	@if kind get clusters 2>/dev/null | grep -q "$(CLUSTER_NAME)"; then \
		echo "Cluster already exists, deleting..."; \
		kind delete cluster --name $(CLUSTER_NAME); \
	fi
	@echo "Creating cluster config..."
	@echo 'kind: Cluster' > /tmp/kind-config.yaml
	@echo 'apiVersion: kind.x-k8s.io/v1alpha4' >> /tmp/kind-config.yaml
	@echo 'nodes:' >> /tmp/kind-config.yaml
	@echo '- role: control-plane' >> /tmp/kind-config.yaml
	@echo '  extraPortMappings:' >> /tmp/kind-config.yaml
	@echo '  - containerPort: 80' >> /tmp/kind-config.yaml
	@echo '    hostPort: 80' >> /tmp/kind-config.yaml
	@echo '    protocol: TCP' >> /tmp/kind-config.yaml
	@echo '  - containerPort: 443' >> /tmp/kind-config.yaml
	@echo '    hostPort: 443' >> /tmp/kind-config.yaml
	@echo '    protocol: TCP' >> /tmp/kind-config.yaml
	@echo '- role: worker' >> /tmp/kind-config.yaml
	@echo 'containerdConfigPatches:' >> /tmp/kind-config.yaml
	@echo '- |-' >> /tmp/kind-config.yaml
	@echo '  [plugins."io.containerd.grpc.v1.cri".registry.mirrors."localhost:5000"]' >> /tmp/kind-config.yaml
	@echo '    endpoint = ["http://jojuhu-registry:5000"]' >> /tmp/kind-config.yaml
	@kind create cluster --name $(CLUSTER_NAME) --config /tmp/kind-config.yaml --image kindest/node:v1.29.2
	@echo "Connecting registry to kind network..."
	@docker network connect kind $(REGISTRY_NAME) 2>/dev/null || true
	@echo "Cluster created successfully"

ingress:
	@echo "Installing NGINX Ingress Controller..."
	kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
	@echo "Waiting for NGINX Ingress to be ready..."
	kubectl wait --namespace ingress-nginx \
		--for=condition=ready pod \
		--selector=app.kubernetes.io/component=controller \
		--timeout=90s
	@echo "NGINX Ingress Controller installed"

build-backend:
	@echo "Building backend image..."
	cd backend && docker build -t $(BACKEND_IMAGE) .
	@echo "Backend image built successfully"

build-frontend:
	@echo "Building Flutter frontend (Mobile App)..."
	cd frontend && docker build -t $(FRONTEND_IMAGE) .
	@echo "Flutter frontend built successfully"

build-website:
	@echo "Building Astro website (Marketing Site)..."
	cd website && docker build -t $(WEBSITE_IMAGE) .
	@echo "Astro website built successfully"

build: build-backend build-frontend build-website
	@echo "All images built successfully"

push:
	@echo "Pushing images to local registry..."
	docker push $(BACKEND_IMAGE)
	docker push $(FRONTEND_IMAGE)
	docker push $(WEBSITE_IMAGE)
	@echo "Images pushed successfully"

deploy:
	@echo "Deploying to Kubernetes..."
	kubectl apply -f $(K8S_DIR)/base/namespace.yml
	kubectl apply -f $(K8S_DIR)/base/configmap.yml
	kubectl apply -f $(K8S_DIR)/base/secret.yml
	
	@echo "Deploying databases..."
	kubectl apply -f $(K8S_DIR)/services/postgres.yml
	kubectl apply -f $(K8S_DIR)/services/redis.yml
	kubectl apply -f $(K8S_DIR)/services/minio.yml
	
	@echo "Waiting for databases..."
	@sleep 30
	
	@echo "Deploying services..."
	kubectl apply -f $(K8S_DIR)/services/backend.yml
	kubectl apply -f $(K8S_DIR)/services/frontend.yml
	kubectl apply -f $(K8S_DIR)/services/website.yml
	kubectl apply -f $(K8S_DIR)/services/ingress.yml
	
	@echo "Deploying monitoring..."
	kubectl apply -f $(K8S_DIR)/monitoring/prometheus.yml
	kubectl apply -f $(K8S_DIR)/monitoring/grafana.yml
	kubectl apply -f $(K8S_DIR)/monitoring/jaeger.yml
	
	@echo "Waiting for deployments..."
	kubectl rollout status deployment/jojuhu-backend -n jojuhu --timeout=120s
	kubectl rollout status deployment/jojuhu-frontend -n jojuhu --timeout=60s
	
	@echo "Deployment completed"
	@echo "Don't forget to add '127.0.0.1 jojuhu.local' to your /etc/hosts file"

all: registry setup ingress build push deploy

status:
	@echo "=== Pods ==="
	kubectl get pods -n jojuhu
	@echo ""
	@echo "=== Services ==="
	kubectl get svc -n jojuhu
	@echo ""
	@echo "=== Ingress ==="
	kubectl get ingress -n jojuhu

logs:
	kubectl logs -n jojuhu -l app=jojuhu-backend -f --all-containers

destroy:
	@echo "Destroying cluster..."
	kind delete cluster --name $(CLUSTER_NAME)
	@echo "Removing registry..."
	@docker rm -f $(REGISTRY_NAME) 2>/dev/null || true
	@echo "Cluster destroyed"

clean:
	@echo "Cleaning Docker resources..."
	docker system prune -f
	docker volume prune -f
	@echo "Cleanup completed"

.PHONY: port-forward
port-forward:
	kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080