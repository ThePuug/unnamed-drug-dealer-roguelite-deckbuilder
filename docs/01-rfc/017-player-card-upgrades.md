# RFC-017: Player Card Upgrades (Per-Run)

## Status

**Approved** - 2025-11-25

## Feature Request

### Player Need

From player perspective: **My favorite cards should get stronger the more I use them, making each run feel unique and rewarding commitment.**

**Current Problem:**
Without per-run card upgrades:
- Cards feel static (same stats every time)
- No in-run progression beyond Heat/cash
- No reward for card commitment
- Permadeath only takes away (Heat), doesn't lose growth
- Runs feel interchangeable (no character identity)

**We need a system that:**
- Tracks how often each card is played during a character's life
- Upgrades cards when play thresholds are reached
- Creates meaningful loss on permadeath (lose upgrades)
- Rewards building around specific cards

### Desired Experience

Players should experience:
- **In-run growth:** Cards visibly improve over time
- **Build identity:** "This run I'm focusing on Cover cards"
- **Commitment reward:** Using same cards pays off
- **Meaningful loss:** Permadeath hurts (lose upgraded cards)
- **Fresh restarts:** New character = base cards = new opportunity

### Specification Requirements

**Play Tracking:**
- Count times each card is played per character
- Increment when card is played during hand (not just held)
- Persist count across decks (same character)
- Reset to 0 on permadeath

**Upgrade Thresholds:**
| Plays | Upgrade Tier | Stat Bonus |
|-------|--------------|------------|
| 0-4 | Base | No bonus |
| 5-11 | Tier 1 | +10% to primary stat |
| 12-24 | Tier 2 | +20% to primary stat |
| 25-49 | Tier 3 | +30% to primary stat |
| 50+ | Tier 4 | +50% to primary stat |

**Stat Modifications:**
- Products: Increase base price
- Locations: Increase Cover, decrease Evidence
- Cover cards: Increase Cover value
- Deal Modifiers: Increase multiplier/bonus
- Insurance: Increase Cover, decrease Heat penalty

**Example - Alibi Card:**
- Base: +30 Cover, -5 Heat
- Tier 1 (5 plays): +33 Cover, -5 Heat
- Tier 2 (12 plays): +36 Cover, -5 Heat
- Tier 3 (25 plays): +39 Cover, -5 Heat
- Tier 4 (50 plays): +45 Cover, -5 Heat

**Upgrade Display:**
- Show current tier on card (visual indicator)
- Show progress to next tier (X/Y plays)
- Highlight upgraded stats vs base
- Notification when upgrade achieved

### MVP Scope

**Phase 1 includes:**
- Play count tracking per character
- 2-3 upgrade tiers (Base, Tier 1, Tier 2)
- Flat percentage bonus to primary stat
- Basic UI indicator (tier badge on card)
- Reset on permadeath

**Phase 1 excludes:**
- Tier 3 and Tier 4 (add later for long runs)
- Unique upgrade paths per card
- Secondary stat modifications
- Upgrade animations/celebrations

### Priority Justification

**MEDIUM PRIORITY** - Adds depth but not blocking

**Why MEDIUM:**
- Core loop works without upgrades
- Enhances run identity and commitment
- Creates meaningful permadeath loss
- Can be added after Heat/Cash are working

**Benefits:**
- In-run progression beyond just accumulating
- Rewards skill (use cards efficiently)
- Makes permadeath sting (lose progress)
- Each run develops unique character

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Character-Scoped Play Counts with Tier Calculation**

#### Core Mechanism

**Character State Extension:**
```rust
struct CharacterState {
    // ... existing fields from RFC-015
    card_play_counts: HashMap<CardId, u32>,
}
```

**Play Tracking (during hand):**
```rust
fn on_card_played(card: &Card, character: &mut CharacterState) {
    let count = character.card_play_counts.entry(card.id).or_insert(0);
    *count += 1;

    let old_tier = get_upgrade_tier(*count - 1);
    let new_tier = get_upgrade_tier(*count);

    if new_tier > old_tier {
        emit_upgrade_event(card, new_tier);
    }
}
```

