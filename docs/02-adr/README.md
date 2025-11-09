# Architecture Decision Records (ADRs)

This directory contains technical architecture decisions for the unnamed drug dealer roguelite deckbuilder. ADRs document **how we build systems**, not what features players experience (see `docs/00-spec/` for that).

## When to Read ADRs

- **Before implementing features** - Check if related ADRs exist, understand established patterns
- **When systems interact** - Understand how components/systems are designed to work together
- **During debugging** - ADRs explain the "why" behind architectural choices
- **Before proposing changes** - Understand rationale for current approach

## ADR Quick Reference

**Legend:** ✅ Accepted | 📋 Proposed | ⏭️ Superseded • 🃏 Cards | 💰 Economy | 🎲 Roguelite | 🗺️ Map | 🧪 Substances | 🎨 UI/Tools

| ADR | Status | Title | Category | Date |
|-----|:------:|-------|:--------:|------|
| [001](001-card-type-system-and-interaction-rules.md) | 📋 | Card Type System and Interaction Rules | 🃏 Cards | 2025-11-09 |
| [002](002-betting-system-and-hand-structure.md) | 📋 | Betting System and Hand Structure (overview) | 🃏 Cards | 2025-11-09 |
| [003](003-insurance-and-conviction-system.md) | 📋 | Insurance and Conviction System | 🃏 Cards | 2025-11-09 |
| [004](004-hand-state-machine.md) | 📋 | Hand State Machine and Round Structure | 🃏 Cards | 2025-11-09 |
| [005](005-initiative-system.md) | 📋 | Initiative System and Raise Control | 🃏 Cards | 2025-11-09 |

---

## ADR Template

Use this as a starting point - tailor to fit your decision. Not all sections may apply.

```markdown
# ADR-XXX: [Decision Title]

## Status
[Proposed | Accepted | Deprecated | Superseded by ADR-XXX]

## Context
What problem are we solving? What are the constraints?
Reference related RFCs, specs, or existing ADRs.

## Decision
What specific approach are we taking? (1-2 paragraphs max)

### Core Mechanism
Code snippets, diagrams, or concrete examples showing how it works.

## Rationale
Why this approach over alternatives? (Key points only)

## Consequences
**Positive:** What this enables or improves
**Negative:** What becomes harder or costs more
**Mitigations:** How we address the negatives

## Implementation Notes
File locations, integration points, system ordering - practical details for implementers.

## References
- Related RFCs, specs, ADRs
- External resources if applicable

## Date
YYYY-MM-DD
```

## ADR Lifecycle

**Creation** → **Implementation** → **Acceptance** → **Active** → (optional) **Obsolescence**

- **Proposed**: Decision documented, seeking review/feedback
- **Accepted**: Approved and ready to implement (or already implemented)
- **Active**: Current architectural pattern in use
- **Deprecated**: No longer recommended, kept for historical context
- **Superseded**: Replaced by newer ADR (reference the replacement)

**Note:** Most ADRs stay "Accepted" and "Active" indefinitely. Only mark as deprecated/superseded when patterns fundamentally change.

## Guidelines

**Keep ADRs focused:**
- One decision per ADR
- Technical architecture only (game design goes in specs)
- Explain tradeoffs, not just final choice

**When to create an ADR:**
- Significant architectural pattern (affects multiple systems)
- Non-obvious tradeoffs (needs justification)
- Future reference value (others will ask "why?")

**When NOT to create an ADR:**
- Obvious/standard patterns (e.g., "use Bevy ECS for entities")
- Implementation details (e.g., specific variable names)
- Temporary decisions (e.g., MVP scope cuts)
