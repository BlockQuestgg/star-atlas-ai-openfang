# sa-kb-mcp

MCP server and CLI for the Star Atlas community knowledge base. Indexes the
community vault with full-text search (Tantivy BM25) and exposes 4 tools for
document discovery, retrieval, and search.

## Tools

| Tool | Description |
|------|-------------|
| `list_sections` | List vault sections with doc counts and descriptions |
| `get_document` | Retrieve a document by path or title |
| `search` | Full-text BM25 search with optional section scoping |
| `reindex` | Rebuild index from disk mid-session |

## Build

```bash
cargo build -p sa-kb-mcp
```

## CLI Usage

```bash
sa-kb-mcp list-sections
sa-kb-mcp search --query "POLIS voting" --scope governance
sa-kb-mcp get-document --path "game-guides/sage-overview.md"
sa-kb-mcp reindex
```

## MCP Server

Run with no arguments to start the stdio MCP server:

```bash
sa-kb-mcp
```

### OpenFang Integration

Add to `config.toml`:

```toml
[[mcp_servers]]
name = "sakb"
timeout_secs = 10
env = ["SA_COMMUNITY_VAULT_PATH"]

[mcp_servers.transport]
type = "stdio"
command = "sa-kb-mcp"
args = []
```

### Claude Code Integration

Add to `.mcp.json`:

```json
{
  "mcpServers": {
    "sakb": {
      "command": "cargo",
      "args": ["run", "-p", "sa-kb-mcp"],
      "env": {
        "SA_COMMUNITY_VAULT_PATH": "/path/to/vaults/community"
      }
    }
  }
}
```

## Configuration

The vault path is resolved in priority order:

1. `SA_COMMUNITY_VAULT_PATH` environment variable
2. `../../vaults/community` relative to the binary (compile-time fallback)
