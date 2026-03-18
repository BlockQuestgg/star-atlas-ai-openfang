# sa-researcher — Market Intelligence

Autonomous hand that monitors the Web3 gaming landscape, Solana ecosystem, and Star Atlas news on a 4-hour schedule.

## Identity

| | |
|---|---|
| **Archetype** | Analyst |
| **Vibe** | Precise, thorough, skeptical |
| **Schedule** | Every 4 hours |
| **Activate** | `just hand-activate-researcher` |

## What It Does

```mermaid
flowchart TD
    A[Heartbeat fires] --> B[Check community vault for priorities]
    B --> C[Web search: Solana, Web3 gaming, Star Atlas]
    C --> D[Check ATLAS/POLIS token activity]
    D --> E{New findings?}
    E -->|Yes| F[Write research brief to inbox/]
    E -->|No| G[Review stale leads from prior briefs]
    G --> H[Update stale briefs with review-needed flag]
    F --> H

    style F fill:#E67E22,color:#fff
```

## Output

Writes research briefs to `vaults/knowledge/inbox/` with:
- YAML frontmatter (title, date, tags, source, status)
- Key findings with citations
- Gaps — what couldn't be verified
- Suggested next steps

Filename format: `2026-03-18-solana-ecosystem-update.md`

## Tools

| Tool | Usage |
|---|---|
| `mcp_sakb_search` | Find relevant community vault docs for context |
| `mcp_filesystem_write_file` | Write briefs to `inbox/` |
| `web_search`, `web_fetch` | External data retrieval |
| `memory_store`, `memory_recall` | Track research leads across sessions |

## Constraints

- Accuracy over speed — never presents unverified information as fact
- Cites sources — every claim traces to a source
- Separates facts from analysis
- Never writes to `vaults/community/`
- No financial advice
