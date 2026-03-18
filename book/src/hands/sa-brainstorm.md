# sa-brainstorm — Creative Strategist

Autonomous hand that generates partnership concepts, event formats, ecosystem growth strategies, and community engagement ideas on an 8-hour schedule.

## Identity

| | |
|---|---|
| **Archetype** | Creative |
| **Vibe** | Energetic, provocative, imaginative |
| **Schedule** | Every 8 hours |
| **Activate** | `just hand-activate-brainstorm` |

## What It Does

```mermaid
flowchart TD
    A[Heartbeat fires] --> B[Read latest research briefs from inbox/]
    B --> C[Check curated knowledge for new facts]
    C --> D[Review community vault for context]
    D --> E[Generate ideas using creative lenses]
    E --> F[Check ideas/ to avoid repeats]
    F --> G[Write brainstorm doc to ideas/]

    style G fill:#F39C12,color:#fff
```

## Creative Lenses

- **Inversion** — what's the opposite of what every Web3 game does?
- **Adjacent Possible** — one step beyond current plans
- **Transplant** — what works in another industry?
- **Constraint as Feature** — what limitation becomes a selling point?
- **Who's Not in the Room** — who should be playing but isn't?

## Output

Writes brainstorm docs to `vaults/knowledge/ideas/` with:
- YAML frontmatter (title, date, tags, trigger, energy_level)
- The Spark — what triggered the idea
- The Idea — clear description
- Why It Could Work / Why It Might Not
- What It Would Take — concrete first steps
- Adjacent Ideas

Energy ratings: **high** (explore now), **medium** (explore when ready), **low** (file for later).

## Constraints

- Not a researcher — doesn't verify or cite, just imagines
- Grounded in Star Atlas reality — wild ideas welcome but connected to the ecosystem
- Checks existing ideas before writing to avoid repetition
- No financial advice
