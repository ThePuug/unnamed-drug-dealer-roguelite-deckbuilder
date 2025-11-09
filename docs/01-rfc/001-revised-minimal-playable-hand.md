# RFC-001-revised: Minimal Playable Hand

## Status

**✅ Approved** - 2025-11-09

**Supersedes:** RFC-001 (scope too large)

**SOW Created:** [SOW-001: Minimal Playable Hand](../03-sow/001-minimal-playable-hand.md) - Ready for DEVELOPER implementation

## Feature Request

### Player Need

From player perspective: **I need to see if card interactions are clear and understandable** - Can I look at cards on the table and immediately understand what's happening with Evidence, Cover, Heat, and Profit?

**Current Problem:**
Without a technical prototype:
- Don't know if override rules are intuitive (Product/Location replacement)
- Don't know if additive stacking is clear (Evidence + Evidence = total)
- Don't know if Evidence/Cover values feel balanced (is 30 Evidence high or low?)
- Don't know if Heat accumulation is visible enough
- Don't know if Bevy/Rust can handle card game mechanics

**We need a system that:**
- Demonstrates Product override (play Meth, replaces Weed)
- Demonstrates Location override (play Safe House, replaces School Zone)
- Demonstrates Evidence stacking (Patrol +5, Surveillance +20 = 25 total)
- Demonstrates Cover stacking (Location base 30 + Alibi +30 = 60 total)
- Shows Evidence > Cover bust condition clearly
- Displays Heat accumulation visibly

### Desired Experience

**This is TECHNICAL VALIDATION, not fun validation.**

Players (developers testing) should experience:
- **Clarity** - Look at table, immediately understand totals
- **Predictability** - Play a card, see exactly how it changes totals
- **Override understanding** - "Oh, new Product replaces old one, got it"
- **Additive understanding** - "Evidence cards stack, Cover cards stack, clear"
- **Bust understanding** - "Evidence 40 > Cover 30 = busted, fair"

**This will NOT be fun** - Manual play isn't engaging, but that's okay. We're validating mechanics, not gameplay.

### Specification Requirements

**Minimal Card Collection (8 cards):**
- **3 Products:** Weed ($30, +5 Heat), Meth ($100, +30 Heat), Heroin ($150, +45 Heat)
- **2 Locations:** Safe House (Evidence 10, Cover 30, -5 Heat), School Zone (Evidence 40, Cover 5, +20 Heat)
- **2 Evidence cards (Narc):** Patrol (+5 Evidence, +2 Heat), Surveillance (+20 Evidence, +5 Heat)
- **1 Cover card (Player):** Alibi (+30 Cover, -5 Heat)

**Hand Structure (Single Round):**
- Draw 3 cards each (Narc, Customer, Player)
- Narc plays 1 card face-down (manual selection)
- Customer plays 1 card face-down (manual selection)
- Player plays 1 card face-down (manual selection)
- All cards flip simultaneously
- Calculate totals (Evidence, Cover, Heat delta, Profit)
- Check bust: Evidence > Cover → Game over

**Card Interactions (Minimal Set):**
- **Override:** New Product replaces old Product
- **Override:** New Location replaces old Location (Evidence/Cover base changes)
- **Additive:** Evidence cards add to Location Evidence base
- **Additive:** Cover cards add to Location Cover base
- **Heat:** Sum all Heat modifiers on cards played

**Player Feedback:**
- All cards visible on table (3 groups: Narc, Customer, Player)
- Running totals displayed: Evidence, Cover, Heat Delta, Profit
- Active Product highlighted (shows which Product is in effect)
- Active Location highlighted (shows which Location is in effect)
- Simple result: "Safe" (Evidence ≤ Cover) or "BUSTED" (Evidence > Cover)

**UI Requirements (Minimal):**
- Card display area (3 zones: Narc, Customer, Player)
- Totals display area (4 values: Evidence, Cover, Heat, Profit)
- Status display (Safe / Busted)
- Card play buttons (manual selection from hand)

### MVP Scope

**Phase 1 includes:**
- 8-card data definitions (hardcoded, no deck building)
- Single round hand flow (draw → play → flip → resolve)
- Manual card play (click to select card, click to play)
- Product override logic
- Location override logic
- Evidence additive stacking
- Cover additive stacking
- Heat accumulation (sum modifiers)
- Bust check (Evidence > Cover)
- Minimal Bevy UI (rectangles + text is fine)
- Hot reload for card value tuning

**Phase 1 excludes:**
- Multiple rounds (RFC-002)
- Betting mechanics (Check/Raise/Fold) (RFC-002)
- AI opponents (RFC-002)
- Deck building UI (RFC-002)
- Deal Modifiers (RFC-002)
- Insurance cards (Get Out of Jail) (RFC-003)
- Conviction cards (Make It Stick) (RFC-003)
- Multiplicative modifiers (RFC-003)
- Animation/polish (RFC-003)
- Audio (Phase 3)

### Priority Justification

**HIGH PRIORITY** - Technical validation required before adding complexity

