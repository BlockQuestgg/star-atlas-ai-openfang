# OpenFang Best Practices

Operational notes for running this OpenFang swarm safely.

## Agents vs Hands

OpenFang has two types of runnable agents:

**Hands** (`hands/`) are autonomous, scheduled agents. They run on a cron timer,
do their task (research, brainstorm, curate), write output to the vault, and go
idle until the next tick. They appear under **Hands** in the dashboard sidebar.

**Agents** (`agents/custom/`) are interactive, on-demand chat agents. They exist
as templates until you spawn one. They appear under **Chat > Your Agents** in
the dashboard — but only after being spawned.

### Spawning custom agents

Custom agent templates (sa-game, sa-builder, sa-govern, sa-lore-keeper) do
**not** appear in the dashboard's "Start a New Agent" grid — that grid only
shows OpenFang's built-in templates. To use a custom agent:

```bash
# Spawn via just commands
just game      # sa-game — gameplay Q&A
just builder   # sa-builder — developer Q&A
just govern    # sa-govern — governance Q&A
just lore      # sa-lore-keeper — lore Q&A

# Or directly via openfang CLI
docker compose exec openfang openfang agent new sa-game
```

Once spawned, the agent appears under **Chat > Your Agents** in the dashboard
and you can chat with it there. It stays running until you kill it.

```bash
# List running agents
docker compose exec openfang openfang agent list

# Kill an agent
docker compose exec openfang openfang agent kill <agent-id>
```

### Activating hands

Hands are installed but **not auto-activated** to avoid burning LLM tokens on
autonomous ticks. Activate only what you need:

```bash
just hand-activate-researcher
just hand-activate-brainstorm
just hand-activate-pip-advisor
just hand-activate-knowledge-keeper

# Check status
just hand-list
just hand-status sa-researcher
```

Activated hands appear under **Hands** in the dashboard with their schedule
and last-run time.

## Skill Installation Security

**Never install skills directly from the dashboard's ClawHub tab.** The
dashboard install is an OS-level action — skills go straight into the runtime
with no security review. All agents with that skill loaded will have it
injected into their context immediately.

Instead:

1. Ask any agent with `skill-vetter` loaded to review the skill first
2. The agent reads all skill files, checks for red flags (data exfiltration,
   credential access, obfuscated code), and produces a vetting report
3. Only install after the agent clears it with a SAFE verdict

### Why this matters

Prompt-only skills get injected into agent context and can influence agent
behavior — what it writes, where it sends data, what it prioritizes. A
malicious skill doesn't need code execution to cause harm. The `skill-vetter`
skill can only protect you if an agent actually runs the vetting protocol
before installation.

### Reference

- [VirusTotal: How OpenClaw AI Agent Skills Are Being Weaponized](https://blog.virustotal.com/2026/02/from-automation-to-infection-how.html)
- [Bitdefender AI Skills Checker](https://www.bitdefender.com/en-us/consumer/ai-skills-checker)
- Skill vetting protocol: `skills/skill-vetter/SKILL.md`
