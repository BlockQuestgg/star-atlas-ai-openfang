# OpenFang Best Practices

## Agents vs Hands

**Hands** are autonomous, scheduled agents. They run on a cron timer, do their task, write output to the vault, and go idle. They appear under **Hands** in the dashboard.

**Agents** are interactive, on-demand chat agents. They exist as templates until spawned. They appear under **Chat > Your Agents** — but only after being spawned.

Custom agent templates do **not** appear in the dashboard's "Start a New Agent" grid — that grid shows OpenFang's built-in templates only.

## Skill Installation Security

**Never install skills directly from the dashboard's ClawHub tab.** Dashboard install is an OS-level action — skills go straight into the runtime with no security review.

Instead:

1. Ask any agent with `skill-vetter` loaded to review the skill first
2. The agent reads all skill files, checks for red flags (data exfiltration, credential access, obfuscated code)
3. Only install after the agent clears it with a SAFE verdict

### Why this matters

Prompt-only skills get injected into agent context and can influence behavior — what it writes, where it sends data, what it prioritizes. A malicious skill doesn't need code execution to cause harm.

### Reference

- [VirusTotal: How OpenClaw AI Agent Skills Are Being Weaponized](https://blog.virustotal.com/2026/02/from-automation-to-infection-how.html)
- [Bitdefender AI Skills Checker](https://www.bitdefender.com/en-us/consumer/ai-skills-checker)
- Skill vetting protocol: `skills/skill-vetter/SKILL.md`

## Cost Management

- Hands are disabled by default — each tick consumes LLM tokens
- Activate only the hands you need: `just hand-activate-researcher`
- The knowledge-keeper runs every 2h and is the most frequent; consider activating it only when other hands have produced output
- Interactive agents only consume tokens during active conversation

## IaC Workflow

1. **Configure** — make changes in the running OpenFang OS (dashboard or CLI)
2. **Snapshot** — run `just snapshot` to capture the current live configuration
3. **Reconcile** — review the snapshot diff against the repo
4. **Commit** — changes are committed to the repo

Live configuration is ephemeral. The repo is the source of truth.
