# RFC-004: Card Retention Between Hands

## Status

**Approved** - 2025-11-09

## Feature Request

### Player Need

From player perspective: **Playing cards should feel meaningful, but currently there's no reason NOT to play every card since you discard them anyway at end of hand.**

**Current Problem:**
Without card retention between hands:
- Every card gets discarded at end of hand (you lose unplayed cards)
- No strategic reason to hold cards back (will lose them anyway)
- DecisionPoint "Continue/Fold" feels meaningless (same outcome either way)
- Playing conservatively (saving cards) is actively punished
- Optimal strategy is always "play everything" (removes interesting choices)

**We need a system that:**
- Rewards conservative play (keeping cards for later hands)
- Creates meaningful choice between playing cards now vs. saving for later
- Prevents repeated folding to avoid risk (folding must have cost)
- Makes DecisionPoint decision actually matter

### Desired Experience

Players should experience:
- **Strategic card retention:** "Do I play Safe House now or save it for a riskier hand later?"
- **Deck depletion tension:** "I've folded 3 times, my deck is getting thin..."
- **Meaningful DecisionPoint:** "Continue or fold? If I fold I keep my cards but lose progress"
- **Risk/reward calculation:** "Play aggressively now (use all cards) or conservatively (save for later)?"

### Specification Requirements

**Card Retention System:**
- Unplayed cards return to deck (not discarded) between hands
- Hand shuffles back into remaining deck at end of Safe hand
- Deck persists across entire run (depletes over time)
- Folding costs something (prevents spam folding)

**Folding Cost:**
- Folding discards 1 random card from deck permanently
- Creates consequence for folding (deck depletion)
- Prevents repeated folding to avoid all risk
- Makes "Continue" vs "Fold" at DecisionPoint meaningful

**DecisionPoint Changes:**
- Remove DecisionPoint entirely (no longer needed)
- After each round, automatically continue to next round
- Only way to exit early: Fold during Betting (which costs a card)

### MVP Scope

**Phase 1 includes:**
- Card retention (hand → deck shuffle at end of hand)
- Folding penalty (discard 1 random card from deck)
- Remove DecisionPoint state and UI
- Deck depletion tracking (show remaining deck size)

**Phase 1 excludes:**
- Deck exhaustion mechanics (what happens at 0 cards - defer to Phase 2)
- Discard pile viewing (not needed for MVP)
- Card-specific retention rules (all cards shuffle back equally)

### Priority Justification

**HIGH PRIORITY** - Fixes fundamental strategic flaw in current design

