FROM rust:1.85-slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev cmake build-essential
WORKDIR /app

COPY . .
RUN cargo build --release -p bff-service

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/bff-service /app/bff-service
ENV RUST_LOG=info
ENV PORT=8000
EXPOSE 8000
CMD ["/app/bff-service"]
