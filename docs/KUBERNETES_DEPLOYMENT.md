# Plano de Implementação Kubernetes - Jojuhu

## Visão Geral

Este documento detalha o plano para containerização e deploy do ecossistema Jojuhu em Kubernetes, incluindo ambiente local com Kind.

## Componentes do Ecossistema

| Componente | Tecnologia | Porta | Descrição |
|------------|-----------|-------|-----------|
| Backend API | Rust/Actix-web | 8080 | API REST principal |
| Frontend | Astro/Node.js | 3000 | Interface web |
| PostgreSQL | Banco de dados | 5432 | Persistência |
| Redis | Cache/Session | 6379 | Cache e sessões |
| MinIO | Object Storage | 9000 | Upload de midias |
| Prometheus | Monitoramento | 9090 | Métricas |
| Grafana | Dashboards | 3001 | Visualização |
| Jaeger | Tracing | 16686 | Distributed tracing |

---

## Estrutura de Diretórios

```
jojuhu/
├── k8s/
│   ├── base/
│   │   ├── namespace.yml
│   │   ├── configmap.yml
│   │   ├── secret.yml
│   │   └── storage.yml
│   ├── services/
│   │   ├── backend.yml
│   │   ├── frontend.yml
│   │   ├── postgres.yml
│   │   ├── redis.yml
│   │   ├── minio.yml
│   │   └── ingress.yml
│   ├── monitoring/
│   │   ├── prometheus.yml
│   │   ├── grafana.yml
│   │   └── jaeger.yml
│   └── overlays/
│       ├── local/
│       │   └── kustomization.yml
│       └── prod/
│           └── kustomization.yml
├── docker-compose.k8s.yml
└── Makefile
```

---

## Fases de Implementação

### Fase 1: Preparação do Ambiente Local

**Objetivo**: Configurar Kind e criar registry local

#### 1.1 Instalação de Dependências
```bash
# Kind
curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.20.0/kind-linux-amd64
chmod +x ./kind
sudo mv ./kind /usr/local/bin/kind

# Kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
chmod +x kubectl
sudo mv kubectl /usr/local/bin/

# Helm (opcional)
curl -fsSL https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

#### 1.2 Criar Registry Local
```bash
# Criar registry Docker local
docker run -d --name jojuhu-registry -p 5000:5000 --network bridge registry:2

# Conectar ao network do Kind (após criar o cluster)
docker network connect kind jojuhu-registry
```

#### 1.3 Criar Cluster Kind
```bash
# Criar cluster com config file
cat <<EOF | kind create cluster --name jojuhu --config -
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
networking:
  apiServerAddress: "0.0.0.0"
  apiServerPort: 6443
containerdConfigPatches:
- |-
  [plugins."io.containerd.grpc.v1.cri".registry.mirrors."localhost:5000"]
    endpoint = ["http://jojuhu-registry:5000"]
nodes:
- role: control-plane
  kubeadmConfigPatches:
  - |
    kind: InitConfiguration
    nodeRegistration:
      kubeletExtraArgs:
        node-labels: "ingress-ready=true"
  extraPortMappings:
  - containerPort: 80
    hostPort: 80
    protocol: TCP
  - containerPort: 443
    hostPort: 443
    protocol: TCP
- role: worker
EOF

# Verificar cluster
kubectl cluster-info
```

---

### Fase 2: Containerização dos Componentes

**Objetivo**: Criar Dockerfiles otimizados para cada serviço

#### 2.1 Backend API (existing)
- ✅ Dockerfile existente em `backend/Dockerfile`
- ✅ Multi-stage build com Rust
- ✅ Usa porta 8080 com healthcheck

#### 2.2 Frontend Astro
```dockerfile
# website/Dockerfile
FROM node:24-alpine AS builder

# Install pnpm
RUN npm install -g pnpm@8.15.0

WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .
RUN pnpm run build

