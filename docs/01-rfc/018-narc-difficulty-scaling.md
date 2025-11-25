# RFC-018: Narc Difficulty Scaling

## Status

**Draft** - 2025-11-25

## Feature Request

### Player Need

From player perspective: **The Narc should get more dangerous as my Heat rises, creating escalating tension that makes high-Heat runs thrilling and terrifying.**

**Current Problem:**
Without Heat-based Narc scaling:
- Narc feels same difficulty regardless of Heat
- High Heat has no gameplay consequence (just a number)
- No tension escalation over a run
- Death spiral doesn't feel earned

**We need a system that:**
- Upgrades Narc cards based on player's Heat tier
- Creates noticeable difficulty increase at tier thresholds
- Mirrors player card upgrades (symmetry)
- Makes Heat meaningful beyond just a bust threshold

### Desired Experience

Players should experience:
- **Escalating danger:** Low Heat = easy Narc, high Heat = deadly Narc
- **Predictable scaling:** Know what you're getting into before starting deck
- **Fair difficulty:** Narc strength matches player's risk-taking
- **Symmetry:** Narc upgrades mirror player upgrades (clean mental model)

### Specification Requirements

**Heat Tier → Narc Upgrade Level:**
| Heat Range | Heat Tier | Narc Card Level |
|------------|-----------|-----------------|
| 0-25 | Cold | Base stats |
| 26-50 | Warm | Tier 1 (+10%) |
| 51-75 | Hot | Tier 2 (+20%) |
| 76-100 | Scorching | Tier 3 (+30%) |
| 101+ | Inferno | Tier 4 (+50%) |

**Narc Card Modifications:**
- Evidence cards: Increase Evidence value
- Conviction cards: Threshold unchanged, but paired Evidence higher
- Heat contribution: Increase Heat modifier

**Example - Surveillance Card:**
| Tier | Evidence | Heat |
|------|----------|------|
| Base | +15 | +3 |
| Tier 1 | +17 | +3 |
| Tier 2 | +18 | +4 |
| Tier 3 | +20 | +4 |
| Tier 4 | +23 | +5 |

**Example - Wiretap Card:**
| Tier | Evidence | Heat |
|------|----------|------|
| Base | +25 | +8 |
| Tier 1 | +28 | +9 |
| Tier 2 | +30 | +10 |
| Tier 3 | +33 | +10 |
| Tier 4 | +38 | +12 |

**Timing:**
- Heat tier determined at deck start (before hands)
- Narc upgrade level locked for entire deck
- Heat changes during deck don't affect current Narc
- Next deck uses new Heat tier

**Display:**
- Show current Heat tier before deck starts
- Show Narc difficulty indicator ("Narc: Dangerous" at Tier 2+)
- Optionally show Narc card upgrade indicators during play

### MVP Scope

**Phase 1 includes:**
- Heat tier → Narc upgrade mapping
- 2-3 upgrade tiers (Base, Tier 1, Tier 2)
- Evidence card scaling
- Heat tier display before deck
- Basic difficulty indicator

**Phase 1 excludes:**
- Tier 3 and Tier 4 (match player upgrade rollout)
- Conviction card modifications (thresholds stay same)
- Per-card unique scaling curves
- Animated difficulty warnings

### Priority Justification

**MEDIUM PRIORITY** - Makes Heat meaningful

**Why MEDIUM:**
- Heat system works without Narc scaling (just feels flat)
- Core loop functional with static Narc
- Adds significant tension and strategy
- Should follow Heat system implementation

**Benefits:**
- Heat becomes gameplay-relevant (not just number)
- Natural difficulty curve (no manual tuning per run)
- Rewards Heat management (keep Heat low = easier Narc)
- Creates "death spiral" feel at high Heat

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Heat-Based Stat Multiplier for Narc Cards**

#### Core Mechanism

**Narc Upgrade Calculation:**
```rust
fn get_narc_upgrade_tier(character_heat: u32) -> UpgradeTier {
    match character_heat {
        0..=25 => UpgradeTier::Base,
        26..=50 => UpgradeTier::Tier1,
        51..=75 => UpgradeTier::Tier2,
        76..=100 => UpgradeTier::Tier3,
        _ => UpgradeTier::Tier4,
    }
}
```

