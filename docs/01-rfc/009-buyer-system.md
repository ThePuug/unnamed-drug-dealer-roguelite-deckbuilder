# RFC-009: Buyer System (Merged Dealer + Customer)

## Status

**Draft** - 2025-11-10

**Supersedes:** RFC-006 (3-player structure with separate Dealer and Customer)

**Depends On:** RFC-006 (sequential play, progressive reveals)

## Feature Request

### Player Need

From player perspective: **I need the deal to feel central to gameplay, not background noise. The person I'm selling to should matter - who they are, what they want, how they react.**

**Current Problem:**
After RFC-006 implementation:
- Customer feels like just another player (small modifiers, forgettable)
- Dealer feels passive (just reveals cards, no personality)
- 3-player turn structure complex (Narc → Customer → Player rotation)
- Customer and Dealer both weak individually
- No clear "who am I dealing with?" identity

**We need a system that:**
- Merges Dealer + Customer into single "Buyer" entity
- Gives Buyers distinct personalities (different demands, reactions, risk profiles)
- Creates "is this Buyer worth the risk?" decisions
- Simplifies turn structure (2 active players: Narc vs Player)
- Allows easy expansion (new Buyer personas = new content)
- Fits into progression/variety systems

### Desired Experience

Players should experience:
- **Buyer identity** - "I'm dealing with College Party Host, they want volume"
- **Risk assessment** - "Mom is paranoid (Heat < 30), Party Host has high Evidence locations"
- **Readable threat** - See Buyer's 3 reaction cards face-up, anticipate danger
- **Push-your-luck meta** - "I succeeded once, do I risk another Buyer or lay low?"
- **Deck building strategy** - Build deck optimized for specific Buyer demands
- **Session variety** - Different Buyers create different challenges

### Specification Requirements

**Buyer Entity (Merged Dealer + Customer):**

Buyer has 3 components:

**1. Setup (Revealed Before Deck Building)**
```
Buyer Persona: "College Party Host"
  
  Demand: Schedule 3 drugs (Weed or Pills), high volume
  Preferred Location: Public (Dorm, Party, Park)
  Profit Multiplier: ×2.5 (high because volume)
  Special Rules: +10 Evidence if public Location used
  Heat Threshold: None (not paranoid)
```

**Setup determines:**
- Product demand (which Products meet demand)
- Location preference (affects multiplier or adds conditions)
- Profit multiplier (×1.2 to ×3.0 range)
- Threshold triggers (Heat, Evidence, etc.)
- Special rules (conditional effects)

**2. Reaction Deck (7 Cards)**

Each Buyer persona has unique 7-card deck representing their behavior:
```
"College Party Host" Reaction Deck:
  - "Invite More People" (+15 Evidence, +10 Heat)
  - "VIP Room" (-10 Evidence, +$50)
  - "Cops Called" (+20 Evidence)
  - "Cash Bar" (+$30)
  - "Party Shutdown" (if Evidence > 60, ×0.5 multiplier)
  - "Word of Mouth" (+20 Heat, next Buyer ×1.2)
  - "Last Call" (+10 Evidence)
```

**3. Reaction Hand (3 Cards Visible)**

At hand start:
- Buyer draws 3 random cards from their 7-card deck (face-up)
- All players see these 3 cards
- Each round, Buyer plays 1 random card from hand
- Creates anticipation ("They might play Cops Called this round...")

**Turn Structure (Simplified to 2 Players):**

**Each round:**
```
1. Narc plays 1 card face-up
2. Player plays 1 card face-up
3. Buyer plays 1 random card from visible hand
4. Totals update
5. Player fold decision (Rounds 1-2 only)
```

**No turn order rotation** - always Narc → Player → Buyer reveal

**Product + Location Requirement:**

**New hard requirement:** Must play at least 1 Product AND 1 Location to complete valid deal.

**Resolution checks:**
```
1. Validity: active_product.exists() && active_location.exists()
   - If false: Deal invalid, $0 profit
   - Buyer bail also results in $0 profit

2. Demand: Does Product/Location meet Buyer's demand?
   - If yes: Apply full multiplier (×2.5)
   - If no: Apply reduced multiplier (×1.0)

3. Threshold: Did any threshold trigger? (Heat, Evidence, etc.)
   - If yes: Buyer bails (deal invalid, $0 profit, no bust check)
   - If no: Continue to bust check

4. Bust: Evidence > Cover?
   - If yes and no insurance: Run ends
   - If no: Bank profit, continue
```

