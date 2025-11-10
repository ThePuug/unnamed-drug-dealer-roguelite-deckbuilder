# RFC-007: Meaningful Betting Decisions

## Status

**Draft** - 2025-11-10

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

*To be completed by ARCHITECT*

---

## Discussion

*To be populated during RFC iteration*

---

## Approval

**Status:** Draft

**Approvers:**
- PLAYER: [Pending]
- ARCHITECT: [Pending]

**Date:** 2025-11-10
