FROM rust:slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev cmake build-essential protobuf-compiler
WORKDIR /app

COPY . .
RUN cargo build --release -p ingest-service

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/ingest-service-service /app/ingest-service-service
ENV RUST_LOG=info
ENV PORT=8001
EXPOSE 8001
CMD ["/app/ingest-service"]
