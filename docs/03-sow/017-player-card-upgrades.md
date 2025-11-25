# SOW-017: Player Card Upgrades (Per-Run)

## Status

**Implemented** - 2025-11-25

## References

- **RFC-017:** [Player Card Upgrades (Per-Run)](../01-rfc/017-player-card-upgrades.md)
- **Spec:** [Progression System](../00-spec/progression-meta.md)
- **Branch:** main (direct implementation on approved RFC)
- **Implementation Time:** 6-10 hours

---

## Implementation Plan

### Phase 1: Play Count Storage

**Goal:** Track how many times each card has been played per character.

**Deliverables:**
- Add `card_play_counts: HashMap<String, u32>` to `CharacterState`
- Add serialization support (serde)
- Add helper methods: `increment_play_count()`, `get_play_count()`
- Ensure counts reset on permadeath (character deleted)

**Architectural Constraints:**
- Card ID is the card's `name` field (String)
- Counts stored in CharacterState (not AccountState) - lost on permadeath
- Use `#[serde(default)]` for backward compatibility with existing saves
- Validate counts don't overflow (cap at reasonable max)

**Success Criteria:**
- Play counts persist across game sessions
- Play counts reset when character dies (permadeath)
- Backward-compatible with existing saves

**Duration:** 1-2 hours

---

### Phase 2: Play Tracking Hook

**Goal:** Increment play count when cards are played during a hand.

**Deliverables:**
- Hook into card play system to track plays
- Increment count in CharacterState when player plays a card
- Save character state after play count update

**Architectural Constraints:**
- Only track player cards (not Narc or Buyer cards)
- Increment when card is actually played (not just held in hand)
- Don't track cards that are discarded without being played

**Success Criteria:**
- Play count increments when card is played
- Count persists to save file
- Multiple plays of same card accumulate

**Duration:** 2-3 hours

---

### Phase 3: Tier Calculation & Stat Modification

**Goal:** Apply upgrade bonuses based on play count tiers.

**Deliverables:**
- `UpgradeTier` enum (Base, Tier1 for MVP)
- `get_upgrade_tier(play_count: u32) -> UpgradeTier` function
- Modify `calculate_totals()` to apply tier multipliers
- Apply multiplier to primary stat based on card type

**Architectural Constraints:**
- MVP: Only Base and Tier 1 (threshold: 5 plays)
- Tier 1 bonus: +10% to primary stat
- Primary stat by card type:
  - Product: price
  - Location: cover (and -10% evidence)
  - Cover cards: cover value
  - Deal Modifiers: multiplier bonus
  - Insurance: cover value
- Calculate at play time (not stored), cache if needed

**Success Criteria:**
- Cards with 5+ plays get +10% bonus
- Bonus applies correctly to each card type
- Base stats unchanged for cards with <5 plays

**Duration:** 2-3 hours

---

### Phase 4: UI Display

**Goal:** Show upgrade tier on cards in deck builder and during play.

**Deliverables:**
- Tier badge/indicator on card display
- Show play count progress (e.g., "3/5 to Tier 1")
- Visual distinction for upgraded cards

**Architectural Constraints:**
- Subtle indicator (don't clutter card display)
- Show in deck builder where players make decisions
- Consider showing during hand play as well

**Success Criteria:**
- Players can see which cards are upgraded
- Players can see progress toward next tier
- Upgraded cards visually distinct from base

**Duration:** 1-2 hours

---

## Acceptance Criteria

**Functional:**
- Play counts track per card per character
- Tier 1 threshold at 5 plays with +10% bonus
- Counts persist across sessions
- Counts reset on permadeath

**UX:**
- Upgrade tier visible on cards
- Progress toward next tier visible
- Clear feedback on upgraded stats

**Performance:**
- No perceptible delay from tier calculation
- Minimal save file size increase

**Code Quality:**
- Tests cover play tracking, tier calculation, persistence
- Backward-compatible with existing saves
- Clean integration with existing card/stat systems

---

## Discussion

### Implementation Notes

**Phase 1 (Play Count Storage):**
- Added `UpgradeTier` enum to `src/save/types.rs` with MVP scope (Base, Tier1 only)
- Added `card_play_counts: HashMap<String, u32>` to `CharacterState`
- Helper methods: `increment_play_count()`, `get_play_count()`, `get_card_tier()`
- Uses `#[serde(default)]` for backward compatibility
- 9 unit tests added for tier and play count functionality

**Phase 2 (Play Tracking Hook):**
- Modified `save_after_resolution_system` in `src/systems/save_integration.rs` to track plays
- Play counts increment ONLY on successful deals (Safe outcome with profit > 0)
- Only player card types tracked (Product, Location, Cover, DealModifier, Insurance)
- Narc/Buyer cards (Evidence, Conviction) are NOT tracked
- Saves updated counts to character state after each successful deal

**Phase 3 (Tier Calculation & Stat Modification):**
- Added `card_play_counts` field to `HandState` (copied from CharacterState on run start)
- Modified `calculate_totals()` in `card_engine.rs` to apply tier multipliers
- Tier bonuses by card type:
  - Product: +10% price
  - Location: +10% cover, -10% evidence
  - Cover: +10% cover value
  - DealModifier: +10% to price multiplier
  - Insurance: +10% cover value
- 4 unit tests added for upgrade tier bonuses

**Phase 4 (UI Display):**
- Added `UpgradeInfo` struct to `src/ui/helpers.rs`
- Added `spawn_card_button_with_upgrade` function for cards with upgrade display
- Modified `spawn_card_text_overlays` to show tier badge (gold star) for upgraded cards
- Shows play count progress toward next tier (e.g., "0/1" for testing, "0/5" for production)
- Updated `populate_deck_builder_cards_system` in `src/systems/ui_update.rs` to show upgrade info
- Deck builder cards now show tier badges and progress for all cards

### Deviations from Plan

- MVP limited to Base + Tier1 only (as planned in RFC-017 MVP Scope Decision)
- TESTING: Threshold temporarily set to 1 play (production should be 5)
- Upgrade notification ("Card upgraded to Tier 1!") deferred to follow-up

---

## Acceptance Review

*This section is populated after implementation is complete.*

---

## Sign-Off

**Reviewed By:** [ARCHITECT Role]
**Date:** TBD
**Decision:** TBD
**Status:** TBD
