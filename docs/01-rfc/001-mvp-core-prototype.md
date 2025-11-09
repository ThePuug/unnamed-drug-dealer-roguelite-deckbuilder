# RFC-001: MVP Core Prototype

## Status

**✅ Consensus Reached - Split Approved** - 2025-11-09

**Decision:** Original scope too large (40-55 hours). Splitting into 3 tight RFCs (12-16h + 15-18h + 14-18h).

**This RFC is SUPERSEDED by:**
- RFC-001-revised: Minimal Playable Hand (to be created)
- RFC-002: Betting System + AI (to be created)
- RFC-003: Insurance & Complete Cards (to be created)

## Feature Request

### Player Need

From player perspective: **I need a playable prototype that demonstrates the core push-your-luck drug dealing gameplay loop** - I want to experience the tension of "stay in or fold?" decisions without getting bogged down in meta-progression or complex systems.

**Current Problem:**
Without a playable prototype:
- Can't validate if the core loop is fun (is the decision-making engaging?)
- Can't test if Evidence/Cover/Heat/Profit values are balanced
- Can't feel the tension of betting rounds (does 3 rounds create good pacing?)
- Can't judge if card interactions are clear (override rules understandable?)
- Can't assess session length (is 15 minutes achievable?)

**We need a system that:**
- Implements the complete hand structure (3 rounds of betting per hand)
- Provides meaningful cards (enough variety for strategic decisions)
- Creates risk/reward tension (Evidence climbing, fold or push?)
- Demonstrates bust mechanics (Evidence > Cover = game over)
- Shows Heat accumulation (consequences visible)
- Feels complete for one session (3-5 hands playable)

### Desired Experience

Players should experience:

- **Immediate engagement** - Draw cards, make decisions, see consequences within 30 seconds
- **Tension escalation** - Round 1 feels exploratory, Round 2 raises stakes, Round 3 is commitment
- **Clear feedback** - Always know: Am I safe? Am I at risk? Should I fold?
- **Meaningful choices** - Which card to play matters (not random)
- **Fair consequences** - Getting busted feels like "I chose that risk" not "I got screwed"
- **Replayability** - Want to play "one more hand" after bust

### Specification Requirements

**Hand Structure:**
- 3 rounds of betting per hand
- Turn order: Narc AI → Customer AI → Player (you always go last)
- Actions per turn: Check (pass), Raise (play card), Fold (exit hand)
- Initiative system (first to raise can raise again after all call)
- Max 3 raises per round (prevents infinite loops)
- Cards flip and resolve after betting closes
- Draw back to 3 cards between rounds

**Card Collection (20 cards minimum):**
- 5 Products: Weed ($30, +5 Heat), Pills ($60, +15 Heat), Meth ($100, +30 Heat), Heroin ($150, +45 Heat), + 1 variant
- 3 Locations: Safe House (10 Evidence, 30 Cover, -5 Heat), Parking Lot (25 Evidence, 15 Cover, 0 Heat), School Zone (40 Evidence, 5 Cover, +20 Heat)
- 4 Deal Modifiers: 2 Player (Disguise, Lookout), 1 Narc (Surveillance), 1 Customer (Bulk Order)
- 4 Evidence cards: Patrol (+5), Surveillance (+20), Wiretap (+30), + 1 more
- 4 Cover cards: Alibi (+30, -5 Heat), Lawyer Up (+40, 0 Heat), Lay Low (+10, -15 Heat), + 1 more
- 2 Get Out of Jail: Plea Bargain (+20 Cover, $1k cost, +20 Heat), Fake ID (+15 Cover, $0 cost, +40 Heat)
- 2 Make It Stick: Warrant (threshold 40), DA Approval (threshold 60)

**Bust Mechanics:**
- Evidence > Cover check at hand end
- Simple bust (run ends, no restart yet)
- Insurance activation (Get Out of Jail works if can afford)
- Conviction override (Make It Stick overrides insurance if Heat >= threshold)

**Heat System (accumulation only):**
- Sum Heat modifiers from cards played
- Apply at hand end (if not busted)
- Display current Heat level
- NO decay yet (Phase 2 feature)

**AI Opponents:**
- Narc: Static deck (10× Donut Break, 3× Patrol, 2× Surveillance)
- Customer: Static deck (5× Regular Order, 5× Haggling, 3× Bulk Order, 2× Fair Deal)
- Simple AI behavior (random play from hand, 50% fold chance if Evidence > 50)

**Player Feedback:**
- Running totals visible: Evidence, Cover, Heat delta, Profit
- Color-coded safety: Green (safe), Yellow (close), Red (busted)
- Card display: All cards on table, grouped by player
- Turn indicator: Whose turn? Can I act?
- Initiative indicator: Who has initiative?

