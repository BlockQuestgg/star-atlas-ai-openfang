# OpenFang Best Practices

Operational notes for running this OpenFang swarm safely.

## OpenFang Documentation

- [OpenFang Docs](https://www.openfang.sh/docs) — official documentation
- [OpenFang Source Docs](https://github.com/RightNow-AI/openfang/tree/main/docs) — docs in the GitHub repo

## Agents vs Hands

OpenFang has two types of runnable agents:

**Hands** (`hands/`) are autonomous agents. Once activated, they run in a
continuous background loop and tick every **60 seconds**. They appear under
**Hands** in the dashboard sidebar.

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

Activated hands appear under **Hands** in the dashboard.

## How Hand Ticking Works

**This is critical to understand for token cost management.**

When a hand is activated and has `max_iterations` set in its `[agent]`
config, OpenFang runs it in **Continuous mode** with a hardcoded
**60-second tick interval**. Every 60 seconds, the kernel sends:

```
[AUTONOMOUS TICK] You are running in continuous mode.
Check your goals, review shared memory for pending tasks,
and take any necessary actions.
```

The hand then gets up to `max_iterations` tool calls to respond. This
means a hand will attempt to do work every 60 seconds unless its system
prompt tells it to stop.

### The `[hand.schedule]` cron field is NOT used

The `[hand.schedule]` section in HAND.toml is metadata only. The kernel
does not read it when activating a hand. The 60-second interval is
hardcoded in `openfang-kernel/src/background.rs`. The `openfang cron`
CLI is a separate system not connected to hand activation.

Source: `crates/openfang-kernel/src/kernel.rs` lines 3303-3311.

### System prompts must control NO-OP behavior

Since every hand ticks every 60 seconds, **the system prompt is the only
mechanism to control how often real work happens.** Without a NO-OP check,
a hand will do a full work cycle every minute — burning tokens rapidly.

Every hand system prompt should start with a **Phase 0: Work Check**:

```
TICK BEHAVIOR: You tick every 60 seconds. MOST TICKS SHOULD BE NO-OPS.
1. Check memory for when you last did real work
2. If recent enough AND no pending tasks: respond "No work. Idle." and STOP.
3. Only proceed if enough time has passed or new work is available.
4. After doing real work, store the current timestamp in memory.
```

This pattern means:
- **Most ticks:** 1-2 tool calls (memory check → idle) — cheap
- **Work ticks:** Full cycle with research/writing/curation — expensive but infrequent

Our hands use these thresholds:
- **sa-researcher:** Real work every 8 hours (checks `last_research_timestamp`)
- **sa-brainstorm:** Real work every 12 hours (checks `last_brainstorm_timestamp`)
- **sa-pip-advisor:** Real work every 10 hours (checks `last_pip_review_timestamp`)
- **sa-knowledge-keeper:** Real work when new files appear (checks `last_curated_files`)

### Token cost estimation

With the NO-OP pattern:
- **Idle tick:** ~200-500 tokens (memory recall + short response)
- **Work tick:** ~5,000-20,000 tokens (tool calls, web search, file writes)
- **Per hand per day:** ~720 idle ticks + 1-3 work ticks
- **All 4 hands:** ~2,880 idle ticks/day (~700K-1.5M tokens) + work ticks

Without the NO-OP pattern, each hand would do full work every 60 seconds —
roughly 1,440 expensive ticks per day per hand.

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
