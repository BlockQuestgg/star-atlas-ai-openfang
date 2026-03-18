# System Overview

The swarm runs on OpenFang Agent OS inside Docker. All infrastructure is defined as code — agent definitions, schedules, vault structure, and MCP server config are version-controlled.

## Architecture Diagram

```mermaid
graph TB
    subgraph Docker["Docker Container"]
        OF[OpenFang Kernel]

        subgraph Hands["Autonomous Hands"]
            RES[sa-researcher<br/>every 4h]
            BRN[sa-brainstorm<br/>every 8h]
            PIP[sa-pip-advisor<br/>every 6h]
            KK[sa-knowledge-keeper<br/>every 2h]
        end

        subgraph Agents["Interactive Agents"]
            GAME[sa-game]
            BUILD[sa-builder]
            GOV[sa-govern]
            LORE[sa-lore-keeper]
        end

        subgraph MCP["MCP Servers"]
            FS[filesystem<br/>mcp_filesystem_*]
            KB[sa-kb-mcp<br/>mcp_sakb_*]
        end
    end

    subgraph Vaults["Vaults"]
        CV[community/<br/>read-only]
        KV[knowledge/<br/>read-write]
    end

    subgraph External["External"]
        LLM[LLM Provider<br/>Anthropic / OpenAI / OpenRouter]
        WEB[Web Search & Fetch]
        GOV_SITE[govern.staratlas.com]
    end

    OF --> Hands
    OF --> Agents
    OF --> MCP

    FS --> CV
    FS --> KV
    KB --> CV

    Hands --> MCP
    Agents --> MCP
    Hands --> LLM
    Agents --> LLM
    RES --> WEB
    BRN --> WEB
    PIP --> GOV_SITE

    style CV fill:#2ECC71,color:#fff
    style KV fill:#8E44AD,color:#fff
    style LLM fill:#3498DB,color:#fff
```

## Key Design Decisions

- **Infrastructure as Code** — all config is version-controlled. Dashboard is for monitoring, not configuration.
- **Read-only community vault** — agents never write to `vaults/community/`. Agent output goes to `vaults/knowledge/`.
- **Hands disabled by default** — each tick consumes LLM tokens. Activate only what you need.
- **Dual MCP servers** — `filesystem` for direct file access, `sakb` (sa-kb-mcp) for full-text search over the community vault.