**Tier Calculation:**
```rust
fn get_upgrade_tier(play_count: u32) -> UpgradeTier {
    match play_count {
        0..=4 => UpgradeTier::Base,
        5..=11 => UpgradeTier::Tier1,
        12..=24 => UpgradeTier::Tier2,
        25..=49 => UpgradeTier::Tier3,
        _ => UpgradeTier::Tier4,
    }
}
```

**Stat Modification:**
```rust
fn get_upgraded_stats(card: &Card, tier: UpgradeTier) -> CardStats {
    let multiplier = match tier {
        UpgradeTier::Base => 1.0,
        UpgradeTier::Tier1 => 1.1,
        UpgradeTier::Tier2 => 1.2,
        UpgradeTier::Tier3 => 1.3,
        UpgradeTier::Tier4 => 1.5,
    };

    CardStats {
        primary_value: (card.base_stats.primary_value as f32 * multiplier) as i32,
        ..card.base_stats
    }
}
```

#### Performance Projections

- **Storage:** ~4 bytes per card played (card_id + count)
- **Computation:** O(1) tier lookup, O(1) stat modification
- **Development time:** ~6-10 hours

#### Technical Risks

**1. Stat Calculation Timing**
- *Risk:* When to apply upgrades - at deck build or during play?
- *Mitigation:* Calculate at card play time, cache for display
- *Impact:* Low (clear design decision)

**2. UI Complexity**
- *Risk:* Showing upgrades clutters card display
- *Mitigation:* Subtle tier badge, details on hover/inspect
- *Impact:* Medium (UX consideration)

**3. Balance Impact**
- *Risk:* Tier 4 cards too powerful
- *Mitigation:* Start with Tier 1-2 only, add higher tiers after playtesting
- *Impact:* Medium (tuning required)

### System Integration

**Affected Systems:**
- Character state (play count storage)
- Card rendering (upgrade display)
- Hand resolution (stat calculation)
- Save/load (character persistence)

**Compatibility:**
- ✅ Character state exists (RFC-015)
- ✅ Card stats already calculated during play
- ✅ Card rendering has space for tier indicator
- ✅ Save system handles character state

### Alternatives Considered

#### Alternative 1: Account-Wide Card Upgrades

Cards upgrade permanently (never reset).

**Rejected because:**
- Removes permadeath consequence
- Optimal play = use same cards forever
- Reduces variety and experimentation

#### Alternative 2: Deck-Based Upgrades

Cards upgrade within a single deck, reset between decks.

**Rejected because:**
- Too short for meaningful progression
- No run identity
- Doesn't create attachment

#### Alternative 3: XP-Based Upgrades

Cards gain XP, level up at thresholds.

**Rejected because:**
- More complex than needed
- Same outcome as play counts
- Unnecessary abstraction

---

## Discussion

### ARCHITECT Notes

- Play counts stored in CharacterState (RFC-015 dependency)
- Stat modification applies at calculation time (not stored)
- Consider upgrade events for UI/audio feedback
- Start with 2 tiers for MVP, expand based on run length data

### PLAYER Validation

Success criteria from spec:
- ✅ Cards improve with use (visible progression)
- ✅ Rewards card commitment (use same cards = stronger)
- ✅ Meaningful loss on permadeath (lose all upgrades)
- ✅ Fresh starts feel fresh (base cards again)

---

## Approval

**Status:** Approved

**Approvers:**
- ARCHITECT: [✅] Feasible - extends CharacterState, integrates with existing stat calculation
- PLAYER: [✅] Solves player need for in-run progression and meaningful permadeath

**Scope Constraint:** ~6-10 hours (fits in one SOW)

**MVP Scope Decision:** Start with 2 tiers only (Base + Tier 1) for initial implementation. Add higher tiers after playtesting confirms run lengths support them.

**Dependencies:**
- RFC-015: Heat & Character Persistence (character state) ✅ Implemented
- Card system (stats calculation) ✅ Available

**Next Steps:**
1. ~~Review and approve RFC~~ ✅
2. Create SOW-017
3. Add play_counts to CharacterState
4. Implement play tracking hook
5. Add tier calculation to stat resolution
6. Add upgrade indicator to card UI
7. Test upgrade persistence and reset on death

**Date:** 2025-11-25
