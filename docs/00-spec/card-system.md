# Card System Specification

**Last Updated:** 2025-11-27

## Overview

Cards are the core gameplay element. Seven card types interact through override rules and stacking mechanics to determine Evidence, Cover, Heat, and Profit for each hand.

---

## Card Types

| Type | Owner | Purpose | Rule |
|------|-------|---------|------|
| Product | Player | Sets base price, adds Heat | Override (last = active) |
| Location | Player/Buyer | Sets base Evidence/Cover | Override (last = active) |
| Evidence | Narc | Increases Evidence total | Additive (stacks) |
| Cover | Player | Increases Cover total | Additive (stacks) |
| DealModifier | Any | Adjusts price/stats | Additive stats, multiplicative price |
| Insurance | Player | Cover + bust protection | Override (last = active) |
| Conviction | Narc | Blocks insurance at Heat threshold | Override (last = active) |

---

## Card Data Structure

```rust
pub enum CardType {
    Product { price: u32, heat: i32 },
    Location { evidence: u32, cover: u32, heat: i32 },
    Evidence { evidence: u32, heat: i32 },
    Cover { cover: u32, heat: i32 },
    DealModifier { price_multiplier: f32, evidence: i32, cover: i32, heat: i32 },
    Insurance { cover: u32, cost: u32, heat_penalty: i32 },
    Conviction { heat_threshold: u32 },
}
```

---

## Interaction Rules

### Override System

For Product, Location, Insurance, and Conviction:
- Only **last played** card of that type is active
- Previous cards of same type go to discard
- Active card queries: `active_product()`, `active_location()`, `active_insurance()`, `active_conviction()`

### Additive Stacking

For Evidence and Cover:
- All cards of that type contribute to total
- Location provides base, Evidence/Cover cards add to it
- Formula: `total = location_base + sum(card_values)`

### Multiplicative Stacking

For Price (via DealModifiers):
- Product provides base price
- DealModifiers with `price_multiplier` multiply the base
- Formula: `price = base_price × multiplier1 × multiplier2 × ...`

### Heat Accumulation

- Sum all Heat values from played cards
- Applied immediately when card is played (not at resolution)
- Persists on character across sessions

---

## Totals Calculation

```rust
pub fn calculate_totals(&self) -> HandTotals {
    // 1. Get active Product (last played) for base price
    // 2. Get active Location (last played) for base evidence/cover
    // 3. Sum all Evidence card values
    // 4. Sum all Cover card values (including Insurance cover)
    // 5. Apply DealModifier multipliers to price
    // 6. Apply DealModifier additive stats
    // 7. Sum all Heat values
}
```

**Output:**
- `evidence`: Total evidence (bust threshold)
- `cover`: Total cover (defense)
- `profit`: Final price after multipliers
- `heat`: Heat accumulated this hand

---

## Card Data Loading

Cards are defined in RON files under `assets/data/`:
- `products.ron` - Product cards
- `locations.ron` - Location cards
- `cover.ron` - Cover cards
- `insurance.ron` - Insurance cards
- `narc_deck.ron` - Evidence and Conviction cards
- `buyer_*.ron` - Buyer reaction deck cards

Loaded at startup via `CardRegistry` resource.

---

## Card Upgrades (RFC-017/019)

Cards improve through play count (per-character, lost on permadeath).

**Tiers:** Base → Tier 1-5 (Foil at max)

**Thresholds:** 0, 3, 8, 15, 25, 40 plays

**Upgradeable Stats by Type:**
| Type | Stats |
|------|-------|
| Product | Price (+10%), Heat (-10%) |
| Location | Evidence (-10%), Cover (+10%), Heat (-10%) |
| Cover | Cover (+10%), Heat (-10%) |
| Insurance | Cover (+10%), Heat Penalty (-10%) |
| DealModifier | Price Multiplier (+10%), Evidence (-10%), Cover (+10%), Heat (-10%) |

Evidence and Conviction cards are not player-upgradeable (Narc cards scale via Heat tier instead).

---

## Narc Scaling (RFC-018)

Narc card stats scale based on character's Heat tier:

| Heat Tier | Range | Evidence Multiplier |
|-----------|-------|---------------------|
| Cold | 0-29 | 1.0× (base) |
| Warm | 30-59 | 1.1× |
| Hot | 60-89 | 1.2× |
| Blazing | 90-119 | 1.3× |
| Scorching | 120-149 | 1.4× |
| Inferno | 150+ | 1.5× |

---

## Deck Structure

**Player Deck (10-20 cards chosen from pool):**
- Products, Locations, Cover, Insurance, DealModifiers
- NO Evidence or Conviction (Narc only)

**Narc Deck:**
- Evidence cards (threat)
- Conviction cards (blocks insurance)

**Buyer Reaction Deck (7 cards per persona):**
- Locations (can override player's)
- DealModifiers (affect price/stats)

---

## Integration

**Requires:**
- Asset loading system (RON files)
- HandState for card tracking

**Feeds Into:**
- Resolution system (Evidence vs Cover)
- Heat system (accumulation)
- Save system (upgrade progress)
