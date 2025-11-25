# SOW-015: Heat & Character Persistence

## Status

**Complete** - 2025-11-25

## References

- **RFC-015:** [Heat & Character Persistence](../01-rfc/015-heat-and-character-persistence.md)
- **Spec:** [Progression System](../00-spec/progression-system.md) (if applicable)
- **Branch:** sow-015-heat-character-persistence (proposed)
- **Implementation Time:** 14-18 hours

---

## Implementation Plan

### Phase 1: Save System Foundation

**Goal:** Establish the save system infrastructure that all persistence features will use.

**Deliverables:**
- `src/save/mod.rs` - Save system module
- `src/save/types.rs` - SaveFile, SaveData, SaveError types
- `src/save/crypto.rs` - HMAC signature generation and verification
- `src/save/io.rs` - Atomic file operations with backup

**Architectural Constraints:**
- Binary format using bincode (not human-readable JSON)
- HMAC-SHA256 signature over serialized data to prevent casual tampering
- Atomic writes: write to temp file, then rename (prevents corruption on crash)
- Backup system: rename existing save to `.bak` before overwriting
- On load failure, attempt recovery from backup
- Validation on load: verify signature, then validate data sanity
- SaveData versioned for future migration support

**Success Criteria:**
- Save file created in appropriate platform location
- Load rejects tampered files (modified bytes invalidate signature)
- Load recovers from backup when primary save corrupted
- Atomic writes prevent partial/corrupted saves on crash
- Tests verify round-trip serialization

**Duration:** 5-6 hours

---

### Phase 2: Character State & Heat Tracking

**Goal:** Implement character lifecycle with Heat accumulation, decay, and permadeath.

**Deliverables:**
- `src/save/character.rs` - CharacterState, CharacterProfile, HeatTier
- Integration with HandState resolution (Heat accumulation)
- Integration with deck start (decay calculation)
- Integration with bust flow (permadeath trigger)

**Architectural Constraints:**
- CharacterState: profile, heat (u32), last_played (u64 Unix timestamp), decks_played, created_at
- CharacterProfile: enum with Default variant (MVP placeholder)
- HeatTier: Cold (0-25), Warm (26-50), Hot (51-75), Scorching (76-100), Inferno (101+)
- Heat accumulation: add heat delta to character at hand resolution (not just HandState)
- Heat decay: calculate at deck start, -1 per hour elapsed, capped at 168 hours max
- last_played timestamp: update at deck END (not start)
- Permadeath: on bust with no insurance, delete CharacterState from save
- New character: created when no character exists in save

**Success Criteria:**
- Heat accumulates from cards played across multiple hands
- Heat decays based on real elapsed time (capped at 168 hours)
- Heat tier correctly calculated from current heat
- Permadeath deletes character (SaveData.character becomes None)
- New game creates fresh character at Heat 0
- Character state persists across game restarts

**Duration:** 5-6 hours

---

### Phase 3: UI Integration

**Goal:** Display Heat information to the player and provide feedback on tier changes.

**Deliverables:**
- Heat display in game UI (current Heat, tier name)
- Pre-deck display showing decay applied and current tier
- Visual indicator of Heat tier (color coding)

**Architectural Constraints:**
- Heat display: show numeric value and tier name
- Decay feedback: when deck starts, show "Heat decayed by X" if decay occurred
- Tier indicator: color-code based on tier (green→yellow→orange→red→purple)
- No blocking UI: decay/tier info shown inline, not modal

**Success Criteria:**
- Player can see current Heat at all times during gameplay
- Player sees tier name alongside Heat value
- Player receives feedback when Heat decays between sessions
- Heat tier visually distinguishable by color

**Duration:** 4-6 hours

---

## Acceptance Criteria

**Functional:**
- Heat accumulates correctly from card play
- Heat decays at -1/hour with 168-hour cap
- Permadeath triggers on bust without insurance
- Character state persists across game sessions
- Tampered save files are rejected

**UX:**
- Heat value always visible during gameplay
- Clear feedback on decay when returning to game
- Tier changes are visually apparent

**Performance:**
- Save/load completes in < 100ms
- No perceptible delay from HMAC calculation

**Code Quality:**
- Save system is modular and extensible (RFC-016, 017, 018 will extend)
- Tests cover serialization round-trip, tampering detection, decay calculation
- No unwraps on file I/O (proper error handling)

---

## Discussion

*This section is populated during implementation with questions, decisions, and deviations.*

---

## Acceptance Review

*This section is populated after implementation is complete.*

---

## Sign-Off

**Reviewed By:** [ARCHITECT Role]
**Date:** TBD
**Decision:** TBD
**Status:** TBD
