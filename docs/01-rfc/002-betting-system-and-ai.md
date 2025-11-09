# RFC-002: Betting System and AI Opponents

## Status

**Approved** - 2025-11-09 (Ready for SOW-002 Creation)

**Depends On:** ✅ RFC-001-revised (SOW-001 completed and merged)

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

**Analyst:** ARCHITECT Role
**Date:** 2025-11-09
**Based On:** SOW-001 completion (foundation validated)

### Technical Assessment

**Verdict:** ✅ **FEASIBLE** (15-18 hours estimated)

**Foundation Ready from SOW-001:**
- ✅ Card data model supports all new card types (Deal Modifiers are just new CardType variants)
- ✅ State machine extensible (can insert BettingPhase between states)
- ✅ Totals calculation handles multiplicative modifiers (simple extension)
- ✅ UI framework established (can add betting buttons, initiative indicator)
- ✅ HandState component tracks game state (can add betting rounds, initiative)

**New Systems Required:**

**1. Betting Phase System (~4-5 hours)**
- Add `BettingPhase` state (Check/Raise/Fold actions)
- Track initiative (first raiser)
- Track raise count (max 3 per round)
- Handle fold logic (remove player from round, preserve cards)
- All-in detection (last card played ends betting for that player)

**Implementation Approach:**
- Extend `State` enum: Insert `BettingPhase` after `Draw`
- Add `BettingState` component: `{ initiative: Option<Owner>, raise_count: u8, folded_players: Vec<Owner> }`
- Add betting UI buttons: Check, Raise (shows hand), Fold
- Reuse existing `card_click_system` for raise card selection

**2. Multi-Round Loop (~3-4 hours)**
- Extend state machine: `Draw → Betting → Flip → [Decision] × 3 → Resolve`
- Add round counter tracking (1/3, 2/3, 3/3)
- Add decision point UI ("Continue or Fold?" prompt between rounds)
- Reset betting state per round (initiative, raise count)

**Implementation Approach:**
- Add `round: u8` field to `HandState`
- Add `DecisionPoint` state (between Flip and next Draw)
- Loop rounds: After `Flip`, check `round < 3` → `DecisionPoint` or `Resolve`

**3. Basic AI (~4-5 hours)**
- **Narc AI:** Random card selection weighted by round (60%/40% → 80%/20%)
- **Customer AI:** Check until Round 3, then 70% Raise / 30% Fold (if Evidence > 60)
- Both use static decks (no learning, just heuristics)

**Implementation Approach:**
- Add `ai_decision_system` (runs during `BettingPhase` for Narc/Customer turns)
- Use `rand` crate for weighted randomness
- Simple if/else logic (no ML, no minimax)

**4. Card Collection Expansion (~2-3 hours)**
- Add 7 new cards (4 Deal Modifiers + 3 Evidence/Cover variants)
- Implement multiplicative price calculation (base_price × modifier)
- Update `create_narc_deck`, `create_customer_deck`, `create_player_deck`

**Implementation Approach:**
- Add `DealModifier` card type: `{ price_multiplier: f32, evidence: i32, heat: i32 }`
- Modify `calculate_totals`: Check for modifiers, apply to Product price
- Hardcode new cards (RON files deferred to future iteration)

**5. UI Improvements (~2-3 hours)**
- Turn indicator (highlight active player)
- Initiative indicator ("Has Initiative" badge)
- Raise counter ("2 raises left")
- Fold button (always visible during betting)
- Round counter ("Round 2/3")
- Decision point prompt ("Continue or Fold?")

**Implementation Approach:**
- Extend `ui_update_system` (add new UI components)
- Add `BettingUIRoot` component (betting-specific UI)
- Reuse color-coding from SOW-001 (green = your turn, etc.)

**Total Estimate:** 15-18 hours (fits within ≤20 hour SOW constraint)

---

### System Integration

**Integration Points:**

**From SOW-001 (Reuse):**
1. **Card Model** → Extend with new card types (trivial, just add enum variants)
2. **State Machine** → Insert betting phase (extensibility point designed for this)
3. **Totals Calculation** → Add multiplier support (pure function, easy to extend)
4. **UI Framework** → Add betting buttons (Bevy UI already set up)
5. **HandState** → Add betting fields (fields already public for UI access)

**New Dependencies:**
- `rand` crate for AI randomness (already in ecosystem, well-maintained)

**Data Flow:**
```
Draw → [Narc Betting] → [Customer Betting] → [Player Betting] → Flip
  ↓                                                                 ↓
Round++                                                      Show Cards
  ↓                                                                 ↓
Round < 3? → [DecisionPoint] → Continue/Fold → Draw         Resolve (if Round 3)
```

**No Breaking Changes:**
- SOW-001 code untouched (all additions, no modifications)
- Tests remain valid (no regression risk)

---

### Alternatives Considered

**Alternative 1: Single-Round with Betting**
- **Pros:** Simpler (no multi-round loop)
- **Cons:** Doesn't validate escalation (core to game design)
- **Verdict:** ❌ Rejected - Misses fun validation goal

