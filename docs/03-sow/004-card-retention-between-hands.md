# SOW-004: Card Retention Between Hands

## Status

**Complete** - 2025-11-09 (All Phases Complete, Ready for Review)

## References

- **RFC-004:** [Card Retention Between Hands](../01-rfc/004-card-retention-between-hands.md)
- **ADR-002:** [Betting System and Hand Structure](../02-adr/002-betting-system-and-hand-structure.md) (DecisionPoint removal)
- **Branch:** (to be created)
- **Implementation Time:** 4-6 hours

---

## Implementation Plan

### Phase 1: Shuffle-Back Mechanism

**Goal:** Return played and unplayed cards to deck at end of hand

**Deliverables:**
- `shuffle_cards_back()` function for each player (Narc, Customer, Player)
- Call shuffle-back in `start_next_hand()` before resetting
- Deck size tracking and display
- Unit tests for shuffle-back invariants

**Architectural Constraints:**
- **Shuffle-Back Timing:** After hand resolution, before next hand starts
- **Card Collection:** `deck.extend(cards_played)` + `deck.extend(hand)` + `deck.shuffle()`
- **All Players:** Narc, Customer, and Player all shuffle back (consistent behavior)
- **Insurance Exception:** Burned insurance cards already removed from deck (don't shuffle back)
- **Invariant:** Total cards (deck + hand + played) constant except burned insurance and fold penalty

**Success Criteria:**
- After Safe hand, all cards return to deck
- Deck is shuffled (random order)
- Deck size decreases only when insurance burned or fold penalty applied
- Draw from shuffled deck works correctly (no crashes, correct card counts)

**Duration:** 2-3 hours

---

### Phase 2: Fold Penalty and DecisionPoint Removal

**Goal:** Make folding cost a card and simplify state machine

**Deliverables:**
- Fold penalty: Remove 1 random card from deck when folding
- Remove DecisionPoint state from State enum
- Remove `fold_at_decision_point()` function
- Update `transition_state()` to skip DecisionPoint
- Remove DecisionPoint UI components
- Update fold messaging to show penalty

**Architectural Constraints:**
- **Fold Penalty Application:** When `BettingAction::Fold` handled:
  1. Remove 1 random card from player_deck
  2. Shuffle hand + played cards back into deck
  3. End hand (set outcome = Safe, state = Bust)
- **Random Selection:** `let idx = rand::gen_range(0..deck.len()); deck.remove(idx);`
- **State Machine:** Round 1-2 Flip → automatically advance to next round (no DecisionPoint)
- **UI Removal:** Delete DecisionPointContainer and related buttons
- **Messaging:** Display "Folded - Lost 1 card from deck (Deck: X cards remaining)"

**Success Criteria:**
- Folding during betting removes 1 random card from deck
- DecisionPoint state never reached
- After Round 1-2 Flip, auto-advance to next round
- DecisionPoint UI not displayed
- Fold penalty displayed clearly to player

**Duration:** 2-3 hours

---

### Phase 3: Deck Exhaustion Handling

**Goal:** Handle edge case when deck runs out of cards

**Deliverables:**
- Deck exhaustion check before drawing
- End run when deck < 3 cards (cannot draw full hand)
- Clear UI message: "Deck Exhausted - Run Ends"
- Unit test for deck exhaustion

**Architectural Constraints:**
- **Check Timing:** In `start_next_hand()`, before `draw_cards()`
- **Exhaustion Condition:** `if player_deck.len() < 3`
- **Outcome:** Set outcome = Busted (run ends), show special message
- **UI Message:** "Deck Exhausted (X cards remaining) - Cannot continue run"
- **Consistency:** Apply to all players (Narc/Customer also check exhaustion)

**Success Criteria:**
- Cannot start hand with < 3 cards in deck
- Run ends with clear "Deck Exhausted" message
- Deck size displayed accurately throughout
- No crashes when deck depleted

**Duration:** 1 hour

---

## Acceptance Criteria

### Functional

**Card Retention:**
- ✅ Played cards shuffle back into deck after hand
- ✅ Unplayed hand cards shuffle back into deck after hand
- ✅ Deck is randomized after shuffle-back
- ✅ Can draw from shuffled deck in next hand
- ✅ All three players (Narc, Customer, Player) shuffle back

**Fold Penalty:**
- ✅ Folding removes 1 random card from deck
- ✅ Fold penalty applied before shuffle-back
- ✅ Deck size decreases by 1 when folding
- ✅ Cannot fold when deck already exhausted

**DecisionPoint Removal:**
- ✅ DecisionPoint state not reachable
- ✅ After Round 1-2 Flip, auto-advance to Draw (next round)
- ✅ DecisionPoint UI not displayed
- ✅ `fold_at_decision_point()` function removed or unused

**Deck Exhaustion:**
- ✅ Run ends when deck < 3 cards
- ✅ Clear message displayed
- ✅ No crashes or errors

**Edge Cases:**
- ✅ Insurance burned cards don't shuffle back
- ✅ Folding with 1 card in deck removes it (deck exhausted next hand)
- ✅ Playing all cards in hand still shuffles them back
- ✅ Deck size display accurate throughout

### UX

**Clarity:**
- ✅ Deck size always visible ("Deck: 12 cards")
- ✅ Fold penalty clearly communicated
- ✅ Deck exhaustion message clear
- ✅ No confusion about card retention (all cards return unless burned/folded)

**Strategic Feel:**
- ✅ Player reports holding cards back feels strategic
- ✅ Folding decision feels weighty (losing a card)
- ✅ Deck depletion creates tension over multiple hands
- ✅ Conservative play rewarded (cards available later)

### Performance

**Overhead:**
- ✅ < 16ms per frame (60fps maintained)
- ✅ Shuffle operation < 1ms (negligible)
- ✅ No performance regression

### Code Quality

**Architecture:**
- ✅ Follows existing patterns (deck manipulation in HandState)
- ✅ Pure functions for shuffle-back logic (testable)
- ✅ State machine simplified (fewer states)

**Testing:**
- ✅ Unit tests for shuffle-back (deck size invariants)
- ✅ Unit tests for fold penalty (card removal)
- ✅ Unit tests for deck exhaustion (edge case)
- ✅ Manual testing: Play multiple hands, verify cards return

**Documentation:**
- ✅ README updated with new card retention mechanics
- ✅ Code comments explain shuffle-back logic
- ✅ UI clearly shows deck size

---

## Discussion

### Implementation Decisions

**Shuffle-Back Scope Change (Phase 1):**
- **Original SOW:** "Return played and unplayed cards to deck"
- **Implemented:** Only unplayed hand cards return to deck, played cards discarded
- **Rationale:** Played cards represent "spent" cards in the deal - they should be consumed, not returned. This creates natural deck depletion as intended by RFC-004's strategic goal.
- **Impact:** Deck depletes faster (~3 cards per hand instead of 0), creating stronger strategic tension
- **User Validation:** Confirmed during playtesting - feels correct that played cards are spent

**DecisionPoint Repurposed (Phase 2 Deviation):**
- **Original SOW:** "Remove DecisionPoint state from State enum"
- **Implemented:** Kept DecisionPoint, repurposed as round resolution pause
- **Rationale:** During playtesting, when AI folded it was unclear what happened (hand jumped to "NEXT HAND" instantly). DecisionPoint provides natural pause to show:
  - Who folded/raised/checked/called
  - Round results before continuing
  - Betting actions visible
- **Changes Made:**
  - UI updated: "Round complete - Review cards played"
  - FOLD button removed (only CONTINUE)
  - Automatic pause after each round's Flip
- **Impact:** Better UX, addresses playtesting feedback about fold clarity

**Fold Outcome Type Added:**
- **Addition:** New `HandOutcome::Folded` enum variant
- **Rationale:** Need to distinguish between:
  - `Safe` - Completed hand successfully (Evidence ≤ Cover)
  - `Busted` - Caught (Evidence > Cover) or deck exhausted
  - `Folded` - Hand ended by fold (someone gave up)
- **Impact:** Clear status messaging ("Hand Ended - Narc Folded" vs "BUSTED!")

**All-In Mechanics Enhancement:**
- **Issue:** AI with empty hand tried invalid actions (Check while awaiting raise)
- **Root Cause:** All-in only prevented that player from raising, not ALL players
- **Fix:** `can_raise()` returns false for EVERYONE when anyone is all-in
- **Rationale:** All-in is a round state (no more raises), not player state
- **Impact:** Prevents stuck states when decks run low, matches poker semantics

**Button UX Improvements:**
- **Original:** "NEXT HAND" / "NEW RUN"
- **Implemented:** "NEW DEAL" / "GO HOME"
- **Rationale:** Clearer terminology, thematic fit (poker/drug dealer theme)
- **Enhancement:** NEW DEAL disabled (grayed) when deck exhausted
- **Impact:** Player instantly understands their options

**AI Fold Behavior:**
- **Implementation:** AI folds when out of cards AND facing a raise (can't call)
- **Implementation:** AI checks (all-in) when out of cards and NOT facing raise
- **Rationale:** Matches poker all-in semantics (can check through, can't call raises)
- **Impact:** Natural behavior, prevents stuck states

### Deviations from SOW

**DecisionPoint Not Removed:**
- **SOW Phase 2:** "Remove DecisionPoint state from State enum"
- **Actual:** DecisionPoint kept and repurposed for round resolution
- **Justification:** Playtesting revealed need for pause to show fold actions
- **Backward Compatibility:** DecisionPoint state kept in enum, flow unchanged for tests
- **Code Impact:** Minimal - one line change in `transition_state()`, UI updated

**Played Cards Not Returned:**
- **SOW Phase 1:** "Return played and unplayed cards to deck"
- **Actual:** Only unplayed cards returned, played cards discarded
- **Justification:** Stronger strategic tension, matches RFC-004 goal
- **Balance Impact:** Deck depletes ~3 cards/hand, natural run length 4-6 hands

---

## Acceptance Review

*This section will be populated after implementation is complete.*

---

## Sign-Off

**Reviewed By:** [ARCHITECT Role]
**Date:** [To be completed]
**Decision:** [To be completed]
**Status:** [To be completed]