**Buyer Bail Mechanic:**

When Buyer threshold exceeded (e.g., Heat > 30 for Mom):
- Deal invalid ($0 profit)
- No bust check (Narc can't bust deal that didn't happen)
- Hand ends immediately
- **Try again with same Buyer** (deck persists, Heat persists)
- This is NOT session end, just hand restart

**Session Structure (Push-Your-Luck Meta):**

**Phase 1: Pick Buyer**
- See Buyer setup (demands, multiplier, thresholds)
- Know what you're dealing with before building deck

**Phase 2: Build Deck**
- 15 cards optimized for this Buyer
- Consider: What Products do they want? Can I meet demands?

**Phase 3: Play Until First Success**
- Play hands until:
  - SUCCESS: Valid deal completed (bank profit)
  - BUST: Evidence > Cover (run ends, permadeath)
  - EXHAUSTION: Deck empty, no cards left (forced decision)

**Phase 4: Decision Point (After Success)**
- **Continue:** Pick new Buyer → Build new deck → Play again (Heat carries over)
- **Lay Low:** End session → Heat decays over real-world time

**Heat is cumulative across Buyers within session:**
- First Buyer at Heat 10: Easy
- Second Buyer at Heat 50: Harder
- Third Buyer at Heat 90: Extremely risky

**Fold Consequence:**

When player folds:
- Discard 1 random card from deck (removed from remaining deck for this session)
- Card goes to discard pile, cannot be used in future hands with this Buyer
- Deck count decreases (15 → 14 → 13...), eventually forcing "go home" when depleted
- Keep Heat accumulated so far
- No profit
- Try next hand with remaining cards

**Clarification:** Folded cards are NOT permanently lost from collection, just removed from current session's available deck. This creates deck depletion pressure without permanent loss.

**Buyer Personas (Launch Content):**

**Minimum 4 personas for launch, target 5-7:**

**1. "College Party Host"**
- Wants: Schedule 3 (Weed, Pills), volume
- Locations: Public (Dorm, Party, Park) - adds Evidence
- Multiplier: ×2.5 (high profit)
- Threshold: None
- Risk: High Evidence from locations

**2. "Stay-at-Home Mom"**
- Wants: Schedule 2 (Pills), small order
- Locations: Private only (Residence, Warehouse)
- Multiplier: ×1.2 (low profit)
- Threshold: Heat < 30 (paranoid, will bail)
- Risk: Low profit, might bail if sloppy

**3. "Junkie"**
- Wants: Schedule 1 (Meth, Heroin), any amount
- Locations: Any (desperate)
- Multiplier: ×0.8 (broke, very low profit)
- Threshold: None
- Risk: Unpredictable reactions, low profit

**4. "Schoolyard Punk"**
- Wants: Cheap drugs (Weed), low volume
- Locations: Public (School Zone, Park)
- Multiplier: ×1.5
- Threshold: None
- Risk: Might snitch (special reaction cards)

**5. "Executive"**
- Wants: Schedule 2 (Pills), premium quality
- Locations: Private only
- Multiplier: ×2.8 (very high profit)
- Threshold: Heat < 25 (very paranoid)
- Risk: Will bail easily, but huge profit if successful

**6. "Night Club Owner"**
- Wants: Volume, any Product
- Locations: Club only
- Multiplier: ×2.2
- Threshold: Evidence < 60 (will bail if too hot)
- Risk: High Evidence tolerance but has limits

