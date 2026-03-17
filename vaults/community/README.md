# Community Vault

This vault contains community-contributed context for the Star Atlas AI swarm.
Agents read this vault for background knowledge but **never write to it**.

## How to Contribute

Add Markdown files with YAML frontmatter to the appropriate category:

### Suggested Categories

| Category | What to add |
|----------|------------|
| `game-guides/` | SAGE tutorials, fleet composition guides, economy strategies |
| `economy/` | Market analysis, token mechanics, resource pricing data |
| `governance/` | DAO proposals, voting guides, treasury reports |
| `lore/` | Faction histories, timeline events, character profiles |
| `developer/` | SDK guides, integration tutorials, API documentation |

### File Format

Use Obsidian-compatible Markdown with frontmatter:

```markdown
---
title: Your Document Title
date: YYYY-MM-DD
tags: [category, relevant-tags]
author: your-name-or-handle
status: draft | reviewed | published
---

# Your Document Title

Content here. Use standard Markdown formatting.
```

### Naming Convention

- Kebab-case filenames: `sage-fleet-composition-guide.md`
- Date-prefix for time-sensitive content: `2026-03-12-atlas-token-analysis.md`
- Lowercase only, no spaces

### Guidelines

1. **Accuracy matters** — agents use this as ground truth
2. **Cite sources** — link to official docs, on-chain data, etc.
3. **Date your content** — crypto/gaming info gets stale fast
4. **One topic per file** — keep it focused and atomic
5. **No financial advice** — analysis is fine, recommendations are not

## What Agents Do With This

- **sa-researcher** reads this before searching the web for updates
- **sa-brainstorm** uses this as context for generating ideas
- **sa-pip-advisor** references governance docs for PIP context
- **sa-knowledge-keeper** links curated knowledge back to community docs
- **sa-game, sa-builder, sa-govern, sa-lore-keeper** answer questions from this vault
