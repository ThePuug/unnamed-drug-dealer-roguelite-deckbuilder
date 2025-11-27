# RFC-020: Location Card Shops

## Status

**Approved** - 2025-11-27

## Feature Request

### Player Need

From player perspective: **I want to spend my earned cash to unlock new cards** - Currently cash accumulates but has no use. Players need a way to grow their card collection over time.

**Current Problem:**
Without card shops:
- Cash has no purpose after earning it
- All cards available from the start (no progression)
- No reason to keep playing after experiencing all cards
- Meta-game loop incomplete

**We need a system that:**
- Lets players spend cash to permanently unlock cards
- Gates card pools behind locations
- Provides a sense of collection/progression
- Makes cash meaningful

### Desired Experience

Players should experience:
- **Discovery:** New cards to unlock at each location
- **Investment:** Spending hard-earned cash feels meaningful
- **Progression:** Collection grows over multiple runs
- **Strategy:** Choosing which cards to unlock first

### Specification Requirements

**Location Definitions:**
- 5 locations: The Corner (default), The Block, Downtown, The Docks, The Tower
- Each location has a unique card pool (8-12 cards each)
- The Corner is always unlocked; others require achievements (future RFC)

**Card Ownership:**
- Track unlocked cards in AccountState (survives permadeath)
- Starting collection: ~15-20 basic cards unlocked by default
- Unlocked cards available for all characters

**Shop UI:**
- Location tabs/selector in deck builder
- Browse cards at selected location with prices
- Show locked/unlocked status
- Purchase flow: click → confirm → deduct cash → unlock

**Deck Builder Integration:**
- Only show unlocked cards in card pool
- Clearly indicate card's source location

**Pricing Tiers:**
- Basic: $500-$1,500
- Standard: $2,000-$5,000
- Premium: $8,000-$15,000
- Elite: $20,000+

### MVP Scope

**Phase 1 includes:**
- Location data definitions (RON)
- Card-to-location assignments
- AccountState unlock tracking
- Starting collection
- Shop UI (browse + purchase)
- Deck builder filtering

**Phase 1 excludes:**
- Achievement-gated location unlocks (separate RFC)
- Location-specific backgrounds in shop
- Card preview/details modal
- Purchase confirmation animation

### Priority Justification

**HIGH PRIORITY** - Completes the meta-game loop

**Why high:**
- Cash system already implemented but unused
- Core progression missing
- Players need long-term goals

**Benefits:**
- Makes cash meaningful
- Adds collection gameplay
- Extends replayability

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Location-based Card Registry**

#### Core Mechanism

```
Location (RON data)
├── id: "the_corner"
├── display_name: "The Corner"
├── unlock_requirement: None (or achievement ID)
└── card_pool: ["Weed", "Street Corner", "Hoodie", ...]

Card (existing RON, add field)
├── ... existing fields ...
└── location: "the_corner"
└── price: 1000

AccountState (add fields)
├── unlocked_cards: HashSet<String>
└── unlocked_locations: HashSet<String>
```

**Flow:**
1. Load locations from `assets/data/locations.ron`
2. Cards already have location assignment in their RON
3. Shop UI queries cards by location, shows price/locked status
4. Purchase: deduct cash, add to unlocked_cards, save
5. Deck builder filters by unlocked_cards

#### Performance Projections

- Development: ~4-6 hours
- No runtime overhead (just HashSet lookups)
- Minimal save file size increase

#### Technical Risks

**1. Card Assignment Complexity**
- *Risk:* Manually assigning 40+ cards to locations
- *Mitigation:* Add `location` and `price` fields to existing card RON
- *Impact:* Low - one-time data entry

**2. Starting Collection Balance**
- *Risk:* Too few/many starting cards affects early game
- *Mitigation:* Start with ~15-20 covering all card types
- *Impact:* Low - easily tunable

### System Integration

**Affected Systems:**
- AccountState (new fields)
- Card RON schema (new fields)
- DeckBuilder (filtering)
- UI (new shop panel)

**Compatibility:**
- ✅ Existing save files: `#[serde(default)]` for new fields
- ✅ Existing cards: Add location/price during implementation
- ✅ Cash system: Already tracks cash_on_hand

### Alternatives Considered

#### Alternative 1: Random Card Rewards

Cards unlock randomly after successful runs.

**Rejected because:**
- No player agency
- Cash becomes meaningless
- Less satisfying progression

#### Alternative 2: Single Global Shop

All cards in one shop, no locations.

**Rejected because:**
- Overwhelming card list
- No thematic grouping
- Less sense of exploration

---

## Discussion

### ARCHITECT Notes

- Location as enum vs string ID: Use string for flexibility
- Consider lazy-loading location data (not needed for MVP)
- Shop UI can reuse card display helpers

### PLAYER Validation

- Matches spec: locations gate card pools
- Satisfies cash spending need
- Creates meaningful meta-progression

---

## Approval

**Status:** Approved

**Approvers:**
- ARCHITECT: ✅ Feasible, clean integration with existing systems
- PLAYER: ✅ Completes meta-game loop, makes cash meaningful

**Scope Constraint:** Fits in one SOW (~4-6 hours)

**Dependencies:**
- RFC-016 Account Cash System ✅ (implemented)
- Card RON data ✅ (exists)

**Next Steps:**
1. ~~Approve RFC~~ ✅
2. Create SOW-020
3. Implement location data + card assignments
4. Add shop UI to deck builder
5. Update feature matrix

**Date:** 2025-11-27
