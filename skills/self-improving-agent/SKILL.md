---
name: self-improving-agent
description: Captures learnings, errors, and corrections to enable continuous improvement. Log when operations fail, knowledge is outdated, or better approaches are discovered.
---

# Self-Improvement Skill

Log learnings and errors to markdown files for continuous improvement. The
knowledge-keeper hand curates these into permanent knowledge, and important
learnings get promoted to the knowledge vault.

Adapted from [ClawHub](https://clawhub.ai/spclaudehome/self-improving-agent)
(spclaudehome, v3.0.1) for OpenFang.

## Quick Reference

| Situation | Action |
|-----------|--------|
| Command/operation fails | Log to `.learnings/ERRORS.md` |
| Knowledge was outdated | Log to `.learnings/LEARNINGS.md` (knowledge_gap) |
| Found better approach | Log to `.learnings/LEARNINGS.md` (best_practice) |
| API/external tool fails | Log to `.learnings/ERRORS.md` with details |
| Missing capability needed | Log to `.learnings/FEATURE_REQUESTS.md` |
| Similar to existing entry | Link with `**See Also**`, consider priority bump |
| Broadly applicable learning | Promote to `knowledge/` as permanent note |

## Storage

All learning files live in `vaults/knowledge/.learnings/`. Use `mcp_filesystem_*`
tools to read and write:

```
vaults/knowledge/.learnings/
├── LEARNINGS.md          # Corrections, insights, best practices
├── ERRORS.md             # Command failures, exceptions
└── FEATURE_REQUESTS.md   # Missing capabilities
```

## Logging Format

### Learning Entry

Append to `.learnings/LEARNINGS.md`:

```markdown
## [LRN-YYYYMMDD-XXX] category

**Logged**: ISO-8601 timestamp
**Priority**: low | medium | high | critical
**Status**: pending
**Area**: research | governance | lore | gameplay | infra | config

### Summary
One-line description of what was learned

### Details
Full context: what happened, what was wrong, what's correct

### Suggested Action
Specific fix or improvement to make

### Metadata
- Source: conversation | error | autonomous_tick
- Related Files: path/to/file.ext
- Tags: tag1, tag2
- See Also: LRN-20260311-001 (if related to existing entry)

---
```

### Error Entry

Append to `.learnings/ERRORS.md`:

```markdown
## [ERR-YYYYMMDD-XXX] tool_or_command_name

**Logged**: ISO-8601 timestamp
**Priority**: high
**Status**: pending

### Summary
Brief description of what failed

### Error
Actual error message or output

### Context
- Tool/operation attempted
- Input or parameters used
- Environment details if relevant

### Suggested Fix
If identifiable, what might resolve this

### Metadata
- Reproducible: yes | no | unknown
- Related Files: path/to/file.ext
- See Also: ERR-20260311-001 (if recurring)

---
```

### Feature Request Entry

Append to `.learnings/FEATURE_REQUESTS.md`:

```markdown
## [FEAT-YYYYMMDD-XXX] capability_name

**Logged**: ISO-8601 timestamp
**Priority**: medium
**Status**: pending

### Requested Capability
What was needed

### Context
Why it's needed, what problem it solves

### Complexity Estimate
simple | medium | complex

### Suggested Implementation
How this could be built

---
```

## ID Generation

Format: `TYPE-YYYYMMDD-XXX`
- TYPE: `LRN` (learning), `ERR` (error), `FEAT` (feature)
- YYYYMMDD: Current date
- XXX: Sequential number (e.g., `001`, `002`)

## Priority Guidelines

| Priority | When to Use |
|----------|-------------|
| `critical` | Blocks core functionality, data loss risk, security issue |
| `high` | Significant impact, affects common workflows, recurring |
| `medium` | Moderate impact, workaround exists |
| `low` | Minor inconvenience, edge case |

## Detection Triggers

Automatically log when you notice:

**Knowledge Gaps** (→ LEARNINGS.md, `knowledge_gap`):
- Information you relied on was outdated
- API behavior differs from your understanding
- Tool capabilities were different than expected

**Best Practices** (→ LEARNINGS.md, `best_practice`):
- Discovered a better approach after initial attempt
- Found a pattern that should be reused

**Errors** (→ ERRORS.md):
- Tool returns an error
- Unexpected output or behavior
- Timeout or connection failure
- MCP filesystem access denied

**Feature Gaps** (→ FEATURE_REQUESTS.md):
- Capability needed but doesn't exist
- Workaround required for missing functionality

## Resolving Entries

When an issue is fixed:

1. Change `**Status**: pending` → `**Status**: resolved`
2. Add resolution block:

```markdown
### Resolution
- **Resolved**: ISO-8601 timestamp
- **Notes**: Brief description of what was done
```

## Promoting to Permanent Knowledge

When a learning is broadly applicable (not a one-off), promote it to the
knowledge vault.

### When to Promote

- Learning applies across multiple hands or tasks
- Knowledge any hand in the swarm should know
- Prevents recurring mistakes
- Documents conventions discovered through experience
- Recurring pattern (3+ occurrences)

### How to Promote

1. Distill the learning into a concise Obsidian note
2. Write to `vaults/knowledge/knowledge/` following vault conventions
3. Update original entry: `**Status**: promoted`

## Recurring Pattern Detection

If logging something similar to an existing entry:

1. Search existing `.learnings/` files first
2. Link entries with `**See Also**: ERR-20260311-001`
3. Bump priority if issue keeps recurring
4. Consider promotion: recurring issues indicate missing knowledge

## Best Practices

1. **Log immediately** — context is freshest right after the issue
2. **Be specific** — other hands need to understand quickly
3. **Include reproduction steps** — especially for errors
4. **Link related files** — makes fixes easier
5. **Suggest concrete fixes** — not just "investigate"
6. **Promote aggressively** — if broadly applicable, write to knowledge/
7. **Review regularly** — stale learnings lose value
