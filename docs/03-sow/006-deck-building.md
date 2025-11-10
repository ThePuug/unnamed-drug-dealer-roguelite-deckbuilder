# SOW-006: Deck Building

## Status

**Planned** - 2025-11-09

## References

- **RFC-006:** [Deck Building](../01-rfc/006-deck-building.md)
- **Branch:** (to be created)
- **Implementation Time:** 6-8 hours

---

## Implementation Plan

### Phase 1: Deck Builder UI and State

**Goal:** Create deck builder screen with card selection

**Deliverables:**
- `DeckBuilder` resource with available/selected cards
- Deck builder UI screen (card pool grid, selected deck panel)
- Card click to add/remove from deck
- Deck size display and validation indicators
- GameState enum (DeckBuilding, InRun)

**Architectural Constraints:**
- **Game States:** DeckBuilding (shows builder), InRun (shows gameplay)
- **DeckBuilder Resource:**
  ```
  available_cards: Vec<Card> (all 20 player cards)
  selected_cards: Vec<Card> (chosen cards, 10-20)
  ```
- **Card Selection:** Click card in pool → add to selected (if < 20), Click card in selected → remove
- **UI Layout:** Left side = pool grid, Right side = selected deck, Bottom = stats/actions
- **Validation:** Real-time (show red/green indicator for constraints)

**Success Criteria:**
- Deck builder screen displays all 20 cards
- Can click cards to add/remove from deck
- Selected deck updates visually
- Deck size shown (X/20 cards)
- Constraint indicators visible (min 10, max 20, needs Product, needs Location)

**Duration:** 3-4 hours

---

### Phase 2: Deck Validation and Presets

**Goal:** Validate deck constraints and provide preset decks

**Deliverables:**
- Deck validation function (10-20 cards, 1+ Product, 1+ Location)
- 3 preset decks (Default, Aggro, Control)
- Preset buttons (click to load preset)
- START RUN button (enabled when deck valid)
- Validation error messages

**Architectural Constraints:**
- **Validation Rules:**
  - Minimum: 10 cards
  - Maximum: 20 cards
  - Required: At least 1 Product, at least 1 Location
- **Presets:**
  - Default: All 20 cards (current full pool)
  - Aggro: 12-14 cards (high-profit products, risky locations, minimal defense)
  - Control: 15-18 cards (all Cover, all Insurance, defensive modifiers, safe locations)
- **START RUN Button:** Only enabled when deck_valid = true
- **Error Display:** Show what's missing ("Need 1 more card (min 10)" or "Need at least 1 Product")

**Success Criteria:**
- Deck validation prevents invalid decks
- Can't start run with < 10 cards
- Can't start run without Product or Location
- Preset buttons load correct decks
- START RUN button enables/disables based on validity

**Duration:** 2-3 hours

---

### Phase 3: Integration with Game Flow

**Goal:** Connect deck builder to game start and GO HOME

**Deliverables:**
- Game starts in DeckBuilding state (not InRun)
- START RUN transitions to InRun with selected deck
- GO HOME returns to DeckBuilding (preserves selections)
- END RUN returns to DeckBuilding (resets to Default)
- HandState receives custom deck from DeckBuilder

**Architectural Constraints:**
- **Game Start:** App initializes in DeckBuilding state, shows deck builder
- **START RUN:** Transition DeckBuilding → InRun, spawn HandState with selected deck
- **GO HOME:** Transition InRun → DeckBuilding, preserve selected_cards
- **END RUN:** Transition InRun → DeckBuilding, reset to Default deck
- **HandState Creation:** Accept `player_deck: Vec<Card>` parameter instead of calling `create_player_deck()`

**Success Criteria:**
- Game starts showing deck builder (not gameplay)
- START RUN begins game with selected deck
- GO HOME returns to deck builder
- END RUN resets deck to Default
- Custom decks work correctly in gameplay

**Duration:** 1-2 hours

---

## Acceptance Criteria

### Functional

**Deck Builder:**
- ✅ Shows all 20 player cards
- ✅ Can add/remove cards from deck
- ✅ Selected deck updates visually
- ✅ Deck size displayed (X/20)

**Validation:**
- ✅ Prevents < 10 cards
- ✅ Prevents > 20 cards
- ✅ Requires at least 1 Product
- ✅ Requires at least 1 Location
- ✅ START RUN disabled when invalid

**Presets:**
- ✅ Default preset (all 20 cards)
- ✅ Aggro preset (12-14 offensive cards)
- ✅ Control preset (15-18 defensive cards)
- ✅ Preset buttons load correct decks

**Integration:**
- ✅ Game starts in deck builder
- ✅ START RUN begins game with custom deck
- ✅ GO HOME returns to deck builder
- ✅ END RUN resets to default
- ✅ Custom decks work in gameplay

**Edge Cases:**
- ✅ Can't start with 0 cards
- ✅ Can't start without Product
- ✅ Can't start without Location
- ✅ Deck depletion works with custom decks

### UX

**Clarity:**
- ✅ Clear which cards are selected
- ✅ Constraint violations shown clearly
- ✅ Deck stats visible (size, requirements met)

**Usability:**
- ✅ Quick to build deck (click cards)
- ✅ Presets for fast start
- ✅ Can experiment without starting run

### Performance

- ✅ < 16ms per frame (60fps maintained)
- ✅ Deck builder responsive
- ✅ No performance regression

### Code Quality

**Architecture:**
- ✅ Clean separation (DeckBuilder resource, dedicated UI systems)
- ✅ Game state management (DeckBuilding vs InRun)
- ✅ No changes to core mechanics

**Testing:**
- ✅ Unit tests for validation logic
- ✅ Preset decks tested
- ✅ No regressions in existing tests

**Code Size:**
- ✅ Target +400-600 lines (within budget)
- ✅ Total under 4,500 lines (buffer for SOW-007)

**Documentation:**
- ✅ README updated with deck building flow
- ✅ Code comments explain deck builder

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
