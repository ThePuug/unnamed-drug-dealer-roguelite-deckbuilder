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
- Calculate elapsed time since last deck completed
- Apply decay before next deck starts
- Maximum decay per calculation: 168 hours (1 week cap)
- `last_played` timestamp updates at deck END (not start)
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

**Proposed Solution: Character State Component + Real-Time Decay + Save System**

#### Core Mechanism

**Character Profile (MVP placeholder):**
```rust
#[derive(Serialize, Deserialize, Clone, Default)]
enum CharacterProfile {
    #[default]
    Default,
    // Future: Named profiles with narrative flavor
}
```

**Character State (persisted):**
```rust
#[derive(Serialize, Deserialize, Clone)]
struct CharacterState {
    profile: CharacterProfile,
    heat: u32,
    last_played: u64,    // Unix timestamp (seconds)
    decks_played: u32,
    created_at: u64,     // Unix timestamp (seconds)
}
```

**Heat Calculation:**
```rust
// During hand
heat_delta = sum(card.heat_modifier for card in played_cards)

// At hand end (if not busted)
character.heat += heat_delta

// At deck END, update timestamp
character.last_played = current_unix_timestamp()

// Before NEXT deck starts, apply decay
let elapsed_secs = now - character.last_played;
let elapsed_hours = (elapsed_secs / 3600).min(168);  // Cap at 1 week
character.heat = character.heat.saturating_sub(elapsed_hours as u32);
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

#### Save System (Foundation)

This RFC establishes the save system that RFC-016, 017, 018 will extend.

**Save File Structure:**
```rust
#[derive(Serialize, Deserialize)]
struct SaveFile {
    version: u32,
    data: Vec<u8>,       // bincode-serialized SaveData
    signature: [u8; 32], // HMAC-SHA256 signature
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    character: Option<CharacterState>,  // None if permadeath occurred
    // Future (RFC-016): account: AccountState,
}
```

**Anti-Tampering (HMAC Signature):**
- Binary format (bincode) - not human-readable
- HMAC-SHA256 signature over serialized data
- Signature verified on load; tampering → save rejected
- Note: Determined hackers can still reverse-engineer; goal is preventing casual edits

**Save Operations:**
```rust
fn save(data: &SaveData) -> Result<(), SaveError> {
    let payload = bincode::serialize(data)?;
    let signature = hmac_sha256(SAVE_KEY, &payload);

    // Atomic write: temp file → rename (prevents corruption)
    write_temp_then_rename(&SaveFile { version: 1, data: payload, signature })
}

fn load() -> Result<SaveData, SaveError> {
    let file: SaveFile = read_file()?;

    // Verify signature
    if !verify_hmac(SAVE_KEY, &file.data, &file.signature) {
        return Err(SaveError::TamperedOrCorrupted);
    }

    // Validate sanity (defense in depth)
    let data: SaveData = bincode::deserialize(&file.data)?;
    data.validate()?;  // e.g., heat < 10000, reasonable values

    Ok(data)
}
```

**Backup System:**
- Before overwriting, rename existing save to `.bak`
- On load failure, attempt recovery from `.bak`
- Single backup slot (not versioned history)

**Dependencies:**
- `serde` + `bincode` for serialization
- `hmac` + `sha2` for signature

#### Performance Projections

- **Storage:** ~100 bytes per character + ~64 bytes overhead (minimal)
- **Computation:** O(1) for decay calculation, O(n) for Heat delta (n = cards played)
- **Development time:** ~14-18 hours (includes save system foundation)

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
- 🆕 Save system created by this RFC (foundation for RFC-016, 017, 018)
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

- This RFC establishes the save system foundation that RFC-016, 017, 018 extend
- Heat tier affects Narc card upgrade level (see RFC-018)
- Character state is foundation for card upgrades (RFC-017 adds `card_play_counts`)
- Account state for cash persistence added by RFC-016 (extends SaveData)
- Consider event system for Heat threshold crossings (UI feedback)
- Unix timestamps (u64) used instead of DateTime to avoid chrono dependency
- Decay capped at 168 hours to prevent edge cases from extended absence
- HMAC signature prevents casual save tampering; accept that determined hackers can bypass

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
- ARCHITECT: [x] Approved (2025-11-25)
- PLAYER: [ ] Pending review

**Scope Constraint:** ~14-18 hours (fits in one SOW, includes save system foundation)

**Dependencies:**
- Hand resolution (exists)
- Bust mechanics (exists)
- `serde`, `bincode`, `hmac`, `sha2` crates (new)

**Next Steps:**
1. PLAYER approval
2. Create SOW-015
3. Implement save system foundation (SaveFile, SaveData, HMAC, atomic writes)
4. Implement CharacterState persistence
5. Add Heat accumulation to hand resolution
6. Add decay calculation at deck start (with 168-hour cap)
7. Add permadeath trigger to bust flow
8. Add Heat display to UI

**Date:** 2025-11-25