### MVP Scope

**Phase 1 includes:**
- Complete hand loop (setup → 3 rounds → resolution)
- 15-card player deck (build before session)
- Basic AI opponents (Narc, Customer with static decks)
- All 7 card types (Product, Location, Deal Mod, Evidence, Cover, Insurance, Conviction)
- Card interaction rules (override, additive, multiplicative)
- Bust mechanics (Evidence > Cover, insurance, conviction)
- Heat accumulation (no decay)
- Visual feedback (running totals, color-coding)
- Single session play (3-5 hands until deck exhausted)
- Game over on bust (no character persistence yet)

**Phase 1 excludes:**
- Heat decay over real-time (Phase 2)
- Trust system (Phase 2)
- Narc/Customer deck scaling (Phase 2)
- Character profiles/permadeath (Phase 2)
- Card unlocks/progression (Phase 3)
- Leaderboards (Phase 3)
- Save/load (Phase 2)
- Tutorial/onboarding (Phase 3)
- Audio/animations (Phase 3)

### Priority Justification

**HIGH PRIORITY** - This is the foundation for the entire game

**Why HIGH:**
- Nothing else can be built without the core loop working
- Must validate fun factor early (pivot if not engaging)
- Cheapest time to iterate on balance (before meta-systems)
- De-risks the entire project (core loop viability)

**Benefits:**
- **Playtest immediately** - Can validate decision-making tension
- **Balance tuning** - Adjust Evidence/Cover/Heat values based on feel
- **Technical validation** - Verify Bevy/Rust can handle card game mechanics
- **Scope validation** - Confirm 15-minute sessions are achievable
- **Foundation for Phase 2** - All systems hook into this core loop

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Bevy ECS + Card Game Architecture**

#### Scope Concern: TOO LARGE FOR ONE RFC

**Effort Estimate:**
- Card data model (7 types) - 4-6 hours
- Hand/Round/Betting structure - 6-8 hours
- Initiative + raise counter + turn order - 3-4 hours
- Card interaction engine (override, additive, multiplicative) - 5-7 hours
- Bust resolution (Evidence > Cover + insurance + conviction) - 4-6 hours
- Heat accumulation - 2-3 hours
- AI opponents (2 decks + behavior) - 5-7 hours
- Visual feedback systems - 6-8 hours
- Deck building UI - 4-6 hours

**Total: 39-55 hours**

**❌ FAILS SOW constraint (≤20 hours)**

#### Recommendation: SPLIT INTO 3 TIGHT RFCs

**RFC-001: Minimal Playable Hand** (12-16 hours)
- **Goal:** Validate core decision-making (is "stay in or fold" fun?)
- Single round (not 3) - betting later
- Manual play both sides (no AI yet) - just validate card interactions
- 8 cards only: 3 Products, 2 Locations, 2 Evidence, 1 Cover
- Basic interactions: Override (Products/Locations), Additive (Evidence/Cover)
- Simple bust: Evidence > Cover = game over (no insurance/conviction)
- Heat display only (accumulate but no mechanics)
- Minimal UI: Card play area + running totals

**RFC-002: Betting System + AI** (15-18 hours)
- **Goal:** Add poker-like tension with 3 rounds and opponents
- 3-round structure (draw → bet → flip × 3)
- Check/Raise/Fold mechanics
- Initiative + max 3 raises
- Basic AI opponents (Narc + Customer, static decks, simple behavior)
- Expand to 15 cards (add Deal Modifiers)
- UI improvements (turn indicators, initiative)

**RFC-003: Insurance & Complete Cards** (14-18 hours)
- **Goal:** Add high-stakes consequences with insurance/conviction
- Get Out of Jail cards (2 types)
- Make It Stick cards (2 types)
- Insurance activation + requirements
- Conviction override mechanics
- Remaining card types
- Polish UI (color-coding, warnings)

#### Why This Breakdown?

**Tight validation loops:**
- RFC-001: "Are card choices meaningful?" (core loop)
- RFC-002: "Does betting create tension?" (pacing)
- RFC-003: "Does insurance feel clutch?" (stakes)

**Each RFC stands alone:**
- RFC-001 is playable (manual play, simple bust)
- RFC-002 adds betting (still fun without insurance)
- RFC-003 adds consequences (insurance matters at high Heat)

**Learning between RFCs:**
- Balance card values in RFC-001 before expanding collection
- Tune AI difficulty in RFC-002 before adding insurance complexity
- Validate bust frequency before adding insurance escape

### System Integration

**Core Systems Required (RFC-001):**
- Card entity/component model (ECS)
- Hand state machine (setup → play → resolve)
- Card interaction engine (override, additive)
- UI rendering (Bevy UI or egui)

