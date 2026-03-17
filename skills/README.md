# Skills

Custom skills installed automatically on container boot via `entrypoint.sh`.

## Adding a New Skill

1. Create a directory under `skills/` matching the skill name
2. Add three files (see Format below)
3. Add the skill name to `skills = [...]` in each HAND.toml that should use it
4. Rebuild: `just rebuild`

## OpenFang Skill Format

```
skills/<name>/
├── skill.toml          # Metadata + prompt_context (inline)
├── SKILL.md            # Human-readable docs (also used by OpenFang)
└── prompt_context.md   # Prompt content (mirrors prompt_context in skill.toml)
```

### skill.toml

```toml
prompt_context = """
# Skill Name — Short Description

Core instructions that get injected into the agent's context.
"""

[skill]
name = "skill-name"
version = "1.0.0"
description = "One-line description."
author = "author-name"
license = ""
tags = ["prompt-only"]

[runtime]
type = "promptonly"
entry = ""

[tools]
provided = []

[requirements]
tools = []
capabilities = []
```

## Converting ClawHub Skills

ClawHub (OpenClaw's registry) uses a different format. To convert:

| ClawHub | OpenFang | Notes |
|---|---|---|
| `_meta.json` → `ownerId`, `slug`, `version` | `[skill]` table in `skill.toml` | Map `slug` → `name`, drop `ownerId` |
| YAML frontmatter in `SKILL.md` → `name`, `description` | `[skill]` table fields | Move out of frontmatter |
| Body of `SKILL.md` | `prompt_context` in `skill.toml` + `prompt_context.md` | Duplicate into both |
| (implicit) | `[runtime] type = "promptonly"` | Add explicitly |

Steps:

1. Download the skill from ClawHub (gives you `_meta.json` + `SKILL.md`)
2. Create `skill.toml` with `[skill]` metadata from `_meta.json` frontmatter
3. Copy the SKILL.md body into `prompt_context` (inline in skill.toml) and `prompt_context.md`
4. Adapt content if needed — replace `curl` commands with `web_fetch`, remove emoji if noisy
5. Keep `SKILL.md` as human-readable docs (OpenFang also reads it)

## Installed Skills

| Skill | Version | Source | Purpose |
|---|---|---|---|
| obsidian | 0.2.0 | Local | Vault conventions — frontmatter, filenames, wikilinks |
| skill-vetter | 1.0.0 | [ClawHub](https://clawhub.ai/spclaudehome/skill-vetter) | Security-first vetting before installing unknown skills |
| self-improving-agent | 3.0.1 | [ClawHub](https://clawhub.ai/spclaudehome/self-improving-agent) | Continuous improvement — logs learnings, errors, corrections |
