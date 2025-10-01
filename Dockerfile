FROM rust:1.88-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig ca-certificates

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

WORKDIR /app
COPY --from=builder /app/target/release/breeze_ehr /app/breeze_ehr
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

EXPOSE 3000
ENTRYPOINT ["/app/breeze_ehr"]
