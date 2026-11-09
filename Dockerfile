FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 appuser
COPY --from=builder /app/target/release/hello-rust /usr/local/bin/hello-rust

USER appuser
ENTRYPOINT ["hello-rust"]
