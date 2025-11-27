# RFC-019: Upgrade Stat Choice

## Status

**Implemented** - 2025-11-27 (SOW-019 merged)

## Feature Request

### Player Need

From player perspective: **When my card upgrades, I want to choose how it improves, not just watch numbers go up automatically.**

**Current Problem (RFC-017 Implementation):**
- Upgrades apply flat 10% bonus to predetermined "primary stat"
- No player agency in upgrade process
- Every run upgrades the same way - no variety
- Feels like passive progression, not active decision-making
- Optimal upgrade path is "solved" - no adaptation required

**We need a system that:**
- Gives players meaningful choice when cards upgrade
- Adds roguelike variance to upgrade paths
- Creates different card "builds" across runs
- Keeps UI simple and decisions fast

### Desired Experience

Players should experience:
- **Agency:** "I chose to make my Alibi a Cover monster"
- **Adaptation:** "This run I got different options, had to pivot"
- **Meaningful decisions:** "Do I boost profit or reduce risk?"
- **Run identity:** "My Meth card went full price, last run it was low-heat"
- **Memorable moments:** "Got lucky with Cover/Cover options on my location"

### Specification Requirements

**Upgrade Trigger (unchanged from RFC-017):**
- Card reaches play count threshold (5, 12, 25, 50 plays)
- Upgrade event fires, prompting player choice

**Stat Selection:**
- System randomly selects 2 upgradeable stats from the card
- Player must choose one (no skipping)
- Selected stat receives the tier bonus (+10%, +20%, etc.)
- Unchosen stat remains at current value

**Upgradeable Stats by Card Type:**

| Card Type | Available Stats | Notes |
|-----------|-----------------|-------|
| Product | `price`, `heat` | Price up = more profit, Heat down = safer |
| Location | `evidence`, `cover`, `heat` | Evidence down, Cover up, Heat down |
| Cover | `cover`, `heat` | Cover up, Heat down |
| Insurance | `cover`, `heat_penalty` | Cover up, Heat penalty down |
| DealModifier | `price_mult`, `evidence`, `cover`, `heat` | Most variety |

**Stat Modification Direction:**
- "Good" stats increase: `price`, `cover`, `price_mult`
- "Bad" stats decrease: `evidence`, `heat`, `heat_penalty`
- All modifications use same percentage as tier bonus

**Cards with Only 2 Stats:**
- Always present both options (no randomness needed)
- Product, Cover cards fall into this category
- Still creates choice, just predictable options

**Cards with 3+ Stats:**
- Randomly select 2 from available pool
- Creates run-to-run variance
- Location, DealModifier, Insurance have this variance

**Choice Persistence:**
- Store which stat was upgraded at each tier
- Different stats can be upgraded at different tiers
- Example: Tier 1 = Cover, Tier 2 = Heat, Tier 3 = Cover

**UI Flow:**
1. Card reaches upgrade threshold during hand resolution
2. After hand completes, show upgrade choice modal
3. Display both options with before/after values
4. Player selects one, upgrade applies immediately
5. Resume normal play

### MVP Scope

**Phase 1 includes:**
- Random 2-stat selection for cards with 3+ stats
- Fixed 2-stat choice for cards with exactly 2 stats
- Simple modal UI showing both options
- Before/after stat preview
- Immediate application on selection

**Phase 1 excludes:**
- Upgrade history viewing ("what did I pick at Tier 1?")
- Undo/respec mechanics
- Weighted random selection (all stats equal probability)
- Animation/celebration on upgrade

### Priority Justification

**MEDIUM PRIORITY** - Enhances existing system without blocking anything

**Why MEDIUM:**
- RFC-017 upgrades already work (functional baseline)
- This is a depth/engagement enhancement
- No other systems depend on this change
- Can ship whenever convenient

**Benefits:**
- Transforms passive upgrades into active decisions
- Adds roguelike variance without new content
- Increases replayability (different upgrade paths)
- More interesting permadeath (lose YOUR choices)
- Prevents "solved" optimal upgrade meta

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Random Stat Selection with Player Choice**

#### Current Implementation Analysis (RFC-017)

Reviewed existing code:
- `CharacterState.card_play_counts: HashMap<String, u32>` - tracks plays per card name
- `UpgradeTier::from_play_count()` - determines tier (0/1/2/3/4/5 plays in test mode)
- `HandState.get_card_tier()` - looks up tier for stat calculation
- `calculate_totals()` in `card_engine.rs` - applies flat multiplier to predetermined stats

**Key insight:** Current system applies `tier.multiplier()` to hardcoded "primary stat" per card type. We need to instead store and apply per-stat upgrade choices.

