# Data Flow

How information moves through the swarm — from external sources through research, ideation, and curation into structured knowledge that interactive agents can query.

## Hand Pipeline

```mermaid
flowchart LR
    subgraph Sources["External Sources"]
        WEB[Web Search]
        GOV[govern.staratlas.com]
    end

    subgraph Hands["Autonomous Hands"]
        RES[sa-researcher]
        BRN[sa-brainstorm]
        PIP[sa-pip-advisor]
        KK[sa-knowledge-keeper]
    end

    subgraph KnowledgeVault["vaults/knowledge/"]
        INBOX[inbox/]
        IDEAS[ideas/]
        PIPR[pip-reviews/]
        KNOW[knowledge/]
        SESS[sessions/]
        IDX[index.md]
    end

    CV[vaults/community/<br/>read-only]

    WEB --> RES
    GOV --> PIP
    CV --> RES
    CV --> BRN
    CV --> PIP

    RES -->|writes briefs| INBOX
    BRN -->|writes ideas| IDEAS
    PIP -->|writes reviews| PIPR

    INBOX --> KK
    IDEAS --> KK
    PIPR --> KK
    KK -->|curates| KNOW
    KK -->|digests| SESS
    KK -->|maintains| IDX

    style CV fill:#2ECC71,color:#fff
    style KNOW fill:#8E44AD,color:#fff
    style INBOX fill:#E67E22,color:#fff
    style IDEAS fill:#F39C12,color:#fff
    style PIPR fill:#3498DB,color:#fff
```

## Knowledge Vault Structure

| Directory | Written by | Read by | Purpose |
|---|---|---|---|
| `inbox/` | sa-researcher | sa-knowledge-keeper | Raw research briefs |
| `ideas/` | sa-brainstorm | sa-knowledge-keeper | Creative brainstorm output |
| `pip-reviews/` | sa-pip-advisor | sa-knowledge-keeper, sa-govern | Governance analysis |
| `knowledge/` | sa-knowledge-keeper | All agents | Curated atomic facts |
| `sessions/` | sa-knowledge-keeper | — | Session digests |
| `index.md` | sa-knowledge-keeper | All agents | Root index |

## Agent Query Flow

Interactive agents answer questions by searching both vaults:

```mermaid
sequenceDiagram
    participant U as User
    participant A as Agent (e.g. sa-game)
    participant KB as kb-mcp
    participant FS as filesystem MCP
    participant LLM as LLM Provider

    U->>A: Ask gameplay question
    A->>KB: mcp_sakb_search("fleet composition")
    KB-->>A: Ranked results with excerpts
    A->>KB: mcp_sakb_get_document("game-guides/fleet-and-ships.md")
    KB-->>A: Full document content
    A->>FS: mcp_filesystem_read_file("vaults/knowledge/knowledge/...")
    FS-->>A: Curated knowledge notes
    A->>LLM: Synthesize answer from vault content
    LLM-->>A: Response
    A-->>U: Cited, actionable answer
```
