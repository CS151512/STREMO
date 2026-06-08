FROM golang:1.26-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o smtp-service cmd/server/main.go
FROM alpine:3.19

WORKDIR /app

COPY --from=builder /app/smtp-service .
COPY --from=builder /app/templates ./templates

EXPOSE 8085

CMD ["./smtp-service"]
