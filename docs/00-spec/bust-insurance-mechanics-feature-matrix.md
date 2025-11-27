# Bust & Insurance Mechanics - Feature Matrix

**Spec:** [bust-insurance-mechanics.md](bust-insurance-mechanics.md)
**Last Updated:** 2025-11-27
**Overall Status:** 23/23 features complete (100%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Core Bust Rule | 3 | 3 | 100% |
| Insurance Activation | 5 | 5 | 100% |
| Conviction System | 4 | 4 | 100% |
| Resolution Flow | 4 | 4 | 100% |
| Edge Cases | 4 | 4 | 100% |
| Player Feedback | 3 | 3 | 100% |
| **Total** | **23** | **23** | **100%** |

---

## Core Bust Rule - 3/3 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Evidence > Cover check | ✅ | `resolution.rs:42` |
| Bust triggers permadeath | ✅ | `save_integration.rs:167` - character = None |
| Tie goes to player | ✅ | `evidence <= cover` (not strict greater) |

---

## Insurance Activation - 5/5 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Insurance check on bust | ✅ | `try_insurance_activation()` |
| Requirements verification | ✅ | `cash >= cost` check |
| Cost payment | ✅ | `self.cash -= cost` |
| Heat penalty application | ✅ | `current_heat.saturating_add(heat_penalty)` |
| Insurance burn | ✅ | `deck.retain()` removes card |

---

## Conviction System - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Threshold check | ✅ | `current_heat >= heat_threshold` |
| Insurance override | ✅ | Conviction checked before insurance |
| Threshold-based activation | ✅ | CardType::Conviction { heat_threshold } |
| Below threshold behavior | ✅ | Falls through to insurance check |

---

## Resolution Flow - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Complete decision tree | ✅ | Validity → Bail → Evidence → Conviction → Insurance |
| Safe path | ✅ | Banks profit, applies heat |
| Bust path (no insurance) | ✅ | Returns HandOutcome::Busted |
| Bust path (insurance works) | ✅ | Pay cost, add heat, burn card, Safe |

---

## Edge Cases - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Multiple Insurance cards | ✅ | Override rule - last played active |
| Multiple Conviction cards | ✅ | Override rule - last played active |
| Insurance as Cover | ✅ | Insurance has cover stat, not consumed if safe |
| Can't afford insurance | ✅ | Returns HandOutcome::Busted |

---

## Player Feedback - 3/3 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| During-hand warnings | ✅ | Heat bar colors, danger indicator |
| Insurance status display | ✅ | Insurance slot in active cards UI |
| Bust result messaging | ✅ | "BUSTED!" in resolution overlay |

---

## Implementation Notes

- Resolution logic in `src/models/hand_state/resolution.rs`
- Permadeath in `src/systems/save_integration.rs`
- Active card queries in `src/models/hand_state/card_engine.rs`
- UI slots in `src/ui/systems.rs`
- Comprehensive test coverage in `resolution.rs` tests
