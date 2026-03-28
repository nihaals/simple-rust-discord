FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /src/target/release/simple-rust-discord /app
CMD ["/app"]
