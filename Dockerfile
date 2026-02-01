# Multi-stage Dockerfile: build Rust binary, produce minimal runtime image
FROM rust:1.72-slim AS builder
WORKDIR /usr/src/quantum-coherence

# Install required system tools for building and optional Python bindings
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Copy sources and build
COPY . .
RUN cargo build --release --locked || true

FROM debian:bookworm-slim
WORKDIR /app

# Minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Copy built binary and python helpers
COPY --from=builder /usr/src/quantum-coherence/target/release/coherence-test /usr/local/bin/coherence-test
COPY --from=builder /usr/src/quantum-coherence/python /app/python
COPY --from=builder /usr/src/quantum-coherence/python/requirements.txt /app/python/requirements.txt

RUN if [ -f /app/python/requirements.txt ]; then pip3 install --no-cache-dir -r /app/python/requirements.txt || true; fi

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/coherence-test"]
