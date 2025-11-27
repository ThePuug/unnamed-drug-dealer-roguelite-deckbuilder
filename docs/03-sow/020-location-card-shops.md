# SOW-020: Location Card Shops

## Status

**Complete** - 2025-11-27

## References

- **RFC-020:** [Location Card Shops](../01-rfc/020-location-card-shops.md)
- **Spec:** [Progression & Meta-Game](../00-spec/progression-meta.md)
- **Branch:** (proposed)
- **Implementation Time:** 4-6 hours

---

## Implementation Plan

### Phase 1: Data Foundation

**Goal:** Define locations, assign cards to locations, track unlocks in AccountState

**Deliverables:**
- `assets/data/locations.ron` - Location definitions
- Update card RON files with `location` and `price` fields
- `AccountState` additions: `unlocked_cards`, `unlocked_locations`
- Starting collection definition

**Architectural Constraints:**
- Locations defined in RON, not hardcoded
- Each card belongs to exactly one location
- `#[serde(default)]` for backwards compatibility with existing saves
- Starting collection covers all card types (Product, Location, Cover, etc.)

**Success Criteria:**
- Locations load from RON at startup
- Cards have location and price data
- AccountState persists unlocked cards across sessions
- New saves start with ~15-20 unlocked cards

**Duration:** 1-2 hours

---

### Phase 2: Deck Builder Filtering

**Goal:** Only show unlocked cards in the deck builder card pool

**Deliverables:**
- Update `populate_deck_builder_cards_system` to filter by unlocked cards
- Visual distinction for cards (all shown cards are unlocked)

**Architectural Constraints:**
- Filter applied when populating card pool, not at selection time
- Must work with existing card display helpers
- No changes to deck validation logic (unlocked cards are valid)

**Success Criteria:**
- New player sees only starting collection (~15-20 cards)
- Unlocked cards appear in card pool
- Locked cards do not appear

**Duration:** 1 hour

---

### Phase 3: Shop UI

**Goal:** Add shop interface for browsing and purchasing cards

**Deliverables:**
- Location selector (tabs or buttons) in deck builder
- Shop panel showing cards at selected location with prices
- Purchase button with cash deduction
- Locked/unlocked visual states

**Architectural Constraints:**
- Shop UI integrated into deck builder (not separate screen)
- Purchase immediately updates AccountState and saves
- Show card price and player's current cash
- Disable purchase if insufficient funds

**Success Criteria:**
- Player can switch between locations
- Cards show prices and locked/unlocked status
- Purchasing deducts cash and unlocks card
- Newly unlocked cards appear in deck builder pool

**Duration:** 2-3 hours

---

## Acceptance Criteria

**Functional:**
- Locations load from RON data
- Cards assigned to locations with prices
- Starting collection provides playable deck options
- Shop allows browsing cards by location
- Purchase flow works correctly (cash deducted, card unlocked, saved)
- Deck builder only shows unlocked cards

**UX:**
- Clear indication of card prices
- Clear indication of locked vs unlocked
- Current cash visible in shop
- Smooth purchase feedback

**Performance:**
- No noticeable load time increase
- Shop UI responsive

**Code Quality:**
- Location data externalized (RON)
- Clean separation of shop UI from deck builder
- Existing tests still pass

---

## Discussion

*This section is populated during implementation with questions, decisions, and deviations.*

---

## Acceptance Review

*This section is populated after implementation is complete.*
