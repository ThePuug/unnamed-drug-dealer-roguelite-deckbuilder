# RFC-008: Sequential Play with Progressive Dealer Reveals

## Status

**Under Review** - 2025-11-10

**Supersedes:** RFC-002 betting mechanics (simultaneous face-down play)

**Depends On:** RFC-001 (foundation), RFC-002 (betting system), RFC-003 (insurance), RFC-004 (card retention), RFC-005 (deck balance), RFC-006 (deck building)

## Feature Request

### Player Need

From player perspective: **I need rounds to matter individually, not just accumulate to a final calculation. Each round should force decisions based on new information.**

**Current Problem:**
After extensive playtesting:
- Rounds 1-2 feel meaningless (just blind card accumulation)
- Only Round 3 matters (final Evidence vs Cover check)
- Can't react to threats in real-time (cards flip after betting complete)
- Optimal strategy is "spam all cards, then check math at end"
- No betting/bluffing tension (just math calculation)
- Initiative too powerful (Narc forces player to waste cards or fold)

**Core issue discovered:** The game plays like poker where you bet every street blind. You play cards, then see if you won. There's no reactive decision-making.

**We need a system that:**
- Creates progressive information revelation (like Texas Hold'em flop/turn/river)
- Allows reactive play (see threat, respond with appropriate card)
- Makes each round individually meaningful (not just accumulation)
- Creates "river tension" (final reveal can save or doom you)
- Leverages unique override mechanic (not just copying poker)
- Works well vs AI opponents (deterministic, not mind-reading)

### Desired Experience

**This is CORE LOOP VALIDATION - the make-or-break test.**

Players should experience:
- **Progressive revelation** - Each Dealer card reveal changes the situation
- **Reactive decisions** - "Dealer revealed Police Checkpoint, I need Cover NOW"
- **River tension** - "Please don't reveal something dangerous... oh no."
- **Override wars** - "Narc played School Zone, I'll counter with Warehouse"
- **Fold pressure** - "Evidence is climbing, do I bail now or risk Round 3?"
- **Strategic timing** - "Do I play my strong card now or save for later?"
- **Turn order advantage** - "I go last this round, I can respond to everything"

**If rounds don't feel individually meaningful after this change, the game concept is fundamentally broken.**

### Specification Requirements

**Sequential Card Play Structure:**

Each round consists of:
1. **Player Phase** - One card per player, face-up immediately (one at a time, in turn order)
2. **Dealer Reveal** - One Dealer card flips, affecting all players
3. **Totals Update** - Running Evidence/Cover/Profit/Heat display updates
4. **Fold Decision** - Player can fold after seeing Dealer reveal (Rounds 1-2 only)

**Turn Order Rotation:**
- **Round 1:** Narc → Customer → Player (Player has last-mover advantage)
- **Round 2:** Customer → Player → Narc (Narc has last-mover advantage)
- **Round 3:** Player → Narc → Customer (Customer has last-mover advantage)

**Each turn:**
- Player plays ONE card face-up OR checks (no card)
- Card flips immediately (visible to all)
- Totals update in real-time
- Next player's turn

**Dealer Card System (3 Community Cards):**
- Dealer has separate deck of 20 scenario cards
- Each hand, Dealer draws 3 cards (hidden)
- One card revealed per round (after Player Phase)
- Dealer cards represent environmental factors all players must deal with

**Dealer Card Types:**
```
Location Cards (8 cards):
  - Set base Evidence and Cover values
  - Can be overridden by player Location cards
  - Examples:
    * Private Residence (10 Evidence, 25 Cover, -10 Heat) [SAFE]
    * Parking Lot (25 Evidence, 15 Cover, 0 Heat) [NEUTRAL]
    * Police Checkpoint (30 Evidence, 0 Cover, +15 Heat) [DANGEROUS]
    * School Zone (35 Evidence, 5 Cover, +25 Heat) [VERY DANGEROUS]

Modifier Cards (8 cards):
  - Adjust Evidence/Cover/Heat additively
  - Cannot be overridden
  - Examples:
    * Quiet Night (+5 Evidence, +10 Cover, -5 Heat) [HELPFUL]
    * Heat Wave (+15 Evidence, +0 Cover, +10 Heat) [HARMFUL]
    * Rival Dealer (if win: +30 Heat) [CONDITIONAL]

Wild Cards (4 cards):
  - High-impact swings
  - Examples:
    * Lucky Break (-20 Evidence) [VERY HELPFUL]
    * Bad Intel (+25 Evidence) [VERY HARMFUL]
```

**Fold Mechanics:**

**Player Folding:**
- Can fold after Dealer reveal (Rounds 1-2)
- Cannot fold Round 3 (committed to resolution)
- Fold costs:
  - ❌ All cards played so far (discarded, can't reuse)
  - ❌ All profit (deal didn't complete)
  - ✅ Keep Heat accumulated from played cards
  - ✅ Keep unplayed cards in hand

**Narc Folding:**
- Narc CANNOT fold (thematically wrong, mechanically broken)
- Narc is law enforcement (doesn't "give up" on investigations)
- Always plays through to resolution

**Customer Folding:**
- Can fold after Dealer reveal (any round)
- Fold trigger (AI): Evidence > threshold (50/60/80 by round)
- Fold consequences:
  - Customer cards played so far removed from totals
  - Profit multipliers lost (×1.5 becomes ×1.0)
  - Evidence from Customer cards also removed (slight help)
  - Forces player to reassess: "Is reduced profit worth the risk?"

**Override System (Unchanged, But More Impactful):**
- Location cards override previous Location (last one played = active)
- Dealer Locations can be overridden by player/narc Locations
- Override happens immediately when card played (visible effect)
- Creates back-and-forth battle ("I play School Zone, you counter with Warehouse")

**Running Totals Display:**
- Evidence total (updates after each card)
- Cover total (updates after each card)
- Profit calculation (updates when Product/modifiers played)
- Heat delta (cumulative Heat from all cards)
- Active Location highlighted (shows which Location is in effect)

### MVP Scope

**Phase 1 includes:**
- Sequential card play (one at a time, face-up)
- Dealer deck (20 scenario cards: 8 Locations, 8 Modifiers, 4 Wild)
- Dealer reveal system (one card per round)
- Rotating turn order (Narc/Customer/Player rotates each round)
- Running totals display (updates after each card)
- Player fold mechanic (Rounds 1-2)
- Customer fold mechanic (with consequences)
- Narc never folds (hardcoded behavior)
- Check action (player can skip playing card)

**Phase 1 excludes:**
- Complex AI decision-making (simple heuristics sufficient for MVP)
- Animations (functional prototype only)
- Multi-opponent scenarios (single Narc + Customer sufficient for validation)

### Priority Justification

**CRITICAL PRIORITY** - This fixes fundamental broken core loop

**Why CRITICAL:**
- Current design doesn't work (playtesting revealed no meaningful decisions)
- This is a major rework of core gameplay loop
- All existing features (insurance, deck building, card retention) need core loop to be fun
- Without this, game remains a "math calculation exercise" not a strategy game
- Relatively contained change (existing card system compatible)

**Benefits:**
- **Core loop improvement** - Creates meaningful round-by-round decisions
- **River tension** - Progressive reveals create Texas Hold'em-style drama
- **Reactive gameplay** - Player responds to threats in real-time
- **Override leverage** - Existing mechanic becomes central to strategy
- **AI-friendly** - Deterministic play (no bluffing mind-reading)
- **Builds on existing systems** - Insurance, deck building, card retention all remain functional

---

## Feasibility Analysis

### Technical Assessment

**Verdict:** ✅ **HIGHLY FEASIBLE** (6-8 hours estimated)

**Foundation Ready from SOW-001/002:**
- ✅ Card data model supports all card types (Products, Locations, Evidence, Cover, Modifiers)
- ✅ Override rules implemented (Product/Location replacement)
- ✅ Totals calculation exists (just needs real-time updates)
- ✅ Hand state machine extensible (can change turn structure)
- ✅ UI framework established (can add running totals display)

**Core Changes Required:**

**1. Sequential Play Instead of Simultaneous (~2-3 hours)**

**Current flow:**
```rust
// All players play face-down
narc_plays_card_facedown()
customer_plays_card_facedown()
player_plays_card_facedown()

// All cards flip together
flip_all_cards()
calculate_totals()
```

**New flow:**
```rust
// Turn-based, immediate reveals
for player in [narc, customer, player] {
    player_plays_card_faceup()  // Or checks
    update_running_totals()
    display_card_immediately()
}

dealer_reveals_card()
update_running_totals()
player_fold_decision()  // If Round 1 or 2
```

**Implementation:**
- Modify `BettingPhase` to be turn-based (iterate through players)
- Remove face-down card holding (cards flip immediately)
- Add `display_card_event` (triggers UI update per card)
- Add `update_running_totals()` after each card

**2. Dealer Deck System (~2-3 hours)**

**New systems:**
```rust
struct DealerDeck {
    cards: Vec<Card>,
    drawn_this_hand: Vec<Card>,  // 3 cards per hand
}

fn dealer_reveal_card(round: u8) -> Card {
    // Reveal card for this round (1, 2, or 3)
    self.drawn_this_hand[round - 1]
}

// Dealer cards are just Cards with special types
enum CardType {
    // ... existing types
    DealerLocation { evidence: u32, cover: u32, heat: i32 },
    DealerModifier { evidence: i32, cover: i32, heat: i32 },
}
```

**Implementation:**
- Create `DealerDeck` component (separate from narc/customer/player decks)
- Add 20 Dealer scenario cards (hardcoded for MVP)
- Add `dealer_reveal` system (runs after player phase)
- Dealer cards integrate into existing totals calculation (just more cards)

**3. Rotating Turn Order (~1 hour)**

**State tracking:**
```rust
struct HandState {
    // ... existing fields
    round: u8,  // 1, 2, or 3
}

fn get_turn_order(round: u8) -> Vec<Owner> {
    match round {
        1 => vec![Owner::Narc, Owner::Customer, Owner::Player],
        2 => vec![Owner::Customer, Owner::Player, Owner::Narc],
        3 => vec![Owner::Player, Owner::Narc, Owner::Customer],
        _ => unreachable!(),
    }
}
```

**Implementation:**
- Modify `BettingPhase` to use `get_turn_order(round)`
- Turn indicator UI updates per turn (highlight active player)

**4. Fold Mechanics (~1-2 hours)**

**Player fold:**
```rust
fn handle_fold(&mut self) {
    // Discard cards played so far
    for card in self.cards_played.drain(..) {
        // Cards are discarded permanently (not back to deck)
    }
    
    // Keep Heat accumulated
    // (already tracked in HandState)
    
    // Keep unplayed cards
    // (already in player_hand)
    
    self.state = HandState::HandEnd;  // Skip to end
}
```

**Customer fold:**
```rust
fn customer_should_fold(evidence: u32, round: u8) -> bool {
    let threshold = match round {
        1 => 50,
        2 => 60,
        3 => 80,
        _ => unreachable!(),
    };
    
    evidence > threshold && rand::random::<f32>() < 0.3
}

fn apply_customer_fold(&mut self) {
    // Remove customer cards from totals
    self.cards_played.retain(|c| c.owner != Owner::Customer);
    
    // Recalculate totals (profit multipliers lost, Evidence reduced)
    self.totals = calculate_totals(&self.cards_played, &self.dealer_cards);
}
```

**Implementation:**
- Add `Fold` action to player actions
- Add fold button UI (visible Rounds 1-2)
- Add customer fold AI logic (simple threshold check)
- Narc fold disabled (no logic needed)

**5. Running Totals Display (~1 hour)**

**UI components:**
- Evidence bar (visual bar + number)
- Cover bar (visual bar + number)
- Profit display ($ amount with modifiers visible)
- Heat delta (+ or - with color coding)
- Active Location highlight (shows which Location is in effect)

**Updates after each card:**
```rust
fn display_running_totals(totals: &HandTotals) {
    update_evidence_bar(totals.evidence);
    update_cover_bar(totals.cover);
    update_profit_display(totals.profit);
    update_heat_display(totals.heat_delta);
    highlight_active_location(totals.active_location);
}
```

**Total Estimate:** 6-8 hours ✅

---

### System Integration

**Integration Points:**

**From SOW-001/002 (Reuse):**
1. **Card Model** → Dealer cards are just new CardType variants (trivial addition)
2. **Override Rules** → Already implemented, just applied more frequently
3. **Totals Calculation** → Already exists, just called after each card now
4. **UI Framework** → Extend with running totals (additive change)
5. **Hand State** → Already tracks rounds, just modify turn flow

**New Dependencies:**
- None (all logic uses existing Bevy/Rust patterns)

**Data Flow:**
```
Round Start:
  ↓
For each player in turn_order:
  - Player plays card face-up OR checks
  - Card flips immediately
  - Totals update
  ↓
Dealer reveals card
  ↓
Totals update (including Dealer card)
  ↓
Player fold decision (if Round 1 or 2)
  ↓
Next round OR resolve hand
```

**No Breaking Changes:**
- SOW-001 card types unchanged (just used differently)
- Override rules unchanged (just more visible now)
- Bust resolution unchanged (Evidence > Cover check)

---

### Alternatives Considered

**Alternative 1: Keep Simultaneous Play, Add "Betting" Phase**
- **Approach:** Cards face-down → betting round → flip together
- **Rejected:** This is just copying poker, doesn't leverage unique mechanics
- **Why:** Doesn't solve "rounds feel meaningless" problem

**Alternative 2: Stack/Response System (Magic-style)**
- **Approach:** Play card → "Responses?" → Resolve stack
- **Rejected:** Too complex, too many decision points, analysis paralysis
- **Why:** Doesn't work well vs AI (needs complex AI decision tree)

**Alternative 3: No Dealer Cards (Just Player Cards)**
- **Approach:** Sequential play but no community cards
- **Rejected:** Less "river tension," less variance, more predictable
- **Why:** Dealer reveals create drama and uncertainty

**Alternative 4: Dealer Reveals Before Player Phase**
- **Approach:** Dealer card flips first, then players respond
- **Rejected:** Removes suspense (know environment before committing)
- **Why:** Players commit cards, THEN see Dealer reveal = tension

**Chosen Approach:**
- Sequential play (one card at a time, face-up)
- Dealer reveals after player phase (community card drama)
- Rotating turn order (balances advantage)
- **Rationale:** Simplest implementation, leverages unique override mechanic, creates Texas Hold'em-style tension

---

### Risks & Mitigation

**Technical Risks:**

**Risk 1: UI Update Performance**
- **Concern:** Updating totals after each card (9 updates per hand) might cause lag
- **Mitigation:** Totals calculation is pure function (O(n) where n = cards played, max 12), negligible
- **Likelihood:** Very Low
- **Impact:** Low

**Risk 2: AI Turn-Taking Pacing**
- **Concern:** AI playing cards instantly makes it hard to follow
- **Mitigation:** Add 0.5-1s delay between AI actions (visual pacing)
- **Likelihood:** Medium
- **Impact:** Medium (UX issue, not gameplay)

**Risk 3: Dealer Deck Balance**
- **Concern:** Too many dangerous Dealer cards = game too hard
- **Mitigation:** Start with 50% safe, 30% neutral, 20% dangerous, tune based on playtesting
- **Likelihood:** Medium
- **Impact:** Medium (easy to retune card values)

**Gameplay Risks:**

**Risk 4: Rounds Still Feel Meaningless**
- **Concern:** Even with progressive reveals, decisions might not matter
- **Mitigation:** This is THE validation test (if this doesn't work, game concept broken)
- **Likelihood:** Unknown (purpose of this RFC)
- **Impact:** Critical (make or break)

**Risk 5: Too Much Information**
- **Concern:** Everything face-up = no hidden information = boring
- **Mitigation:** Dealer reveals provide uncertainty, override wars create tactical depth
- **Likelihood:** Low
- **Impact:** Medium

**Risk 6: Turn Order Imbalance**
- **Concern:** Going last every round is too powerful
- **Mitigation:** Rotating turn order balances advantage (each player gets last-mover once)
- **Likelihood:** Low
- **Impact:** Medium (easy to adjust rotation if needed)

---

### Recommendations

**Approve RFC-008 with conditions:**

✅ **Approve IF:**
1. Scope stays within 6-8 hours (no feature creep)
2. Immediate playtesting after implementation (does core loop work now?)
3. Success criteria focus on "do rounds feel meaningful?" (not just technical completion)
4. Existing features (insurance, deck building, card retention) remain functional

❌ **Reject IF:**
- Scope expands beyond foundational changes
- Playtesting reveals rounds still don't matter (game concept fundamentally broken)

**Post-Implementation:**
- If rounds feel meaningful → Core loop validated, continue adding content
- If rounds still don't matter → Major redesign needed
- If turn order feels imbalanced → Adjust rotation pattern
- Verify existing features (insurance, deck building) integrate well

---

### Conclusion

**RFC-008 is technically straightforward** and addresses the fundamental core loop problem discovered through playtesting. Estimated 6-8 hours to significantly rework core gameplay.

**Key Success Metric:** After implementation, answer "Does each round create meaningful decisions based on progressive information?"

**This RFC is critical for making the game fun.** If sequential play with Dealer reveals doesn't fix "rounds feel meaningless," all existing features (insurance, deck building, card retention) won't matter because the core loop isn't engaging.

---

## Discussion

### ARCHITECT Notes

**Why This Will Work (Hypothesis):**

**Progressive Information Structure:**
- Round 1: See Narc/Customer cards + 1 Dealer card → Assess (safe or dangerous?)
- Round 2: See more cards + 2nd Dealer card → Reassess (getting worse or better?)
- Round 3: See final cards + 3rd Dealer card → React (saved or doomed?)

**Each round provides NEW information that changes strategy.**

**Reactive Decision-Making:**
- "Narc played Surveillance (+20 Evidence), I need Cover"
- "Dealer revealed Police Checkpoint (+30 Evidence), I need to override or fold"
- "Customer folded, lost profit multiplier, still worth staying in?"

**These are DECISIONS, not just accumulation.**

**Turn Order Strategy:**
- Round 1: Player goes last (can respond to threats)
- Round 2: Narc goes last (can override player's defensive play)
- Round 3: Customer goes last (but typically weakest player)

**Going first vs last creates timing strategy.**

**Override Wars:**
- Narc: School Zone (40 Evidence)
- Player: Warehouse (15 Evidence) - OVERRIDES
- Dealer: Police Checkpoint (30 Evidence) - OVERRIDES Warehouse
- Player: "Shit, my override got overridden by Dealer"

**Back-and-forth control battles.**

**Implementation Simplicity:**
- Most code reusable (card types, override rules, totals calculation)
- Main change: Turn-based instead of simultaneous
- Dealer deck: Just more cards (integrate into existing system)
- Minimal risk, high impact

**Concerns:**

**Dealer Balance is Critical:**
- Too many dangerous cards = too hard
- Too many safe cards = too easy
- Needs careful tuning (recommend 50/30/20 split for MVP)

**AI Pacing Matters:**
- Instant AI plays = hard to follow
- Need visual delays (0.5-1s per AI action)
- Consider "AI is thinking..." indicator

**Customer Fold Frequency:**
- If Customer folds 70% of hands = annoying
- If Customer never folds = mechanic wasted
- Recommend 20-30% fold rate (tune thresholds)

### PLAYER Validation

**What This Needs to Feel Like:**

**Round 1: Information Gathering**
- "Okay, Narc played Surveillance, that's 20 Evidence"
- "Customer played Bulk Order, that's ×1.5 profit but +20 more Evidence"
- "I'll play Meth for the profit"
- "Dealer reveals... Private Residence (safe!)"
- "Total Evidence: 50, Cover: 25. Manageable, I'll stay in."

**Round 2: Escalation**
- "Narc played Patrol, Evidence up to 55"
- "I'll play Alibi for Cover"
- "Dealer reveals... Police Checkpoint (OH NO)"
- "Evidence jumped to 75, my Cover is only 55. Customer just folded."
- "Do I fold now or push to Round 3?"

**Round 3: Commitment**
- "I'm staying in, playing Warehouse to override Police Checkpoint"
- "Narc played Wiretap, Evidence back up"
- "Dealer reveals... please be helpful... Quiet Night (+10 Cover!)"
- "Final totals: Evidence 70, Cover 65. Still short by 5. BUSTED."

**If I can narrate a story like this, the game works.**

**If I can't track what's happening or decisions feel arbitrary, it doesn't work.**

**Critical Success Factors:**

1. **Clear cause and effect** - "I played Warehouse → Evidence dropped from 75 to 35"
2. **Visible momentum shifts** - Dealer reveal changes everything
3. **Meaningful fold decisions** - "Evidence 60, Cover 30, I should fold" vs "Evidence 45, Cover 40, I can make it"
4. **Strategic timing** - "Do I play my best card now or wait?"
5. **Customer fold creates drama** - "Customer bailed, lost my multiplier, re-evaluating"

**Acceptance Criteria:**

After 5 test hands, I should be able to answer YES to:
- [ ] Did each round force a decision? (not just accumulation)
- [ ] Did Dealer reveals change my strategy?
- [ ] Did I fold at least once because of progressive info?
- [ ] Did override mechanic matter? (changed outcome)
- [ ] Could I track Evidence/Cover totals easily?
- [ ] Did Customer folding create interesting choice?

**If 4+ are YES → Core loop works, game is fun, continue development**
**If 3 or fewer YES → Core loop still broken, major redesign needed**

---

## Approval

**Status:** Draft → Under Review

**Approvers:**
- PLAYER: ✅ **APPROVED** - Solves core "rounds feel meaningless" problem, creates progressive decision-making
- ARCHITECT: [Pending Review] - Needs validation of 6-8 hour estimate and integration with existing features (RFC-003/004/005/006)

**Scope Constraint:** 6-8 hours (fits within ≤10 hour SOW limit for critical fixes)

**Dependencies:**
- RFC-001: ✅ Foundation (card model, override rules)
- RFC-002: ✅ Betting system (being reworked)
- RFC-003: ✅ Insurance (must remain functional)
- RFC-004: ✅ Card retention (must remain functional)
- RFC-005: ✅ Deck balance (must remain functional)
- RFC-006: ✅ Deck building (must remain functional)

**Critical Path:**
1. **RFC-008 Implementation** (6-8 hours)
2. **Immediate Playtesting** (5-10 test hands)
3. **Verify Integration** (existing features still work)
4. **Decision Point:**
   - If core loop works → Continue with content/polish
   - If core loop broken → Major redesign needed

**Next Steps:**
1. PLAYER: ✅ Approved (this is the right solution)
2. ARCHITECT: Review feasibility (6-8 hours realistic? Will existing features integrate?)
3. If ARCHITECT approves → Create SOW-008
4. DEVELOPER: Implement per SOW-008
5. CRITICAL: Playtest immediately + verify existing features

**Date:** 2025-11-10