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
ARG OPENFANG_VERSION=v0.5.1
RUN git clone --depth 1 --branch ${OPENFANG_VERSION} \
    https://github.com/RightNow-AI/openfang.git .

# Build the binary
RUN cargo build --release --bin openfang

# --- Stage 1b: Install kb-mcp from GitHub (hybrid: BM25 + vector search) ---
FROM rust:1-slim-bookworm AS mcp-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    git \
    g++ \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install --git https://github.com/ttdonovan/kb-mcp --features hybrid

# Download ONNX embedding model for vector search
RUN mkdir -p /opt/memvid/text-models \
  && curl -L -o /opt/memvid/text-models/bge-small-en-v1.5.onnx \
     https://huggingface.co/BAAI/bge-small-en-v1.5/resolve/main/onnx/model.onnx \
  && curl -L -o /opt/memvid/text-models/bge-small-en-v1.5_tokenizer.json \
     https://huggingface.co/BAAI/bge-small-en-v1.5/resolve/main/tokenizer.json

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
COPY --from=mcp-builder /usr/local/cargo/bin/kb-mcp /usr/local/bin/
COPY --from=builder /build/agents /opt/openfang/agents

# Copy ONNX embedding model for hybrid search
COPY --from=mcp-builder /opt/memvid/text-models /opt/memvid/text-models

# Create non-root user and OpenFang home directory
RUN useradd --create-home --shell /bin/bash openfang \
 && mkdir -p /home/openfang/.openfang \
 && mkdir -p /home/openfang/.cache/memvid \
 && ln -s /opt/memvid/text-models /home/openfang/.cache/memvid/text-models \
 && chown -R openfang:openfang /home/openfang/.openfang /home/openfang/.cache
USER openfang
WORKDIR /home/openfang/project

# Copy project files
COPY --chown=openfang:openfang hands/ ./hands/
COPY --chown=openfang:openfang skills/ ./skills/
COPY --chown=openfang:openfang scripts/entrypoint.sh ./entrypoint.sh

# Dashboard port
EXPOSE 4200

ENTRYPOINT ["./entrypoint.sh"]