**Deck Start:**
```rust
fn start_deck(character: &CharacterState, game: &mut GameState) {
    // Apply decay first
    let current_heat = calculate_decayed_heat(character);

    // Lock Narc tier for this deck
    game.narc_upgrade_tier = get_narc_upgrade_tier(current_heat);
}
```

**Narc Card Stat Resolution:**
```rust
fn get_narc_card_stats(card: &NarcCard, tier: UpgradeTier) -> NarcCardStats {
    let multiplier = match tier {
        UpgradeTier::Base => 1.0,
        UpgradeTier::Tier1 => 1.1,
        UpgradeTier::Tier2 => 1.2,
        UpgradeTier::Tier3 => 1.3,
        UpgradeTier::Tier4 => 1.5,
    };

    NarcCardStats {
        evidence: (card.base_evidence as f32 * multiplier) as i32,
        heat: (card.base_heat as f32 * multiplier) as i32,
        // Conviction thresholds unchanged
        ..card.base_stats
    }
}
```

#### Performance Projections

- **Storage:** 0 bytes (calculated on the fly)
- **Computation:** O(1) tier lookup, O(n) for deck cards
- **Development time:** ~4-8 hours

#### Technical Risks

**1. Balance Spiral**
- *Risk:* High Heat → stronger Narc → more Heat → death spiral too fast
- *Mitigation:* Start with conservative multipliers (1.1, 1.2), tune later
- *Impact:* Medium (affects game feel significantly)

**2. Player Confusion**
- *Risk:* Players don't understand why Narc is harder
- *Mitigation:* Clear pre-deck display, "Heat: 65 (Hot) - Narc cards upgraded"
- *Impact:* Low (UI solves this)

**3. Stat Rounding**
- *Risk:* Multipliers create awkward numbers (+17.6 → 17 or 18?)
- *Mitigation:* Round to nearest integer, or use fixed increments instead
- *Impact:* Low (cosmetic)

### System Integration

**Affected Systems:**
- Deck start (tier determination)
- Narc card resolution (stat calculation)
- UI (difficulty display)
- Heat system (tier boundaries)

**Compatibility:**
- ✅ Heat system exists (RFC-015)
- ✅ Narc cards already have base stats
- ✅ Card stat calculation already exists
- ✅ UI has pre-deck screen

### Alternatives Considered

#### Alternative 1: Narc Deck Composition Change

Higher Heat = different cards in Narc deck (more Warrants, etc).

**Rejected because:**
- More complex to balance
- Less predictable for players
- Spec moved to "variety via locations" model
- Heat should control power, not variety

#### Alternative 2: Fixed Difficulty Tiers (Not Heat-Based)

Player chooses difficulty at run start.

**Rejected because:**
- Removes Heat consequence
- No in-run escalation
- Doesn't match roguelite design

#### Alternative 3: Per-Card Heat Thresholds

Each Narc card upgrades at different Heat levels.

**Rejected because:**
- Complex to communicate
- Hard to balance
- Uniform scaling is cleaner

---

## Discussion

### ARCHITECT Notes

- Uses same UpgradeTier enum as player cards (consistency)
- Narc tier locked at deck start (predictable)
- Same multipliers as player upgrades (symmetry)
- Consider visual indicator on Narc cards showing upgrade level

### PLAYER Validation

Success criteria from spec:
- ✅ Heat controls difficulty (Narc gets stronger)
- ✅ Mirrors player upgrades (same tier system)
- ✅ Predictable (know difficulty before deck)
- ✅ Fair (high Heat = player chose risky plays)

---

## Approval

**Status:** Draft

**Approvers:**
- ARCHITECT: [ ] Pending review
- PLAYER: [ ] Pending review

**Scope Constraint:** ~4-8 hours (fits in one SOW)

**Dependencies:**
- RFC-015: Heat & Character Persistence (Heat tiers)
- Narc card system (exists)

**Next Steps:**
1. Review and approve RFC
2. Create SOW-018
3. Add narc_upgrade_tier to GameState
4. Lock tier at deck start
5. Apply multipliers to Narc card stat resolution
6. Add difficulty indicator to pre-deck UI
7. Playtest and tune multipliers

**Date:** 2025-11-25
