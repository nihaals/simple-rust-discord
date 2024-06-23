FROM rust:slim-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /src/target/release/simple-rust-discord /app
CMD ["/app"]
