# RFC-006: Deck Building

## Status

**Approved** - 2025-11-09

## Feature Request

### Player Need

From player perspective: **Every run uses the same 20 cards with no customization - want to build decks that match my playstyle and create strategic variety between runs.**

**Current Problem:**
Without deck building:
- Forced to use all 20 cards every run (no choice)
- Can't specialize (aggressive profit vs defensive survival)
- Every run feels samey (same cards, same strategies)
- Can't adapt to previous run outcomes (busted at high heat? can't build low-heat deck)
- No expression of player skill/preference through deck composition

**We need a system that:**
- Lets players choose which cards to include in their deck
- Creates different viable strategies (aggressive, defensive, balanced)
- Makes deck composition a meaningful pre-run decision
- Allows adaptation (lost last run to conviction? build lower-heat deck)
- Provides variety between runs (different decks = different experiences)

### Desired Experience

Players should experience:
- **Pre-Run Customization:** "Building a high-profit aggro deck" or "Building a defensive Cover-heavy deck"
- **Strategic Expression:** Deck composition reflects playstyle
- **Meaningful Choices:** "Include expensive insurance or more Cover cards?"
- **Adaptation:** "Last run died to Conviction - going low-heat this time"
- **Variety:** Each run feels different based on deck built

### Specification Requirements

**Deck Building Screen (Pre-Run):**
- Card pool: All 20 player cards available
- Deck size: Choose X cards from pool (10-20 range)
- Categories visible: Products, Locations, Cover, Insurance, Modifiers
- Quick filters: By type, by heat, by cost
- Deck stats preview: Total heat potential, average profit, coverage

**Deck Constraints:**
- Minimum deck size: 10 cards (must be viable)
- Maximum deck size: 20 cards (current full pool)
- Required cards: At least 1 Product, 1 Location (core mechanics)
- No duplicates: Each card can only be included once

**Presets:**
- Default deck: Current 20-card full pool (for beginners)
- Aggro deck: High-profit products, risky locations, minimal defense
- Control deck: Heavy Cover, Insurance, defensive modifiers
- Balanced deck: Mix of offense and defense

**UI Flow:**
1. At "GO HOME" or game start: Show deck builder
2. Select cards (click to add/remove)
3. See deck stats update in real-time
4. Validate deck (meets constraints)
5. "START RUN" button (begins game with chosen deck)

### MVP Scope

**Phase 1 includes:**
- Deck builder screen with card selection
- Add/remove cards from deck
- Deck size constraints (10-20)
- Required card validation (1 Product, 1 Location minimum)
- 3 preset decks (Default, Aggro, Control)
- Start run with selected deck

**Phase 1 excludes:**
- Card unlocks / progression (all 20 cards available for now)
- Deck saving/loading (presets only)
- Advanced filters/search (category filter sufficient)
- Deck statistics/analysis (basic count only)
- Card rarity / unlock system (defer to progression RFC)

### Priority Justification

**HIGH PRIORITY** - Core roguelite feature, enables strategic expression

**Why High:**
- Fundamental to roguelite genre (deck customization = core loop)
- Creates replayability (different decks = different strategies)
- Unlocks future features (progression needs deck building foundation)
- Players asking "why can't I choose my cards?"
- Quick win (4-6 hours) with major impact on variety

**Benefits:**
- Strategic depth (deck composition matters)
- Replayability (try different builds)
- Player expression (aggressive vs defensive playstyle)
- Foundation for progression (unlock cards, build better decks)
- Variety between runs (each deck plays differently)

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Pre-Run Deck Builder UI with Card Selection**

#### Core Mechanism

**Current Flow:**
```
Game Start → HandState::default() (fixed 20-card deck) → Play
```

**New Flow:**
```
Game Start → Deck Builder Screen
  → Player selects cards from pool
  → Validate constraints (10-20 cards, 1 Product, 1 Location)
  → START RUN → HandState with custom deck → Play
```

**Data Structure Changes:**
```rust
// New resource for deck builder state
#[derive(Resource)]
struct DeckBuilder {
    available_cards: Vec<Card>,     // All 20 cards in pool
    selected_cards: Vec<Card>,      // Cards chosen for deck (10-20)
    deck_valid: bool,                // Meets constraints?
}

// HandState takes custom deck instead of calling create_player_deck()
impl HandState {
    fn new_with_deck(player_deck: Vec<Card>) -> Self {
        let mut state = Self::default();
        state.player_deck = player_deck;
        state
    }
}
```

**UI Components:**
- Card pool display (grid of all 20 cards)
- Selected deck display (cards chosen)
- Deck stats panel (size, constraints)
- Preset buttons (Default, Aggro, Control)
- START RUN button

#### Performance Projections

**Overhead:**
- Deck builder UI: One-time setup cost
- Card selection: Negligible (20 cards max)
- No performance concerns

**Development Time:**
- UI layout: 2-3 hours (card grid, selection panel, buttons)
- Deck validation: 1 hour (constraints, required cards)
- Presets: 1 hour (3 preset decks)
- Integration: 1-2 hours (connect to game start)
- Testing: 1 hour (validation, presets)
- **Total: 6-8 hours**

#### Technical Risks

**1. UI Complexity**
- *Risk:* Deck builder UI could be complex (drag-drop, filters, etc.)
- *Mitigation:* MVP = simple click to add/remove, basic layout
- *Impact:* Low - keep UI minimal

