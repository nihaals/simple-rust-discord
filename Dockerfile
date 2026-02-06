FROM rust:slim-trixie AS builder
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:trixie-slim
COPY --from=builder /src/target/release/simple-rust-discord /app
CMD ["/app"]
