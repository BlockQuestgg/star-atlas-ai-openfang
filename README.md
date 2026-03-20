# star-atlas-ai-openfang

An open-source AI agent swarm for the Star Atlas Community and DAO, running on [OpenFang](https://www.openfang.sh/) — autonomous Hands that research markets, brainstorm strategy, analyze governance proposals, and curate knowledge on schedules, plus interactive agents for on-demand Q&A.

**This is not financial advice.** All agent output is informational. Always do your own research (DYOR).

## Hands (Autonomous, Scheduled)

| Hand | Schedule | What it does |
|---|---|---|
| `sa-researcher` | Every 4h | Web3/crypto landscape, Solana ecosystem, Star Atlas news, market trends |
| `sa-brainstorm` | Every 8h | Creative ideation — partnerships, community events, ecosystem growth |
| `sa-pip-advisor` | Every 6h | Neutral PIP analysis — summarizes, identifies gaps/risks, never recommends votes |
| `sa-knowledge-keeper` | Every 2h | Aggregates output from other Hands into structured knowledge vault |

All schedules are **disabled by default**. Activate manually to control LLM costs.

## Agents (Interactive, On-Demand)

| Agent | What it does |
|---|---|
| `sa-game` | SAGE gameplay, Holosim, UE5 Showroom — mechanics Q&A, fleet advice |
| `sa-builder` | Developer tools, F-KIT, SDKs, building on Star Atlas |
| `sa-govern` | DAO governance, POLIS voting, treasury, PIP context |
| `sa-lore-keeper` | Star Atlas lore, narrative, faction stories, world-building Q&A |

Spawn via `just game`, `just builder`, `just govern`, or `just lore`.

## Quick Start

```bash
# Configure environment
cp .env.example .env
# Edit .env — set LLM_PROVIDER and the matching API key:
#   LLM_PROVIDER=anthropic   → ANTHROPIC_API_KEY
#   LLM_PROVIDER=openai      → OPENAI_API_KEY (via VibeProxy)
#   LLM_PROVIDER=openrouter  → OPENROUTER_API_KEY

# Build and start OpenFang
just build    # First build ~10-15 min (Rust compile), cached after
just up

# Check dashboard
open http://localhost:4200

# Activate a Hand (schedules disabled by default)
just hand-activate-researcher

# Check status
just hand-list
just logs

# Spawn an interactive agent
just game
```

## Safety Notes

1. **Schedules are disabled by default.** Each Hand tick consumes LLM tokens. Activate only what you need: `just hand-activate-researcher`
2. **No financial advice.** All agents include disclaimers. The PIP advisor is strictly neutral — it analyzes but never recommends votes.
3. **Community vault is read-only.** Agents never write to `vaults/community/`. Agent-generated content goes to `vaults/knowledge/`.
4. **Skill vetting.** Never install skills from ClawHub without running them through the `skill-vetter` first. See `docs/OPENFANG.md`.

## Project Structure

```
├── config.toml          # OpenFang configuration template (LLM, channels, memory)
├── config/providers/    # LLM provider presets (anthropic, openai, openrouter)
├── docker-compose.yml   # Container orchestration
├── agents/custom/       # Interactive agent templates (on-demand)
│   ├── sa-game.toml
│   ├── sa-builder.toml
│   ├── sa-govern.toml
│   └── sa-lore-keeper.toml
├── hands/               # Autonomous Hand definitions (scheduled)
│   ├── sa-researcher/
│   ├── sa-brainstorm/
│   ├── sa-pip-advisor/
│   └── sa-knowledge-keeper/
├── workspaces/          # Agent identity, personality, and heartbeat definitions
│   ├── sa-researcher/
│   ├── sa-brainstorm/
│   ├── sa-pip-advisor/
│   ├── sa-knowledge-keeper/
│   ├── sa-game/
│   ├── sa-builder/
│   ├── sa-govern/
│   └── sa-lore-keeper/
├── ai/kb-mcp/           # Community vault MCP config (collections.ron, cache)
├── book/                # mdBook documentation (architecture, agents, operations)
├── skills/              # Shared skills loaded by Hands
│   ├── obsidian/
│   ├── skill-vetter/
│   └── self-improving-agent/
├── kanban/               # Human-facing kanban board (git-tracked)
├── vaults/
│   ├── community/       # Community-contributed context (read-only)
│   ├── knowledge/       # Agent-curated knowledge (read-write)
│   └── kanban-ai/       # Agent workflow tracking (read-write)
├── scripts/
│   └── entrypoint.sh    # Container startup
└── justfile             # Commands
```

## Deployment Options

**Docker** (default) — built from source, pinned version, dashboard at `http://localhost:4200`:
```bash
just build     # Build image (first build ~10-15 min, cached after)
just up        # Start container
just rebuild   # Rebuild and restart
just down      # Stop
```

**Desktop app** — native Tauri 2.0 app with system tray and OS notifications.
Runs its own embedded kernel (does not connect to Docker).
See [OpenFang Desktop docs](https://www.openfang.sh/docs/desktop).

## MCP Server — kb-mcp

Uses [kb-mcp](https://github.com/ttdonovan/kb-mcp), an external Tantivy BM25 full-text search server. All agents and hands use it to search and retrieve community knowledge instead of manually browsing files.

**Tools:** `mcp_sakb_list_sections`, `mcp_sakb_search`, `mcp_sakb_get_document`, `mcp_sakb_reindex`

```bash
# Install
just mcp-install

# CLI usage
kb-mcp --config ai/kb-mcp/collections.ron list-sections
kb-mcp --config ai/kb-mcp/collections.ron search --query "POLIS voting"
kb-mcp --config ai/kb-mcp/collections.ron get-document --path "game-guides/sage-overview.md"
```

Configured in `config.toml` under `[[mcp_servers]]` with name `sakb`. Collection config lives in `ai/kb-mcp/collections.ron`.

## Documentation

Full documentation lives in `book/` as an [mdBook](https://rust-lang.github.io/mdBook/) with Mermaid diagrams covering architecture, agent/hand details, data flow, and operations.

```bash
just book-serve    # Dev server with hot reload (opens browser)
just book-build    # Build static HTML to book/book/
```

## Configuration

All config is version-controlled (IaC). The workflow:

1. Edit `config.toml`, `hands/*/HAND.toml`, or system prompts locally
2. `git commit`
3. `just rebuild` — changes take effect

Dashboard is for monitoring, not configuration. Dashboard edits are ephemeral and overwritten on restart.

### LLM Provider

Set `LLM_PROVIDER` in `.env` to select your provider. Provider presets live in `config/providers/`:

| `LLM_PROVIDER` | API Key | Notes |
|---|---|---|
| `anthropic` (default) | `ANTHROPIC_API_KEY` | Claude Sonnet 4.6 |
| `openai` | `OPENAI_API_KEY` | GPT-5.2 via [VibeProxy](https://github.com/automazeio/vibeproxy) |
| `openrouter` | `OPENROUTER_API_KEY` | 200+ models via OpenRouter |

`config.toml` is a template — `entrypoint.sh` injects the selected provider at startup. No manual editing needed to switch providers.

## Contributing

Contributions welcome! Here's how to help:

### Add Community Knowledge
Drop Markdown files into `vaults/community/` — see `vaults/community/README.md` for format guidelines.

### Improve Agents
Edit Hand system prompts or agent templates. Test locally with `just rebuild`.

### Report Issues
Open a GitHub issue with details about the problem or suggestion.

## Future Possibilities

Community-driven expansion opportunities:

- **`sa-lore-tracker` Hand** — scheduled monitoring of new lore drops, story updates, faction narrative changes
- **`sa-lore-archivist` Hand** — curate and index lore into a structured lore vault
- **Dedicated `vaults/lore/`** — separate knowledge base for narrative/world-building content
- **Knowledge vault MCP** — add `vaults/knowledge/` as a second collection in `ai/kb-mcp/collections.ron` for cross-vault search
- **Solana RPC integration** — custom MCP server for direct on-chain data queries
- **Star Atlas API integration** — if/when official APIs become available

## Star Atlas Resources

- [Star Atlas](https://staratlas.com) — official website
- [SAGE](https://play.staratlas.com) — browser game
- [Governance](https://govern.staratlas.com) — DAO voting
- [Build](https://build.staratlas.com) — developer resources
- [Discord](https://discord.gg/staratlas) — community

## Security

OpenFang provides built-in defense-in-depth:
- Capability-based RBAC per Hand
- SSRF protection (blocks private IPs, metadata endpoints)
- Path traversal prevention
- Taint tracking (prevents secret exfiltration)
- Subprocess isolation (env_clear + allowlist)

No external security wrappers needed — tools like `web_search` and `web_fetch` go through the kernel's security pipeline.

## License

MIT — see [LICENSE](LICENSE).
