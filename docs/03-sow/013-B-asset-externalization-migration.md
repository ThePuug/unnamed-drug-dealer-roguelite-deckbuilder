# SOW-013-B: Asset Externalization - Complete Migration

## Status

**Planned** - 2025-11-16

## References

- **RFC-013:** [Asset Externalization (RON Data Files)](../01-rfc/013-asset-externalization.md)
- **Depends on:** SOW-013-A (must be merged first)
- **Branch:** (proposed: `sow-013-b-asset-migration`)
- **Implementation Time:** 12-18 hours

---

## Implementation Plan

### Phase 1: Migrate Remaining Card Types

**Goal:** Create RON files for evidence, cover, insurance, and modifier cards.

**Deliverables:**
- `assets/cards/evidence.ron` - All 17 evidence cards (Narc deck)
- `assets/cards/cover.ron` - All 4 cover cards
- `assets/cards/insurance.ron` - All 2 insurance cards
- `assets/cards/modifiers.ron` - All 5 deal modifier cards

**Architectural Constraints:**
- **WHAT:** Content must match current `narc_deck.rs` and `player_deck.rs` exactly
- **WHAT:** RON files must include documentation comments
- **WHAT:** All narrative_fragments must be preserved
- **WHY:** Ensures no content loss during migration

**Success Criteria:**
- All 4 RON files created and parseable
- Total cards: 47 cards across all files
- Content validates against current implementation
- Asset loader successfully loads all card types

**Duration:** 4-5 hours

---

### Phase 2: Replace Hardcoded Deck Creation

**Goal:** Update deck creation functions to load from asset registry.

**Deliverables:**
- Update `create_player_deck()` to load from registry
- Update `create_narc_deck()` to load from registry
- Update `create_buyer_personas()` to load from registry
- Remove hardcoded data from `src/data/*.rs` files

**Architectural Constraints:**
- **WHAT:** Deck creation must use AssetRegistry Resource
- **WHAT:** All cards must come from loaded assets
- **WHAT:** Deck builders must preserve shuffling behavior
- **WHY:** Completes migration to data-driven system
- **WHY:** Removes code/content coupling

**Success Criteria:**
- Player deck loads all 20 cards from RON
- Narc deck loads all 25 cards from RON
- Buyer personas load from RON with scenarios + reaction decks
- No hardcoded Card creation remains in data/ files
- Game plays identically to pre-migration

**Duration:** 4-6 hours

---

### Phase 3: Testing and Validation

**Goal:** Ensure asset system works correctly and provides good errors.

**Deliverables:**
- Asset loading tests (valid RON, invalid RON, missing files)
- Validation tests (invalid values, missing fields, duplicates)
- Integration tests (game starts, decks load correctly)
- Error message documentation for modders

**Architectural Constraints:**
- **WHAT:** Tests must cover all validation cases
- **WHAT:** Error messages must be actionable
- **WHAT:** Tests must not depend on asset files (use test_helpers)
- **WHY:** Comprehensive tests prevent regressions
- **WHY:** Modders need clear guidance when assets invalid

**Success Criteria:**
- All asset loading tests pass
- Validation rejects invalid assets with helpful errors
- Game starts successfully with valid assets
- All existing gameplay tests pass
- Modder documentation explains error messages

**Duration:** 2-3 hours

---

### Phase 4: Cleanup and Documentation

**Goal:** Remove obsolete code and document asset system.

**Deliverables:**
- Remove `src/data/player_deck.rs` (content now in assets/cards/*.ron)
- Remove `src/data/narc_deck.rs` (content now in assets/cards/evidence.ron)
- Remove `src/data/buyer_personas.rs` (content now in assets/buyers.ron)
- Keep `src/data/presets.rs` (preset deck builder still uses registry)
- Update README with modding guide
- Add asset schema documentation

**Architectural Constraints:**
- **WHAT:** All hardcoded content must be removed
- **WHAT:** Asset schema must be documented
- **WHAT:** Modding guide must explain RON format
- **WHY:** Prevents dual-source-of-truth issues
- **WHY:** Documentation enables community modding

**Success Criteria:**
- No hardcoded Card/Buyer creation in src/data/
- README includes "Modding Guide" section
- Asset schema documented (field meanings, value ranges)
- Build succeeds after cleanup
- All tests pass

**Duration:** 2-4 hours

---

## Acceptance Criteria

**Functional:**
- ✅ All game content loads from RON files
- ✅ No hardcoded cards/buyers remain in code
- ✅ Asset validation catches all error cases
- ✅ Game plays identically to pre-migration
- ✅ All 36+ tests pass

**UX:**
- ✅ Game startup time < 100ms increase
- ✅ Invalid assets show actionable errors
- ✅ Modders can edit RON files and see changes

**Performance:**
- ✅ Total asset loading < 100ms
- ✅ No memory overhead beyond asset data
- ✅ No runtime performance impact

**Code Quality:**
- ✅ Asset module well-documented
- ✅ Modding guide complete
- ✅ RON files have schema comments
- ✅ src/data/ cleaned of obsolete code

---

## Discussion

*This section will be populated during implementation with questions, decisions, and deviations.*

---

## Acceptance Review

*This section will be populated after implementation is complete.*

---

## Conclusion

*Summary of what was achieved, impact, and next steps.*

---

## Sign-Off

**Reviewed By:** ARCHITECT Role
**Date:** (pending)
**Decision:** (pending)
**Status:** (pending)
