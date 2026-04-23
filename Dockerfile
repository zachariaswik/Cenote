FROM rust:1.79-slim AS build
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY migrations ./migrations
COPY assets ./assets
COPY fixtures ./fixtures
RUN cargo build --release --bin cenote

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/cenote /usr/local/bin/cenote
COPY migrations ./migrations
COPY assets ./assets
ENV CENOTE_VAULT_PATH=/data/vault \
    CENOTE_DB_PATH=/data/cenote.db \
    CENOTE_VECTOR_PATH=/data/vector \
    CENOTE_HTTP_BIND=0.0.0.0:8787 \
    CENOTE_OLLAMA_URL=http://ollama:11434 \
    RUST_LOG=cenote=info
VOLUME /data
EXPOSE 8787
ENTRYPOINT ["cenote"]
CMD ["serve"]
