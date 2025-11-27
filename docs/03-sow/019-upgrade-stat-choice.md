# SOW-019: Upgrade Stat Choice

## Status

**Merged** - 2025-11-27

## References

- **RFC-019:** [Upgrade Stat Choice](../01-rfc/019-upgrade-stat-choice.md)
- **RFC-017:** [Player Card Upgrades](../01-rfc/017-player-card-upgrades.md) (dependency)
- **Branch:** merged to main
- **Implementation Time:** ~4 hours

---

## Implementation Plan

### Phase 1: Data Model

**Goal:** Extend save system to track per-stat upgrade choices

**Deliverables:**
- `UpgradeableStat` enum in `src/save/types.rs`
- `CardUpgrades` struct for tracking upgrade history
- Extended `CharacterState` with `card_upgrades` field
- Extended `HandState` with `card_upgrades` field

**Architectural Constraints:**
- `UpgradeableStat`: Price, Cover, Evidence, Heat, HeatPenalty, PriceMultiplier
- `CardUpgrades.upgrades`: Vec storing stat chosen at each tier (index 0 = Tier1)
- Keep existing `card_play_counts` for tier calculation (don't remove)
- Use `#[serde(default)]` for backward compatibility with existing saves
- Both enums and structs must be Serialize/Deserialize

**Success Criteria:**
- Existing saves load without error (empty `card_upgrades` HashMap)
- New saves persist upgrade choices correctly
- `cargo test` passes for save/load roundtrip

**Duration:** 1-2 hours

---

### Phase 2: Stat Calculation Refactor

**Goal:** Replace flat tier bonus with per-stat multipliers based on upgrade choices

**Deliverables:**
- `UpgradeableStat::available_for(card_type)` method
- `UpgradeableStat::random_pair(card_type, rng)` method
- `HandState::get_stat_multiplier(card_name, stat)` method
- Refactored `calculate_totals()` in `card_engine.rs`

**Architectural Constraints:**
- Stat availability by card type:
  - Product: Price, Heat
  - Location: Evidence, Cover, Heat
  - Cover: Cover, Heat
  - Insurance: Cover, HeatPenalty
  - DealModifier: PriceMultiplier, Evidence, Cover, Heat
- Random selection: Pick 2 from available (shuffle if 3+, fixed if exactly 2)
- Stat stacking: Additive (+10% per upgrade to same stat), not multiplicative
- Direction: Price/Cover/PriceMultiplier increase; Evidence/Heat/HeatPenalty decrease
- Evidence cards (Narc) continue using `narc_upgrade_tier`, not player upgrades

**Success Criteria:**
- Card with 1 Cover upgrade shows +10% cover in totals
- Card with 3 Cover upgrades shows +30% cover in totals
- Location with Evidence upgrade shows -10% evidence in totals
- Product with no upgrades shows base stats (no regression)
- `cargo test` passes for all card type calculations

**Duration:** 2-3 hours

---

### Phase 3: Upgrade Choice UI

**Goal:** Modal for player to choose stat when card tiers up

**Deliverables:**
- `PendingUpgrade` struct (card name, tier, two stat options)
- `pending_upgrades: Vec<PendingUpgrade>` queue in CharacterState
- Upgrade detection in hand resolution (compare tier before/after play count increment)
- Modal UI showing card name, tier, two stat options with before/after values
- Input handling for stat selection (keyboard/mouse)
- Save integration: sync pending_upgrades and card_upgrades

**Architectural Constraints:**
- Trigger: After hand resolution completes (not during play)
- Queue: Multiple cards can tier up in same hand; process one at a time
- Persistence: pending_upgrades survives quit (show modal on next session)
- UI layout: Card name, current tier, two buttons with stat preview
- Preview format: "Cover: 30 → 33 (+10%)" or "Heat: 10 → 9 (-10%)"
- No skip option: Player must choose one of the two stats
- After choice: Remove from pending queue, add to card_upgrades, save

**Success Criteria:**
- Playing card 5th time (Tier1 threshold) triggers upgrade modal after hand
- Modal shows exactly 2 options with correct before/after values
- Selecting option applies upgrade and dismisses modal
- Multiple pending upgrades process sequentially
- Quitting mid-modal preserves pending upgrade for next session
- `cargo test` passes for upgrade detection and queue management

**Duration:** 2-3 hours

---

## Acceptance Criteria

**Functional:**
- Cards upgrade at same thresholds as RFC-017 (play counts unchanged)
- Player chooses 1 of 2 randomly selected stats at each tier
- Upgrade bonuses stack additively (+10% per upgrade)
- Different stats can be upgraded at different tiers on same card
- Narc (Evidence) cards unaffected (still use Heat-based scaling)

**UX:**
- Upgrade modal appears after hand resolution, not during play
- Clear before/after stat preview for each option
- No way to skip or defer upgrade choice
- Pending upgrades persist across sessions

**Performance:**
- No measurable impact on `calculate_totals()` performance
- Save file size increase negligible (<1KB per upgraded card)

**Code Quality:**
- Unit tests for stat calculation with various upgrade combinations
- Integration test for upgrade detection and queue processing
- Backward compatibility test for saves without `card_upgrades`

---

## Discussion

*This section is populated during implementation with questions, decisions, and deviations.*

---

## Acceptance Review

*This section is populated after implementation is complete.*

---

## Conclusion

*To be completed after merge.*

---

## Sign-Off

**Reviewed By:** ARCHITECT Role
**Date:** (pending)
**Decision:** (pending)
**Status:** Planned
