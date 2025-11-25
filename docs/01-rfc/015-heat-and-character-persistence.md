# RFC-015: Heat & Character Persistence

## Status

**Draft** - 2025-11-25

## Feature Request

### Player Need

From player perspective: **My choices should have lasting consequences that create tension over multiple play sessions.**

**Current Problem:**
Without Heat and character persistence:
- Each deck feels isolated (no connection to previous sessions)
- No risk escalation over time (every deck equally dangerous)
- No meaningful permadeath (nothing to lose)
- No reason to pace play sessions (binge without consequence)

**We need a system that:**
- Tracks Heat that accumulates from risky plays
- Decays Heat over real-world time (rewarding patience)
- Creates escalating danger as Heat rises
- Persists character state across play sessions
- Ends characters permanently when busted (permadeath)

### Desired Experience

Players should experience:
- **Escalating tension:** Early decks feel safe, late decks feel dangerous
- **Meaningful pacing:** Playing one deck per day is strategically optimal
- **Attachment to character:** Building Heat/progress creates investment
- **Fair permadeath:** Death feels earned (player chose risky plays)
- **Fresh starts:** New character means reset Heat, new opportunity

### Specification Requirements

**Heat Accumulation:**
- Sum all Heat modifiers from cards played during hand
- Apply Heat delta at end of hand (if not busted)
- If folded: Keep Heat from rounds played before fold
- Heat persists on character across decks

**Heat Decay:**
- Decay rate: -1 Heat per real-world hour
- Calculate elapsed time since last deck played
- Apply decay before next deck starts
- Display: Current Heat, time until next decay, projected Heat

**Heat Tiers:**
| Heat Range | Tier Name | Effect |
|------------|-----------|--------|
| 0-25 | Cold | Narc cards at base stats |
| 26-50 | Warm | Narc cards at Tier 1 upgrade |
| 51-75 | Hot | Narc cards at Tier 2 upgrade |
| 76-100 | Scorching | Narc cards at Tier 3 upgrade |
| 101+ | Inferno | Narc cards at Tier 4 upgrade |

**Character Lifecycle:**
- Create character: Choose profile (narrative only), start at Heat 0
- Play decks: Heat accumulates, cash earned, cards upgraded
- Permadeath trigger: Bust (Evidence > Cover, insurance failed)
- On death: Character gone forever

**Permadeath Consequences:**
- **Lost:** Character, Heat, card upgrade progress
- **Preserved:** Cash on hand (account-wide), card unlocks, achievements

### MVP Scope

**Phase 1 includes:**
- Heat accumulation from card play
- Heat decay (real-time calculation)
- Heat tier determination
- Character creation (single profile for now)
- Permadeath on bust
- Basic persistence (Heat survives between sessions)

**Phase 1 excludes:**
- Multiple character profiles (use placeholder)
- Character slots (single character only)
- Heat reduction cards (defer balancing)
- Heat cap (allow infinite for now)

### Priority Justification

**HIGH PRIORITY** - Foundation for all progression systems

**Why HIGH:**
- Every other progression feature depends on character persistence
- Heat is the core difficulty scaling mechanism
- Permadeath creates the roguelite stakes
- Without this, the game has no long-term consequences

**Benefits:**
- Transforms isolated sessions into connected narrative
- Creates natural difficulty curve without manual tuning
- Establishes roguelite identity (meaningful death)
- Enables future systems (cash, upgrades, unlocks)

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: Character State Component + Real-Time Decay**

#### Core Mechanism

**Character State (persisted):**
```rust
struct CharacterState {
    profile: CharacterProfile,
    heat: u32,
    last_played: DateTime<Utc>,
    decks_played: u32,
    card_play_counts: HashMap<CardId, u32>,  // For RFC-017
    created_at: DateTime<Utc>,
}
```

**Heat Calculation:**
```
// During hand
heat_delta = sum(card.heat_modifier for card in played_cards)

// At hand end (if not busted)
character.heat += heat_delta

// Before next deck
elapsed_hours = (now - character.last_played).hours()
character.heat = max(0, character.heat - elapsed_hours)
```

**Heat Tier Lookup:**
```rust
fn get_heat_tier(heat: u32) -> HeatTier {
    match heat {
        0..=25 => HeatTier::Cold,
        26..=50 => HeatTier::Warm,
        51..=75 => HeatTier::Hot,
        76..=100 => HeatTier::Scorching,
        _ => HeatTier::Inferno,
    }
}
```

#### Performance Projections

- **Storage:** ~100 bytes per character (minimal)
- **Computation:** O(1) for decay calculation, O(n) for Heat delta (n = cards played)
- **Development time:** ~8-12 hours

#### Technical Risks

**1. Time Zone / Clock Manipulation**
- *Risk:* Players change system clock to accelerate decay
- *Mitigation:* Use server time if online, or accept single-player exploit
- *Impact:* Low (single-player game, self-cheating)

**2. Save Data Corruption**
- *Risk:* Character state lost or corrupted
- *Mitigation:* Atomic saves, backup previous state, validation on load
- *Impact:* Medium (could lose progress)

**3. Decay During Play**
- *Risk:* Heat decays while actively playing (feels wrong)
- *Mitigation:* Only calculate decay at deck start, not during play
- *Impact:* Low (clear design decision)

### System Integration

**Affected Systems:**
- Hand resolution (Heat accumulation)
- Bust mechanics (permadeath trigger)
- Save/load system (character persistence)
- UI (Heat display, tier warnings)

**Compatibility:**
- ✅ Hand resolution already calculates card stats
- ✅ Bust mechanics already detect Evidence > Cover
- ✅ Save system exists (needs character state extension)
- ✅ UI can display Heat (new HUD element)

### Alternatives Considered

#### Alternative 1: Session-Based Heat (No Persistence)

Heat only lasts within a single play session, resets when game closes.

**Rejected because:**
- No long-term consequences
- No reason to pace play sessions
- Removes roguelite identity

#### Alternative 2: Play-Based Decay (Not Real-Time)

Heat decays by fixed amount per deck played, not real-time.

**Rejected because:**
- Encourages binging (play more = decay more)
- No pacing incentive
- Removes "take a day off" strategy

---

## Discussion

### ARCHITECT Notes

- Heat tier affects Narc card upgrade level (see RFC-018)
- Character state is foundation for card upgrades (RFC-017)
- Cash persistence handled separately (RFC-016) but uses same save system
- Consider event system for Heat threshold crossings (UI feedback)

### PLAYER Validation

Success criteria from spec:
- ✅ Heat feels fair (player chose high-Heat cards)
- ✅ Death spiral feels inevitable but not cheap
- ✅ Daily return rate encouraged (decay incentive)
- ✅ Permadeath feels meaningful (lose Heat progress)

---

## Approval

**Status:** Draft

**Approvers:**
- ARCHITECT: [ ] Pending review
- PLAYER: [ ] Pending review

**Scope Constraint:** ~8-12 hours (fits in one SOW)

**Dependencies:**
- Hand resolution (exists)
- Bust mechanics (exists)
- Save system (exists, needs extension)

**Next Steps:**
1. Review and approve RFC
2. Create SOW-015
3. Implement character state persistence
4. Add Heat accumulation to hand resolution
5. Add decay calculation at deck start
6. Add permadeath trigger to bust flow

**Date:** 2025-11-25
