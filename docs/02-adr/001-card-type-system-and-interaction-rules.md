# ADR-001: Card Type System and Interaction Rules

## Status

**Proposed** - 2025-11-09

**Related RFC:** RFC-001-revised (Minimal Playable Hand)

## Context

The game requires a card system that models drug dealing dynamics with clear, predictable interactions. Players need to understand how cards combine to create Evidence, Cover, Heat, and Profit totals.

**Key Requirements:**
- Multiple card types representing different aspects of drug deals (Product, Location, Evidence, Cover)
- Clear rules for how cards interact (some replace, some stack)
- Intuitive mental model (players should predict outcomes)
- Extensible for future card types (Deal Modifiers, Insurance, Conviction)

**Design Constraints:**
- Must support 3-player hands (Narc, Customer, Player)
- Must calculate totals in deterministic order
- Must allow hot-reload for balance tuning
- Must fit Bevy ECS architecture

## Decision

**We use a card type system with two interaction modes: Override and Additive.**

### Core Card Types (RFC-001 MVP)

```rust
enum CardType {
    // Override types (latest played becomes active)
    Product {
        price: u32,      // Base profit
        heat: i32,       // Heat modifier
    },
    Location {
        evidence: u32,   // Evidence base
        cover: u32,      // Cover base
        heat: i32,       // Heat modifier
    },

    // Additive types (stack on base values)
    Evidence {
        evidence: u32,   // Evidence bonus
        heat: i32,       // Heat modifier
    },
    Cover {
        cover: u32,      // Cover bonus
        heat: i32,       // Heat modifier
    },
}
```

### Interaction Rules

**Override Rule:**
- **Product cards:** Latest Product played becomes active (replaces previous)
- **Location cards:** Latest Location played becomes active (replaces previous)
- Rationale: You can only sell one product at one location at a time

**Additive Rule:**
- **Evidence cards:** Add to Location's Evidence base
- **Cover cards:** Add to Location's Cover base
- Rationale: Multiple pieces of evidence stack, multiple cover elements stack

**Heat Accumulation:**
- Sum ALL Heat modifiers from all cards played (additive across all types)
- Rationale: Heat represents cumulative risk from all activities

### Totals Calculation Algorithm

```rust
fn calculate_totals(cards_played: &[Card]) -> HandTotals {
    let mut totals = HandTotals::default();

    // Step 1: Find active Product/Location (last of each type)
    for card in cards_played {
        match card.card_type {
            CardType::Product { price, heat } => {
                totals.active_product = Some(card.id);
                totals.profit = price;
                totals.heat_delta += heat;
            },
            CardType::Location { evidence, cover, heat } => {
                totals.active_location = Some(card.id);
                totals.evidence = evidence;  // Base
                totals.cover = cover;        // Base
                totals.heat_delta += heat;
            },
            _ => {}
        }
    }

    // Step 2: Add Evidence/Cover bonuses
    for card in cards_played {
        match card.card_type {
            CardType::Evidence { evidence, heat } => {
                totals.evidence += evidence;
                totals.heat_delta += heat;
            },
            CardType::Cover { cover, heat } => {
                totals.cover += cover;
                totals.heat_delta += heat;
            },
            _ => {}
        }
    }

    totals
}
```

**Order of operations:**
1. Override pass: Determine active Product/Location (last wins)
2. Additive pass: Sum Evidence/Cover bonuses
3. Heat pass: Already accumulated during 1 & 2

## Rationale

### Why Override for Product/Location?

**Mental Model:** "You can only deal at one place at a time"
- Playing Meth at Safe House, then moving to School Zone → Location changes
- Playing Weed, then upgrading to Meth → Product changes
- Intuitive: Latest action overrides previous

**Gameplay Value:**
- Creates dynamic decisions (swap locations mid-hand for better Evidence/Cover)
- Allows course correction (started at School Zone, too risky, switch to Safe House)
- Enables counterplay (Narc pressure high, play safe Location)

**Alternative Considered:** Additive Products (sell multiple drugs)
- **Rejected:** Breaks fiction (dealing Weed + Meth + Heroin simultaneously is confusing)
- **Rejected:** Creates exponential profit scaling (balance nightmare)

### Why Additive for Evidence/Cover?

**Mental Model:** "Evidence stacks up, Cover protects in layers"
- Patrol + Surveillance = Multiple pieces of evidence
- Alibi + Lawyer = Multiple cover strategies
- Intuitive: More evidence = more danger, more cover = more protection

**Gameplay Value:**
- Creates stacking tension (Evidence climbs as Narc plays cards)
- Rewards defensive play (stack Cover to stay safe)
- Enables counterplay (respond to Evidence with Cover)

**Alternative Considered:** Override Evidence/Cover (latest wins)
- **Rejected:** Breaks gameplay (first Evidence card becomes meaningless)
- **Rejected:** Reduces tension (no accumulation, no escalation)

