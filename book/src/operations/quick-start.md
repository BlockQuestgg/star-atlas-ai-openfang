# Quick Start

## Prerequisites

- Docker and Docker Compose
- An LLM API key (Anthropic, OpenAI, or OpenRouter)
- [just](https://github.com/casey/just) command runner (optional but recommended)

## Setup

```bash
# Clone the repo
git clone https://github.com/blockquestgg/star-atlas-ai-openfang.git
cd star-atlas-ai-openfang

# Configure environment
cp .env.example .env
# Edit .env — set LLM_PROVIDER and the matching API key
```

## Build and Start

```bash
just build    # First build ~10-15 min (Rust compile), cached after
just up

# Check dashboard
open http://localhost:4200
```

## Spawn an Interactive Agent

Custom agents don't appear in the dashboard's built-in template grid. Spawn them via CLI:

```bash
just game      # sa-game — gameplay Q&A
just builder   # sa-builder — developer Q&A
just govern    # sa-govern — governance Q&A
just lore      # sa-lore-keeper — lore Q&A
```

Once spawned, the agent appears under **Chat > Your Agents** in the dashboard.

## Activate a Hand

Hands are installed but disabled by default to avoid burning LLM tokens:

```bash
just hand-activate-researcher
just hand-activate-brainstorm
just hand-activate-pip-advisor
just hand-activate-knowledge-keeper

# Check status
just hand-list
just logs
```

## Useful Commands

```bash
just rebuild   # Rebuild and restart after config changes
just down      # Stop
just shell     # Shell into the container
just doctor    # Health check
just nuke      # Reset all runtime state (fresh start)
```
