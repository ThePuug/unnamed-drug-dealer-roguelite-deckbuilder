# ADR-003: Insurance and Conviction System

## Status

**Accepted** - 2025-11-09 (Implemented in SOW-003)

**Related RFC:** RFC-003 (Insurance and Complete Cards)

## Context

The game needs **high-stakes consequences** for getting busted (Evidence > Cover), but also **clutch escape mechanics** to reward planning and create dramatic moments. This is the risk/reward climax: "I'm busted... but I have insurance!"

**Key Requirements:**
- Insurance mechanic (Get Out of Jail cards save you from bust)
- Cost/Heat tradeoff (insurance isn't free)
- Single-use limitation (can't spam insurance)
- Conviction override (Make It Stick cards bypass insurance at high Heat)
- Clear decision flow (when does insurance activate? when does it fail?)

**Design Goals:**
- **Clutch moments:** Insurance activation feels like "Whew! Survived!"
- **Dread moments:** Conviction override feels like "Oh no, insurance won't work!"
- **Deck building decisions:** Include insurance (survival) vs. more products (profit)?
- **Heat management:** High Heat = conviction threat = must manage carefully

**Design Constraints:**
- Must integrate with bust check (Evidence > Cover from ADR-001)
- Must be understandable (clear when insurance works/fails)
- Must prevent abuse (can't stack infinite insurance)
- Must create meaningful Heat thresholds (Warrant at 40, DA Approval at 60)

## Decision

**We use a two-tier bust resolution system: Get Out of Jail cards (insurance with cost) + Make It Stick cards (conviction overrides insurance at Heat thresholds).**

### Get Out of Jail Cards (Insurance)

```rust
enum CardType {
    // ... existing types from ADR-001

    GetOutOfJail {
        cover: u32,      // Acts as Cover card during hand
        cost: u32,       // Cash requirement to activate
        heat_penalty: i32, // Heat gained if activated
    },
}
```

**MVP Insurance Cards:**
```
Plea Bargain:
  Cover: +20
  Cost: $1,000
  Heat Penalty: +20
  (Balanced: moderate cost, moderate Heat)

Fake ID:
  Cover: +15
  Cost: $0
  Heat Penalty: +40
  (Free but expensive Heat - desperate play)
```

**Insurance Mechanics:**

1. **During hand:** Acts as normal Cover card (adds to Cover total)
2. **At bust (Evidence > Cover):** Attempts to activate
3. **Activation requirements:**
   - Must be able to afford cost (cash ≥ cost)
   - Must not be overridden by Make It Stick (see below)
4. **On activation:**
   - Pay cost (deduct cash)
   - Gain Heat (Evidence overage + heat_penalty)
   - Burn card (remove from deck, can't reuse)
   - Run continues (avoid bust)
5. **On failure:**
   - Run ends (busted)

**Override Rule:** Only ONE insurance active per hand (last Get Out of Jail played)

### Make It Stick Cards (Conviction)

```rust
enum CardType {
    // ... existing types

    MakeItStick {
        heat_threshold: u32,  // Heat level required to activate
    },
}
```

**MVP Conviction Cards:**
```
Warrant:
  Heat Threshold: 40
  (Early conviction - activates at moderate Heat)

DA Approval:
  Heat Threshold: 60
  (Late conviction - activates at high Heat)
```

**Conviction Mechanics:**

1. **Threshold check:** `current_heat >= heat_threshold`
2. **If threshold met:** Overrides Get Out of Jail (insurance fails, run ends)
3. **If threshold not met:** Conviction inactive (insurance works normally)

**Override Rule:** Only ONE conviction active per hand (last Make It Stick played)

### Bust Resolution Flow (Complete)

```rust
fn resolve_bust(
    totals: HandTotals,
    active_insurance: Option<GetOutOfJail>,
    active_conviction: Option<MakeItStick>,
    player_cash: u32,
    current_heat: u32,
) -> BustResult {
    // Step 1: Check if busted
    if totals.evidence <= totals.cover {
        return BustResult::Safe;  // Not busted
    }

    // Step 2: Check conviction override
    if let Some(conviction) = active_conviction {
        if current_heat >= conviction.heat_threshold {
            return BustResult::ConvictionOverride {
                card: conviction,
                heat: current_heat,
            };  // Insurance bypassed, run ends
        }
    }

    // Step 3: Check insurance activation
    if let Some(insurance) = active_insurance {
        if player_cash >= insurance.cost {
            let heat_gain = (totals.evidence - totals.cover) + insurance.heat_penalty;
            return BustResult::InsuranceActivated {
                card: insurance,
                cost_paid: insurance.cost,
                heat_gained: heat_gain,
            };  // Survived
        } else {
            return BustResult::InsuranceFailedCost {
                card: insurance,
                required: insurance.cost,
                available: player_cash,
            };  // Can't afford, run ends
        }
    }

    // Step 4: No insurance
    BustResult::Busted {
        evidence: totals.evidence,
        cover: totals.cover,
    }  // Run ends
}
```

### Heat Overage Calculation

**When insurance activates, Heat gain = Overage + Penalty**

```
Evidence: 85
Cover: 60
Overage: 25

Plea Bargain:
  Heat Penalty: +20
  Total Heat Gain: 25 + 20 = +45 Heat

Fake ID:
  Heat Penalty: +40
  Total Heat Gain: 25 + 40 = +65 Heat
```

**Rationale:** Barely surviving (overage 5) costs less Heat than barely surviving (overage 30)

## Rationale

### Why Cost Requirement?

**Problem without cost:**
- Insurance becomes auto-include (no tradeoff)
- No decision-making ("always have insurance")

**Solution:**
- Cost creates tension ("can I afford it?")
- Profit vs. survival tradeoff (spend $1k to survive or let run end?)
- Deck building decision (include expensive insurance or go without?)

**Poker parallel:** Paying blinds/antes to stay in pot

### Why Heat Penalty?

**Problem without penalty:**
- Insurance becomes consequence-free (bust has no downside)
- Heat becomes meaningless (no reason to manage it)

**Solution:**
- Heat penalty creates long-term consequences (insurance saves short-term, hurts long-term)
- Conviction becomes more likely (high Heat = Make It Stick activates)
- Deck building decision (Fake ID is free but +40 Heat = dangerous later)

**Design goal:** Insurance saves you TODAY, but hurts you TOMORROW

### Why Single-Use (Burn Card)?

**Problem without burn:**
- Insurance reusable = no consequence (spam insurance every hand)
- Deck building trivial ("max out insurance, ignore Cover")

**Solution:**
- Burn after use = limited saves (can't spam)
- Deck building tradeoff (how many insurance cards? 2? 3?)
- Strategic timing (save insurance for late hands or use early?)

**Roguelite parallel:** Consumable items (health potions, revives)

### Why Make It Stick Override?

**Problem without conviction:**
- Insurance always works (no failure case)
- Heat is irrelevant (no reason to keep it low)
- No high-stakes moments (insurance = guaranteed save)

**Solution:**
- Conviction creates dread ("I have insurance but it won't work!")
- Heat management matters (keep Heat below thresholds)
- Deck building changes at high Heat (MUST have insurance, but it might fail)

**Design goal:** Insurance is reliable at low Heat, unreliable at high Heat

### Why Heat Thresholds (40, 60)?

**Balance targets:**
- 0-39 Heat: Safe zone (insurance reliable)
- 40-59 Heat: Warrant zone (insurance risky)
- 60+ Heat: DA Approval zone (insurance very risky)

**Gameplay implications:**
- Early hands: Low Heat, insurance works
- Mid-run: Heat climbing, Warrant threat
- Late-run: High Heat, DA Approval threat, must fold often

**Tunable:** Thresholds adjustable based on playtesting

## Consequences

### Positive

- **Clutch moments:** Insurance activation creates "Whew!" relief
- **Dread moments:** Conviction override creates "Oh no!" panic
- **Deck building depth:** Insurance vs. profit tradeoff
- **Heat matters:** High Heat = conviction threat = strategic Heat management
- **Risk/reward climax:** Push-your-luck with escape hatch (core innovation)

### Negative

- **Complexity:** Bust resolution flow has many branches (conviction → insurance → cost → burn)
- **Insurance might feel mandatory:** Every deck needs 2-3 insurance cards (reduces variety)
- **Fake ID might be dominant:** Free insurance (no cost) might be auto-include
- **Conviction might feel cheap:** "I had insurance but it didn't work" (frustration)

### Mitigations

- **Complexity:** Clear UI messaging (show exactly why bust happened, why insurance failed)
- **Insurance mandatory:** Balance via Heat accumulation (don't need insurance if keep Heat low)
- **Fake ID dominance:** +40 Heat penalty is severe (makes conviction more likely later)
- **Conviction frustration:** BIG warnings when Heat near threshold ("Heat 38 - Warrant at 40")

## Implementation Notes

### File Structure (Rust/Bevy)

```
src/
  cards/
    types.rs       - Add GetOutOfJail, MakeItStick to CardType enum
    insurance.rs   - Insurance activation logic
    conviction.rs  - Conviction threshold check

  hand/
    bust_resolution.rs - Complete bust resolution flow
    totals.rs          - Track active insurance/conviction

  systems/
    bust_check.rs      - Run bust resolution after Round 3
    ui_warnings.rs     - Display conviction warnings, Heat threshold alerts
```

### Data Definitions (RON)

```ron
Card(
    id: "plea_bargain",
    name: "Plea Bargain",
    card_type: GetOutOfJail(
        cover: 20,
        cost: 1000,
        heat_penalty: 20,
    ),
)

Card(
    id: "warrant",
    name: "Warrant",
    card_type: MakeItStick(
        heat_threshold: 40,
    ),
)
```

### Integration Points

- **Bust Check (ADR-001):** After `calculate_totals()`, run `resolve_bust()`
- **Heat System:** Insurance activation adds Heat (overage + penalty)
- **Cash System:** Insurance activation deducts cost (requires cash tracking)
- **Deck System:** Burned insurance cards removed from deck
- **UI System:** Warnings for Heat thresholds, conviction active, insurance status

### System Ordering (Bevy)

**After Round 3:**
1. `TotalsCalculationSystem` - Calculate Evidence/Cover/Heat/Profit (ADR-001)
2. `BustCheckSystem` - Run `resolve_bust()`
3. `InsuranceActivationSystem` - If insurance activates: deduct cost, add Heat, burn card
4. `ConvictionOverrideSystem` - If conviction overrides: end run
5. `UIUpdateSystem` - Display bust result, insurance activation, or conviction override

### UI Messaging (Critical for UX)

**Insurance Activation:**
```
⚠️ BUSTED (Evidence 85 > Cover 60)
✅ INSURANCE ACTIVATED: Plea Bargain
   - Paid: $1,000
   - Heat Gained: +45 (overage 25 + penalty 20)
   - Run continues
```

**Conviction Override:**
```
⚠️ BUSTED (Evidence 85 > Cover 60)
❌ CONVICTION OVERRIDES INSURANCE
   - Warrant Active (Heat Threshold: 40)
   - Your Heat: 65
   - Insurance Failed: Plea Bargain (not used)
   - Run Ends
```

**Insurance Failure (Cost):**
```
⚠️ BUSTED (Evidence 85 > Cover 60)
❌ INSURANCE FAILED: Insufficient Funds
   - Plea Bargain requires: $1,000
   - Your cash: $800
   - Run Ends
```

**Heat Threshold Warnings (Before Hand):**
```
⚠️ WARNING: Heat 38 - Close to Warrant threshold (40)
   - If Narc plays Warrant and you get busted, insurance will fail
   - Consider folding early or playing heavy Cover
```

### Testing Strategy

**Unit Tests:**
- Evidence > Cover, no insurance → Busted
- Evidence > Cover, insurance, can afford → Insurance activates
- Evidence > Cover, insurance, can't afford → Insurance fails
- Evidence > Cover, insurance, conviction threshold met → Conviction overrides
- Evidence ≤ Cover, insurance → Insurance not consumed (still available)

**Integration Tests:**
- Full hand with insurance: Bust → Plea Bargain activates → Heat +45 → Run continues
- Full hand with conviction: Bust → Warrant active (Heat 65) → Insurance fails → Run ends
- Multiple insurance cards: Play 2× Get Out of Jail → Last one active (override rule)

**Balance Tests (Playtest):**
- Insurance value: Is $1,000 worth it for Plea Bargain? (compare to average profit)
- Fake ID viability: Is +40 Heat penalty too harsh? (does it cause conviction later?)
- Conviction frequency: Do Warrant/DA Approval activate too often? (insurance unreliable?)
- Heat thresholds: Are 40/60 the right levels? (tune based on average Heat accumulation)

## Future Extensions (Post-MVP)

**Additional Insurance Types (Phase 2):**
- Bribe Officer: $2,000 cost, +10 Heat (expensive, clean)
- Disappear: $500 cost, +60 Heat (cheap, very dirty)
- Witness Intimidation: $1,500 cost, +35 Heat (mid-range)

**Additional Conviction Types (Phase 2):**
- Caught Red-Handed: Threshold 0 (always active, rare Narc card)
- Federal Case: Threshold 80 (only at extreme Heat)

**Partial Insurance (Phase 3):**
- Insurance covers first $X of overage (e.g., Plea Bargain covers 20 Evidence overage)
- If overage > coverage, player pays extra cost or run ends
- Requires economic model (pay per overage point)

**Insurance Stacking (Phase 3):**
- Allow multiple insurance cards to activate sequentially
- First insurance fails (can't afford) → Try second insurance
- Requires UI complexity (show fallback insurance)

## References

- **RFC-003:** Insurance and Complete Cards (Get Out of Jail, Make It Stick mechanics)
- **ADR-001:** Card Type System and Interaction Rules (bust check integration)
- **ADR-002:** Betting System and Hand Structure (Resolution phase after Round 3)

## Date

2025-11-09
