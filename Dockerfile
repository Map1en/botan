FROM rust:1.87-slim AS builder

RUN apt-get update && apt-get install -y \
    gcc \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml ./

RUN sed -i '/"src-tauri"/d' Cargo.toml

COPY core ./core/
COPY worker ./worker/

RUN cargo build --release --package botan_worker

FROM debian:bookworm-slim AS runner

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG="info"

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/botan_worker /usr/local/bin/botan_worker

RUN mkdir -p /app/data
VOLUME /app/data

ENTRYPOINT ["/usr/local/bin/botan_worker"]