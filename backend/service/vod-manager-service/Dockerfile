FROM golang:1.26-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o vod-manager-service cmd/server/main.go
FROM alpine:3.19

WORKDIR /app

COPY --from=builder /app/vod-manager-service .

EXPOSE 8084

CMD ["./vod-manager-service"]
