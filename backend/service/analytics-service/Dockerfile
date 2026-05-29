FROM rust:1.77-slim-bookworm AS chef
USER root

RUN apt-get update && apt-get install -y lld clang protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
WORKDIR /app/service/analytics-service
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/service/analytics-service/recipe.json /app/service/analytics-service/recipe.json

WORKDIR /app/service/analytics-service
RUN cargo chef cook --release --recipe-path recipe.json

WORKDIR /app
COPY . .
WORKDIR /app/service/analytics-service
RUN cargo build --release -p analytics-service

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=builder /app/target/release/analytics-service /usr/local/bin/analytics-service

EXPOSE 8080 50051

ENV RUST_LOG="info,analytics_service=debug"
ENV APP_ENVIRONMENT="production"

ENTRYPOINT ["/usr/local/bin/analytics-service"]
