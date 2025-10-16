# base go image
FROM golang:1.24-alpine as builder

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN CGO_ENABLED=0 go build -o user-service ./

RUN chmod +x /app/user-service

# build a tiny docker image
FROM alpine:latest

RUN mkdir /app

COPY --from=builder /app/user-service /app

CMD [ "/app/user-service" ]