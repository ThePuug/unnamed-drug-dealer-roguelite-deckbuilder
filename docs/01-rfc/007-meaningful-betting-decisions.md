# RFC-007: Meaningful Betting Decisions

## Status

**Approved** - 2025-11-10

## Feature Request

### Player Need

From player perspective: **Raises don't matter - I just play all my cards every round and fold at the end if the numbers look bad. There's no meaningful decision-making during betting.**

**Current Problem:**
Without betting constraints:
- No reason to consider raises (just ignore them and keep playing cards)
- No tension during rounds (just dump all cards mindlessly)
- No risk/reward decisions (betting doesn't affect my options)
- Fold decision happens too late (after I've already committed all cards)
- No counter-play to opponent aggression (raising doesn't pressure me)

**We need a system that:**
- Makes raises create real pressure and force decisions
- Creates meaningful choice between "play more cards" vs "fold early"
- Builds tension during rounds (not just at the end)
- Rewards reading opponent patterns and managing risk
- Punishes mindless "dump everything" strategy

### Desired Experience

Players should experience:
- **Pressure from raises:** "They raised again - do I commit more cards or fold now?"
- **Risk/reward tradeoffs:** "Playing another card might win this, but if I bust I lose everything"
- **Bluffing dynamics:** "Are they raising because they have the goods, or trying to scare me off?"
- **Resource management:** "I only have X cards I can play this round - which ones matter most?"
- **Tension building:** Each raise increases stakes and forces harder decisions

### Specification Requirements

**Option A: Card Limit Per Round (Poker-style)**
- Each round has a max cards you can play (e.g., 3 cards max)
- Raises don't increase limit (they increase stakes/pot)
- Forces "which cards do I commit?" decisions
- Problem: Doesn't make raises matter more

**Option B: Raises Cost Cards (Resource Pressure)**
- Matching a raise costs you 1 card from hand (discard it)
- Can't play cards if you don't match the raise
- Creates "pay to stay in" pressure
- Forces fold-or-commit decisions each raise
- Problem: May feel punishing, reduces card playing

**Option C: Card Play Limit Tied to Betting**
- Start round with 1 card play allowed
- Each time you MATCH a raise, you get to play 1 more card
- Raising yourself doesn't give you more plays (only matching gives benefit)
- Creates economy: "Do I match to unlock more card plays?"
- Opponents raising gives YOU opportunity (match to play more)
- Problem: Complex, may be unclear

**Option D: Hand Size as Resource (Spend Cards to Match)**
- Matching a raise requires discarding a card from hand
- Raises create hand depletion pressure
- Forces "is this hand worth the cost?" decisions
- Natural bluffing: High raises push people out
- Problem: Could make hands too short

### MVP Scope

The RFC needs ARCHITECT input to determine which approach is:
- Most fun (creates meaningful decisions)
- Technically feasible (integrates with existing systems)
- Completable in one SOW (≤20 hours)

**MVP must include:**
- Whatever mechanic makes raises create real decisions
- Clear UI feedback showing constraints/costs
- Fold option during betting (not just at decision point)
- Balance adjustments to make new system feel fair

**MVP excludes:**
- Advanced bluffing mechanics
- Complex betting psychology
- Statistical analysis tools
- Multiplayer considerations (this is single-player roguelite)

### Priority Justification

**CRITICAL PRIORITY** - Core gameplay is not fun

**Why Critical:**
- Current game loop is mindless (dump cards, check numbers, fold/continue)
- No skill expression or meaningful decisions
- Player explicitly reports "not fun"
- Betting system exists but serves no purpose
- Kills replayability (every hand plays the same way)

**Impact of NOT fixing:**
- Game remains a number-checking exercise
- No tension or excitement
- Deck building doesn't matter (strategy doesn't affect decisions)
- Players won't engage long-term

**Benefits of fixing:**
- Moment-to-moment decisions become interesting
- Raises create tension and pressure
- Skill matters (reading situations, managing resources)
- Deck building becomes strategic (different decks = different playstyles)
- Replayability increases (each hand feels different)

---

## Feasibility Analysis

**ARCHITECT Assessment - 2025-11-10**

### Technical Evaluation of Options

**Option A: Card Limit Per Round**
- ❌ **Rejected** - Doesn't solve core problem (raises still don't matter)
- Would just shift from "dump all cards" to "dump 3 cards max"
- No connection between betting and card play
- Doesn't create pressure from opponent raises

**Option B: Raises Cost Cards (Discard to Match)**
- ⚠️ **Viable but harsh**
- Technical: Easy to implement (add card discard on Check/Call when raise pending)
- Fun factor: Creates pressure but feels punishing
- Problem: Reduces card playing (core mechanic), hand gets small fast
- Integration: Clean (modify betting_button_system, update BettingState)

**Option C: Card Play Limit Tied to Betting**
- ⚠️ **Interesting but complex**
- Technical: Moderate complexity (track "plays remaining", unlock on match)
- Fun factor: Creates economy/resource management
- Problem: Unintuitive ("why can't I play cards?"), complex to explain
- Integration: Requires new resource tracking in BettingState

**Option D: Hand Size as Resource (Discard from Hand to Match)**
- ✅ **RECOMMENDED**
- Technical: Simple implementation (discard from hand on Check/Call)
- Fun factor: Creates genuine pressure without eliminating card play
- Clear cause-effect: "To stay in, I must pay with a card"
- Integration: Minimal changes (modify betting actions, add discard logic)
- Natural bluffing: Big raises force hard decisions

### Recommended Solution: Enhanced Option D

**"Cards as Betting Currency"**

**Core Mechanic:**
- Matching a raise costs 1 card from your HAND (discard it, not play it)
- This creates immediate pressure: "Is this round worth losing a card?"
- Preserves card-playing (you still play cards after matching)
- Natural fold incentive: "They keep raising, I'm running out of cards to match"

**Implementation Details:**

```
Current flow (broken):
  Narc raises → Player: CHECK (no cost) → Play 3 cards → Fold if bad

New flow (meaningful):
  Narc raises → Player: CALL (discard 1 card) → Can now play cards
               OR FOLD (keep cards, exit round)
```

**Changes Required:**
1. **BettingState tracking:**
   - Add `pending_raise: bool` (is there a raise to match?)
   - Track who needs to match

2. **Betting actions cost:**
   - CHECK: Free if no raise pending
   - CALL: Costs 1 card from hand (if raise pending)
   - FOLD: Free, exit round immediately
   - RAISE: Play 1 card face-down + create pending_raise for others

3. **UI updates:**
   - Show "RAISE PENDING - Call (costs 1 card) or Fold"
   - Display hand count (so players know cost)
   - Disable CHECK when raise pending (must CALL or FOLD)

4. **Balance adjustments:**
   - May need to increase hand size (currently 3) to 4-5 cards
   - Adjust AI raise frequency (too aggressive = player always folds)

### Performance Projections

**Development Time:**
- Betting logic changes: 2-3 hours
- UI updates (show costs, disable CHECK): 1-2 hours
- Hand size rebalancing: 1 hour
- Testing (betting costs, edge cases): 2 hours
- **Total: 6-8 hours** (fits in one SOW ✅)

**Code Impact:**
- Modify: `betting_button_system` (add discard logic)
- Modify: `BettingState` (add pending_raise tracking)
- Modify: `update_betting_button_states` (disable CHECK when raise)
- Add: Unit tests for discard-on-match logic
- Estimated: +200-300 lines

### Technical Risks

**1. Hand Depletion Too Fast**
- *Risk:* Players run out of cards quickly, can't play enough
- *Mitigation:* Increase starting hand size to 4-5 cards, tune AI aggression
- *Impact:* Medium - requires playtesting and balance

**2. Confusing New Players**
- *Risk:* "Why did my hand decrease when I matched?"
- *Mitigation:* Clear UI messaging, tutorial hints
- *Impact:* Low - cause-effect is clear with good feedback

**3. Breaking Existing Balance**
- *Risk:* All current balance assumes 3-card hands
- *Mitigation:* Adjust hand size, deck counts may need tuning
- *Impact:* Medium - may need iteration

### System Integration

**Affected Systems:**
- Betting phase (add discard-on-match logic)
- UI (show costs, hand count)
- Hand state (track hand size, discard mechanism)
- AI behavior (adjust raise frequency to not over-pressure)

**Compatibility:**
- ✅ Works with all existing card mechanics
- ✅ Works with conviction/insurance (doesn't change)
- ✅ Works with deck building (different decks = different hand sizes)
- ✅ No changes to Evidence/Cover calculations

**Integration Points:**
- Betting actions → Hand discard
- UI → Show pending raise + costs
- AI → Adjust raise probability

### Recommendation

**✅ Proceed with Enhanced Option D: "Cards as Betting Currency"**

**Rationale:**
- Solves core problem: Raises create real pressure (cost cards to stay in)
- Simple to understand: "Match raise = lose 1 card from hand"
- Natural tension: Multiple raises deplete your hand
- Preserves card-playing core mechanic
- Fits in one SOW (6-8 hours)
- Clean integration with existing systems

**Next Steps:**
1. PLAYER validates this solves the fun problem
2. If approved, ARCHITECT creates SOW-007
3. DEVELOPER implements with playtesting for balance

---

## Discussion

### PLAYER Validation

**Does Enhanced Option D solve the fun problem?**

✅ **YES** - Creates all the missing elements:

1. **Raises create pressure:** Each raise costs me a card to match - real cost/benefit analysis
2. **Meaningful choice:** "Do I spend a card to stay in, or fold and preserve my hand?"
3. **Tension builds:** Multiple raises = depleting hand = increasing pressure
4. **Bluffing matters:** Opponent raise patterns affect my decision
5. **Resource management:** Hand size is precious, every discard hurts

**Key insight:** Raises should make staying in the round HURT. Discarding from hand creates that hurt without eliminating card-playing (which is the fun part).

**Concerns addressed:**
- Won't feel too punishing if hand size increases to 5-6 cards
- Clear feedback ("You discarded X to match raise") makes cost obvious
- Fold option always available (escape valve)

### ARCHITECT Validation

**Is Enhanced Option D feasible and maintainable?**

✅ **YES** - Clean integration:

1. **Simple changes:** Add discard logic to CALL action, track pending_raise
2. **No architectural changes:** Works within existing betting system
3. **Testable:** Pure function for discard logic, unit tests straightforward
4. **Scalable:** Doesn't introduce complexity that compounds
5. **6-8 hours:** Realistic for one SOW

**Technical confidence:** HIGH - straightforward implementation, low risk

---

## Approval

**Status:** Draft → **Approved**

**Approvers:**
- PLAYER: ✅ Solves core fun problem, creates meaningful decisions
- ARCHITECT: ✅ Feasible, maintainable, fits in one SOW

**Scope Constraint:** ✅ 6-8 hours (fits in one SOW)

**Dependencies:**
- No blocking dependencies
- Requires balance tuning after implementation

**Next Steps:**
1. ARCHITECT creates SOW-007
2. DEVELOPER implements with balance iteration
3. Playtest and tune hand size + AI aggression

**Date:** 2025-11-10
