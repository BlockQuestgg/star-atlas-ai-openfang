---
name: obsidian-markdown
description: "Obsidian vault conventions for writing to vaults/knowledge/. Use when creating or editing knowledge vault files."
user-invocable: true
---

# Obsidian Markdown Conventions

Apply these conventions when creating or editing files in `vaults/knowledge/`.

**Never write to `vaults/community/` — it is read-only.**

## Properties (Frontmatter)

Every note starts with YAML frontmatter:

```
---
title: Brief title
date: 2026-03-07
tags: [research, topic-tags]
source: sa-researcher | sa-brainstorm | sa-pip-advisor | sa-knowledge-keeper
status: draft | reviewed | archived
---
```

- Properties appear at the very top, wrapped in `---`
- Use YAML syntax: `key: value`, lists with `[a, b]` or `- item`
- Common properties: `title`, `date`, `tags`, `source`, `status`

## Internal Links (Wikilinks)

Use wikilinks, not standard markdown links:

- `[[note-name]]` — link to a note
- `[[note-name|display text]]` — link with custom display text
- `[[folder/note-name]]` — link to a note in a subfolder
- Do NOT use `[text](path.md)` style links in vault files

## Embeds

- `![[note-name]]` — embed another note's content
- `![[note-name#heading]]` — embed a specific section

## Tags

- Inline tags: `#research`, `#governance`
- Tags in frontmatter: `tags: [research, solana, star-atlas]`
- Use kebab-case for multi-word tags

## Callouts

```markdown
> [!info] Title
> Callout content here

> [!warning] Title
> Warning content
```

Types: `note`, `abstract`, `info`, `tip`, `success`, `question`, `warning`, `failure`, `danger`, `bug`, `example`, `quote`

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

Keep notes atomic — one topic per note. Link between notes rather than writing mega-documents.

Research briefs should include:
- Summary (2-3 sentences)
- Key findings (bulleted, source-cited)
- Gaps / what couldn't be verified
- Suggested next steps

**Separate facts from analysis** — present sourced facts first, then analysis clearly labeled.

## Comments

`%% This is a comment that won't render %%`

## Highlighting

`==highlighted text==`