**7. "Tourist"**
- Wants: "Authentic experience", any Product
- Locations: Risky (School Zone, Public)
- Multiplier: ×2.0
- Threshold: None (naive, won't bail)
- Risk: Naive reactions add Evidence

**Each persona has:**
- 2 setup variants (different demands/conditions) = 8-14 total encounters
- 7-card reaction deck = 28-49 total Buyer reaction cards

### MVP Scope

**Phase 1 includes:**
- Buyer entity system (replaces Dealer + Customer)
- 4-5 Buyer personas (8-10 encounter variants)
- Buyer reaction decks (7 cards × 4-5 personas = 28-35 cards)
- Visible 3-card hand (drawn at hand start, face-up)
- Random play from hand (1 card per round)
- Product + Location requirement (validity check)
- Buyer bail mechanic (threshold exceeded = deal invalid, no bust)
- Session structure (Buyer → Build → Play → Decision)
- Heat accumulation across Buyers
- 2-player turn structure (Narc → Player, no rotation)

**Phase 1 excludes:**
- More than 5 Buyer personas (can add post-launch)
- Complex reaction card effects (keep simple for MVP)
- Buyer reputation/relationship systems (defer to Phase 2)
- Heat decay system (defer to Phase 2)
- Multi-run progression (defer to Phase 2)

### Priority Justification

**HIGH PRIORITY** - Simplifies structure, increases strategic depth, enables content expansion

**Why HIGH:**
- Fixes weak Customer/Dealer problem (both were underwhelming)
- Simplifies turn structure (3 players → 2 players)
- Creates clear deck building goals (optimize for known Buyer)
- Enables easy content expansion (new Buyers = new personas with unique decks)
- Fits push-your-luck meta (cumulative Heat across Buyers)
- Small card counts per Buyer (7 cards) makes balancing manageable

**Benefits:**
- **Structural simplification** - Fewer moving parts
- **Strategic depth** - Buyer identity shapes deck building
- **Content scalability** - Easy to add new Buyers
- **Session variety** - Different Buyers = different challenges
- **Meta-game hook** - Push-your-luck across multiple Buyers

---

## Feasibility Analysis

### Technical Assessment

**Verdict:** ✅ **FEASIBLE** (8-10 hours estimated)

**Changes from RFC-006:**

**Delete:**
- Customer as active player (~2 hours saved)
- Customer turn logic
- Customer AI decision making
- 3-player turn rotation

**Add:**
- Buyer entity system (~3 hours)
- Buyer persona data (setup + reaction decks)
- Visible 3-card hand display (~1 hour)
- Random card play from hand (~1 hour)
- Product + Location requirement check (~1 hour)
- Buyer bail logic (~1 hour)
- Session structure UI (Continue/Lay Low) (~2 hours)

**Modify:**
- Turn structure (simplify to Narc → Player) (~1 hour)
- Hand resolution (add validity check, bail check) (~1 hour)

**Total Estimate:** 8-10 hours

**Breakdown:**
- Entity refactor: 3 hours (remove Customer, add Buyer)
- Buyer data creation: 2 hours (4-5 personas × 7 cards = 28-35 cards)
- Visible hand UI: 1 hour (display 3 Buyer cards)
- Random play logic: 1 hour (select from visible hand)
- Requirement checks: 2 hours (validity, bail, threshold)
- Session UI: 2 hours (Continue/Lay Low decision)

### System Integration

**Integration Points:**

**From RFC-006 (Reuse):**
1. **Sequential play** → Unchanged (just remove Customer turn)
2. **Progressive reveals** → Now Buyer reaction cards (same timing)
3. **Override system** → Unchanged (Location override still works)
4. **Totals calculation** → Add Buyer card effects (same as Dealer cards)

**New Systems:**
1. **Buyer personas** → Data-driven (JSON/RON files per persona)
2. **Visible hand** → UI display of 3 cards
3. **Validity checks** → New resolution step (Product + Location required)
4. **Bail conditions** → New resolution branch (threshold checks)
5. **Session loop** → New state machine (Pick Buyer → Play → Decision)

**No Breaking Changes:**
- Card types unchanged (Buyer cards use existing CardType variants)
- Override rules unchanged
- Bust resolution mostly unchanged (add validity check before bust check)

### Alternatives Considered

**Alternative 1: Keep 3-Player Structure**
- **Rejected:** Customer too weak, Dealer too passive, turn order complex

**Alternative 2: Buyer Plays Face-Down**
- **Rejected:** Removes anticipation, makes threat unpredictable (frustrating RNG)

**Alternative 3: Buyer Plays All 3 Cards at Once**
- **Rejected:** Too much at once, no progressive tension

**Alternative 4: Buyer Deck Size 15 Cards**
- **Rejected:** Too many cards to design/balance per persona, want variety through personas not deck size

**Chosen Approach:**
- Merge into Buyer (simpler structure)
- 7-card decks (manageable, variety through personas)
- Visible 3-card hand (readable threat)
- Random play from hand (anticipation without predictability)

---

## Discussion

### ARCHITECT Notes

**Why This Works:**

**Structural Simplification:**
- 3 players → 2 players (Narc vs Player)
- Simpler turn order (no rotation needed)
- Fewer AI decision points (just Narc AI)

**Strategic Depth:**
- Buyer identity shapes deck building (build for Party Host vs Mom)
- Product + Location requirement creates constraints
- Visible hand creates risk assessment ("They might play Cops Called")

**Content Scalability:**
- Adding Buyer = 1 setup + 7 cards (~2 hours design/balance)
- Can ship with 4 Buyers, add more post-launch
- Each Buyer feels distinct (different demands, reactions, risk profiles)

**Meta-Game Hook:**
- Heat accumulation across Buyers creates push-your-luck
- "One more Buyer?" is the daily engagement hook
- Heat decay encourages returning tomorrow

**Concerns:**

**Buyer Bail Frequency:**
- If Mom bails 70% of time → frustrating (wasted cards)
- If Mom never bails → threshold meaningless
- Recommend: 20-30% bail rate (threshold should matter but not dominate)

**Visible Hand Predictability:**
- Seeing 3 cards reduces surprise
- But random play from hand preserves some uncertainty
- Balance: Anticipation (know it COULD happen) vs surprise (don't know WHEN)

**Session Length Variance:**
- First Buyer success in 1 hand = 5 minutes
- Second Buyer fails 4 times = 20 minutes
- Could be issue if players expect consistent length
- Mitigation: Clear feedback on Heat level, encourage laying low

---

### ARCHITECT Review

**Date:** 2025-11-11

**Technical Assessment:** ✅ **FEASIBLE** with minor scope adjustments

#### System Integration Analysis

**Current State (SOW-008 Implementation):**
- 3-player turn structure: Narc → Customer → Player (with rotation)
- Dealer deck (20 cards) reveals 1 card per round as "community cards"
- Sequential play with progressive reveals
- Override system for Locations
- Fold mechanics (Player can fold R1-R2, Customer has AI fold logic)
- Running totals display

**Proposed Changes:**

**1. Entity Refactor (DELETE Customer/Dealer, ADD Buyer):**
```
Current:
  - Customer (active player with AI, provides profit multipliers)
  - Dealer (passive deck, reveals community cards)

Proposed:
  - Buyer (merged entity: persona setup + 7-card reaction deck)
```

**Impact:** MODERATE - Customer AI removal simplifies code, Buyer system is mostly data-driven

**2. Turn Structure Simplification:**
```
Current: Narc → Customer → Player (rotation per round)
Proposed: Narc → Player (no rotation)
```

**Impact:** LOW - Existing turn order system supports 2-player easily (just remove Customer from rotation)

**3. Buyer Reaction System:**
```
New Components:
  - BuyerPersona (data structure: demands, multiplier, thresholds, special rules)
  - Buyer reaction deck (7 cards, drawn 3 visible at hand start)
  - Random play from visible hand (1 card per round)
```

**Impact:** MODERATE - New data structures but similar to existing Dealer deck pattern

**4. Validity Checks (Product + Location Requirement):**
```
New Resolution Logic:
  1. Validity: active_product.exists() && active_location.exists()
  2. Demand: Does Product/Location meet Buyer demands?
  3. Threshold: Did any threshold trigger (Heat, Evidence)?
  4. Bust: Evidence > Cover?
```

**Impact:** LOW - Extends existing resolution logic, adds conditional branches

#### Architecture Concerns

**Concern 1: Buyer Deck vs Dealer Deck Confusion**

Current codebase has "Dealer deck" (20 scenario cards revealing progressively). RFC-009 introduces "Buyer reaction deck" (7 cards, 3 visible).

**Naming collision risk:**
- Old: `dealer_deck: Vec<Card>` (scenario cards)
- New: Buyer reaction deck (also environment/reaction cards)

**Recommendation:** Clear naming convention required
```rust
// Suggested naming:
scenario_deck: Vec<Card>      // Current "Dealer deck" (20 cards, community)
buyer_deck: Vec<Card>         // Buyer reaction deck (7 cards)
buyer_hand: Vec<Card>         // Visible 3 cards
```

**Resolution:** MUST address in SOW - explicit naming to avoid confusion

---

**Concern 2: Two Separate "Dealer Reveal" Systems**

RFC-009 states: "Each round: Narc plays → Player plays → **Buyer plays 1 random card from visible hand**"

But SOW-008 already has: "After player phase, **Dealer reveals 1 scenario card**"

**Are these the same system or two separate reveals?**

Reading RFC-009 more carefully: "Dealer + Customer → Buyer" suggests Buyer **replaces both**.

**Interpretation:**
- OLD: Dealer scenario deck (community cards) **DELETED**
- NEW: Buyer reaction deck (replaces Dealer scenario cards)

**BUT** - This creates game design question: If we remove scenario cards entirely, do we lose environmental variety?

**Recommendation:** RFC should clarify:
- Are scenario cards removed entirely?
- Or do scenario cards remain as "environment" separate from Buyer reactions?

**Reading RFC-009 lines 89-98:** Turn structure shows "Buyer plays 1 random card from visible hand" - this suggests Buyer deck **replaces** Dealer scenario deck entirely.

**Conclusion:** Buyer reaction deck is the ONLY community card system (scenario deck removed).

**Architectural Impact:** GOOD - Simpler (one community card system not two), but large deletion of SOW-008 Phase 2 work.

---

**Concern 3: Fold Mechanic Clarification (Resolved)**

Original RFC said "discard permanently" - PLAYER clarified this means "discard for session, not from collection."

This is now clear in updated RFC lines 162-169. ✅ Resolved.

---

**Concern 4: Session Structure Requires New State Machine**

RFC-009 introduces:
- Phase 1: Pick Buyer → Phase 2: Build Deck → Phase 3: Play Hands → Phase 4: Decision (Continue/Lay Low)

Current state machine: `DeckBuilding` → `InRun`

**New states needed:**
```rust
enum GameState {
    BuyerSelection,    // Pick which Buyer to deal with
    DeckBuilding,      // Build 15-card deck for this Buyer
    InRun,             // Play hands
    SessionDecision,   // After success: Continue or Lay Low?
}
```

**Impact:** MODERATE - New state flow, but follows existing patterns

---

#### Time Estimate Review

**Original Estimate:** 8-10 hours

**Revised Estimate:** 10-12 hours

**Breakdown:**
1. **Entity refactor** (3-4 hours)
   - Delete Customer AI logic (turn decisions, fold logic)
   - Delete Dealer scenario deck system
   - Add BuyerPersona data structure
   - Add Buyer deck initialization

2. **Buyer persona data** (2-3 hours)
   - Create 4-5 BuyerPersona configs (JSON/RON)
   - Design 28-35 Buyer reaction cards (7 per persona)
   - Balance demands/multipliers/thresholds

3. **Visible hand UI** (1-2 hours)
   - Display 3 Buyer cards face-up at hand start
   - Highlight which card gets played each round
   - Update UI when card removed from hand

4. **Random play logic** (1 hour)
   - Draw 3 cards at hand start
   - Each round: pick random card from visible hand
   - Remove played card, continue with remaining

5. **Resolution checks** (2-3 hours)
   - Product + Location validity check
   - Demand satisfaction check (apply correct multiplier)
   - Threshold check (Buyer bail logic)
   - Integrate with existing bust resolution

6. **Session state machine** (1-2 hours)
   - Add BuyerSelection state
   - Add SessionDecision state (Continue/Lay Low)
   - Wire up transitions

**Total: 10-13 hours**

**Fits within ≤20 hour SOW limit?** ✅ YES (with 7-10 hours buffer)

---

#### Recommendations

**1. Clarify Buyer Deck vs Scenario Deck (CRITICAL)**

RFC should explicitly state:
- ✅ Scenario deck (SOW-008 Dealer deck) **REMOVED**
- ✅ Buyer reaction deck **REPLACES** scenario deck as only community card system
- ✅ Naming convention: `scenario_deck` deleted, `buyer_deck` added

**2. Start with 3 Polished Buyers, Not 5 (PLAYER feedback)**

PLAYER noted: "Better to ship with 3 GREAT Buyers than 5 mediocre ones"

**Recommendation:**
- MVP: 3 Buyers (21 cards total)
- Post-MVP: Add 2-4 more Buyers

**Time savings:** 2 Buyers × 7 cards = 14 fewer cards to design/balance (~1 hour saved)

**3. Buyer Bail Tuning Should Be Data-Driven**

RFC suggests 20-30% bail rate. PLAYER suggests 10-15%.

**Recommendation:** Make thresholds configurable per Buyer, tune during playtesting
```rust
pub struct BuyerPersona {
    heat_threshold: Option<u32>,  // None = never bails
    evidence_threshold: Option<u32>,
    // ...
}
```

**4. Junkie Persona Needs Buff (PLAYER feedback)**

PLAYER: "Why would I ever pick Junkie? (×0.8 profit, unpredictable)"

**Recommendation:** Either:
- Buff multiplier to ×1.0 (low but not punishing)
- Add unique mechanic: "Junkie never increases Heat" or "Always available at high Heat"

**5. Session Decision UI Should Show Heat Context**

"Continue or Lay Low?" decision needs:
- Current Heat level (e.g., "Heat: 50")
- Next Buyer difficulty context (e.g., "⚠️ High Heat makes next deal risky")
- Estimated Heat decay time (e.g., "Lay Low: -20 Heat over 4 hours")

---

#### Final Verdict

**✅ APPROVED with conditions:**

1. **Clarify in SOW:** Buyer deck replaces Dealer scenario deck (not additive)
2. **Reduce scope:** 3 Buyers for MVP (not 5)
3. **Address naming:** Clear distinction between old/new systems
4. **Tune Junkie persona:** Buff or add unique mechanic
5. **Make thresholds configurable:** Data-driven bail tuning

**Feasibility:** ✅ YES - 10-13 hours (fits ≤20 hour SOW limit)

**Integration Risk:** 🟡 MODERATE - Large refactor (delete Customer/Dealer, add Buyer), but follows existing patterns

**Architectural Fit:** ✅ GOOD - Simplifies structure, enables content expansion, data-driven design

**Technical Debt:** ✅ NONE - Actually reduces complexity (3 players → 2 players)

**Recommendation:** Proceed to SOW creation with above conditions addressed.

---

### PLAYER Validation

**What This Needs to Feel Like:**

**Buyer Selection:**
- "College Party Host wants Pills and volume, ×2.5 profit but high Evidence"
- "Mom wants small Pills deal, ×1.2 profit but will bail if Heat > 30"
- "Which Buyer matches my current Heat level?"

**Visible Hand:**
- See: "Cops Called" (+20 Evidence), "VIP Room" (-10 Evidence), "Cash Bar" (+$30)
- Think: "If they play Cops Called this round, I'm screwed"
- Each round: "Which card will they play?"

**Deal Completion:**
- Success: "Made the deal! $250 profit. Heat is 50 now. Do I push for another?"
- Buyer bail: "Mom bailed, Heat was 35 (over 30). Try again with better Heat management."
- Deck exhausted: "Out of cards, failed to make deal. Gotta find new Buyer or lay low."

**Critical Success Factors:**

1. **Buyer identity clear** - Know who you're dealing with, what they want
2. **Risk readable** - Visible hand shows possible threats
3. **Demand satisfaction rewarding** - ×2.5 feels meaningfully better than ×1.0
4. **Bail feels fair** - Threshold was clear, you knew the risk
5. **Push-your-luck compelling** - "One more Buyer" decision is interesting

**Acceptance Criteria:**

After 5 Buyer encounters, I should answer YES to:
- [ ] Can I describe Buyer personality? (Party Host = volume + risky)
- [ ] Did visible hand create anticipation? (dreading/hoping for specific card)
- [ ] Did meeting demands feel rewarding? (×2.5 vs ×1.0 mattered)
- [ ] Did bail feel fair? (knew threshold, made informed choice)
- [ ] Did "one more Buyer?" decision feel interesting? (weighing Heat vs profit)

**If 4+ YES → Buyer system works**
**If 3 or fewer → Need iteration on personas/mechanics**

---

## Approval

**Status:** Approved

**Approvers:**
- ARCHITECT: ✅ **APPROVED** (2025-11-11)
  - Feasible in 10-13 hours (fits ≤20 hour SOW limit)
  - Simplifies structure (3 players → 2 players, one community card system)
  - Follows existing patterns (data-driven personas, similar to Dealer deck)
  - Conditions: Clarify Buyer deck replaces scenario deck, reduce to 3 Buyers for MVP, buff Junkie persona, make thresholds configurable
- PLAYER: ✅ **APPROVED** (2025-11-11)
  - Buyer identity system addresses "who am I dealing with?" confusion
  - Visible hand creates anticipation without frustration
  - "One more Buyer?" push-your-luck is strong engagement hook
  - Fold clarification (discard vs permanent loss) resolves death spiral concern
  - Concerns noted: Buyer bail frequency (recommend 10-15% not 20-30%), Junkie persona needs buff/unique mechanic

**Scope Constraint:** 10-13 hours (fits within ≤20 hour SOW limit)

**Dependencies:**
- RFC-006: ✅ Sequential play foundation

**Next Steps:**
1. ARCHITECT: Review feasibility (8-10 hours realistic?)
2. PLAYER: Validate Buyer personas (do these create variety?)
3. If both approve → Create SOW-009
4. DEVELOPER: Implement per SOW-009
5. Playtest: Does Buyer identity matter? Is "one more?" compelling?

**Date:** 2025-11-10