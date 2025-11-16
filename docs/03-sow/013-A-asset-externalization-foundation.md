# SOW-013-A: Asset Externalization - Foundation

## Status

**Planned** - 2025-11-16

## References

- **RFC-013:** [Asset Externalization (RON Data Files)](../01-rfc/013-asset-externalization.md)
- **Branch:** (proposed: `sow-013-a-asset-foundation`)
- **Implementation Time:** 12-16 hours

---

## Implementation Plan

### Phase 1: Dependencies and Asset Structures

**Goal:** Add RON support and create serializable asset structures.

**Deliverables:**
- Add `ron` crate to `Cargo.toml`
- Add `serde` derives to existing types (Card, CardType, BuyerPersona, BuyerScenario, NarrativeFragments)
- Create `src/assets/` module for asset types and loading
- Asset error types for validation

**Architectural Constraints:**
- **WHAT:** All game data types must derive Serialize + Deserialize
- **WHAT:** Asset structures must mirror existing runtime types
- **WHAT:** RON format must support all Rust enum variants
- **WHY:** serde enables automatic RON ↔ Rust conversion
- **WHY:** Mirroring types simplifies migration (minimal code changes)

**Success Criteria:**
- `ron` crate added to dependencies
- Card, CardType, BuyerPersona, BuyerScenario derive Serialize/Deserialize
- NarrativeFragments derives Serialize/Deserialize
- Code compiles with new derives
- All tests pass

**Duration:** 2-3 hours

---

### Phase 2: Create RON Asset Files

**Goal:** Export current game content to RON files.

**Deliverables:**
- `assets/cards/products.ron` - All 9 product cards
- `assets/cards/locations.ron` - All 10 location cards
- `assets/buyers.ron` - All 3 buyer personas with scenarios
- Schema documentation (comments in RON files)

**Architectural Constraints:**
- **WHAT:** RON files must be human-readable with comments
- **WHAT:** Each file must be valid RON syntax
- **WHAT:** All current game content must be represented
- **WHY:** Human-readable enables modding
- **WHY:** Comments serve as schema documentation

**Success Criteria:**
- All 3 RON files created and parseable
- Content matches current hardcoded data exactly
- Files include helpful comments for modders
- RON syntax validates with `ron::from_str()`

**Duration:** 3-4 hours

---

### Phase 3: Asset Loader with Bevy AssetServer

**Goal:** Implement asset loading system using Bevy AssetServer.

**Deliverables:**
- `src/assets/loader.rs` - Asset loading with Bevy AssetServer
- `src/assets/registry.rs` - Asset registry as Bevy Resource
- `src/assets/mod.rs` - Module exports
- Bevy asset plugin integration

**Architectural Constraints:**
- **WHAT:** Asset loading must use Bevy AssetServer
- **WHAT:** Assets must load before game systems run
- **WHAT:** Asset registry must be accessible as Bevy Resource
- **WHAT:** Loading must be asynchronous (Bevy pattern)
- **WHY:** AssetServer provides change detection for future hot-reload
- **WHY:** Resource pattern makes assets globally accessible

**Success Criteria:**
- AssetServer loads RON files successfully
- Asset registry populated with all loaded content
- Loading happens before game initialization
- Asset handles properly tracked

**Duration:** 4-5 hours

---

### Phase 4: Validation and Error Handling

**Goal:** Add strict validation with helpful error messages.

**Deliverables:**
- Value range validation (prices > 0, heat in valid range, etc.)
- Required field validation
- Duplicate ID detection
- Clear error messages for modders
- Validation runs on asset load

**Architectural Constraints:**
- **WHAT:** Invalid assets must prevent game startup
- **WHAT:** Error messages must identify file, line, and issue
- **WHAT:** Validation must check business rules (not just schema)
- **WHY:** Strict validation prevents broken game states
- **WHY:** Clear errors help modders fix issues quickly

**Success Criteria:**
- Invalid RON syntax shows file + line number
- Missing required fields show clear error
- Invalid value ranges rejected with explanation
- Duplicate IDs detected and reported
- Game refuses to start with invalid assets

**Duration:** 3-4 hours

---

## Acceptance Criteria

**Functional:**
- ✅ RON files load successfully via Bevy AssetServer
- ✅ Asset registry populates with all content
- ✅ Loaded assets match hardcoded data exactly
- ✅ Validation rejects invalid assets with clear errors
- ✅ All existing tests pass (use test_helpers, not assets)

**UX:**
- ✅ Game starts successfully with valid assets
- ✅ Error messages helpful for modders
- ✅ No performance degradation (< 50ms load time)

**Performance:**
- ✅ Asset loading < 50ms on startup
- ✅ No memory overhead beyond asset data itself
- ✅ No runtime performance impact

**Code Quality:**
- ✅ Asset module is self-contained
- ✅ Error types are comprehensive
- ✅ RON files have documentation comments
- ✅ Validation tests cover edge cases

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
