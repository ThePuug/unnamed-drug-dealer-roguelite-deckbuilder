# SOW-008: Sequential Play with Progressive Dealer Reveals

## Status

**Planned** - 2025-11-10

## References

- **RFC-008:** [Sequential Play with Progressive Dealer Reveals](../01-rfc/008-sequential-play-with-dealer-reveals.md)
- **Branch:** (to be created)
- **Implementation Time:** 8-12 hours

---

## Implementation Plan

### Phase 1: Core Sequential Play

**Goal:** Replace simultaneous face-down play with turn-based sequential play where cards flip immediately

**Deliverables:**
- Turn-based state machine modification (`State::Betting` → sequential iteration)
- Turn order system (rotates each round: Narc/Customer/Player → Customer/Player/Narc → Player/Narc/Customer)
- Immediate card reveal (no face-down holding)
- Running totals update after each card played
- UI indicators for active player turn
- Check action (player can skip playing a card)

**Architectural Constraints:**
- **State machine:** `State::Betting` must iterate through players in turn order (not simultaneous)
- **Turn order rotation:** `get_turn_order(round: u8) -> Vec<Owner>` returns player order based on round number
- **Card visibility:** Cards flip immediately when played (remove `cards_played_this_round` face-down tracking)
- **Totals calculation:** Must be called after EACH card played (not just at end of round)
- **UI responsiveness:** Active player indicator must update in real-time (highlight current player)
- **AI pacing:** 0.5-1s delay between AI actions (prevent instant plays that are hard to follow)

**Success Criteria:**
- Cards play one at a time in correct turn order
- Totals (Evidence/Cover/Profit/Heat) update visibly after each card
- Turn order rotates correctly: Round 1 (N→C→P), Round 2 (C→P→N), Round 3 (P→N→C)
- Player can see who's turn it is (active player highlighted)
- Check action allows skipping card play
- All 61 existing tests still pass

**Duration:** 3-4 hours

---

### Phase 2: Dealer Deck System

**Goal:** Add Dealer deck as community cards that reveal progressively (one per round)

**Deliverables:**
- `HandState` extended with `dealer_deck` and `dealer_hand` fields
- `create_dealer_deck()` function (20 scenario cards: 8 Locations, 8 Modifiers, 4 Wild)
- `CardType::DealerLocation` and `CardType::DealerModifier` enum variants
- Dealer reveal system (one card flips after player phase each round)
- Dealer cards integrate into totals calculation (same as player cards)
- Dealer deck initialization in deck building flow

**Architectural Constraints:**
- **HandState structure:**
  ```rust
  pub struct HandState {
      // ... existing fields
      dealer_deck: Vec<Card>,    // 20 scenario cards (shuffled)
      dealer_hand: Vec<Card>,    // 3 cards drawn for this hand
  }
  ```
- **Dealer deck composition:**
  - 8 Location cards (set Evidence/Cover/Heat values, can be overridden)
  - 8 Modifier cards (adjust Evidence/Cover/Heat additively, cannot be overridden)
  - 4 Wild cards (high-impact swings: +/- significant values)
- **Dealer reveal timing:** After all players finish turn-based play, reveal `dealer_hand[round-1]`
- **Totals integration:** Dealer cards processed identically to player cards (override rules apply)
- **Deck initialization:** Dealer deck created and shuffled at hand start (in `HandState::new()` or reset logic)
- **Separation:** Dealer deck completely separate from narc/customer/player decks (no cross-contamination)

**Success Criteria:**
- `create_dealer_deck()` returns 20 cards (8 Locations, 8 Modifiers, 4 Wild)
- Dealer deck shuffled and 3 cards drawn at hand start
- One dealer card reveals per round (after player phase)
- Dealer Locations override player Locations (existing override logic works)
- Dealer Modifiers affect totals additively (cannot be overridden)
- Totals correctly reflect Dealer card effects
- Deck building flow initializes Dealer deck without errors

**Duration:** 2-3 hours

---