**Alternative 2: Smart AI (Minimax/Monte Carlo)**
- **Pros:** Better opposition, more realistic
- **Cons:** 10-15 extra hours, not needed for fun validation
- **Verdict:** ❌ Deferred - Random AI sufficient for MVP

**Alternative 3: RON Files for Card Data**
- **Pros:** Hot-reloadable, easier balancing
- **Cons:** 3-4 extra hours for minimal benefit (only 15 cards)
- **Verdict:** ⏸️ Deferred - Hardcode for now, add when 20+ cards (RFC-003)

**Alternative 4: Animated Card Flips**
- **Pros:** Juicier, more satisfying
- **Cons:** 5-6 extra hours, doesn't affect fun validation
- **Verdict:** ⏸️ Deferred - Focus on mechanics first

**Chosen Approach:**
- Multi-round with basic AI (random + simple heuristics)
- Hardcoded cards (15 cards manageable)
- Minimal UI (functional, not polished)
- **Rationale:** Fastest path to "is this fun?" answer

---

### Risks & Mitigation

**Technical Risks:**

**Risk 1: State Machine Complexity**
- **Concern:** Adding betting phase + multi-round loop complicates state machine
- **Mitigation:** SOW-001 designed for this (extensibility validated)
- **Likelihood:** Low
- **Impact:** Medium

**Risk 2: AI Too Dumb / Too Smart**
- **Concern:** Random AI creates no pressure OR overpowers player
- **Mitigation:** Weighted randomness tunable (adjust % per round), static decks prevent cheese
- **Likelihood:** Medium
- **Impact:** Medium (but easy to tweak)

**Risk 3: Betting UI Confusing**
- **Concern:** Player doesn't understand initiative, raise count, fold timing
- **Mitigation:** Clear indicators (turn highlight, initiative badge, raise counter)
- **Likelihood:** Medium
- **Impact:** High (game not fun if UI unclear)

**Risk 4: Performance Degradation**
- **Concern:** More systems → frame drops
- **Mitigation:** Bevy ECS scales well, no heavy computations per frame
- **Likelihood:** Very Low
- **Impact:** Low

**Risk 5: Not Fun**
- **Concern:** After implementation, game still boring
- **Mitigation:** This is THE validation (if not fun here, pivot/kill project)
- **Likelihood:** Unknown (this RFC's purpose is to find out)
- **Impact:** Critical (make or break)

**Process Risks:**

**Risk 6: Scope Creep**
- **Concern:** "Just one more feature" expands past 20 hours
- **Mitigation:** Strict MVP scope (defer animations, smart AI, RON files)
- **Likelihood:** Medium
- **Impact:** High (delays fun validation)

**Risk 7: Playtest Bias**
- **Concern:** Developer thinks it's fun, but it's not
- **Mitigation:** Immediate external playtest after implementation
- **Likelihood:** Medium
- **Impact:** High (false positive on fun)

---

### Recommendations

**Approve RFC-002 with conditions:**

✅ **Approve IF:**
1. Scope stays within 15-18 hours (no animations, no smart AI, no RON files)
2. Immediate playtest after implementation (external validation)
3. Acceptance criteria focus on "is this fun?" (not just "does it work?")

❌ **Reject IF:**
- Scope expands beyond 20 hours (must stay tight for fast validation)
- Player adds "nice-to-have" features (animations, polish) before fun validation

**Post-Implementation:**
- If fun → Proceed to RFC-003 (Insurance & Stakes)
- If not fun → Pivot (maybe single-round with different mechanics?) or Kill project

---

### Conclusion

**RFC-002 is technically feasible** and builds cleanly on SOW-001 foundation. Estimated 15-18 hours fits within ≤20 hour constraint. No major technical risks (state machine extensibility validated).

**Key Success Metric:** After implementation, answer "Is this game fun?" with confidence.

**This RFC is make-or-break for game concept.** If betting + AI + multi-round doesn't create engaging loop, no amount of polish (RFC-003) will save it.

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

**Status:** ✅ **APPROVED** - Ready for SOW-002 Creation

**Approvers:**
- ARCHITECT: ✅ Approved (15-18 hours estimated, technically feasible, builds cleanly on SOW-001)
- PLAYER: ✅ Approved (player need defined, fun validation goals clear, builds on RFC-001 foundation)

**Scope Constraint:** 15-18 hours (fits within ≤20 hour SOW limit)

**Dependencies:**
- RFC-001-revised: ✅ SOW-001 completed and merged (foundation validated)

**Conditions:**
1. Scope must stay within 15-18 hours (no scope creep)
2. Immediate playtest after implementation (external validation)
3. Focus on "is this fun?" (not just technical completion)

**Next Steps:**
1. ✅ **SOW-001 Complete** (foundation validated)
2. ✅ **ARCHITECT:** Feasibility analysis complete (15-18 hours estimated)
3. ➡️ **ARCHITECT:** Create SOW-002 (define phases, deliverables, acceptance criteria)
4. **DEVELOPER:** Implement per SOW-002
5. **Critical:** Playtest immediately after implementation (is it fun?)

**Date:** 2025-11-09