FROM nginx:alpine AS runtime
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]
```

```nginx
# website/nginx.conf
server {
    listen 3000;
    server_name localhost;
    root /usr/share/nginx/html;
    index index.html;
    
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    location /api {
        proxy_pass http://jojuhu-backend:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

#### 2.3 Push para Registry Local
```bash
# Build e push backend
docker build -t localhost:5000/jojuhu-backend:latest ./backend
docker push localhost:5000/jojuhu-backend:latest

# Build e push frontend
docker build -t localhost:5000/jojuhu-frontend:latest ./website
docker push localhost:5000/jojuhu-frontend:latest
```

---

### Fase 3: Manifestos Kubernetes

**Objetivo**: Criar todos os recursos K8s necessários

#### 3.1 Namespace e ConfigMaps
```yaml
# k8s/base/namespace.yml
apiVersion: v1
kind: Namespace
metadata:
  name: jojuhu
  labels:
    app.kubernetes.io/name: jojuhu
---
# k8s/base/configmap.yml
apiVersion: v1
kind: ConfigMap
metadata:
  name: jojuhu-config
  namespace: jojuhu
data:
  DATABASE_URL: "postgres://jojuhu:jojuhu_secret@postgres:5432/jojuhu_backend"
  REDIS_URL: "redis://redis:6379"
  MINIO_ENDPOINT: "http://minio:9000"
  MINIO_BUCKET_NAME: "jojuhu-uploads"
  RUST_LOG: "info"
  APP_ENVIRONMENT: "production"
---
# k8s/base/secret.yml
apiVersion: v1
kind: Secret
metadata:
  name: jojuhu-secret
  namespace: jojuhu
type: Opaque
stringData:
  JWT_SECRET: "change-me-in-production-min-32-chars"
  POSTGRES_PASSWORD: "jojuhu_secret"
  MINIO_ACCESS_KEY: "minioadmin"
  MINIO_SECRET_KEY: "minioadmin"
```

#### 3.2 Backend Deployment
```yaml
# k8s/services/backend.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: jojuhu-backend
  namespace: jojuhu
spec:
  replicas: 2
  selector:
    matchLabels:
      app: jojuhu-backend
  template:
    metadata:
      labels:
        app: jojuhu-backend
    spec:
      containers:
      - name: backend
        image: localhost:5000/jojuhu-backend:latest
        ports:
        - containerPort: 8080
        envFrom:
        - configMapRef:
            name: jojuhu-config
        - secretRef:
            name: jojuhu-secret
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: jojuhu-backend
  namespace: jojuhu
spec:
  selector:
    app: jojuhu-backend
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP
```

#### 3.3 Frontend Deployment
```yaml
# k8s/services/frontend.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: jojuhu-frontend
  namespace: jojuhu
spec:
  replicas: 2
  selector:
    matchLabels:
      app: jojuhu-frontend
  template:
    metadata:
      labels:
        app: jojuhu-frontend
    spec:
      containers:
      - name: frontend
        image: localhost:5000/jojuhu-frontend:latest
        ports:
        - containerPort: 3000
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "256Mi"
            cpu: "200m"
---
apiVersion: v1
kind: Service
metadata:
  name: jojuhu-frontend
  namespace: jojuhu
spec:
  selector:
    app: jojuhu-frontend
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
```

#### 3.4 PostgreSQL
```yaml
# k8s/services/postgres.yml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgres
  namespace: jojuhu
spec:
  serviceName: postgres
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:16-alpine
        ports:
        - containerPort: 5432
        env:
        - name: POSTGRES_USER
          value: "jojuhu"
        - name: POSTGRES_DB
          value: "jojuhu_backend"
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: jojuhu-secret
              key: POSTGRES_PASSWORD
        volumeMounts:
        - name: postgres-data
          mountPath: /var/lib/postgresql/data
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
  volumeClaimTemplates:
  - metadata:
      name: postgres-data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 5Gi
---
apiVersion: v1
kind: Service
metadata:
  name: postgres
  namespace: jojuhu
spec:
  clusterIP: None
  ports:
  - port: 5432
    targetPort: 5432
```

#### 3.5 Redis
```yaml
# k8s/services/redis.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
  namespace: jojuhu
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
        command: ["redis-server", "/redis.conf"]
        volumeMounts:
        - name: redis-config
          mountPath: /redis.conf
          subPath: redis.conf
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
        volumes:
        - name: redis-config
          configMap:
            name: redis-config
---
apiVersion: v1
kind: Service
metadata:
  name: redis
  namespace: jojuhu
spec:
  selector:
    app: redis
  ports:
  - port: 6379
    targetPort: 6379
```

#### 3.6 MinIO (Object Storage)
```yaml
# k8s/services/minio.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: minio
  namespace: jojuhu
spec:
  replicas: 1
  selector:
    matchLabels:
      app: minio
  template:
    metadata:
      labels:
        app: minio
    spec:
      containers:
      - name: minio
        image: minio/minio:latest
        ports:
        - containerPort: 9000
        env:
        - name: MINIO_ROOT_USER
          valueFrom:
            secretKeyRef:
              name: jojuhu-secret
              key: MINIO_ACCESS_KEY
        - name: MINIO_ROOT_PASSWORD
          valueFrom:
            secretKeyRef:
              name: jojuhu-secret
              key: MINIO_SECRET_KEY
        command: ["server", "/data", "--console-address", ":9001"]
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
---
apiVersion: v1
kind: Service
metadata:
  name: minio
  namespace: jojuhu
spec:
  selector:
    app: minio
  ports:
  - port: 9000
    targetPort: 9000
  - port: 9001
    targetPort: 9001
```

#### 3.7 Ingress
```yaml
# k8s/services/ingress.yml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: jojuhu-ingress
  namespace: jojuhu
  annotations:
    nginx.ingress.kubernetes.io/proxy-body-size: "50m"
    nginx.ingress.kubernetes.io/proxy-read-timeout: "300"
spec:
  ingressClassName: nginx
  rules:
  - host: jojuhu.local
    http:
      paths:
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: jojuhu-backend
            port:
              number: 8080
      - path: /
        pathType: Prefix
        backend:
          service:
            name: jojuhu-frontend
            port:
              number: 3000
```

---

### Fase 4: Stack de Observabilidade

**Objetivo**: Configurar monitoramento completo

#### 4.1 Prometheus
```yaml
# k8s/monitoring/prometheus.yml
apiVersion: monitoring.coreos.com/v1
kind: Prometheus
metadata:
  name: prometheus
  namespace: jojuhu
spec:
  replicas: 1
  serviceAccountName: prometheus
  serviceMonitorSelector:
    matchLabels:
      team: jojuhu
```

#### 4.2 Grafana
```yaml
# k8s/monitoring/grafana.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: grafana
  namespace: jojuhu
spec:
  replicas: 1
  template:
    spec:
      containers:
      - name: grafana
        image: grafana/grafana:latest
        ports:
        - containerPort: 3001
        env:
        - name: GF_SERVER_ROOT_URL
          value: "http://jojuhu.local:3001"
        - name: GF_SERVER_SERVE_FROM_SUB_PATH
          value: "true"
---
apiVersion: v1
kind: Service
metadata:
  name: grafana
  namespace: jojuhu
spec:
  selector:
    app: grafana
  ports:
  - port: 3001
    targetPort: 3001
```

#### 4.3 Jaeger
```yaml
# k8s/monitoring/jaeger.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: jaeger
  namespace: jojuhu
spec:
  replicas: 1
  template:
    spec:
      containers:
      - name: jaeger
        image: jaegertracing/all-in-one:latest
        ports:
        - containerPort: 16686
        - containerPort: 6831
        - containerPort: 14268
---
apiVersion: v1
kind: Service
metadata:
  name: jaeger
  namespace: jojuhu
spec:
  selector:
    app: jaeger
  ports:
  - port: 16686
    targetPort: 16686
    name: ui
  - port: 6831
    targetPort: 6831
    name: agent
```

---

### Fase 5: Deploy Local

**Objetivo**: Executar o deploy local completo

#### 5.1 Script de Deploy
```bash
#!/bin/bash
# deploy-local.sh

set -e

echo "=== Criando namespace ==="
kubectl apply -f k8s/base/namespace.yml

echo "=== Aplicando configmaps e secrets ==="
kubectl apply -f k8s/base/configmap.yml
kubectl apply -f k8s/base/secret.yml

echo "=== Deploying banco de dados ==="
kubectl apply -f k8s/services/postgres.yml

echo "=== Deploying Redis ==="
kubectl apply -f k8s/services/redis.yml

echo "=== Deploying MinIO ==="
kubectl apply -f k8s/services/minio.yml

echo "=== Deploying Backend ==="
kubectl apply -f k8s/services/backend.yml

echo "=== Deploying Frontend ==="
kubectl apply -f k8s/services/frontend.yml

echo "=== Deploying Ingress ==="
kubectl apply -f k8s/services/ingress.yml

echo "=== Deploying Monitoramento ==="
kubectl apply -f k8s/monitoring/

echo "=== Verificando status ==="
kubectl get pods -n jojuhu
kubectl get svc -n jojuhu

echo "=== Deploy concluído! ==="
echo "Acesse: http://jojuhu.local"
echo "Grafana: http://jojuhu.local:3001"
echo "Jaeger: http://jojuhu.local:16686"
```

#### 5.2 Adicionar hosts
```bash
# /etc/hosts (Linux/Mac)
echo "127.0.0.1 jojuhu.local" | sudo tee -a /etc/hosts
```

---

### Fase 6: Operações

#### 6.1 Comandos Úteis
```bash
# Ver pods
kubectl get pods -n jojuhu

# Ver logs
kubectl logs -n jojuhu -l app=jojuhu-backend

# Restart deploy
kubectl rollout restart deployment/jojuhu-backend -n jojuhu

# Port forward
kubectl port-forward -n jojuhu svc/jojuhu-backend 8080:8080

# Excluir tudo
kubectl delete namespace jojuhu
```

---

## Cronograma Sugerido

| Fase | Descrição | Tempo Estimado |
|------|-----------|----------------|
| 1 | Preparação Kind + Registry | 30 min |
| 2 | Containerização Frontend | 1 hora |
| 3 | Manifestos K8s | 2 horas |
| 4 | Observabilidade | 1 hora |
| 5 | Deploy Local | 30 min |
| **Total** | | **5 horas** |

---

## Próximos Passos

1. **Aprovar o plano** - Concordar com a estrutura proposta
2. **Criar diretório k8s/** - Estrutura de arquivos
3. **Criar Dockerfile frontend** - Containerizar Astro
4. **Gerar Manifestos** - Todos os arquivos YAML
5. **Testar Deploy Local** - Executar em K3d

---

> Este plano será executado em fases sequenciais após aprovação.