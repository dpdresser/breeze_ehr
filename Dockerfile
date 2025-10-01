FROM rust:1.88-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY . .
RUN mkdir -p ./public
RUN cargo build --release

# Use debian slim instead of distroless to get curl
FROM debian:bookworm-slim

# Install curl for health checks
RUN apt-get update && apt-get install -y curl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/breeze_ehr /app/breeze_ehr
COPY --from=builder /app/public ./public

EXPOSE 3000
ENTRYPOINT ["/app/breeze_ehr"]