#### Core Mechanism

**Data Model Changes (in `src/save/types.rs`):**

```rust
/// Which stat was upgraded (serializable for save/load)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeableStat {
    Price,           // Product: increase price
    Cover,           // Location/Cover/Insurance: increase cover
    Evidence,        // Location/DealModifier: decrease evidence
    Heat,            // All types: decrease heat
    HeatPenalty,     // Insurance: decrease heat_penalty
    PriceMultiplier, // DealModifier: increase price_mult
}

/// Upgrade choices for a single card (max 5 tiers)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardUpgrades {
    /// Stat chosen at each tier (None = not yet upgraded to this tier)
    pub upgrades: Vec<UpgradeableStat>, // Index 0 = Tier1, Index 1 = Tier2, etc.
}

/// In CharacterState, replace:
/// - card_play_counts: HashMap<String, u32>
/// With:
/// - card_upgrades: HashMap<String, CardUpgrades>
/// - card_play_counts: HashMap<String, u32>  // Keep for tier calculation
```

**Why Vec instead of fixed fields:** Simpler serialization, easier to extend to more tiers, cleaner iteration for stat calculation.

**Stat Selection (new module `src/systems/upgrade_choice.rs`):**

```rust
impl UpgradeableStat {
    /// Get available upgrade stats for a card type
    pub fn available_for(card_type: &CardType) -> Vec<Self> {
        match card_type {
            CardType::Product { .. } => vec![Self::Price, Self::Heat],
            CardType::Location { .. } => vec![Self::Evidence, Self::Cover, Self::Heat],
            CardType::Cover { .. } => vec![Self::Cover, Self::Heat],
            CardType::Insurance { .. } => vec![Self::Cover, Self::HeatPenalty],
            CardType::DealModifier { .. } => vec![
                Self::PriceMultiplier, Self::Evidence, Self::Cover, Self::Heat
            ],
            _ => vec![],
        }
    }

    /// Select 2 random stats (or both if only 2 available)
    pub fn random_pair(card_type: &CardType, rng: &mut impl Rng) -> [Self; 2] {
        let mut available = Self::available_for(card_type);
        if available.len() <= 2 {
            [available[0], available[1]]
        } else {
            available.shuffle(rng);
            [available[0], available[1]]
        }
    }

    /// Whether this stat improves by increasing (true) or decreasing (false)
    pub fn improves_by_increase(&self) -> bool {
        matches!(self, Self::Price | Self::Cover | Self::PriceMultiplier)
    }
}
```

**Stat Calculation Refactor (in `card_engine.rs`):**

```rust
/// Calculate upgrade multiplier for a specific stat on a card
fn get_stat_multiplier(&self, card_name: &str, stat: UpgradeableStat) -> f32 {
    let upgrades = self.card_upgrades.get(card_name);
    let Some(upgrades) = upgrades else { return 1.0 };

    // Count how many times this stat was upgraded
    let upgrade_count = upgrades.upgrades.iter()
        .filter(|&&s| s == stat)
        .count();

    // Each upgrade adds 10% (stacks additively: 1, 2, 3 upgrades = 1.1, 1.2, 1.3)
    1.0 + (upgrade_count as f32 * 0.1)
}

// Then in calculate_totals(), replace:
//   let tier_mult = self.get_card_tier(&card.name).multiplier();
// With:
//   let cover_mult = self.get_stat_multiplier(&card.name, UpgradeableStat::Cover);
//   let heat_mult = self.get_stat_multiplier(&card.name, UpgradeableStat::Heat);
//   // Apply per-stat multipliers
```

**Design decision:** Stack additively (10% per upgrade to same stat) rather than multiplicatively. Simpler math, more predictable for players.

#### Performance Projections

- **Storage:** ~1 byte per upgrade choice + HashMap overhead (~20-40 bytes per card with upgrades)
- **Computation:** O(k) where k = number of upgrades (max 5), called during `calculate_totals()`
- **Memory:** Negligible increase over current HashMap<String, u32>

**Development time:** ~6-8 hours (revised up)
- Data model changes + migration: 2 hours
- Stat calculation refactor: 2 hours
- Upgrade choice UI modal: 2-3 hours
- Tests: 1 hour

#### Technical Risks

