# Heat System - Feature Matrix

Implementation tracking for Heat System specification.

**Spec:** [heat-system.md](heat-system.md)

**Last Updated:** 2025-11-27

---

## Summary

**Overall Completion:** 15/19 features (79%)

| Category | Complete | Partial | Not Started | Deferred |
|----------|----------|---------|-------------|----------|
| Heat Accumulation | 4 | 0 | 0 | 0 |
| Heat Decay | 2 | 2 | 1 | 0 |
| Heat Tiers | 5 | 0 | 1 | 0 |
| Narc Card Upgrades | 4 | 0 | 0 | 0 |
| **Total** | **15** | **2** | **2** | **0** |

---

## Heat Accumulation: 4/4 complete (100%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Heat delta calculation | ✅ | RFC-015 | Sum all Heat modifiers on cards played |
| Heat application when cards played | ✅ | RFC-015/018 | Heat added immediately when cards played (not at resolution) |
| Heat on fold | ✅ | RFC-015 | Heat accumulated from rounds played |
| Heat persistence | ✅ | RFC-015 | Heat persists in SaveData across sessions |

---

## Heat Decay: 2/5 complete (40%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Real-time decay (-1 Heat/hour) | ✅ | RFC-015 | Calculated at deck start |
| Decay calculation | ✅ | RFC-015 | Capped at 168 hours (7 days) |
| Decay display | 🚧 | RFC-015 | Shows "Heat decayed by X while away" |
| Decay projection | ❌ | - | "In 24 hours: Heat will be X" |
| Decay feedback | 🚧 | RFC-015 | Shows decay amount, not rate |

---

## Heat Tiers: 5/6 complete (83%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Cold tier (0-25) | ✅ | RFC-015 | Implemented with green color |
| Warm tier (26-50) | ✅ | RFC-015 | Implemented with yellow color |
| Hot tier (51-75) | ✅ | RFC-015 | Implemented with orange color |
| Scorching tier (76-100) | ✅ | RFC-015 | Implemented with red color |
| Inferno tier (101+) | ✅ | RFC-015 | Implemented with purple color |
| Tier transition feedback | ❌ | - | No warning messages on tier change |

---

## Narc Card Upgrades: 4/4 complete (100%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Heat-based Narc card upgrades | ✅ | RFC-018 | Evidence cards scaled by heat tier |
| Upgrade tier display | ✅ | RFC-018 | Shows "Narc: Alert/Dangerous/etc" during play |
| Heat affects NEXT deck (not current) | ✅ | RFC-018 | Tier locked at deck start from character heat |
| Upgrade preview | ✅ | RFC-018 | Danger indicator visible in totals display |

---

## Implementation Deviations

**RFC-015 Implementation:**
- Save system uses HMAC-SHA256 anti-tampering (not documented in spec)
- Permadeath implemented: character deleted on bust
- Decay shown at deck builder entry, not as persistent countdown
- No tier transition warnings (UI polish deferred)

**RFC-018 Implementation:**
- Heat simplified to single cumulative model: accumulated when cards played, not at resolution
- Conviction check uses current deck heat directly (not projected hand-end heat)
- Evidence cards display ⚖ tier badges (scales of justice symbol for Narc cards)

---

## Notes

- Heat decay is TIME-based (real-world hours), not play-based
- This creates anti-binge mechanic (rewards daily play)
- Heat persists on character until permadeath
- Heat affects NEXT deck difficulty (not current) for predictability - implemented in RFC-018
- All 5 Heat tiers implemented
- **Trust system removed** - See progression-meta.md for per-run card upgrades as replacement progression mechanic
- **RFC-018 Complete** - Narc Evidence cards now scale with Heat tier (Cold→Base, Warm→+10%, Hot→+20%, Scorching→+30%, Inferno→+40%)
