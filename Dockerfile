FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev ca-certificates
WORKDIR /src
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /src/target/release/simple-rust-discord /app
CMD ["/app"]
