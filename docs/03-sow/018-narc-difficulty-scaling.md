# SOW-018: Narc Difficulty Scaling

## Status

**Merged** - 2025-11-27

## References

- **RFC-018:** [Narc Difficulty Scaling](../01-rfc/018-narc-difficulty-scaling.md)
- **Spec:** [Heat System](../00-spec/heat-system.md)
- **Branch:** (proposed)
- **Implementation Time:** 4-8 hours

---

## Implementation Plan

### Phase 1: Core Narc Tier System

**Goal:** Add Narc upgrade tier tracking based on character Heat at deck start

**Deliverables:**
- `src/models/hand_state/mod.rs` - Add `narc_upgrade_tier: UpgradeTier` field
- `src/models/hand_state/card_engine.rs` - Apply tier multiplier to Evidence cards

**Architectural Constraints:**
- Reuse existing `UpgradeTier` enum from `src/save/types.rs`
- Tier determined by character Heat at deck start (not play count)
- Tier locked for entire deck (Heat changes during deck don't affect it)
- Multiplier applies to Evidence card `evidence` and `heat` values

**Success Criteria:**
- At Heat 0, Evidence cards have base stats
- At Heat 30 (Warm), Evidence cards have +10% stats
- At Heat 60 (Hot), Evidence cards have +20% stats
- Tier is stable throughout deck (doesn't change mid-hand)
- Unit tests pass for all tier boundary conditions

**Duration:** 2-3 hours

---

### Phase 2: Deck Start Integration

**Goal:** Set Narc tier at deck creation from character state

**Deliverables:**
- `src/models/hand_state/mod.rs` - Update `from_assets` to accept Heat tier
- Integration with deck builder / game start flow

**Architectural Constraints:**
- Heat tier calculation uses `HeatTier::from_heat()`
- Mapping: Cold→Base, Warm→Tier1, Hot→Tier2, Scorching→Tier3, Inferno→Tier4
- Character's decayed Heat used (decay already applied before deck start)

**Success Criteria:**
- New deck starts with correct Narc tier based on character Heat
- Narc tier persists through all hands in the deck
- No character state mutation during tier lookup

**Duration:** 1-2 hours

---

### Phase 3: UI Difficulty Indicator

**Goal:** Display Narc difficulty to player before/during play

**Deliverables:**
- UI component showing Heat tier and Narc difficulty
- Indicator in game view during play

**Architectural Constraints:**
- Display tier name and/or danger indicator
- Color-coded to match Heat tier colors
- Non-intrusive (info available but not obstructive)

**Success Criteria:**
- Player sees current Heat tier before starting deck
- Narc difficulty level visible during play
- Colors match existing Heat tier palette

**Duration:** 1-2 hours

---

## Acceptance Criteria

**Functional:**
- Heat tier correctly maps to Narc upgrade tier
- Evidence card stats scale with tier multiplier (1.0/1.1/1.2/1.3/1.5)
- Tier locked at deck start, stable through all hands
- Works with new and existing characters

**UX:**
- Player understands Narc is harder at higher Heat
- Difficulty indicator visible and clear
- No confusion about when tier changes apply

**Performance:**
- O(1) tier lookup (simple match)
- No additional per-frame overhead

**Code Quality:**
- Unit tests for tier mapping and stat scaling
- Reuses existing UpgradeTier infrastructure
- Clean integration with HandState

---

## Discussion

### Implementation Note: HeatTier → UpgradeTier Mapping

Added `narc_upgrade_tier()` method to `HeatTier` enum for clean mapping:
- Cold → Base (no bonus)
- Warm → Tier1 (+10%)
- Hot → Tier2 (+20%)
- Scorching → Tier3 (+30%)
- Inferno → Tier4 (+40%, capped - not using Tier5's +50% for balance)

### Implementation Note: Tier Preservation

The `start_next_hand()` function now preserves `narc_upgrade_tier` across hands within a deck, ensuring the tier is locked at deck start and stable throughout all hands.

### Implementation Note: Danger Indicator

Added inline danger indicator to totals display: "| Narc: Alert/Dangerous/Intense/Deadly"
Only shown when Narc tier is above Base (no indicator for Cold/Relaxed).

### Implementation Note: Heat System Simplification (Bug Fix)

During testing, discovered heat display discrepancy: resolution overlay showed hand heat while heat bar showed cumulative deck heat. Root cause was dual tracking (totals.heat + current_heat).

**Fix:** Simplified to single cumulative heat model:
- Heat now accumulated immediately when cards are played (in `play_card()` and `buyer_plays_card()`)
- Removed heat accumulation from `resolve_hand()`
- Conviction check now uses `current_heat` directly (not projected heat)
- UI displays only `current_heat` (no double-counting)

This aligns with player mental model: "as I play cards, heat goes up" rather than "heat calculated at resolution".

### Implementation Note: Evidence Card Tier Display

Added visual tier indicator on Evidence cards (using ⚖ scales of justice symbol instead of ★ for player cards):
- Base: No indicator
- Tier 1+: Grey/Bronze/Silver/Gold/Foil colored ⚖ badges
- Stats displayed are already upgraded values

---

## Acceptance Review

### Scope Completion: 100%

**Phases Complete:**
- ✅ Phase 1: Core Narc Tier System
- ✅ Phase 2: Deck Start Integration
- ✅ Phase 3: UI Difficulty Indicator

### Test Coverage

- 3 new unit tests for HeatTier → UpgradeTier mapping
- 5 new unit tests for Narc tier stat scaling
- Updated 3 heat-related tests for new immediate-accumulation model
- All 95 tests passing

### Files Modified

- `src/save/types.rs` - Added `narc_upgrade_tier()` and `danger_name()` to HeatTier
- `src/models/hand_state/mod.rs` - Added `narc_upgrade_tier` field, updated `from_assets`
- `src/models/hand_state/state_machine.rs` - Updated `with_custom_deck`, preserved tier in `start_next_hand`, added `get_card_heat()` helper, immediate heat accumulation in `play_card()` and `buyer_plays_card()`
- `src/models/hand_state/card_engine.rs` - Apply narc tier multiplier to Evidence cards
- `src/models/hand_state/resolution.rs` - Removed heat accumulation (now happens at play time), conviction uses `current_heat` directly
- `src/systems/input.rs` - Pass heat tier to `with_custom_deck`, draw cards after start_next_hand
- `src/systems/ui_update.rs` - Added Narc difficulty indicator to totals display, Evidence card tier badges
- `src/ui/systems.rs` - Simplified heat bar and resolution display to use `current_heat` directly
- `src/ui/helpers.rs` - Evidence/Conviction/DealModifier stat scaling with upgrade multipliers
- `docs/00-spec/heat-system-feature-matrix.md` - Updated completion status

---

## Conclusion

RFC-018 Narc Difficulty Scaling is complete with all phases implemented. Additionally, a heat system bug discovered during testing was fixed (heat display discrepancy). The implementation:
- Scales Narc Evidence cards based on character Heat tier
- Locks tier at deck start for predictable difficulty
- Displays tier indicators on Evidence cards during play
- Simplifies heat tracking to a single cumulative model

---

## Sign-Off

**Reviewed By:** ARCHITECT Role
**Date:** 2025-11-27
**Decision:** ✅ **ACCEPTED**
**Status:** Ready to merge to main
