# SOW-016: Account Cash System

## Status

**Complete** - 2025-11-25

## References

- **RFC-016:** [Account Cash System](../01-rfc/016-account-cash-system.md)
- **Spec:** [Progression System](../00-spec/progression-meta.md)
- **Branch:** main (direct implementation on approved RFC)
- **Implementation Time:** 4-6 hours

---

## Implementation Plan

### Phase 1: Save System Extension

**Goal:** Add AccountState to the existing save system infrastructure.

**Deliverables:**
- `AccountState` struct in `src/save/types.rs`
- Integration with `SaveData` (account field, persists across permadeath)
- Validation and tests for account state

**Architectural Constraints:**
- AccountState is separate from CharacterState (survives permadeath)
- `cash_on_hand: u64` - Spendable currency (reduced by purchases)
- `lifetime_revenue: u64` - Total ever earned (never reduced, for leaderboards)
- `hands_completed: u32` - Counter for successful deals
- Use `#[serde(default)]` for backward compatibility with existing saves
- Validation: cap at reasonable maximum (~1 trillion)

**Success Criteria:**
- AccountState persists in save file
- AccountState survives character permadeath
- Backward-compatible with existing save files (default values)
- Tests verify add_profit, spend, validation

**Duration:** 1-2 hours

---

### Phase 2: Cash Earning Integration

**Goal:** Connect profit calculation to account-wide cash accumulation.

**Deliverables:**
- `last_profit` field on HandState to track per-hand earnings
- Update resolution code to set `last_profit`
- Update `save_after_resolution_system` to add profit to account on Safe outcome

**Architectural Constraints:**
- Cash earned only on Safe outcome (not Folded, Busted, InvalidDeal, BuyerBailed)
- Profit calculated from existing `totals.profit` (Product price * multipliers)
- Both `cash_on_hand` and `lifetime_revenue` increment on earn
- Character death does NOT affect account cash (survives permadeath)

**Success Criteria:**
- Successful deal adds profit to account.cash_on_hand
- Successful deal adds profit to account.lifetime_revenue
- Failed deals don't add cash
- Cash persists after character permadeath

**Duration:** 1-2 hours

---

### Phase 3: UI Display

**Goal:** Show account cash to the player in the deck builder.

**Deliverables:**
- Cash display component in deck builder UI
- Show current cash on hand
- Show lifetime revenue (smaller/secondary display)

**Architectural Constraints:**
- Display in deck builder (persistent currency visible before runs)
- Format large numbers with commas for readability
- Cash display separate from per-run cash (HandState.cash)
- No spending UI in this SOW (deferred to location unlock RFC)

**Success Criteria:**
- Player sees current cash on hand in deck builder
- Player sees lifetime revenue
- Display updates after completing hands

**Duration:** 1-2 hours

---

## Acceptance Criteria

**Functional:**
- Account cash accumulates from successful deals
- Account cash survives character permadeath
- Lifetime revenue tracks total ever earned (not reduced by spending)
- Save file contains account state

**UX:**
- Cash visible in deck builder before starting runs
- Clear distinction between cash on hand and lifetime revenue

**Performance:**
- No additional save/load overhead (integrated with existing system)

**Code Quality:**
- AccountState modular and extensible for future spending features
- Tests cover profit accumulation, persistence, permadeath survival
- Backward-compatible with existing saves

---

## Discussion

*This section is populated during implementation with questions, decisions, and deviations.*

### Implementation Notes

**Phase 1 Complete:**
- Added `AccountState` struct to `src/save/types.rs`
- Added `account` field to `SaveData` with `#[serde(default)]`
- Added `MAX_CASH` constant (999,999,999,999)
- Implemented `add_profit()`, `spend()`, `validate()` methods
- Added 7 unit tests for AccountState
- All 34 save module tests passing

**Phase 2 Complete:**
- Added `last_profit` field to `HandState` in `src/models/hand_state/mod.rs`
- Updated resolution code (`src/models/hand_state/resolution.rs`) to set `last_profit` on resolution
- Updated `save_after_resolution_system` to add profit to account on Safe outcome
- Profit is added to account BEFORE checking for permadeath, so cash earned in final hand survives
- Account cash persists through permadeath (character dies, cash stays)

**Phase 3 Complete:**
- Added `AccountCashText` and `LifetimeRevenueText` UI components
- Added cash display to deck builder in `src/ui/setup.rs`
- Added `update_account_cash_display_system` to update UI text
- Added `format_number()` helper for comma-separated display
- Cash shown in green, lifetime revenue in grey

---

## Acceptance Review

### Acceptance Criteria Assessment

**Functional:**
- ✅ Account cash accumulates from successful deals
- ✅ Account cash survives character permadeath
- ✅ Lifetime revenue tracks total ever earned (not reduced by spending)
- ✅ Save file contains account state

**UX:**
- ✅ Cash visible in deck builder before starting runs
- ✅ Clear distinction between cash on hand and lifetime revenue

**Performance:**
- ✅ No additional save/load overhead (integrated with existing system)

**Code Quality:**
- ✅ AccountState modular and extensible for future spending features
- ✅ Tests cover profit accumulation, persistence (7 new tests)
- ✅ Backward-compatible with existing saves via `#[serde(default)]`
- ✅ All 70 tests passing

---

## Sign-Off

**Reviewed By:** ARCHITECT
**Date:** 2025-11-25
**Decision:** APPROVED
**Status:** Complete - Ready to merge
