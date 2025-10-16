# Builder stage
FROM golang:1.24-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o /jojuhu-backend .

# Development stage
FROM builder AS development

RUN go install github.com/go-delve/delve/cmd/dlv@latest

EXPOSE 5001
EXPOSE 2345

CMD ["/go/bin/dlv", "--listen=:2345", "--headless=true", "--api-version=2", "--accept-multiclient", "exec", "/jojuhu-backend"]

# Production stage
FROM alpine:latest AS production

COPY --from=builder /jojuhu-backend /jojuhu-backend
COPY --from=builder /app/docs /app/docs

EXPOSE 5001

CMD ["/jojuhu-backend"]