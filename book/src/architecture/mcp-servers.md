# MCP Servers

The swarm uses two MCP (Model Context Protocol) servers to access vault content. Both are configured in `config.toml` and available to all agents and hands.

## kb-mcp (sakb)

An external MCP server ([kb-mcp](https://github.com/ttdonovan/kb-mcp)) configured via `ai/kb-mcp/collections.ron`. Indexes `vaults/community/` with Tantivy BM25 full-text search.

| Tool | Purpose |
|---|---|
| `mcp_sakb_list_sections` | Discover content areas (developer, economy, game-guides, governance, lore) |
| `mcp_sakb_search` | Full-text search with BM25 ranking, optional section scoping |
| `mcp_sakb_get_document` | Retrieve a document by path or title |
| `mcp_sakb_reindex` | Rebuild index from disk after vault changes |

```mermaid
flowchart LR
    A[Agent / Hand] -->|mcp_sakb_search| SAKB[kb-mcp]
    SAKB -->|Tantivy BM25| IDX[(In-RAM Index)]
    IDX -.->|built from| CV[vaults/community/]

    style SAKB fill:#3498DB,color:#fff
    style CV fill:#2ECC71,color:#fff
```

### Why not just use filesystem?

The `filesystem` MCP server gives raw file access — list directories, read files. That works but is slow for discovery. An agent looking for "POLIS voting mechanics" would need to list directories, guess filenames, and read files hoping to find the right one.

`kb-mcp` indexes all documents at startup and returns ranked results with excerpts. One search call replaces multiple list + read cycles.

## filesystem

The standard `@modelcontextprotocol/server-filesystem` server, giving hands direct read/write access to vault directories.

| Tool | Purpose |
|---|---|
| `mcp_filesystem_read_file` | Read file content |
| `mcp_filesystem_write_file` | Write file content |
| `mcp_filesystem_list_directory` | List directory contents |
| `mcp_filesystem_create_directory` | Create directories |

Mounted paths:
- `/home/openfang/project/vaults/community` (read-only by convention)
- `/home/openfang/project/vaults/knowledge` (read-write)
- `/home/openfang/project/vaults/kanban-ai` (read-write)

## Tool Access by Agent

| Agent/Hand | sakb tools | filesystem tools | web tools |
|---|---|---|---|
| sa-researcher | search, get, list, reindex | read, write, list, mkdir | web_search, web_fetch |
| sa-brainstorm | search, get, list, reindex | read, write, list, mkdir | web_search, web_fetch |
| sa-pip-advisor | search, get, list, reindex | read, write, list, mkdir | web_fetch |
| sa-knowledge-keeper | search, get, list, reindex | read, write, list, mkdir | — |
| sa-game | search, get, list | read, list | — |
| sa-builder | search, get, list | read, list | — |
| sa-govern | search, get, list | read, list | — |
| sa-lore-keeper | search, get, list | read, list | — |
