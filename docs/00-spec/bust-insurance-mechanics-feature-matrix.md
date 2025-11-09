# Bust & Insurance Mechanics - Feature Matrix

Implementation tracking for Bust & Insurance Mechanics specification.

**Spec:** [bust-insurance-mechanics.md](bust-insurance-mechanics.md)

**Last Updated:** 2025-11-09

---

## Legend

- ✅ **Complete** - Fully implemented per spec
- 🔄 **In Progress** - Currently being developed (SOW active)
- 🎯 **Planned** - RFC approved, SOW created, ready for implementation
- ❌ **Not Started** - Planned but not implemented
- ⏸️ **Deferred** - Intentionally postponed to post-MVP

---

## Summary

**Overall Completion:** 1/26 features (4%)

| Category | Complete | Not Started | Deferred |
|----------|----------|-------------|----------|
| Core Bust Rule | 1 | 2 | 0 |
| Insurance Activation | 0 | 6 | 0 |
| Conviction System | 0 | 5 | 0 |
| Resolution Flow | 0 | 4 | 0 |
| Edge Cases | 0 | 5 | 0 |
| Player Feedback | 0 | 3 | 0 |
| **Total** | **1** | **25** | **0** |

---

## Core Bust Rule: 1/3 complete (33%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Evidence > Cover check | ✅ | SOW-001 | Simple bust condition → Game over |
| Bust triggers character death | ❌ | Phase 2 | Run ends, character arrested (permadeath) |
| Tie goes to player (Evidence = Cover) | ❌ | Phase 2 | Safe if equal |

---

## Insurance Activation: 0/6 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Insurance check on bust | ❌ | - | If Evidence > Cover, check for Get Out of Jail |
| Overage calculation | ❌ | - | overage = Evidence - Cover |
| Requirements verification | ❌ | - | Check if can afford cost |
| Cost payment | ❌ | - | Deduct from profit |
| Heat penalty application | ❌ | - | Gain overage + card_penalty Heat |
| Insurance burn (single use) | ❌ | - | Card removed from deck after use |

---

## Conviction System: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Make It Stick threshold check | ❌ | - | Check current_heat >= threshold |
| Insurance override | ❌ | - | Make It Stick overrides Get Out of Jail if threshold met |
| Threshold-based activation | ❌ | - | Warrant: 40, DA: 60, Federal: 80, Caught Red-Handed: 0 |
| Conviction below threshold | ❌ | - | Make It Stick inactive if Heat < threshold |
| Conviction feedback | ❌ | - | "⚠️ DA APPROVAL OVERRIDES INSURANCE" |

---

## Resolution Flow: 0/4 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Complete decision tree | ❌ | - | Evidence check → Conviction check → Insurance check |
| Safe path (Evidence <= Cover) | ❌ | - | Bank profit, apply Heat, continue |
| Bust path (no insurance) | ❌ | - | Run ends immediately |
| Bust path (insurance works) | ❌ | - | Pay cost, gain Heat, burn insurance, continue |

---

## Edge Cases: 0/5 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| Multiple Insurance cards | ❌ | - | Override rule: only last played active |
| Multiple Conviction cards | ❌ | - | Override rule: only last played active |
| Insurance played after fold | ❌ | - | No bust check, insurance unused |
| Insurance as Cover (not busted) | ❌ | - | Acts as Cover only, not consumed |
| Can't afford insurance cost | ❌ | - | Insurance fails, run ends |

---

## Player Feedback: 0/3 complete (0%)

| Feature | Status | RFC/ADR | Notes |
|---------|:------:|---------|-------|
| During-hand warnings | ❌ | - | "⚠️ EVIDENCE EXCEEDS COVER" |
| Insurance status display | ❌ | - | "Insurance active: Plea Bargain (cost $1k)" |
| Bust result messaging | ❌ | - | Clear success/failure messaging |

---

## Implementation Deviations

_No deviations yet - SOW-001 in progress._

---

## Implementation Status by RFC/SOW

### SOW-001: Minimal Playable Hand (~4h actual) - ✅ Complete

**Status:** Approved - Ready to Merge

**Features Delivered:**
- ✅ Evidence > Cover check (simple bust condition → Game over)

**Completion:** 1/26 features (4%)

**Note:** SOW-001 includes basic bust checking only. No insurance, no conviction, no character persistence (just "game over" on bust). All other features deferred to RFC-003 or Phase 2.

### RFC-003: Insurance and Complete Cards (14-18h) - Draft

**Planned Features:**
- Get Out of Jail cards (Insurance Activation: all 6 features)
- Make It Stick cards (Conviction System: all 5 features)
- Complete bust resolution flow
- Bust warnings and feedback

---

## Notes

- **SOW-001:** Simple bust check only (Evidence > Cover → Game over)
- **RFC-003:** Full insurance/conviction system
- **Phase 2:** Character permadeath integration
- Bust mechanics are the CORE failure condition (permadeath trigger)
- Insurance is expensive but necessary at high Heat
- Make It Stick escalates with Heat (conviction common at Inferno tier)
- Clear feedback critical (players need to understand why they got busted)
- MVP: Simple bust check + 2 Insurance + 2 Conviction cards
- Phase 2: Full insurance variety (8-10 cards) + conviction types (6-8 cards)
