---
name: self-improving-agent
description: "Reference for the OpenFang self-improving-agent skill. Captures learnings, errors, and corrections to enable continuous agent improvement. Use when reviewing agent learning patterns or updating the learning system."
user-invocable: true
---

# Self-Improving Agent — Claude Code Reference

This skill documents the OpenFang self-improving-agent pattern used by all Hands in this swarm. The actual OpenFang skill lives at `skills/self-improving-agent/` and is installed into the container at boot.

## How It Works

All Hands (sa-researcher, sa-brainstorm, sa-pip-advisor, sa-knowledge-keeper) have the `self-improving-agent` skill enabled. When they encounter failures, outdated knowledge, or better approaches, they log entries to `.learnings/` files in the knowledge vault.

## Storage

All learning files live in `vaults/knowledge/.learnings/`:

```
vaults/knowledge/.learnings/
├── LEARNINGS.md          # Corrections, insights, best practices
├── ERRORS.md             # Command failures, exceptions
└── FEATURE_REQUESTS.md   # Missing capabilities
```

## Entry Types

| Situation | File | ID Prefix |
|---|---|---|
| Command/operation fails | ERRORS.md | ERR |
| Knowledge was outdated | LEARNINGS.md | LRN |
| Found better approach | LEARNINGS.md | LRN |
| Missing capability needed | FEATURE_REQUESTS.md | FEAT |

## ID Format

`TYPE-YYYYMMDD-XXX` (e.g., `LRN-20260313-001`)

## Priority Levels

- `critical` — Blocks core functionality, data loss risk, security issue
- `high` — Significant impact, affects common workflows, recurring
- `medium` — Moderate impact, workaround exists
- `low` — Minor inconvenience, edge case

## Lifecycle

1. **Log** — Hand encounters issue, appends entry with `Status: pending`
2. **Resolve** — Issue is fixed, status updated to `resolved` with resolution notes
3. **Promote** — Broadly applicable learnings get distilled into `vaults/knowledge/knowledge/` as permanent notes

## When to Review

- Before modifying Hand system prompts — check if learnings suggest prompt improvements
- Before adding new tools — check FEATURE_REQUESTS.md for capability gaps
- After deploying changes — check ERRORS.md for new failures
- During knowledge-keeper sessions — the knowledge-keeper Hand curates learnings into permanent knowledge

## Full Skill Reference

See `skills/self-improving-agent/SKILL.md` for the complete logging format, detection triggers, and promotion criteria used by OpenFang Hands.