**Why High:**
- Current system undermines strategy (no reason to hold cards)
- Creates illusion of choice (DecisionPoint doesn't matter)
- Playing conservatively is actively punished (wrong incentive)
- Simple fix with major strategic impact

**Benefits:**
- Deck building choices matter more (expensive cards worth saving)
- Risk management becomes strategic (fold early vs. push through)
- DecisionPoint removal simplifies UI (one less modal)
- Creates natural deck depletion tension over run

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Deck Persistence with Shuffle-Back Mechanism**

#### Core Mechanism

**Current Flow (SOW-002/003):**
```
Hand Start: Draw 3 cards from deck
Round 1-3: Play cards → cards_played_this_round
Between rounds: cards_played_this_round → cards_played
Hand End: All cards discarded, deck unchanged
Next Hand: Draw 3 from full deck (same cards available)
```

**New Flow:**
```
Hand Start: Draw 3 cards from deck
Round 1-3: Play cards → cards_played_this_round
Between rounds: cards_played_this_round → cards_played
Hand End (Safe):
  - Shuffle cards_played back into deck
  - Shuffle current hand back into deck
  - Deck now smaller (only contains unpla cards + returned cards)
Hand End (Fold during Betting):
  - Discard 1 random card from deck permanently
  - Shuffle hand + played cards back into deck
Next Hand: Draw 3 from depleted deck
```

**Key Changes:**
1. **Deck is mutable**: Cards removed when played, returned at hand end
2. **Fold penalty**: `deck.remove(random_index)` when fold action taken
3. **DecisionPoint removed**: Auto-advance after each round (no Continue/Fold modal)
4. **Deck size tracking**: Display "Deck: 12 cards" to show depletion

**Data Structure Changes:**
```rust
// HandState already has mutable decks (Vec<Card>)
// No struct changes needed

// New function needed:
fn shuffle_cards_back(&mut self) {
    // Combine cards_played + current hands → deck
    self.player_deck.extend(self.cards_played.drain(..));
    self.player_deck.extend(self.player_hand.drain(..));
    self.player_deck.shuffle(&mut thread_rng());

    // Same for narc/customer
}

// New function for fold penalty:
fn apply_fold_penalty(&mut self) {
    if !self.player_deck.is_empty() {
        let idx = rand::thread_rng().gen_range(0..self.player_deck.len());
        self.player_deck.remove(idx);
    }
}
```

#### Performance Projections

**Overhead:**
- Shuffle operation: O(n) where n = deck size (~15 cards max)
- Negligible (<1ms per hand end)

**Development Time:**
- Implementation: 2-3 hours (modify hand end flow, add shuffle-back, remove DecisionPoint)
- Testing: 1-2 hours (unit tests for shuffle-back, fold penalty, deck depletion)
- UI Updates: 1 hour (remove DecisionPoint UI, add deck size display)
- **Total: 4-6 hours**

#### Technical Risks

**1. Deck Exhaustion Edge Case**
- *Risk:* Player runs out of cards (can't draw 3 for next hand)
- *Mitigation:*
  - Option A: End run when deck < 3 (acceptable - folding too much)
  - Option B: Draw what's available (1-2 cards, continue with smaller hand)
  - Option C: Prevent folding when deck < 6 (always have 2 hands left)
- *Recommendation:* Option A for MVP (simple, makes folding cost clear)

**2. Insurance Card Burning**
- *Risk:* Insurance burns on activation, but also shuffles back?
- *Mitigation:* Burn REMOVES from deck permanently (before shuffle-back)
- *Implementation:* Already handled in SOW-003 (`player_deck.retain()`)

**3. Testing Complexity**
- *Risk:* Randomness in shuffle makes testing harder
- *Mitigation:* Test invariants (deck size, card counts) not order
- *Impact:* Low - existing tests already handle shuffled decks

### System Integration

**Affected Systems:**
- Hand state machine (remove DecisionPoint state)
- Card flow (add shuffle-back at hand end)
- Betting system (add fold penalty)
- UI system (remove DecisionPoint UI, add deck size)

**Compatibility:**
- ✅ Works with SOW-002 multi-round system (no conflicts)
- ✅ Works with SOW-003 insurance (burning already prevents shuffle-back)
- ✅ Works with betting/initiative (fold penalty independent)
- ✅ No breaking changes to card types or totals calculation

**Integration Points:**
- `resolve_hand()` - Add shuffle-back after outcome determination
- `handle_action(Fold)` - Add fold penalty before ending hand
- `transition_state()` - Remove DecisionPoint transitions
- UI visibility system - Remove DecisionPoint container
- Deck size display - Add to totals/status area

### Alternatives Considered

#### Alternative 1: Keep DecisionPoint, Add Different Fold Cost

**Approach:** Keep Continue/Fold choice, but make Fold cost cash or heat instead of cards

**Rejected because:**
- Cash cost: Undermines insurance affordability (need cash for that)
- Heat cost: Weird flavor (folding makes you hotter?)
- Doesn't solve core issue (still no reason to save cards for later hands)

#### Alternative 2: Card Drafting (Keep N Cards Between Hands)

**Approach:** At hand end, choose 3 cards to keep, discard rest

**Rejected because:**
- Adds UI complexity (card selection modal)
- Optimal strategy clear (always keep best cards)
- Doesn't create deck depletion tension
- More complex than shuffle-back

#### Alternative 3: Partial Retention (Only Unplayed Cards Return)

**Approach:** Played cards discarded, hand cards shuffle back

**Rejected because:**
- Still incentivizes playing everything (cards_played lost forever)
- Doesn't fully solve strategic problem
- More complex rules (players must track played vs. unplayed)

---

## Discussion

### ARCHITECT Notes

**Design Implications:**

1. **Deck Building Weight Increases:**
   - Current: 15-card deck, draw 3 per hand, unlimited hands
   - New: 15-card deck depletes over ~5-7 hands if folding
   - Impact: Every card in deck matters more (no infinite draw assumption)

2. **Insurance Becomes More Valuable:**
   - Current: Can fold to avoid bust (no consequence)
   - New: Folding costs a card (depletion)
   - Impact: Insurance saves your run AND your deck (double value)

3. **Conviction Pressure Increases:**
   - New: Can't fold repeatedly to avoid conviction triggers
   - Folding depletes deck → fewer cards → harder to win
   - Impact: High heat becomes more dangerous (can't just fold out)

4. **Strategic Depth:**
   - Conservative play: Save high-value cards for critical hands
   - Aggressive play: Play everything, maximize profit, risk depletion
   - Deck management: Balance between playing optimally and preserving deck

**Implementation Simplicity:**
- Simpler than current (remove entire DecisionPoint system)
- Less UI (one fewer modal)
- Clearer rules (all cards shuffle back, folding costs 1 card)

**Concerns:**
- Deck exhaustion needs clear messaging ("Cannot start hand - deck exhausted")
- Fold penalty needs clear feedback ("Folded - 1 card discarded from deck")

### PLAYER Validation

This change makes the game significantly more strategic:
- ✅ Holding cards back now has purpose (can use them later)
- ✅ Folding has real cost (deck depletion)
- ✅ DecisionPoint removal simplifies flow (faster gameplay)
- ✅ Creates natural run length (~5-10 hands before depletion)

**Expected Feel:**
- Clutch moments: "Saved Safe House for when I really needed it"
- Tension moments: "My deck is down to 8 cards, can't afford to fold again"
- Strategic depth: "Do I play Cocaine now or save for a safer hand?"

---

## Approval

**Status:** Draft → Ready for Approval

**Approvers:**
- ARCHITECT: ✅ Feasible, improves design, simple implementation
- PLAYER: ✅ Solves strategic flaw, creates meaningful choices

**Scope Constraint:** ✅ Fits in one SOW (~4-6 hours)

**Dependencies:**
- Requires SOW-002/003 (multi-round + insurance system)
- No blocking dependencies

**Next Steps:**
1. ARCHITECT creates SOW-004
2. DEVELOPER implements card retention
3. Playtest to verify strategic depth improvement

**Date:** 2025-11-09


---

## Discussion

*To be populated during RFC iteration*

---

## Approval

**Status:** Draft

**Approvers:**
- ARCHITECT: [Pending]
- PLAYER: [Pending]

**Date:** 2025-11-09
