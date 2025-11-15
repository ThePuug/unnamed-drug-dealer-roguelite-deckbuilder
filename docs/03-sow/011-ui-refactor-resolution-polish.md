# SOW-011-B: UI Refactor - Hand Resolution & Polish

## Status

**Merged** - 2025-11-15 (Squashed commit: 1698391)

## References

- **RFC-011:** [UI Refactor - Hand Resolution and Card Display](../01-rfc/011-ui-refactor.md)
- **SOW-011-A:** [Core Layout & Foundation](./011-ui-refactor-layout-foundation.md) (prerequisite)
- **Branch:** `feature/sow-011-b-ui-resolution-polish`
- **Implementation Time:** 8-10 hours

---

## Implementation Plan

### Phase 1: Hand Resolution Overlay

**Goal:** Create modal overlay that appears when hand resolves, showing dramatic results breakdown

**Deliverables:**
- Hand resolution overlay component (modal with semi-transparent backdrop)
- Results breakdown display (Evidence, Cover, Heat, Profit, Conviction outcome)
- Dismiss button to continue to next hand
- Overlay visibility toggle system

**Architectural Constraints:**
- Overlay MUST use Display::Flex/None pattern (proven in existing codebase)
- Backdrop MUST dim/semi-transparent background (Color::srgba with alpha)
- Overlay MUST appear on hand resolution (State::Bust)
- Results MUST show final totals (Evidence, Cover, Heat, Profit)
- Conviction outcome MUST be displayed if threshold met
- Buyer bail outcome MUST be displayed if heat exceeded threshold
- Safe/Busted/Folded/InvalidDeal outcomes MUST be color-coded
- Overlay MUST be dismissible via button click only (not click-outside for MVP)
- Overlay entity hierarchy MUST support targeted updates

**Success Criteria:**
- [ ] Overlay appears when hand reaches Bust state
- [ ] Backdrop dims underlying game view
- [ ] Results show Evidence vs Cover comparison
- [ ] Profit/loss displayed with color coding (green=profit, red=loss)
- [ ] Conviction outcome shown if applicable
- [ ] Buyer bail reason shown if applicable
- [ ] "Continue" button dismisses overlay and advances to next hand
- [ ] No visual regressions in existing UI

**Duration:** 3-4 hours

---

### Phase 2: UI Polish & Refinements

**Goal:** Address remaining UI issues and improve visual consistency

**Deliverables:**
- Fix any layout spacing issues
- Improve card replacement visual feedback (highlight effect)
- Refine totals display formatting
- Improve scenario card information layout
- Polish button states and interactions
- Ensure consistent theming throughout

**Architectural Constraints:**
- Card replacement highlight MUST be instant despawn/respawn with BackgroundColor change
- No slide animations for MVP (deferred to post-MVP)
- Highlight duration MUST be brief (~0.5s or instant)
- All text formatting MUST be consistent with theme constants
- Button states MUST be clearly distinguishable (enabled/disabled/hover)
- Scenario card MUST display all key info: scenario name, description, wants, prefers, heat limit
- All spacing MUST use theme constants (no magic numbers)

**Success Criteria:**
- [ ] Card replacement visually noticeable (highlight or flash)
- [ ] Scenario card readable and well-formatted
- [ ] Totals bar clearly displays current values
- [ ] Button states clearly visible
- [ ] No layout overflow at 16:9 aspect ratio
- [ ] Consistent spacing throughout UI
- [ ] All theme constants used (no hardcoded values)

**Duration:** 2-3 hours

---

### Phase 3: Narc Played Cards & Additional Polish

**Goal:** Show Narc's played cards in Narc panel and final polish pass

**Deliverables:**
- Display Narc's played Product/Location cards in Narc panel
- Show Narc's played Evidence/Cover/DealMod in shared pool (already working)
- Update Narc panel to show active cards vs visible hand
- Final visual polish and consistency check

**Architectural Constraints:**
- Narc's Product/Location/Conviction/Insurance MUST go to active slots (shared with Player/Buyer)
- Narc's Evidence/Cover/DealMod MUST go to shared played pool (already implemented)
- Narc panel SHOULD show what Narc has contributed to the deal
- Face-down cards MUST remain face-down until played
- All card displays MUST use helper functions (consistent styling)