**Why HIGH:**
- Must prove card interaction rules are implementable
- Must prove Bevy/Rust can handle card game
- Cheapest time to discover fundamental problems
- Fast iteration on card values (hot reload)

**Benefits:**
- **Technical de-risk** - Prove core mechanics work in Bevy
- **Balance foundation** - Establish baseline Evidence/Cover values
- **Clear specification** - Document exactly how override/additive work
- **Fast to build** - 12-16 hours (1-2 days)
- **Foundation for RFC-002** - Betting/AI hook into this base

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Bevy ECS + Simple Card State Machine**

#### Core Architecture

**Card Entity Model:**
```rust
enum CardType {
    Product { price: u32, heat: i32 },
    Location { evidence: u32, cover: u32, heat: i32 },
    Evidence { evidence: u32, heat: i32 },
    Cover { cover: u32, heat: i32 },
}

struct Card {
    id: CardId,
    name: String,
    card_type: CardType,
    owner: Player, // Narc, Customer, Player
}
```

**Hand State Machine:**
```rust
enum HandState {
    Draw,
    NarcPlay,
    CustomerPlay,
    PlayerPlay,
    Resolve,
    Bust,
}
```

**Totals Calculation:**
```rust
struct HandTotals {
    active_product: Option<CardId>,
    active_location: Option<CardId>,
    evidence: u32,
    cover: u32,
    heat_delta: i32,
    profit: u32,
}

fn calculate_totals(cards_played: &[Card]) -> HandTotals {
    // Override: Find last Product/Location played
    // Additive: Sum Evidence/Cover from Location base + cards
    // Heat: Sum all heat modifiers
}
```

#### Effort Breakdown

- **Card data model** - 2 hours (enums, structs, data definitions)
- **Hand state machine** - 3 hours (state transitions, draw → play → resolve)
- **Override logic** - 2 hours (Product/Location replacement)
- **Additive logic** - 2 hours (Evidence/Cover stacking)
- **Totals calculation** - 2 hours (aggregate all card effects)
- **Bust check** - 1 hour (Evidence > Cover comparison)
- **Bevy UI layout** - 4 hours (3 play zones + totals display + status)
- **Manual card play input** - 2 hours (click to select/play)
- **Hot reload setup** - 1 hour (tune card values without recompile)
- **Testing/polish** - 3 hours (verify all interactions work)

**Total: 12-16 hours** ✅

#### Performance Projections

**Overhead:**
- 8 cards × 3 players = 24 entities (negligible)
- Single round = no accumulation concerns
- UI updates on card play (1-2ms render)

**Development Time:**
- Experienced Bevy dev: 12 hours
- Learning Bevy: 16 hours

#### Technical Risks

**1. Bevy UI Learning Curve**
- *Risk:* Bevy UI verbose/complex for card layout
- *Mitigation:* Start with simplest possible (colored rectangles + text), consider egui if painful
- *Impact:* Medium (UI is 30% of effort)

**2. Override Logic Edge Cases**
- *Risk:* What if no Product played? No Location played?
- *Mitigation:* Require Location in every hand (baseline Evidence/Cover), Product optional (profit 0 if missing)
- *Impact:* Low (design decision)

**3. Hot Reload Instability**
- *Risk:* Bevy hot reload may be buggy
- *Mitigation:* Use RON files for card data (reload on file change), acceptable if manual restart needed
- *Impact:* Low (nice-to-have)

### System Integration

**Core Systems Required:**
- Bevy ECS (entities, components, systems)
- Bevy UI or egui (card display, totals display)
- State machine (hand flow control)
- Card data definitions (RON or hardcoded)

**Dependencies:**
- None (foundational)

**Integration Points:**
- RFC-002 will add betting phase to state machine (before resolve)
- RFC-002 will add AI decision-making (replaces manual play)
- RFC-003 will add insurance check (after bust check)

### Alternatives Considered

#### Alternative 1: Web-Based (HTML/CSS/JS)
**Pros:** Easier UI, hot reload trivial, faster prototype
**Cons:** Different tech stack from target (Bevy), throw-away code
**Rejected:** Want to validate Bevy specifically (target platform)

#### Alternative 2: Pure Terminal UI
**Pros:** No UI complexity, ASCII art cards
**Cons:** Harder to visualize, not representative of final game
**Rejected:** Need visual feedback to validate clarity

#### Alternative 3: Recommended (Bevy + Minimal UI)
**Pros:** Validates target platform, reusable code, visual feedback
**Cons:** Bevy UI learning curve
**Selected:** Best fit for goals (technical validation + foundation)

---

## Discussion

### ARCHITECT Notes

**Tight Scope Validation:**
- 8 cards: ✅ Enough to test override + additive rules
- Single round: ✅ Enough to test bust check
- Manual play: ✅ Acceptable for technical validation (not fun, but proves mechanics)
- 12-16 hours: ✅ Fits SOW constraint (≤20 hours)

**What This Validates:**
- ✅ Card interaction rules are implementable
- ✅ Bevy/Rust can handle card game mechanics
- ✅ Evidence/Cover values are roughly balanced
- ✅ Override/additive rules are clear
- ❌ NOT validating: Fun, pacing, AI, betting

