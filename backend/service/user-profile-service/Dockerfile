FROM golang:1.26-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o user-profile-service cmd/server/main.go

FROM alpine:3.19

WORKDIR /app

COPY --from=builder /app/user-profile-service .

EXPOSE 8082

CMD ["./user-profile-service"]