**1. Save Migration**
- *Risk:* Existing saves have `card_play_counts` without `card_upgrades`
- *Mitigation:* Use `#[serde(default)]` for backward compatibility; old cards start with no upgrades but keep play counts (they'll get choices on next tier-up)
- *Impact:* Low

**2. RNG Source**
- *Risk:* Need RNG for stat selection; Bevy's `rand` integration
- *Mitigation:* Use `rand::thread_rng()` for now; can add seeded RNG later if replay needed
- *Impact:* Low

**3. UI State Management**
- *Risk:* Pending upgrade choices need to persist if player closes game mid-choice
- *Mitigation:* Queue pending upgrades in CharacterState; show modal on next session start
- *Impact:* Medium (adds state complexity)

**4. Stat Calculation Refactor Scope**
- *Risk:* Current `calculate_totals()` has card-type-specific logic; need to touch all branches
- *Mitigation:* Extract stat modification into helper functions; test each card type
- *Impact:* Medium (main development work)

### System Integration

**Affected Files:**
- `src/save/types.rs` - Add `UpgradeableStat`, `CardUpgrades`, extend `CharacterState`
- `src/models/hand_state/card_engine.rs` - Refactor `calculate_totals()` for per-stat multipliers
- `src/models/hand_state/mod.rs` - Add `card_upgrades` field to `HandState`
- `src/systems/save_integration.rs` - Sync `card_upgrades` between HandState and CharacterState
- `src/ui/systems.rs` - Add upgrade choice modal
- `src/systems/input.rs` - Handle upgrade choice input

**Compatibility:**
- ✅ Builds on RFC-017 play count tracking (kept for tier calculation)
- ✅ Extends existing save system with serde defaults
- ✅ Uses existing UI patterns (modal like fold decision)
- ✅ No changes to card data files (upgrade choices are runtime state)

### Alternatives Considered

#### Alternative 1: Show All Stats, Player Picks Any

Player sees all upgradeable stats for the card, picks one.

**Rejected because:**
- Optimal path becomes "solved" (always pick best stat)
- No run-to-run variance
- More UI complexity (3-4 options instead of 2)

#### Alternative 2: Fully Random (No Choice)

System randomly picks which stat to upgrade.

**Rejected because:**
- No player agency
- Feels like current system but more chaotic
- "RNG screwed me" with no recourse

#### Alternative 3: Allow Skipping Upgrade

Player can decline both options and wait for next tier.

**Rejected because:**
- Optimal play = skip until good roll
- Encourages save-scumming mentality
- Reduces tension of meaningful choice

#### Alternative 4: Per-Tier Multiplier (Current RFC-017)

Keep flat bonus but let player choose stat.

**Considered but modified:** The RFC proposes additive stacking instead. If you upgrade Cover 3 times, you get +30% cover (not 1.1 × 1.1 × 1.1 = 1.331). Simpler for players to reason about.

---

## Discussion

### ARCHITECT Notes

**Reviewed 2025-11-27:**

1. **Clean extension of RFC-017**: The existing `card_play_counts` HashMap remains for tier calculation. We add `card_upgrades` HashMap alongside it. No breaking changes to existing save files.

2. **Additive stacking decision**: Chose +10% per upgrade rather than multiplicative (1.1^n). Reasoning:
   - Easier for players to predict ("3 upgrades = +30%")
   - Prevents exponential scaling at high tiers
   - Same stat can be upgraded multiple times with diminishing relative returns

3. **UI timing**: Upgrade choice modal triggers after hand resolution, not during play. This maintains game flow and gives player time to consider. Similar pattern to existing fold decision UI.

4. **Pending upgrades queue**: If multiple cards tier up in same hand, queue all pending choices. Process one at a time. Persist queue to save file in case of quit mid-choice.

5. **Evidence cards (Narc)**: These are NOT player-upgradeable. They use `narc_upgrade_tier` from Heat system (RFC-018). Only player cards get stat choice.

6. **Scope fits SOW**: 6-8 hours is within the ≤20 hour SOW limit. Main work is the `calculate_totals()` refactor and UI modal.

### PLAYER Validation

Success criteria from player perspective:
- ✅ Active choice replaces passive bonus
- ✅ Roguelike variance via random stat selection
- ✅ Simple UI (always exactly 2 options)
- ✅ No "correct" answer - both options viable
- ✅ Run identity through upgrade path divergence

---

## Approval

**Status:** Approved

**Approvers:**
- ARCHITECT: [✅] Feasible - clean extension of RFC-017, reasonable scope, no breaking changes
- PLAYER: [✅] Solves autonomy/agency concern, adds meaningful variance

**Scope Constraint:** ~6-8 hours (fits in one SOW)

**Dependencies:**
- RFC-017: Player Card Upgrades ✅ Implemented

**Next Steps:**
1. ~~ARCHITECT feasibility review~~ ✅
2. ~~Approve RFC~~ ✅
3. Create SOW-019
4. Implement upgrade choice system
5. Add UI modal for stat selection

**Date:** 2025-11-27