**Exit Criteria (Definition of Done):**
- Can play single round manually (Narc → Customer → Player)
- Override works (play Meth, replaces Weed visibly)
- Additive works (Evidence 10 + 20 = 30 displayed correctly)
- Bust check works (Evidence 40 > Cover 30 → "BUSTED" shown)
- Heat accumulation visible (+75 Heat displayed)
- Card values tunable (change Evidence value, see effect immediately)

**Handoff to RFC-002:**
- Card entity model reusable
- Hand state machine extensible (add betting phase)
- Totals calculation reusable
- UI layout expandable (add turn indicators)

### PLAYER Validation

**✅ I approve this scope with clear expectations:**

**What I'm validating in RFC-001-revised:**
- ✅ Override rules are intuitive (Product/Location replacement is clear)
- ✅ Additive rules are clear (Evidence stacks, Cover stacks, math is obvious)
- ✅ Bust condition is fair (Evidence > Cover feels right)
- ✅ Card values are in the right ballpark (can tune later, just need baseline)
- ✅ Bevy/Rust can handle card game (technical de-risk)

**What I'm NOT validating:**
- ❌ NOT validating fun (manual play isn't fun, I accept this)
- ❌ NOT validating pacing (single round doesn't show escalation)
- ❌ NOT validating AI quality (no AI yet)
- ❌ NOT validating betting tension (no betting yet)

**My acceptance criteria:**
- I can look at the screen and immediately understand what's happening
- Playing Meth replaces Weed → I see Meth become "active Product"
- Playing Safe House replaces School Zone → Evidence/Cover totals change visibly
- Evidence 40 > Cover 30 → "BUSTED" displays clearly
- Heat +75 → I see "+75 Heat" somewhere on screen

**8 Cards is Enough:**
- 3 Products: Tests override (Weed → Meth → Heroin chain)
- 2 Locations: Tests override (School Zone → Safe House)
- 2 Evidence: Tests additive (Patrol +5, Surveillance +20 = +25 total)
- 1 Cover: Tests additive (Location base + Alibi)
- Missing Deal Modifiers acceptable (RFC-002 adds them)

**Single Round is Enough:**
- Just need to prove ONE complete loop works (draw → play → resolve)
- 3 rounds add no new mechanics (just repetition)
- Betting adds no new mechanics (just decision-making)
- RFC-002 will add these (that's where fun validation happens)

**Manual Play is Acceptable:**
- I understand this won't be fun to play
- I'm okay with clicking cards for all 3 players
- This is a **technical prototype** for developers, not a playable game
- Goal: Prove mechanics work, not prove game is fun

**Critical Requirement:**
- Once RFC-001-revised is complete (12-16 hours = 1-2 days), **RFC-002 MUST start immediately**
- I don't want to wait weeks with a boring prototype
- By end of RFC-002 (another 15-18 hours = 2-3 days), I need to feel "yes, this could be fun"

**Questions I expect answered by RFC-001-revised completion:**
1. Is the Evidence/Cover math clear? (Can I glance and understand safety margin?)
2. Do override rules feel intuitive? (Does replacing Product/Location make sense?)
3. Are card values roughly balanced? (Is Evidence 40 high? Is Cover 30 enough?)
4. Does Bevy handle this well? (Performance okay? UI workable?)

If answers are "yes" → Proceed to RFC-002 immediately
If answers are "no" → Iterate on card values/UI, then RFC-002

---

## Approval

**Status:** ✅ **APPROVED - Ready for SOW Creation**

**Approvers:**
- ARCHITECT: ✅ **Approved** (tight scope, fits constraint, validates core mechanics)
- PLAYER: ✅ **Approved** (meets technical validation goals, accepts "not fun" limitation)

**Scope Constraint:** 12-16 hours ✅ (within ≤20 hour SOW limit)

**Consensus:**
- 8 cards sufficient for validation ✅
- Single round sufficient for mechanics validation ✅
- Manual play acceptable for technical prototype ✅
- RFC-002 must start immediately after ✅

**Dependencies:**
- None (foundational)

**Success Criteria (From PLAYER):**
1. Evidence/Cover math is clear (glance and understand)
2. Override rules feel intuitive (Product/Location replacement obvious)
3. Card values roughly balanced (tunable later)
4. Bevy handles card game well (performance + UI workable)

**Next Steps:**
1. ✅ **DONE:** PLAYER approved RFC-001-revised
2. **NEXT:** ARCHITECT creates SOW-001 (implementation plan with phases, deliverables, constraints)
3. **THEN:** DEVELOPER implements per SOW-001 (12-16 hours)
4. **IMMEDIATELY AFTER:** ARCHITECT creates RFC-002 (Betting System + AI)

**Critical Path:**
- RFC-001-revised SOW → Implementation (1-2 days)
- RFC-002 Draft → Approval → SOW → Implementation (2-3 days)
- Total to "fun validation": 3-5 days

**Date:** 2025-11-09
