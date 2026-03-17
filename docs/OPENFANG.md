# OpenFang Best Practices

Operational notes for running this OpenFang swarm safely.

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
