# justfile — star-atlas-ai-openfang

# List available commands
default:
    @just --list

# --- Docker ---

# Build the Docker image
build:
    docker compose build

# Start OpenFang in Docker
up:
    docker compose up -d

# Stop OpenFang container
down:
    docker compose down

# Rebuild and restart
rebuild:
    docker compose up -d --build

# Nuke runtime state (stops container, removes volume, starts fresh)
nuke:
    docker compose down -v
    docker compose up -d

# Tail container logs
logs:
    docker compose logs -f openfang

# Open a shell inside the running container
shell:
    docker compose exec openfang bash

# Run an openfang command inside the container
run *args:
    docker compose exec openfang openfang {{ args }}

# --- Hand management ---

# Activate the sa-researcher Hand
hand-activate-researcher:
    docker compose exec openfang openfang hand activate sa-researcher

# Activate the sa-brainstorm Hand
hand-activate-brainstorm:
    docker compose exec openfang openfang hand activate sa-brainstorm

# Activate the sa-pip-advisor Hand
hand-activate-pip-advisor:
    docker compose exec openfang openfang hand activate sa-pip-advisor

# Activate the sa-knowledge-keeper Hand
hand-activate-knowledge-keeper:
    docker compose exec openfang openfang hand activate sa-knowledge-keeper

# Check Hand status
hand-status name="sa-researcher":
    docker compose exec openfang openfang hand status {{ name }}

# List all Hands
hand-list:
    docker compose exec openfang openfang hand list

# --- Agent templates ---

# Spawn the sa-game agent for gameplay Q&A
game:
    docker compose exec openfang openfang agent new sa-game

# Spawn the sa-builder agent for developer Q&A
builder:
    docker compose exec openfang openfang agent new sa-builder

# Spawn the sa-govern agent for governance Q&A
govern:
    docker compose exec openfang openfang agent new sa-govern

# Spawn the sa-lore-keeper agent for lore Q&A
lore:
    docker compose exec openfang openfang agent new sa-lore-keeper

# Spawn a built-in agent template by name
spawn name:
    docker compose exec openfang openfang spawn {{ name }}

# --- Book (mdBook) ---

# Build the documentation book
book-build:
    cd book && mdbook build

# Serve the book locally with hot reload
book-serve:
    cd book && mdbook serve --open

# Clean the book build output
book-clean:
    rm -rf book/book

# --- MCP: sa-kb-mcp ---

# Build the community vault MCP server
mcp-build:
    cargo build --manifest-path ai/sa-kb-mcp/Cargo.toml

# List community vault sections
mcp-sections:
    cargo run --manifest-path ai/sa-kb-mcp/Cargo.toml -- list-sections

# Search the community vault (usage: just mcp-search "POLIS voting")
mcp-search query:
    cargo run --manifest-path ai/sa-kb-mcp/Cargo.toml -- search --query "{{ query }}"

# Get a document from the community vault (usage: just mcp-get "game-guides/sage-overview.md")
mcp-get path:
    cargo run --manifest-path ai/sa-kb-mcp/Cargo.toml -- get-document --path "{{ path }}"

# --- Snapshots ---

# Snapshot OpenFang runtime state to tmp/snapshot-<timestamp>
snapshot:
    #!/usr/bin/env bash
    set -euo pipefail
    ts=$(date +%Y%m%d-%H%M%S)
    dest="tmp/snapshot-${ts}"
    mkdir -p "$dest"
    docker compose cp openfang:/home/openfang/.openfang/config.toml "$dest/"
    docker compose cp openfang:/home/openfang/.openfang/cron_jobs.json "$dest/"
    docker compose cp openfang:/home/openfang/.openfang/daemon.json "$dest/"
    docker compose cp openfang:/home/openfang/.openfang/data/ "$dest/data/"
    docker compose cp openfang:/home/openfang/.openfang/agents/ "$dest/agents/"
    docker compose cp openfang:/home/openfang/.openfang/skills/ "$dest/skills/"
    docker compose cp openfang:/home/openfang/.openfang/workspaces/ "$dest/workspaces/"
    echo "Snapshot saved to $dest"

# --- Diagnostics ---

# Run OpenFang diagnostics
doctor:
    docker compose exec openfang openfang doctor
