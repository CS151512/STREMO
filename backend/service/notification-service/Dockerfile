FROM alpine:3.19 AS builder

RUN apk add --no-cache \
    build-base \
    cmake \
    git \
    linux-headers \
    librdkafka-dev

WORKDIR /app
COPY . .

RUN mkdir build && cd build && \
    cmake -DCMAKE_BUILD_TYPE=Release .. && \
    make -j$(nproc)

FROM alpine:3.19

RUN apk add --no-cache libstdc++ librdkafka
WORKDIR /app
COPY --from=builder /app/build/notification_service .

EXPOSE 8087

CMD ["./notification_service"]