### Phase 3: Fold Mechanics

**Goal:** Add fold option after Dealer reveal (Rounds 1-2) with different consequences for player vs customer

**Deliverables:**
- Player fold action (available after Dealer reveal, Rounds 1-2 only)
- Player fold consequences: discard played cards, keep unplayed cards, keep Heat, exit to next hand
- Customer fold AI logic (threshold-based: Evidence > 50/60/80 by round)
- Customer fold consequences: remove customer cards from totals, lose profit multipliers, continue hand
- Narc never folds (hardcoded, skip fold check)
- Fold button UI (visible after Dealer reveal, disabled Round 3)

**Architectural Constraints:**
- **Player fold action:**
  - Available states: After `dealer_reveal` in Rounds 1-2
  - Fold effects:
    - ❌ Discard `cards_played` (cards played so far are lost)
    - ✅ Keep `player_hand` (unplayed cards retained)
    - ✅ Keep accumulated Heat (Heat from played cards persists)
    - ❌ Lose profit from this hand (deal didn't complete)
  - Transition: `State::Fold` → skip to next hand or end run
- **Customer fold logic:**
  - AI trigger: `evidence > threshold` where threshold = 50 (R1), 60 (R2), 80 (R3)
  - Fold effects:
    - Remove customer cards from `cards_played` (recalculate totals)
    - Profit multipliers lost (×1.5 becomes ×1.0)
    - Evidence from customer cards also removed (slight help to player)
  - Hand continues with remaining players (narc + player)
- **Narc behavior:** Narc CANNOT fold (skip fold check entirely, always plays through)
- **Fold button:** Only enabled after Dealer reveal, only in Rounds 1-2, grayed out in Round 3

**Success Criteria:**
- Player can fold after Dealer reveal (Rounds 1-2)
- Player fold discards played cards but keeps unplayed cards in hand
- Player fold keeps Heat accumulated from played cards
- Customer folds ~20-30% of hands (tune threshold if needed)
- Customer fold removes customer cards and recalculates totals
- Narc never folds (no fold option for narc)
- Fold button visible/enabled only when appropriate
- Round 3 has no fold option (commitment round)

**Duration:** 1-2 hours

---

### Phase 4: Integration Testing and Balance

**Goal:** Verify all existing features work with new sequential play system and tune balance

**Deliverables:**
- Insurance/Conviction integration verification (still triggers at resolution)
- Deck building integration verification (Dealer deck initializes correctly)
- Card retention integration verification (unplayed cards still retained)
- Override wars verification (Locations still override progressively)
- Balance tuning (Dealer deck difficulty: 50% safe, 30% neutral, 20% dangerous)
- Regression testing (all 61 existing tests pass)
- Playtesting (5-10 hands to validate core loop)

**Architectural Constraints:**
- **Insurance/Conviction:** Must still work at resolution (no changes to `resolve_hand()` logic)
- **Deck building:** `start_run_button_system` must initialize Dealer deck in `HandState`
- **Card retention:** Unplayed cards still return to hand next deal (no regression)
- **Override rules:** Location cards still replace previous Location (last played = active)
- **Balance targets:**
  - Average 2-3 cards played per round
  - Player folds ~20-30% of hands (not too punishing)
  - Customer folds ~20-30% of hands (creates interesting situations)
  - ~50% of hands involve raises (from existing betting system)
- **Performance:** Maintain 60fps (<16ms per frame)

**Success Criteria:**
- Insurance activates correctly when Evidence > Cover at resolution
- Conviction overrides insurance when Heat >= threshold
- Deck building flow creates HandState with Dealer deck
- Unplayed cards retained between hands (card retention still works)
- Location overrides work with sequential play (Narc plays School Zone → Player overrides with Warehouse → Dealer overrides with Checkpoint)
- All 61 existing unit tests pass (no regressions)
- Playtesting validation: 4+ of 6 acceptance criteria met (from RFC-008)
- Performance: No lag during card plays or totals updates

**Duration:** 2-3 hours

---

## Acceptance Criteria

### Functional

**Sequential Play:**
- ✅ Cards play one at a time in turn order (not simultaneous)
- ✅ Turn order rotates each round (R1: N→C→P, R2: C→P→N, R3: P→N→C)
- ✅ Cards flip immediately when played (no face-down phase)
- ✅ Totals update after each card (Evidence/Cover/Profit/Heat)
- ✅ Player can Check (skip playing a card)
- ✅ AI actions paced (0.5-1s delay between plays)

**Dealer Deck:**
- ✅ Dealer deck created (20 cards: 8 Locations, 8 Modifiers, 4 Wild)
- ✅ 3 cards drawn at hand start (one per round)
- ✅ One dealer card reveals per round (after player phase)
- ✅ Dealer Locations override player Locations
- ✅ Dealer Modifiers affect totals additively
- ✅ Dealer deck separate from player decks

**Fold Mechanics:**
- ✅ Player can fold after Dealer reveal (Rounds 1-2 only)
- ✅ Player fold discards played cards, keeps unplayed cards
- ✅ Player fold keeps accumulated Heat
- ✅ Customer folds when Evidence > threshold (~20-30% of hands)
- ✅ Customer fold removes customer cards, recalculates totals
- ✅ Narc never folds (no fold option)
- ✅ Round 3 has no fold option (commitment round)

**Integration:**
- ✅ Insurance/Conviction still work at resolution
- ✅ Deck building initializes Dealer deck correctly
- ✅ Card retention still works (unplayed cards retained)
- ✅ Override wars work with sequential play
- ✅ All 61 existing tests pass (no regressions)

### UX

**Clarity:**
- ✅ Active player indicator visible (highlight current turn)
- ✅ Running totals display (Evidence/Cover/Profit/Heat bars + numbers)
- ✅ Dealer card reveal is dramatic (clear visual moment)
- ✅ Fold button visible/enabled only when appropriate
- ✅ Active Location highlighted (shows which Location in effect)

**Feel:**
- ✅ Rounds feel individually meaningful (not just accumulation)
- ✅ Dealer reveals create tension ("river" moments)
- ✅ Override wars are visible and impactful
- ✅ Fold decisions feel strategic (not arbitrary)
- ✅ Sequential play feels natural (not confusing)

**Playtesting Validation (from RFC-008):**

After 5-10 test hands, evaluate:
- [ ] Did each round force a decision? (not just accumulation)
- [ ] Did Dealer reveals change strategy?
- [ ] Did I fold at least once because of progressive info?
- [ ] Did override mechanic matter? (changed outcome)
- [ ] Could I track Evidence/Cover totals easily?
- [ ] Did Customer folding create interesting choice?

**Target:** 4+ of 6 criteria = core loop works, game is fun

### Performance

- ✅ < 16ms per frame (60fps maintained)
- ✅ No lag during card plays or totals updates
- ✅ No performance regression from baseline

### Code Quality

**Architecture:**
- ✅ Clean integration with existing state machine
- ✅ Dealer deck encapsulated (doesn't pollute player deck logic)
- ✅ Fold logic isolated (doesn't break existing betting system)
- ✅ Turn order logic reusable (easy to modify rotation pattern)

**Testing:**
- ✅ No regressions (all 61 existing tests pass)
- ✅ Edge cases covered (fold with 1 card, multiple raises, all players fold)
- ✅ Integration tests added (dealer deck + fold mechanics)

**Code Size:**
- ✅ Target +300-500 lines (dealer deck, turn order, fold logic, UI updates)
- ✅ Total under 5,500 lines (currently ~5,000)

---

## Discussion

*This section will be populated during implementation with questions, decisions, and deviations.*

---

## Acceptance Review

*This section will be populated after implementation is complete.*

---

## Sign-Off

**Reviewed By:** [ARCHITECT Role]
**Date:** [To be completed]
**Decision:** [To be completed]
**Status:** [To be completed]
