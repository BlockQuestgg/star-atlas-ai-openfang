# Domain Expertise — Knowledge Curation

## Obsidian Vault Conventions

### Frontmatter

Every note starts with YAML frontmatter:

```yaml
---
title: Brief title
created: YYYY-MM-DD
updated: YYYY-MM-DD
tags: [knowledge, topic-tags]
source: sa-researcher | sa-pip-advisor | community
confidence: high | medium | low
---
```

### Internal Links

Use wikilinks for all internal references:

- `[[note-name]]` — link to a note
- `[[note-name|display text]]` — link with custom text
- `[[folder/note-name]]` — cross-folder link

### File Naming

Lowercase kebab-case: `solana-token-extensions.md`
Date-prefixed for time-sensitive content: `2026-03-12-market-update.md`

### Tags

Use kebab-case in frontmatter: `tags: [solana-ecosystem, defi-trends]`

## Star Atlas Tag Taxonomy

| Category | Tags |
|----------|------|
| Game | `sage`, `holosim`, `ue5-showroom`, `gameplay`, `fleet` |
| Economy | `atlas-token`, `polis-token`, `marketplace`, `defi` |
| Governance | `dao`, `pip`, `voting`, `treasury` |
| Ecosystem | `solana`, `web3-gaming`, `nft`, `developer` |
| Lore | `lore`, `faction`, `galia-expanse`, `narrative` |
| Meta | `research`, `brainstorm`, `contradiction`, `stale` |

## Knowledge Graph Principles

From the shared memory taxonomy:

| Memory Type | Your Role |
|---|---|
| Semantic memory | Curate stable facts into `knowledge/` |
| Episodic memory | Write session digests to `sessions/` |
| Retrieval layer | Maintain `index/` topic maps and wikilinks |

### Usefulness Scoring

Track which facts are actually referenced and useful:
- Facts retrieved often → increase confidence
- Facts never referenced → candidates for pruning
- Facts contradicted by newer evidence → flag for resolution

### Contradiction Handling

When new facts contradict stored knowledge:

1. Do NOT silently overwrite the old fact
2. Create a note documenting both positions
3. Tag with `contradiction`
4. Note which sources support each position
5. Flag for human review if both sources are credible

## CRAAP Source Evaluation

When assessing sources inherited from researcher briefs:

- **Currency** — when was it published?
- **Relevance** — does it address the topic?
- **Authority** — who published it?
- **Accuracy** — is it supported by evidence?
- **Purpose** — why does it exist? Any bias?
