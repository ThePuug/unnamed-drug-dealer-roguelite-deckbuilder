# RFC-002: Betting System and AI Opponents

## Status

**Draft** - 2025-11-09

**Depends On:** RFC-001-revised (must complete SOW-001 first)

## Feature Request

### Player Need

From player perspective: **I need to feel the tension of betting rounds and opposition from AI players** - Does the 3-round structure create escalating stakes? Do Check/Raise/Fold decisions feel meaningful? Does the AI create pressure?

**Current Problem:**
After RFC-001-revised (technical validation):
- Manual play is boring (no opposition, no tension)
- Single round doesn't show pacing (no escalation across rounds)
- No betting mechanics (can't test Check/Raise/Fold decisions)
- Can't validate "is this game fun?" (just proved mechanics work)
- Don't know if 3 rounds is right pacing (too slow? too fast?)

**We need a system that:**
- Implements 3-round hand structure (draw → bet → flip × 3)
- Provides betting mechanics (Check, Raise, Fold with clear rules)
- Creates AI opponents that feel like opposition (not perfect, just pressure)
- Shows initiative system working (first to raise can raise again)
- Demonstrates fold decision-making (when is it right to bail?)
- Validates ~15 minute session length (3-5 hands achievable)

### Desired Experience

**This is FUN VALIDATION - the critical test.**

Players should experience:
- **Escalating tension** - Round 1 feels exploratory, Round 2 raises stakes, Round 3 is "all-in or fold?"
- **Meaningful opposition** - Narc feels threatening (plays Evidence), Customer feels unpredictable (good/bad deals)
- **Decision weight** - Fold early (preserve cards) vs. push through (bank profit)?
- **Initiative advantage** - First to raise controls pacing (can re-raise after all call)
- **Relief or regret** - Made it through Round 3 = "Yes!" / Folded Round 2 = "Should I have stayed?"
- **Replayability** - "One more hand" feeling (want to try different strategy)

**If this RFC doesn't feel fun, the game concept is questionable.**

### Specification Requirements

**3-Round Hand Structure:**
- Each hand consists of 3 rounds
- Each round: Draw phase → Betting phase → Flip phase → Decision point
- Between rounds: Can fold (exit hand) or continue (draw to 3 cards)
- After Round 3: Resolve (calculate totals, check bust)

**Betting Mechanics:**
- Turn order: Narc → Customer → Player (player always goes last)
- Actions available: Check (stay in, no card), Raise (play card face-down), Fold (exit hand)
- Initiative: First player to raise gains initiative
- Re-raise rule: After all players call, player with initiative can raise again
- Raise limit: Maximum 3 raises per round (prevents infinite loops)
- All-in: Playing last card ends betting immediately for that player

**AI Opponents (Basic Implementation):**

**Narc AI:**
- Static deck: 10× Donut Break (0 Evidence, 0 Heat), 3× Patrol (+5 Evidence, +2 Heat), 2× Surveillance (+20 Evidence, +5 Heat)
- Simple strategy:
  - Round 1: 60% Check, 40% Raise (play random card)
  - Round 2: 40% Check, 60% Raise (play random card)
  - Round 3: 20% Check, 80% Raise (play random card)
  - Never folds (Narc always investigates)
- Goal: Apply pressure (not be smart, just be consistent threat)

**Customer AI:**
- Static deck: 5× Regular Order (×1.0 price, +10 Evidence, +10 Heat), 5× Haggling (-$30, +5 Evidence), 3× Bulk Order (×1.5 price, +25 Evidence, +20 Heat), 2× Fair Deal (+$0, +0 Evidence, +0 Heat)
- Simple strategy:
  - Round 1-2: Always Check (waits for player to commit)
  - Round 3: 70% Raise (play random card), 30% Fold (if Evidence > 60)
  - Fold if: Evidence > 60 (customer gets scared)
- Goal: Create unpredictability (sometimes good deal, sometimes bad)

**Player Feedback Additions:**
- Turn indicator: Highlight whose turn it is
- Initiative indicator: Show who has initiative (can re-raise)
- Raises remaining: Display "3 raises left" → "2 raises left" → etc.
- Fold option: Button always visible (can fold anytime during betting)
- Round counter: "Round 1/3" display
- Decision point: After each round flip, "Continue or Fold?" prompt

**Card Collection Expansion:**
- Add Deal Modifiers (3 types):
  - Player: Disguise (+20 Cover, -5 Heat), Lookout (+15 Cover, 0 Heat)
  - Narc: Heat Wave (+15 Evidence, +10 Heat)
  - Customer: Bulk Sale Pressure (×1.3 price, +10 Evidence)
- Total cards: 15 (8 from RFC-001 + 4 Deal Modifiers + 3 more Evidence/Cover variants)

**Multiplicative Modifiers:**
- Implement Price multipliers (×1.0, ×1.3, ×1.5 from Customer cards)
- Apply to Product base price
- Display calculation clearly ("$100 × 1.5 = $150")

### MVP Scope

**Phase 1 includes:**
- 3-round hand loop (draw → bet → flip × 3)
- Betting phase (Check/Raise/Fold actions)
- Initiative tracking (first to raise, can re-raise)
- Raise counter (max 3 per round)
- Fold decision point (between rounds)
- Basic AI for Narc (pressure AI)
- Basic AI for Customer (unpredictability AI)
- Card collection expansion (15 cards total)
- Deal Modifiers (4 types)
- Multiplicative price calculation
- UI improvements (turn indicators, initiative, fold button)
- Session play (3-5 hands until deck exhausted)

**Phase 1 excludes:**
- Insurance cards (Get Out of Jail) (RFC-003)
- Conviction cards (Make It Stick) (RFC-003)
- Deck building UI (can hardcode player deck for now)
- Smart AI (just random + basic heuristics)
- Heat decay (no persistence yet)
- Trust system (Customer deck doesn't scale yet)
- Character profiles/permadeath (RFC-003)
- Animation/polish (RFC-003)

### Priority Justification

**HIGH PRIORITY** - This is the fun validation (make or break for game concept)

**Why HIGH:**
- Must validate core loop is engaging (if not fun, pivot or kill project)
- Betting mechanics are the core innovation (poker-like drug dealing)
- 3-round structure is untested (need to feel the pacing)
- AI opposition critical for tension (manual play proved nothing about fun)
- Cheapest time to discover "this isn't fun" (before meta-systems)

**Benefits:**
- **Fun validation** - Can answer "is this game concept viable?"
- **Pacing validation** - Confirm 3 rounds create escalation
- **Betting validation** - Check/Raise/Fold feel meaningful?
- **AI validation** - Basic AI creates enough pressure?
- **Session length validation** - Can play 3-5 hands in ~15 minutes?

---

## Feasibility Analysis

*[ARCHITECT to complete]*

### Technical Assessment

*To be added by ARCHITECT*

### System Integration

*To be added by ARCHITECT*

### Alternatives Considered

*To be added by ARCHITECT*

---

## Discussion

### PLAYER Notes

**What "Fun" Means Here:**

**Tension Escalation:**
- Round 1: Exploring (what's the Narc playing? what's the Customer offering?)
- Round 2: Commitment (Evidence climbing, do I stay in?)
- Round 3: Climax (all-in or fold? this is the moment)

**If I don't feel this arc, the game isn't working.**

**AI Quality Expectations:**
- NOT expecting chess-level AI (that's overkill)
- Expecting: Narc feels like a threat (plays Evidence consistently)
- Expecting: Customer feels unpredictable (sometimes good deal, sometimes bad)
- Acceptable: AI is dumb but creates pressure

**Fold Decision Clarity:**
- Need clear feedback: "If you fold now: Keep +20 Heat, lose 4 cards, no profit"
- Need clear feedback: "If you continue: Need 30 Cover to stay safe"
- Should feel informed, not guessing

**Betting Mechanics:**
- Initiative matters: First to raise controls pacing (can re-raise)
- Raises cap matters: Max 3 prevents infinite stalling
- All-in matters: Playing last card forces hand resolution

**3 Rounds Pacing:**
- Concerned: Is 3 rounds too slow? (9 player decisions per hand)
- Concerned: Is 3 rounds too fast? (not enough escalation)
- Need to playtest: Feel the pacing, adjust if needed

**15 Cards Enough?**
- 15 cards allows ~3 hands per session (5 cards per hand)
- Should demonstrate variety (Products, Locations, Modifiers)
- Full 20-card collection in RFC-003 (adds insurance/conviction)

### PLAYER Validation

**What makes this fun?**
1. **Opponent pressure** - Narc plays Surveillance, I need to respond (Cover or fold?)
2. **Betting control** - I raise first, gain initiative, can re-raise after they call
3. **Fold timing** - Evidence 50 after Round 2, fold now or push to Round 3?
4. **Customer unpredictability** - Bulk Order (great profit) or Haggling (terrible deal)?
5. **Escalation arc** - Round 3 feels climactic (all cards on table, bust or profit)

**What could go wrong?**
- **AI too dumb** - Narc always plays Donut Break (no threat)?
  - *Mitigation:* Static deck prevents "always Donut Break", 3× Patrol + 2× Surveillance guaranteed
- **Betting too slow** - 3 rounds × 3 turns = 9 decisions per hand feels tedious?
  - *Mitigation:* Fast animations, auto-pass if Check, measure actual time per hand
- **Fold feels bad** - Losing cards to fold creates negative emotion?
  - *Mitigation:* Clear feedback on what you preserve (Heat, remaining cards), frame as strategic retreat
- **No tension** - Rounds feel same (no escalation)?
  - *Mitigation:* Narc AI ramps aggression (60% Round 1 → 80% Round 3), Evidence climbs naturally

---

## Approval

**Status:** Awaiting ARCHITECT feasibility analysis

**Approvers:**
- ARCHITECT: [Pending]
- PLAYER: ✅ Approved (player need defined, fun validation goals clear, builds on RFC-001 foundation)

**Scope Constraint:** [ARCHITECT to estimate - target ≤20 hours]

**Dependencies:**
- RFC-001-revised: Must complete SOW-001 first (card model, state machine, UI foundation)

**Next Steps:**
1. **Wait for RFC-001-revised completion** (SOW-001 implementation, 12-16 hours)
2. **ARCHITECT:** Add feasibility analysis (effort estimate, integration points)
3. **If Approved:** ARCHITECT creates SOW-002
4. **DEVELOPER:** Implements per SOW-002
5. **Critical:** Playtest immediately after implementation (is it fun?)

**Date:** 2025-11-09