**Dependencies:**
- None (foundational)

**Integration Points:**
- RFC-002 hooks into hand state machine (add betting phase)
- RFC-003 hooks into bust resolution (add insurance checks)

### Alternatives Considered

#### Alternative 1: Full MVP in One RFC
**Pros:** Everything playable immediately
**Cons:** 40-55 hours (2-3 SOWs), high risk, hard to validate incrementally
**Rejected:** Violates ≤20 hour constraint, too much risk

#### Alternative 2: Card Types First, Then Mechanics
**Approach:** RFC-001 = implement all 7 card types, RFC-002 = add betting
**Pros:** Complete card system upfront
**Cons:** Can't validate fun without betting, card balance unknown
**Rejected:** No validation loop, premature card implementation

#### Alternative 3: Recommended Approach (3 Tight RFCs)
**Approach:** Minimal playable → Betting → Insurance
**Pros:** Tight validation loops, incremental risk, ≤20 hours each
**Cons:** Slower to "complete game"
**Selected:** Best balance of validation and scope control

---

## Discussion

### ARCHITECT Notes

**Scope Violation:**
Original RFC-001 is 40-55 hours of work - well over ≤20 hour SOW limit. This isn't an RFC, it's the entire game.

**Proposed Split:**
- **RFC-001-revised:** Minimal Playable Hand (12-16 hours)
- **RFC-002:** Betting System + AI (15-18 hours)
- **RFC-003:** Insurance & Complete Cards (14-18 hours)

**Critical Question for PLAYER:**
Can we validate the core loop with:
- Single round (not 3)?
- Manual play (no AI yet)?
- 8 cards (not 20)?

If YES → Approve revised RFC-001 scope
If NO → Explain what minimum is needed for validation

**Technical Risks (Even with Split):**

**1. Card Interaction Complexity**
- *Risk:* Override + additive + multiplicative rules are complex
- *Mitigation:* Start with override + additive only (RFC-001), add multiplicative later (RFC-003)
- *Impact:* Medium (core mechanic)

**2. Bevy UI Performance**
- *Risk:* Bevy UI may be too low-level for card game
- *Mitigation:* Prototype with simple rectangles + text first, consider egui if Bevy UI painful
- *Impact:* Low (can swap UI library)

**3. Card Balance Unknown**
- *Risk:* Evidence/Cover values may be way off
- *Mitigation:* RFC-001 with 8 cards allows fast iteration before expanding
- *Impact:* Low (expected, part of validation)

**4. Manual Play Boring?**
- *Risk:* Playing both sides yourself isn't fun
- *Mitigation:* Keep RFC-001 short (12-16 hours), add AI in RFC-002
- *Impact:* Medium (but cheap to pivot)

### PLAYER Notes

**Session Length Target:**
- 3-5 hands per deck
- ~3-4 minutes per hand (1 min per round × 3 + 1 min resolution)
- Total: 10-20 minutes per session
- Acceptable variance for MVP (validate in playtesting)

**AI Simplicity:**
- MVP AI can be dumb (50% random, 50% reactive)
- Goal: Make PLAYER decisions feel meaningful, not challenge AI skill
- Improve AI in Phase 2 after core loop validated

**Bust "Game Over" Acceptable:**
- No character persistence yet = acceptable for MVP
- Players can restart immediately (build new deck, play again)
- Phase 2 will add Heat persistence and permadeath consequences