### Why Universal Heat Accumulation?

**Mental Model:** "Every action increases Heat"
- Product sales generate Heat (higher value = more Heat)
- Risky Locations generate Heat (School Zone is hot)
- Evidence activities generate Heat (investigations raise profile)
- Cover activities can reduce Heat (laying low cools things down)

**Gameplay Value:**
- Creates long-term consequences (actions accumulate across hands)
- Enables strategic Heat management (choose low-Heat cards when Heat is high)
- Feeds into conviction system (Make It Stick cards in RFC-003)

## Consequences

### Positive

- **Clear mental model:** Players quickly learn "Products/Locations replace, Evidence/Cover stack"
- **Predictable outcomes:** Can calculate totals before cards flip
- **Strategic depth:** Override creates swap decisions, additive creates stacking decisions
- **Extensible:** Easy to add new card types (Deal Modifiers in RFC-002, Insurance/Conviction in RFC-003)
- **Balance tunable:** Change card values without changing interaction rules

### Negative

- **Override can feel wasteful:** Playing Weed then Meth makes Weed "wasted" (no profit)
- **Order matters for override:** Last card wins (must track play order)
- **Heat accumulation unbounded:** Heat only goes up (mitigated by Heat decay in Phase 2)

### Mitigations

- **"Wasted" cards:** Intentional design (creates upgrade decisions, not a bug)
- **Play order tracking:** Bevy ECS handles this naturally (iteration order = play order)
- **Unbounded Heat:** Heat decay system planned for Phase 2 (RFC-003+ will address)

## Implementation Notes

### File Structure (Rust/Bevy)

```
src/
  cards/
    mod.rs         - Public card API
    types.rs       - CardType enum, Card struct
    interactions.rs - calculate_totals(), override/additive logic
    data.rs        - Card definitions (RON or hardcoded)

  hand/
    mod.rs         - Hand state machine
    totals.rs      - HandTotals struct, display logic
```

### Data Definition Example (RON)

```ron
Card(
    id: "weed",
    name: "Weed",
    card_type: Product(
        price: 30,
        heat: 5,
    ),
)

Card(
    id: "safe_house",
    name: "Safe House",
    card_type: Location(
        evidence: 10,
        cover: 30,
        heat: -5,
    ),
)
```

### Integration Points

- **Hand Resolution:** Calls `calculate_totals()` after cards flip
- **UI Display:** Reads `HandTotals` to show running totals, highlights active Product/Location
- **Bust Check:** Compares `totals.evidence > totals.cover`
- **Heat Persistence:** Applies `totals.heat_delta` to run's cumulative Heat (Phase 2)

### System Ordering (Bevy)

1. **CardPlaySystem** - Handles card play input, adds cards to table
2. **TotalsCalculationSystem** - Runs `calculate_totals()` when cards flip
3. **DisplaySystem** - Reads `HandTotals`, updates UI
4. **BustCheckSystem** - Reads `HandTotals`, checks Evidence > Cover

**Important:** `TotalsCalculationSystem` must run BEFORE `DisplaySystem` and `BustCheckSystem`

### Testing Strategy

**Unit Tests:**
- Override: Play Product A, then Product B → Only B active
- Override: Play Location A, then Location B → Only B active
- Additive: Evidence 5 + Evidence 20 = 25 total
- Additive: Location (Cover 30) + Cover card (30) = 60 total
- Heat: Sum all Heat modifiers across all cards

**Integration Tests:**
- Full hand: Narc plays Evidence, Customer plays Product, Player plays Cover → Totals correct
- Override chain: Product A → Product B → Product C → Only C profit counted
- Mixed: Product + Location + Evidence + Cover → All interact correctly

## Future Extensions (Post-MVP)

**Deal Modifiers (RFC-002):**
- Multiplicative modifiers for price (×1.3, ×1.5)
- Applied AFTER Product selection, BEFORE profit display

**Insurance Cards (RFC-003):**
- Get Out of Jail: Acts as Cover (additive) + insurance activation
- Override rule: Last insurance played = active (only one)

**Conviction Cards (RFC-003):**
- Make It Stick: No interaction with totals (affects bust resolution only)
- Override rule: Last conviction played = active (only one)

**Combo Bonuses (Phase 3):**
- Product + Location pairs (e.g., Heroin at Safe House = extra Cover)
- Requires extending interaction engine (check pairs, apply bonuses)

## References

- **RFC-001-revised:** Minimal Playable Hand (card types, interaction rules)
- **RFC-002:** Betting System + AI (Deal Modifiers, multiplicative modifiers)
- **RFC-003:** Insurance & Complete Cards (Get Out of Jail, Make It Stick)

## Date

2025-11-09