**Success Criteria:**
- [ ] Can see which Product/Location Narc played (in active slots)
- [ ] Can see Narc's Evidence/Cover contributions (in played pool)
- [ ] Narc's unplayed hand still shows as face-down "?"
- [ ] Visual hierarchy clear throughout entire UI
- [ ] All features from RFC-011 Phase 1 implemented

**Duration:** 2-3 hours

---

## Acceptance Criteria

### Functional
- Hand resolution overlay appears on hand completion
- All outcome types displayed correctly (Safe/Busted/Folded/InvalidDeal/BuyerBailed)
- Conviction outcome shown when applicable
- Discard pile tracks replaced cards correctly
- Active slots show current Product/Location/Conviction/Insurance
- Heat bar updates dynamically (fill, color, text)
- Totals bar shows current Evidence/Cover/Multiplier
- All cards route to correct locations (slots vs pools)

### UX
- Hand resolution provides satisfying feedback
- Results breakdown clearly explains outcome
- Card replacement visually noticeable
- Scenario card displays all critical information
- Buttons clearly labeled and positioned
- Layout fits 16:9 without overflow
- Visual hierarchy guides player attention
- No cluttered or ambiguous UI elements

### Performance
- Frame rate stable (same as SOW-011-A)
- Overlay appears instantly (no lag)
- Card updates smooth and responsive

### Code Quality
- Resolution overlay uses proven Display::Flex/None pattern
- All new code uses theme constants
- No hardcoded colors or sizes introduced
- System organization maintained (ui/systems.rs)
- Build succeeds, no warnings introduced
- No regressions in existing functionality

---

## Discussion

*This section will be populated during implementation with questions, decisions, and deviations.*

---

## Acceptance Review

**Date:** 2025-11-15
**Reviewer:** ARCHITECT

### Scope Completion: 100%

**All Phases Complete:**
- ✅ Phase 1: Hand Resolution Overlay (~2 hours)
- ✅ Phase 2: UI Polish & Consistent Spacing (~1 hour)
- ✅ Phase 3: Layout & Behavior Fixes (~2 hours)

**Total Time:** ~5 hours (well under 8-10h estimate)

### Implementation Quality

**Hand Resolution Overlay:** ✅ EXCELLENT
- Clean modal pattern using proven Display::Flex/None approach
- Outcome-specific messaging clear and helpful
- Color-coded titles provide instant feedback
- NEW DEAL/GO HOME buttons appropriately placed

**Spacing & Theming:** ✅ EXCELLENT
- Two-tier card sizing system well-designed (Small vs Medium)
- Consistent spacing rules (Small WITH margin, Medium WITHOUT)
- All theme constants properly utilized
- No hardcoded values introduced

**Layout Improvements:** ✅ EXCELLENT
- Slot-based hand system preserves card positions (no confusing reordering)
- Placeholders maintain stable layout during play
- Bottom alignment achieved across all panels
- Discard pile as vertical list (clean, space-efficient)

**UX Improvements:** ✅ EXCELLENT
- Buttons always visible (never disappear unexpectedly)
- Deck builder starts with sensible default
- Deck preserved on return from run
- Alphabetical sorting in deck builder

### Code Quality

**Strengths:**
- Slot-based hand system elegant solution to position preservation
- Discard pile vertical list clean and efficient
- Proper use of SpaceBetween for layout control
- Button visibility/disable logic clear

**No significant issues identified.**

### Player Experience

- Hand resolution overlay provides satisfying conclusion
- No unexpected layout shifts
- Buttons behave predictably (visible but disabled)
- Card positions preserved (less cognitive load)
- Discard pile visible and informative

### Recommendation

✅ **ACCEPT AND MERGE**

SOW-011-B delivers excellent UX improvements and completes RFC-011 implementation. Code quality high, all acceptance criteria met, well under time estimate.

---

## Conclusion

SOW-011-B successfully completes the UI refactor:
- Hand resolution overlay provides dramatic conclusion to each hand
- Stable layouts prevent confusing shifts
- Consistent spacing and sizing throughout
- All player feedback addressed

Combined with SOW-011-A, RFC-011 is now fully implemented with excellent results.

---

## Sign-Off

**Reviewed By:** ARCHITECT Role
**Date:** 2025-11-15
**Decision:** ✅ **ACCEPTED**
**Status:** Ready to merge to main

---

## Conclusion

*To be completed after implementation.*

---

## Sign-Off

**Reviewed By:** (pending implementation)
**Date:** (pending)
**Decision:** (pending)
**Status:** Planned