**Card Balance Expectations:**
- MVP cards likely unbalanced (that's okay!)
- Expect Evidence to climb too fast OR too slow
- Expect Heat to feel inconsequential (no decay yet)
- Tuning will happen during playtesting

**20 Cards Enough?**
- 20 cards × 15-card decks = limited variety
- But enough to demonstrate:
  - Product choices (Weed vs. Meth risk/reward)
  - Location swapping (School Zone → Safe House)
  - Insurance value (Plea Bargain vs. Fake ID)
  - Conviction threat (Warrant at Heat 40)
- Full collection (80-100 cards) in Phase 3

### PLAYER Validation

**What makes this fun?**
1. **Informed risk-taking** - You see Evidence climbing, decide to fold or push
2. **Counterplay moments** - Narc plays Surveillance, you respond with Cover
3. **Tension climax** - Round 3 decision: All-in or fold?
4. **Relief or regret** - Evidence 70 vs. Cover 75 = "Whew!" / Evidence 80 vs. Cover 75 = "Damn!"
5. **Learning curve** - Bust once, next game you play more Cover cards

**What could go wrong?**
- **Too much math** - Calculating Evidence/Cover/Heat/Profit overwhelming?
  - *Mitigation:* Auto-calculate, show clear totals
- **Pacing too slow** - 3 rounds × 3 turns = 9 decisions per hand?
  - *Mitigation:* Fast animations, auto-pass if no action
- **AI feels dumb** - Narc always plays Donut Break?
  - *Mitigation:* Static decks prevent "dumb draws", acceptable for MVP
- **Bust feels cheap** - "I had insurance but couldn't afford it"?
  - *Mitigation:* Show cost clearly, warn before bust

### PLAYER Response to ARCHITECT Scope Split

**✅ I accept the 3-RFC split with clarifications:**

**RFC-001 (Minimal Playable Hand) - Accepted with caveat:**
- Single round: ✅ Acceptable (validates mechanics, not fun)
- Manual play: ✅ Acceptable (technical validation only)
- 8 cards: ✅ Acceptable (enough for override + additive rules)

**BUT I acknowledge:**
- RFC-001 will NOT validate "is this game fun?"
- RFC-001 is a **technical prototype** - proves card interactions work
- Fun validation happens in RFC-002 (betting + AI)

**My requirements for accepting this split:**

1. **RFC-002 must start IMMEDIATELY after RFC-001**
   - Don't want to wait weeks between technical validation and fun validation
   - If RFC-001 takes 12-16 hours, RFC-002 should start within days

2. **RFC-001 must prove the CORE mechanic is sound:**
   - Override rules work (Product/Location replacement)
   - Additive rules work (Evidence/Cover stacking)
   - Evidence > Cover bust condition is clear
   - Card values are ROUGHLY balanced (can tune in RFC-002)

3. **RFC-002 is the REAL validation:**
   - This is where we answer "is the game fun?"
   - 3 rounds must create escalating tension
   - AI must feel like opposition (not chess-level, but not random)
   - Betting (Check/Raise/Fold) must feel meaningful

**What I'm validating in RFC-001:**
- ❌ NOT validating fun (manual play isn't fun)
- ❌ NOT validating pacing (single round doesn't show escalation)
- ✅ Validating card interaction clarity (do I understand what's happening?)
- ✅ Validating Evidence/Cover calculation (does the math feel right?)
- ✅ Validating Product/Location override (is swapping cards clear?)

**What I need from RFC-002:**
- ✅ Validate tension escalation (Round 1 → 2 → 3)
- ✅ Validate AI opposition (do I feel pressured?)
- ✅ Validate betting decisions (do Check/Raise/Fold matter?)
- ✅ Validate fold timing (when is the right time to bail?)

**Compromise I'm willing to make:**
- RFC-001 can be "not fun" if it's QUICK (12-16 hours = 1-2 days of work)
- RFC-002 must happen FAST after (don't leave me hanging with a boring prototype)
- By end of RFC-002, I need to feel "yes, this core loop has potential"

**Question for ARCHITECT:**
Can we overlap work? E.g., once RFC-001 proves card interactions work, can DEVELOPER start RFC-002 while we're still tuning card values in RFC-001?

---

## Approval

**Status:** ✅ **CONSENSUS - Split Approved**

**Approvers:**
- ARCHITECT: ✅ **Approves split approach** (3 tight RFCs, each ≤20 hours)
- PLAYER: ✅ **Accepts split with conditions** (RFC-002 must follow quickly, RFC-001 is technical validation only)

**Scope Constraint:** Original: 40-55 hours ❌ | Approved split: 12-16 + 15-18 + 14-18 hours ✅

**Consensus:**
Split RFC-001 into 3 sequential RFCs:
1. **RFC-001-revised:** Minimal Playable Hand (12-16 hours) - Technical validation: 8 cards, single round, manual play, override + additive rules
2. **RFC-002:** Betting System + AI (15-18 hours) - Fun validation: 3 rounds, Check/Raise/Fold, AI opponents, 15 cards
3. **RFC-003:** Insurance & Complete Cards (14-18 hours) - Stakes validation: Get Out of Jail, Make It Stick, 20 cards, polish

**PLAYER Conditions:**
- RFC-001 is NOT fun validation (technical prototype only)
- RFC-002 must start within days of RFC-001 completion
- Fun validation happens in RFC-002 (betting + AI = core loop validation)
- Overlap work if possible (start RFC-002 while tuning RFC-001)

**Next Steps:**
1. ✅ **DONE:** Original RFC-001 identified as too large, split proposed
2. ✅ **DONE:** PLAYER accepted split with conditions
3. **NEXT:** Close original RFC-001 (scope too large, superseded by split)
4. **NEXT:** ARCHITECT creates RFC-001-revised with tight scope (12-16 hours)
5. **NEXT:** PLAYER validates RFC-001-revised meets technical validation goals
6. **THEN:** Approve RFC-001-revised → ARCHITECT creates SOW-001 → DEVELOPER implements

**Date:** 2025-11-09
