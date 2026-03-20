---
name: vault-conventions
description: "Project-specific vault conventions for writing to vaults/knowledge/. Use when creating or editing knowledge vault files. Complements obsidian-markdown skill with project rules."
user-invocable: true
---

# Vault Conventions

Project-specific rules for writing to the Star Atlas knowledge vaults.
For Obsidian Markdown syntax (wikilinks, embeds, callouts, properties),
see the `obsidian-markdown` skill.

**Never write to `vaults/community/` — it is read-only.**

## Required Frontmatter

Every note in `vaults/knowledge/` starts with:

```yaml
---
title: Brief title
date: 2026-03-07
tags: [research, topic-tags]
source: sa-researcher | sa-brainstorm | sa-pip-advisor | sa-knowledge-keeper
status: draft | reviewed | archived
---
```

- `source` identifies which Hand or agent created the note
- `status` tracks the note's lifecycle
- `tags` use kebab-case for multi-word tags

## File Naming

- Lowercase kebab-case with date prefix: `2026-03-07-solana-ecosystem-update.md`
- Keep names descriptive but concise

## Folder Structure

Within `vaults/knowledge/`:

- `inbox/` — raw researcher handoffs awaiting synthesis
- `ideas/` — brainstorm output
- `pip-reviews/` — PIP advisor analysis
- `sessions/` — knowledge keeper session digests
- `index.md` — Map of Content

## Note Structure

Keep notes atomic — one topic per note. Link between notes rather than
writing mega-documents.

Research briefs should include:
- Summary (2-3 sentences)
- Key findings (bulleted, source-cited)
- Gaps / what couldn't be verified
- Suggested next steps

**Separate facts from analysis** — present sourced facts first, then
analysis clearly labeled.
