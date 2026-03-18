# sa-knowledge-keeper — Knowledge Curator

Autonomous hand that curates raw research, brainstorm output, and PIP reviews into structured, linked, atomic knowledge on a 2-hour schedule. The swarm's institutional memory.

## Identity

| | |
|---|---|
| **Archetype** | Librarian |
| **Vibe** | Meticulous, calm, structured |
| **Schedule** | Every 2 hours |
| **Activate** | `just hand-activate-knowledge-keeper` |

## What It Does

```mermaid
flowchart TD
    A[Heartbeat fires] --> B[Sweep inbox/ for research briefs]
    B --> C[Sweep ideas/ for brainstorm output]
    C --> D[Sweep pip-reviews/ for governance analyses]
    D --> E[Filter: fact or opinion? new or duplicate? contradictory?]
    E --> F[Curate: extract atomic facts, link notes, tag, cite]
    F --> G[Write curated notes to knowledge/]
    G --> H[Write session digest to sessions/]
    H --> I[Update index.md]

    style G fill:#8E44AD,color:#fff
    style H fill:#9B59B6,color:#fff
```

## Curation Workflow

1. **Sweep** — scan incoming directories for new material
2. **Filter** — evaluate each piece: is it fact or opinion, new or duplicate, contradictory?
3. **Curate** — extract atomic facts, link related notes with wikilinks, tag, cite sources
4. **Digest** — write session summary to `sessions/`
5. **Maintain** — periodically prune, consolidate, and flag stale knowledge

## Knowledge Vault Structure

```mermaid
graph TB
    subgraph "vaults/knowledge/"
        INBOX[inbox/<br/>raw briefs]
        IDEAS[ideas/<br/>brainstorm output]
        PIPR[pip-reviews/<br/>governance analysis]
        KNOW[knowledge/<br/>curated facts]
        SESS[sessions/<br/>session digests]
        IDX[index/<br/>topic maps]
        ROOT[index.md<br/>root index]
    end

    INBOX -->|curated by KK| KNOW
    IDEAS -->|curated by KK| KNOW
    PIPR -->|curated by KK| KNOW
    KNOW -.->|linked| KNOW
    KNOW -.->|indexed in| IDX
    IDX -.->|summarized in| ROOT

    style KNOW fill:#8E44AD,color:#fff
    style INBOX fill:#E67E22,color:#fff
    style IDEAS fill:#F39C12,color:#fff
    style PIPR fill:#3498DB,color:#fff
```

## Tag Taxonomy

| Category | Tags |
|---|---|
| Game | `sage`, `holosim`, `ue5-showroom`, `gameplay`, `fleet` |
| Economy | `atlas-token`, `polis-token`, `marketplace`, `defi` |
| Governance | `dao`, `pip`, `voting`, `treasury` |
| Ecosystem | `solana`, `web3-gaming`, `nft`, `developer` |
| Lore | `lore`, `faction`, `galia-expanse`, `narrative` |
| Meta | `research`, `brainstorm`, `contradiction`, `stale` |

## Constraints

- Never modifies `vaults/community/`
- Atomic notes — one topic per note, linked between notes
- Contradictions are valuable — flags them, doesn't hide them
- Every fact has a confidence level (high/medium/low) and source
- No web access — curates existing material, doesn't research
