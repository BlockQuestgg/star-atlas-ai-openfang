---
name: writing-plans
description: "Use when you have a spec or requirements for a multi-step task, before touching code"
user-invocable: true
---

# Writing Plans

## Overview

"Write comprehensive implementation plans assuming the engineer has zero context for our codebase and questionable taste. Document everything they need to know: which files to touch for each task, code, testing, docs they might need to check, how to test it. Give them the whole plan as bite-sized tasks. DRY. YAGNI. Frequent commits."

Assume they are a skilled developer, but know almost nothing about your toolset or problem domain.

**Announce at start:** "I'm using the writing-plans skill to create the implementation plan."

**Save plans to:** `docs/plans/YYYY-MM-DD-<feature-name>.md`

## Bite-Sized Task Granularity

**Each step is one action (2-5 minutes):**
- "Write the config file" - step
- "Run `just build` to verify it compiles" - step
- "Add the Hand definition" - step
- "Run `just doctor` to verify health" - step
- "Commit" - step

## Plan Document Header

**Every plan MUST start with this header:**

```markdown
# [Feature Name] Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** [One sentence describing what this builds]

**Architecture:** [2-3 sentences about approach]

**Tech Stack:** [Key technologies/libraries]

---
```

## Task Structure

```markdown
### Task N: [Component Name]

**Files:**
- Create: `exact/path/to/file`
- Modify: `exact/path/to/existing`

**Step 1: Create/modify file**

\`\`\`yaml
# complete content, not pseudocode
\`\`\`

**Step 2: Verify**

Run: `just build` or `just doctor`
Expected: clean build / healthy status

**Step 3: Commit**

\`\`\`bash
git add path/to/files
git commit -m "feat: add specific feature"
\`\`\`
```

## Remember

- Exact file paths always
- Complete content in plan (not "add config here")
- Exact commands with expected output
- DRY, YAGNI, frequent commits
- All config changes go through version-controlled files (IaC — Rule 1)

## Execution Handoff

After saving the plan, offer execution choice:

**"Plan complete and saved to `docs/plans/<filename>.md`. Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?"**
