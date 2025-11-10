# SOW-007: Meaningful Betting Decisions

## Status

**Planned** - 2025-11-10

## References

- **RFC-007:** [Meaningful Betting Decisions](../01-rfc/007-meaningful-betting-decisions.md)
- **Branch:** (to be created)
- **Implementation Time:** 6-8 hours

---

## Implementation Plan

### Phase 1: Betting Cost Mechanism

**Goal:** Make matching raises cost cards from hand

**Deliverables:**
- Add `pending_raise: bool` to BettingState
- Modify CALL action to discard 1 card from hand when raise pending
- Add FOLD option during betting (exit round immediately)
- Unit tests for discard-on-match logic

**Architectural Constraints:**
- **Discard from hand:** Matching raise removes 1 card from player's hand (random card)
- **Raise creates pending:** RAISE action sets `pending_raise = true` for other players
- **CHECK disabled:** When `pending_raise = true`, CHECK button disabled (must CALL or FOLD)
- **FOLD during betting:** New option - fold immediately, skip to next round
- **No cost to fold:** Folding is always free (preserve hand for next round)

**Success Criteria:**
- Matching a raise reduces hand size by 1
- CHECK button disabled when raise pending
- FOLD button available during betting
- AI properly discards when matching raises
- Tests verify discard logic

**Duration:** 3-4 hours

---

### Phase 2: UI Feedback and Balance

**Goal:** Show costs clearly and tune hand size

**Deliverables:**
- UI shows pending raise status ("CALL costs 1 card")
- Display current hand count
- Increase starting hand size to 5 cards
- Adjust AI raise frequency
- Balance testing

**Architectural Constraints:**
- **Hand size:** Increase from 3 to 5 cards per round
- **UI messaging:** Clear cost display ("Match raise - costs 1 card")
- **Hand count display:** Always visible during betting
- **AI tuning:** Reduce raise probability to ~30% (from current levels)

**Success Criteria:**
- Players see "CALL (costs 1 card)" when raise pending
- Hand count visible in UI
- Starting with 5 cards feels balanced
- AI doesn't over-pressure (allows some safe rounds)
- Gameplay feels tense but fair

**Duration:** 2-3 hours

---

### Phase 3: Testing and Iteration

**Goal:** Verify gameplay improvements and balance

**Deliverables:**
- Comprehensive unit tests (8-10 new tests)
- Edge case handling (discard with 1 card, multiple raises)
- Balance verification (hand size, AI aggression)
- Regression testing (all existing tests pass)

**Architectural Constraints:**
- **Test coverage:** Discard logic, pending raise tracking, fold during betting
- **Edge cases:** Empty hand handling, max raises, all players fold
- **Balance targets:**
  - Average 2-3 cards played per round
  - ~50% of rounds involve raises
  - Player folds ~20-30% of time (not too punishing)

**Success Criteria:**
- 8-10 new unit tests (all passing)
- No regressions (all existing 61 tests pass)
- Edge cases handled gracefully
- Balance feels fair (playtesting required)

**Duration:** 1-2 hours

---

## Acceptance Criteria

### Functional

**Betting Costs:**
- ✅ CALL action discards 1 card from hand when raise pending
- ✅ CHECK disabled when raise pending (must CALL or FOLD)
- ✅ FOLD available during betting (not just decision point)
- ✅ Raise creates pending_raise for other players
- ✅ AI discards cards when matching raises

**UI Feedback:**
- ✅ Shows "CALL (costs 1 card)" when raise pending
- ✅ Hand count displayed during betting
- ✅ Clear visual distinction (CHECK disabled, CALL highlighted)

**Balance:**
- ✅ Starting hand size increased to 5 cards
- ✅ AI raise frequency tuned (~30%)
- ✅ Gameplay feels tense but fair
- ✅ Average 2-3 cards played per round

**Edge Cases:**
- ✅ Discard with 1 card left (auto-folds)
- ✅ Multiple raises deplete hand correctly
- ✅ All players fold handling
- ✅ Discard doesn't break card retention

### UX

**Clarity:**
- ✅ Cost of matching raise is obvious
- ✅ Fold option always clear
- ✅ Hand count visible (resource tracking)

**Feel:**
- ✅ Raises create genuine pressure
- ✅ Decisions feel meaningful (not arbitrary)
- ✅ Folding feels strategic (not punishing)
- ✅ Winning feels earned (not lucky)

### Performance

- ✅ < 16ms per frame (60fps maintained)
- ✅ No performance regression

### Code Quality

**Architecture:**
- ✅ Clean integration with existing betting system
- ✅ No breaking changes to core mechanics
- ✅ Discard logic isolated and testable

**Testing:**
- ✅ 8-10 new unit tests
- ✅ No regressions (all 61 existing tests pass)
- ✅ Edge cases covered

**Code Size:**
- ✅ Target +200-300 lines
- ✅ Total under 5,000 lines

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
