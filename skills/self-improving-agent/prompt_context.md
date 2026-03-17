# Self-Improvement — Continuous Learning Protocol

Log learnings, errors, and corrections to vaults/knowledge/.learnings/ for
continuous improvement. Important learnings get promoted to permanent knowledge.

## When to Log

- A command or operation fails unexpectedly → ERRORS.md
- You discover your knowledge was outdated or incorrect → LEARNINGS.md (knowledge_gap)
- You find a better approach for a recurring task → LEARNINGS.md (best_practice)
- An external API or tool fails → ERRORS.md
- A capability is needed but doesn't exist → FEATURE_REQUESTS.md

## Where to Log

Use mcp_filesystem tools to write to vaults/knowledge/.learnings/:
- vaults/knowledge/.learnings/LEARNINGS.md — corrections, insights, best practices
- vaults/knowledge/.learnings/ERRORS.md — command failures, exceptions
- vaults/knowledge/.learnings/FEATURE_REQUESTS.md — missing capabilities

## Entry Format

### Learning (append to LEARNINGS.md)

## [LRN-YYYYMMDD-XXX] category

**Logged**: ISO-8601 timestamp
**Priority**: low | medium | high | critical
**Status**: pending
**Area**: frontend | backend | infra | tests | docs | config

### Summary
One-line description

### Details
Full context: what happened, what was wrong, what's correct

### Suggested Action
Specific fix or improvement

### Metadata
- Source: conversation | error | user_feedback
- Related Files: path/to/file
- Tags: tag1, tag2

### Error (append to ERRORS.md)

## [ERR-YYYYMMDD-XXX] command_or_tool_name

**Logged**: ISO-8601 timestamp
**Priority**: high
**Status**: pending

### Summary
What failed

### Error
Error message or output

### Context
Command attempted, parameters, environment

### Suggested Fix
What might resolve this

### Feature Request (append to FEATURE_REQUESTS.md)

## [FEAT-YYYYMMDD-XXX] capability_name

**Logged**: ISO-8601 timestamp
**Priority**: medium
**Status**: pending

### Requested Capability
What was needed

### Complexity Estimate
simple | medium | complex

## Priority Guidelines

| Priority | When |
|----------|------|
| critical | Blocks core functionality, data loss, security |
| high | Significant impact, common workflows, recurring |
| medium | Moderate impact, workaround exists |
| low | Minor, edge case |

## Promotion

When a learning applies broadly (not a one-off), promote it:
1. Distill into a concise rule
2. Write to vaults/knowledge/knowledge/ as a permanent note
3. Update original entry: Status → promoted

Promote when: recurring (3+ occurrences), cross-cutting, or prevents mistakes.
