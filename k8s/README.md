# Kubernetes Deployment

Deploys do ecossistema Jojuhu em Kubernetes com Kind.

## Pré-requisitos

- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [kind](https://kind.sigs.k8s.io/) v0.20+
- [Docker](https://www.docker.com/)

## Estrutura

```
k8s/
├── base/              # Namespace, ConfigMaps, Secrets
│   ├── namespace.yml
│   ├── configmap.yml
│   └── secret.yml
├── services/          # Deployments and Services
│   ├── backend.yml
│   ├── frontend.yml
│   ├── postgres.yml
│   ├── redis.yml
│   ├── minio.yml
│   └── ingress.yml
├── monitoring/        # Prometheus, Grafana, Jaeger
│   ├── prometheus.yml
│   ├── grafana.yml
│   └── jaeger.yml
└── overlays/          # Kustomize overlays
    ├── local/
    └── prod/
```

## Deploy Local

### Opção 1: Script de Deploy

```bash
# Deploy completo
./scripts/deploy-local.sh all

# Ou passo a passo
./scripts/deploy-local.sh registry  # Criar registry local
./scripts/deploy-local.sh setup    # Criar cluster Kind
./scripts/deploy-local.sh ingress  # Instalar NGINX Ingress
./scripts/deploy-local.sh build    # Build das imagens
./scripts/deploy-local.sh push     # Push para registry
./scripts/deploy-local.sh deploy   # Deploy no K8s
```

### Opção 2: Makefile

```bash
# Deploy completo
make all

# Ou passo a passo
make registry   # Criar registry
make setup      # Criar cluster
make ingress    # Instalar NGINX Ingress
make build      # Build das imagens
make push       # Push para registry
make deploy     # Deploy no K8s
```

## Troubleshooting

### Erro: "failed to create cluster"

Se encontrar erros de cgroups, tente:

```bash
# Remover clusters existentes
kind delete cluster --name jojuhu

# Verificar Docker está rodando
docker ps

#Executar setup novamente
make setup
```

### Verificar cluster

```bash
# Verificar nós do cluster
kubectl get nodes

# Verificar pods do sistema
kubectl get pods -A
```

## Acessos

Após o deploy, adicione ao `/etc/hosts`:

```
127.0.0.1 jojuhu.local
```

| Serviço | URL | Credenciais |
|---------|-----|-------------|
| App | http://jojuhu.local | - |
| API | http://jojuhu.local/api | - |
| Grafana | http://jojuhu.local/grafana | admin/admin |
| Jaeger | http://jojuhu.local/jaeger | - |

## Comandos Úteis

```bash
# Ver status
make status

# Ver logs do backend
make logs

# Port-forward para desenvolvimento
make port-forward

# Destruir cluster
make destroy

# Limpar recursos Docker
make clean
```

## Sobreposição de Ambiente

```bash
# Local
kubectl apply -k k8s/overlays/local

# Produção
kubectl apply -k k8s/overlays/prod
```

## Escalar Deployments

```bash
kubectl scale deployment jojuhu-backend -n jojuhu --replicas=3
kubectl scale deployment jojuhu-frontend -n jojuhu --replicas=3
```

## Verificar Saúde

```bash
kubectl get pods -n jojuhu
kubectl describe pod <pod-name> -n jojuhu
kubectl logs <pod-name> -n jojuhu
```

## Troubleshooting

### Pods não iniciam

```bash
kubectl describe pod <pod-name> -n jojuhu
kubectl logs <pod-name> -n jojuhu
```

### Banco de dados não conecta

```bash
kubectl get svc postgres -n jojuhu
kubectl exec -it <postgres-pod> -n jojuhu -- psql -U jojuhu -d jojuhu_backend
```

### Registry não acessível

```bash
k3d cluster list
docker ps | grep registry
```

## Segurança em Produção

1. Altere secretos em `k8s/base/secret.yml`
2. Use secrets do Kubernetes ou Vault
3. Configure TLS via cert-manager
4. Configure networkpolicies
5. Aplique resourcequotas