# SOW-004: Card Retention Between Hands

## Status

**Planned** - 2025-11-09

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
