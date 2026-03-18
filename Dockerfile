# Dockerfile — star-atlas-ai-openfang
#
# Multi-stage build: compiles OpenFang from source, then packages with
# project config. Builds natively for the host architecture (ARM64/x86_64).
#
# First build is slow (~10-15 min for cargo). Subsequent builds use
# Docker layer cache unless OpenFang version changes.
#
# Config uses placeholder tokens injected by entrypoint.sh at startup.

# --- Stage 1: Build OpenFang from source ---
FROM rust:1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Clone OpenFang source — pin to a specific tag for reproducibility
ARG OPENFANG_VERSION=v0.3.45
RUN git clone --depth 1 --branch ${OPENFANG_VERSION} \
    https://github.com/RightNow-AI/openfang.git .

# Build the binary
RUN cargo build --release --bin openfang

# --- Stage 1b: Build sa-kb-mcp from project source ---
FROM rust:1-slim-bookworm AS mcp-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY ai/sa-kb-mcp/ ./ai/sa-kb-mcp/
RUN cargo build --release --manifest-path ai/sa-kb-mcp/Cargo.toml

# --- Stage 2: Runtime image ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    sqlite3 \
    nodejs \
    npm \
  && rm -rf /var/lib/apt/lists/*

# Copy compiled binaries and bundled agents
COPY --from=builder /build/target/release/openfang /usr/local/bin/
COPY --from=mcp-builder /build/ai/sa-kb-mcp/target/release/sa-kb-mcp /usr/local/bin/
COPY --from=builder /build/agents /opt/openfang/agents

# Create non-root user and OpenFang home directory
RUN useradd --create-home --shell /bin/bash openfang \
 && mkdir -p /home/openfang/.openfang \
 && chown openfang:openfang /home/openfang/.openfang
USER openfang
WORKDIR /home/openfang/project

# Copy project files
COPY --chown=openfang:openfang hands/ ./hands/
COPY --chown=openfang:openfang skills/ ./skills/
COPY --chown=openfang:openfang scripts/entrypoint.sh ./entrypoint.sh

# Dashboard port
EXPOSE 4200

ENTRYPOINT ["./entrypoint.sh"]