**2. Deck Persistence**
- *Risk:* Where to store selected deck between runs?
- *Mitigation:* For MVP, reset to Default on "GO HOME" (no persistence)
- *Impact:* Low - acceptable for MVP, add persistence in progression RFC

**3. Balance Issues**
- *Risk:* Some deck compositions might be overpowered
- *Mitigation:* Constraints prevent degenerate decks (min 10, required Product/Location)
- *Impact:* Low - can adjust constraints post-merge

### System Integration

**Affected Systems:**
- Game initialization (add deck builder screen)
- HandState creation (accept custom deck)
- GO HOME button (return to deck builder, not restart)
- UI system (new deck builder UI)

**Compatibility:**
- ✅ Works with all existing systems (just changes initial deck)
- ✅ No changes to card mechanics
- ✅ No changes to betting/insurance/conviction
- ✅ Deck retention (SOW-004) works with custom decks

**Integration Points:**
- Deck builder → HandState (pass selected_cards as player_deck)
- GO HOME → Deck builder (return to selection screen)
- START RUN → Game (begin with selected deck)

### Alternatives Considered

#### Alternative 1: In-Run Deck Editing

**Approach:** Edit deck between hands during run

**Rejected because:**
- Breaks flow (interrupts gameplay)
- Confusing (when can you edit?)
- Deck building is pre-run decision (roguelite standard)

#### Alternative 2: Random Deck Each Run

**Approach:** Game randomly selects 10-15 cards for you

**Rejected because:**
- Removes player agency (want choice, not randomness)
- Frustrating (might get bad deck)
- Doesn't enable strategic expression

#### Alternative 3: Card Drafting (Draft Pool Each Run)

**Approach:** See 3 random cards, pick 1, repeat until deck built

**Rejected because:**
- More complex than needed for MVP
- Slower (many choices before playing)
- Full pool selection simpler and faster

---

## Discussion

### ARCHITECT Notes

**Implementation Approach:**

**New Game State: DeckBuilding**
```rust
enum GameState {
    DeckBuilding,  // New state for deck builder screen
    InRun,         // Existing gameplay
}
```

**UI Structure:**
```
Deck Builder Screen:
├── Card Pool (left) - Grid of all 20 cards
├── Selected Deck (right) - Cards chosen (10-20)
├── Stats Panel (bottom) - Deck size, constraints
└── Actions (bottom) - Presets, START RUN
```

**Preset Decks:**
```rust
fn create_default_deck() -> Vec<Card> {
    // All 20 cards (current behavior)
}

fn create_aggro_deck() -> Vec<Card> {
    // High-profit products (Fentanyl, Cocaine, Heroin)
    // Risky locations (School Zone, Nightclub)
    // Minimal defense (1-2 Cover, no Insurance)
    // ~12-14 cards
}

fn create_control_deck() -> Vec<Card> {
    // All Cover cards (4)
    // All Insurance (2)
    // Defensive modifiers (5)
    // Safe locations (Safe House, Warehouse, Apartment)
    // Conservative products (Weed, Meth)
    // ~15-18 cards
}
```

**Deck Validation:**
```rust
fn validate_deck(deck: &[Card]) -> Result<(), String> {
    if deck.len() < 10 { return Err("Minimum 10 cards"); }
    if deck.len() > 20 { return Err("Maximum 20 cards"); }

    let has_product = deck.iter().any(|c| matches!(c.card_type, CardType::Product { .. }));
    let has_location = deck.iter().any(|c| matches!(c.card_type, CardType::Location { .. }));

    if !has_product { return Err("Must include at least 1 Product"); }
    if !has_location { return Err("Must include at least 1 Location"); }

    Ok(())
}
```

**Code Size Impact:**
- Estimated: +400-600 lines (UI systems, deck builder state, presets)
- Projection: 3,804 + 500 = ~4,300 lines
- Still under 5,000 threshold (buffer: 700 lines)
- Module extraction SOW-007 remains on track

**Simplifications for MVP:**
- No drag-drop (click to add/remove)
- No deck saving (just presets)
- No advanced stats (just card count)
- No filters beyond basic type grouping

### PLAYER Validation

**This enables core roguelite loop:**
- ✅ Strategic variety (each run can be different)
- ✅ Player expression (build reflects playstyle)
- ✅ Adaptation (learn from previous runs)
- ✅ Replayability (try different deck strategies)

**Expected Feel:**
- "Going all-in on profit - Fentanyl, Cocaine, Heroin, no insurance"
- "Survived 6 hands last run, trying pure defense this time"
- "This low-heat build avoids conviction triggers"

**Foundation for Future:**
- Unlocks: Add cards to pool over time
- Progression: Earn better cards
- Achievements: "Win with 10-card deck", "Win without insurance"

---

## Approval

**Status:** Draft → Ready for Approval

**Approvers:**
- ARCHITECT: ✅ Straightforward UI, no technical risks, under size budget
- PLAYER: ✅ Core roguelite feature, high impact on variety

**Scope Constraint:** ✅ Fits in one SOW (6-8 hours)

**Dependencies:**
- Requires SOW-005 (20-card pool available)
- No blocking dependencies

**Next Steps:**
1. ARCHITECT creates SOW-006
2. DEVELOPER implements deck builder
3. Playtest different deck strategies

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
