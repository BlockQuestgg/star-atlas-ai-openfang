---
name: obsidian
description: Obsidian vault conventions — frontmatter, filenames, wikilinks, and atomic note structure.
---

# Obsidian Vault Conventions

An Obsidian vault is a folder of plain Markdown files. Follow these conventions
when reading or writing notes.

## File Structure

- Notes: `*.md` (plain text Markdown)
- Attachments: images, PDFs in designated subfolder

## Frontmatter (required on every note)

```yaml
---
title: Descriptive Title
date: YYYY-MM-DD
tags: [tag1, tag2]
source: agent-name-or-url
status: draft | reviewed | published
---
```

## Filenames

- Kebab-case, date-prefixed: `2026-03-11-solana-ecosystem-update.md`
- No spaces, no special characters
- Lowercase only

## Internal Links

- Use `[[wikilinks]]` to connect notes: `[[2026-03-11-solana-ecosystem-update]]`
- For sections: `[[note-name#section-heading]]`
- Keep links relative (no absolute paths in link text)

## Writing Style

- One topic per note (atomic notes)
- Lead with the key insight
- Use headers (`##`) to structure content
- Separate facts from analysis
- Cite sources inline
