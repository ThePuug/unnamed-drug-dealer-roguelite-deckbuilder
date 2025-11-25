# Heat System - Feature Matrix

Implementation tracking for Heat System specification.

**Spec:** [heat-system.md](heat-system.md)

**Last Updated:** 2025-11-25

---

## Summary

**Overall Completion:** 0/19 features (0%)

| Category | Complete | Partial | Not Started | Deferred |
|----------|----------|---------|-------------|----------|
| Heat Accumulation | 0 | 0 | 4 | 0 |
| Heat Decay | 0 | 0 | 5 | 0 |
| Heat Tiers | 0 | 0 | 6 | 0 |
| Narc Card Upgrades | 0 | 0 | 4 | 0 |
| **Total** | **0** | **0** | **19** | **0** |

---

## Heat Accumulation: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Heat delta calculation | ❌ | - | Sum all Heat modifiers on cards played |
| Heat application at hand end | ❌ | - | Add delta to character Heat (if not busted) |
| Heat on fold | ❌ | - | Keep Heat from rounds played before fold |
| Heat persistence | ❌ | - | Heat persists across decks on same character |

---

## Heat Decay: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Real-time decay (-1 Heat/hour) | ❌ | - | Time-based, not play-based |
| Decay calculation | ❌ | - | Calculate elapsed time since last deck |
| Decay display | ❌ | - | Show time until next -1 Heat |
| Decay projection | ❌ | - | "In 24 hours: Heat will be X" |
| Decay feedback | ❌ | - | Show decay rate (-24 Heat/day) |

---

## Heat Tiers: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Cold tier (0-25) | ❌ | - | Easy Narc, 0% Make It Stick |
| Warm tier (26-50) | ❌ | - | Moderate Narc, 0% Make It Stick |
| Hot tier (51-75) | ❌ | - | Hard Narc, ~13% Make It Stick |
| Scorching tier (76-100) | ❌ | - | Extreme Narc, ~33% Make It Stick |
| Inferno tier (101+) | ❌ | - | Nuclear Narc, ~60% Make It Stick |
| Tier transition feedback | ❌ | - | "Warning: Entering HOT tier - Narc much harder" |

---

## Narc Card Upgrades: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Heat-based Narc card upgrades | ❌ | - | Narc cards upgrade based on Heat tier (mirrors player upgrades) |
| Upgrade tier display | ❌ | - | Show Narc card upgrade level |
| Heat affects NEXT deck (not current) | ❌ | - | Current Heat determines next Narc difficulty |
| Upgrade preview | ❌ | - | Show expected Narc strength before deck starts |

---

## Implementation Deviations

_No implementations yet._

---

## Notes

- Heat decay is TIME-based (real-world hours), not play-based
- This creates anti-binge mechanic (rewards daily play)
- Heat persists on character until permadeath
- Heat affects NEXT deck difficulty (not current) for predictability
- MVP: Focus on 3 tiers (Cold/Warm/Hot)
- Phase 2: Implement all 5 Heat tiers
- **Trust system removed** - See progression-meta.md for per-run card upgrades as replacement progression mechanic
