# CLAUDE.md — star-atlas-ai-openfang

## What This Project Is

Open-source OpenFang-based AI swarm for the Star Atlas Community and DAO. Uses
OpenFang's autonomous Hands model for scheduled research, governance analysis,
and knowledge curation.

## Architecture

- **Approach:** Infrastructure as Code (IaC) — all infrastructure, agent configs, schedules, and environments are defined in version-controlled definition files, never configured manually or interactively.
- **Runtime:** OpenFang (Agent OS, Docker — see Deployment Options below)
- **Hands** (autonomous, scheduled):
  - `sa-researcher` — Web3/crypto landscape, Solana ecosystem, Star Atlas news
  - `sa-brainstorm` — creative ideation, partnerships, community events, ecosystem growth
  - `sa-pip-advisor` — neutral PIP analysis, governance context, risk assessment
  - `sa-knowledge-keeper` — knowledge curation, indexing, session digests
- **Agents** (interactive, on-demand):
  - `sa-game` — SAGE gameplay, Holosim, fleet strategy Q&A
  - `sa-builder` — F-KIT, SDKs, developer tools Q&A
  - `sa-govern` — DAO governance, POLIS voting, PIP context Q&A
  - `sa-lore-keeper` — Star Atlas lore, narrative, faction stories Q&A
- **Vaults:**
  - `vaults/community/` — community-contributed context (read-only)
  - `vaults/knowledge/` — agent-curated knowledge vault (read-write)
  - `vaults/kanban-ai/` — agent workflow tracking (read-write)
- **Kanban:**
  - `kanban/` — human-facing board (repo root, git-tracked)
  - `vaults/kanban-ai/` — agent workflow tracking (Docker-mounted, read-write)
- **Workspaces** (`workspaces/`): IDENTITY.md, SOUL.md, HEARTBEAT.md per agent
- **MCP Servers:**
  - `filesystem` — direct vault file access (read/write)
  - `sakb` (kb-mcp) — Tantivy BM25 full-text search over community vault
- **Security:** OpenFang built-in (capability RBAC, SSRF protection, taint tracking)
- **Channels:** CLI for local testing, Discord available but commented out
- **Dashboard:** `http://localhost:4200`

## Data Flow

```
Researcher Hand          Brainstorm Hand          PIP Advisor Hand
(every 4h)               (every 8h)               (every 6h)
  │                        │                        │
  ├─ reads vaults/community ├─ reads vaults/community ├─ reads vaults/community
  ├─ searches web          ├─ reads knowledge/      ├─ fetches govern.staratlas.com
  └─ writes inbox/         └─ writes ideas/         └─ writes pip-reviews/

                    Knowledge Keeper Hand
                    (every 2h)
                      │
                      ├─ reads inbox/ + ideas/ + pip-reviews/
                      ├─ curates → knowledge/
                      ├─ writes sessions/
                      └─ updates index.md

All paths under vaults/knowledge/. vaults/community/ is always read-only.
CLI/Dashboard (localhost:4200) → OpenFang OS AgentRouter → routes to Agents
```

## Documentation

Full project documentation lives in `book/` as an mdBook with Mermaid diagrams.
Read it first to understand the architecture, agent roles, data flow, and
operations. Build and serve with `just book-serve`.

## Commands

```bash
# Docker
just build                        # Build Docker image from source
just up                           # Start container
just down                         # Stop container
just rebuild                      # Rebuild and restart
just logs                         # Tail logs
just shell                        # Shell into container

# Hands (autonomous, scheduled)
just hand-activate-researcher     # Activate researcher Hand
just hand-activate-brainstorm     # Activate brainstorm Hand
just hand-activate-pip-advisor    # Activate PIP advisor Hand
just hand-activate-knowledge-keeper # Activate knowledge keeper Hand
just hand-list                    # List installed Hands
just hand-status                  # Check Hand status

# Agents (interactive, on-demand)
just game                         # Spawn sa-game agent for gameplay Q&A
just builder                      # Spawn sa-builder agent for developer Q&A
just govern                       # Spawn sa-govern agent for governance Q&A
just lore                         # Spawn sa-lore-keeper agent for lore Q&A
just spawn <name>                 # Spawn any agent template

# Book (mdBook documentation)
just book-build                   # Build the documentation book
just book-serve                   # Dev server with hot reload
just book-clean                   # Remove build output

# MCP: kb-mcp (community vault search)
just mcp-install                  # Install kb-mcp from GitHub
just mcp-sections                 # List vault sections
just mcp-search "query"           # Full-text search
just mcp-get "path"               # Get a document

# Diagnostics
just doctor                       # Version and health check
```

## Verification

After making changes, verify with:

- `just doctor` — OpenFang health check
- `docker compose build` — Confirm Docker image builds cleanly
- `docker compose up -d && docker compose logs openfang` — Container starts without errors

## Deployment Options

| Option | How | Dashboard | Best for |
|---|---|---|---|
| **Docker** (current) | `just build` — compiled from source, pinned version | `http://localhost:4200` | Version-controlled config, server deployment |
| **Desktop app** | Tauri 2.0 native app (macOS/Windows/Linux) | Built-in window | Local development, system tray + OS notifications |

The desktop app runs its own embedded kernel — it does not connect to Docker.
Both options use the same Hands, config, and vault formats.

## IaC Workflow

This project follows a snapshot-reconcile loop to keep all configuration in version control:

1. **Configure** — make changes in the running OpenFang OS (dashboard or CLI)
2. **Snapshot** — run `just snapshot` to capture the current live configuration
3. **Reconcile** — an AI agent reviews the snapshot diff against the repo and implements matching changes in the definition files
4. **Commit** — changes are committed to the repo, keeping IaC as the source of truth

Live configuration is ephemeral. The repo is the source of truth. Any config that only exists in a running instance and not in the repo is considered drift.

## Workflow

The project supports a full workflow pipeline:

1. **Brainstorm** — `/brainstorming` — explore ideas, ask questions, design before building
2. **Plan** — `/writing-plans` — create detailed implementation plans from designs
3. **Execute** — `/executing-plans` — implement plans in batches with verification
4. **Track** — `/kanban` — manage work items on a Markdown-based kanban board

## Code Quality Rules

### Think Before Coding

- State assumptions explicitly. If uncertain, ask.
- If multiple approaches exist, present them — don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.

### Simplicity First

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.

### Surgical Changes

- Touch only what you must.
- Don't "improve" adjacent code, comments, or formatting.
- Match existing style and conventions.

### Goal-Driven Execution

Transform tasks into verifiable goals:
1. [Step] → verify: [check]

## Rules

1. **Infrastructure as Code** — all infrastructure, agent definitions, schedules, and environment configs are managed through version-controlled definition files. No manual or interactive provisioning.
2. **Community vault is read-only** — agents never write to `vaults/community/`.
3. **Use OpenFang built-in tools** — `web_search`, `web_fetch` with SSRF protection.
4. **Research briefs follow obsidian-markdown conventions** — frontmatter, wikilinks, kebab-case filenames.
5. **Separate facts from analysis** in all research output.
6. **Cite sources** — every claim traces to a source.
7. **Never commit `.env`** — use `.env.example` as reference.
8. **No financial advice** — all output is informational only (DYOR).
9. **PIP advisor is neutral** — analyzes proposals, never recommends votes.
10. **UTC timezones** — all schedules use UTC for global community.
