# Heat & Trust Systems - Feature Matrix

Implementation tracking for Heat & Trust Systems specification.

**Spec:** [heat-trust-systems.md](heat-trust-systems.md)

**Last Updated:** 2025-11-09

---

## Summary

**Overall Completion:** 0/32 features (0%)

| Category | Complete | Partial | Not Started | Deferred |
|----------|----------|---------|-------------|----------|
| Heat Accumulation | 0 | 0 | 4 | 0 |
| Heat Decay | 0 | 0 | 5 | 0 |
| Heat Tiers | 0 | 0 | 6 | 0 |
| Trust Gain/Loss | 0 | 0 | 4 | 0 |
| Trust Tiers | 0 | 0 | 4 | 0 |
| Narc Deck Scaling | 0 | 0 | 5 | 0 |
| Customer Deck Scaling | 0 | 0 | 4 | 0 |
| **Total** | **0** | **0** | **32** | **0** |

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
| Tier transition feedback | ❌ | - | "⚠️ Entering HOT tier - Narc much harder" |

---

## Trust Gain/Loss: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Trust +1 on successful hand | ❌ | - | Stayed in all 3 rounds, didn't get busted |
| Trust -1 on fold | ❌ | - | Player folds before Round 3 |
| Trust persistence | ❌ | - | Trust persists across decks on same character |
| Trust feedback | ❌ | - | "Trust: 5 → 6 (+1 for completing hand)" |

---

## Trust Tiers: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Low Trust (0-3) | ❌ | - | Bad Customer deals, ×0.4 profit multiplier avg |
| Medium Trust (4-10) | ❌ | - | Decent Customer deals, ×1.1 profit multiplier avg |
| High Trust (11+) | ❌ | - | Excellent Customer deals, ×1.6 profit multiplier avg |
| Tier transition feedback | ❌ | - | "Medium Trust achieved - Customer offers better deals" |

---

## Narc Deck Scaling: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Procedural Narc deck generation | ❌ | - | Generate 15-card deck based on current Heat |
| Heat-based card distribution | ❌ | - | Different card counts per tier |
| Make It Stick frequency scaling | ❌ | - | More conviction cards at high Heat |
| Narc deck preview | ❌ | - | Show expected Evidence range before deck starts |
| Heat affects NEXT deck (not current) | ❌ | - | Current Heat determines next Narc difficulty |

---

## Customer Deck Scaling: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Procedural Customer deck generation | ❌ | - | Generate 15-card deck based on current Trust |
| Trust-based card distribution | ❌ | - | Different card counts per tier |
| Profit multiplier scaling | ❌ | - | Better deals at higher Trust |
| Customer deck preview | ❌ | - | Show expected profit range before deck starts |

---

## Implementation Deviations

_No implementations yet._

---

## Notes

- Heat decay is TIME-based (real-world hours), not play-based
- This creates anti-binge mechanic (rewards daily play)
- Heat/Trust both persist on character until permadeath
- Heat affects NEXT deck difficulty (not current) for predictability
- Trust affects CURRENT deck immediately
- MVP: Focus on 3 tiers (Cold/Warm/Hot for Heat, Low/High for Trust)
- Phase 2: Implement all 5 Heat tiers and 3 Trust tiers
