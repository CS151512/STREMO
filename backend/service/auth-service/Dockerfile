FROM debian:bookworm-slim AS builder
RUN apt-get update && apt-get install -y cmake build-essential
WORKDIR /app

COPY . .
WORKDIR /app/service/auth-service
RUN mkdir build && cd build && cmake .. -DCMAKE_BUILD_TYPE=Release && make -j$(nproc)

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/service/auth-service/build/auth_service /app/auth_service
ENV PORT=8080
EXPOSE 8080
CMD ["/app/auth_service"]
