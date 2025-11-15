# RFC-011: UI Refactor - Hand Resolution and Card Display

## Status

**Approved** - 2025-11-15 (PLAYER and ARCHITECT approved, ready for SOW-011-A)

## Feature Request

### Player Need

From player perspective: **The current hand resolution screen is cluttered and confusing - I can't quickly identify what matters and the scenario "card" doesn't even look like a card in a deckbuilder game.**

**Current Problem:**
Without a proper hand resolution overlay and improved card presentation:
- Resolution happens in the main game space, cluttering the view and making it hard to focus on the outcome
- Scenario information looks like a text box, not a card - breaks the deckbuilder theme
- Product and Location cards don't have clear "active" zones - I can't tell at a glance which is currently in play
- Heat threshold is buried in text - no visual heat bar to show how close I am to the limit
- Conviction/Insurance cards have no designated "active" zone like Product/Location do
- Cover/Modifier cards scattered everywhere - no organized location for support cards
- Product/Location override mechanic is invisible - when a Buyer card replaces mine, there's no visual feedback
- Overall visual hierarchy is flat - everything competes for attention

**We need a system that:**
- Creates an overlay for hand resolution that embellishes the outcome and tells the story
- Makes the scenario display look like an actual card (deckbuilder identity)
- Highlights Product and Location "slots" with clear active/inactive states
- Shows a visual heat bar that fills as heat accumulates toward the threshold
- Provides dedicated "active" slots for Conviction/Insurance cards
- Creates a common zone for Cover/Modifier cards to reduce clutter
- Shows card replacement visually when Buyer overrides Product/Location
- Establishes clear visual hierarchy (scenario → active cards → support cards → hand)

### Desired Experience

**Resolution Clarity:**
- Hand resolves → overlay appears with dramatic results presentation
- Clear success/failure state with profit/loss breakdown
- Evidence, Cover, Heat displayed prominently with visual indicators
- Can see exactly which cards contributed to the outcome
- Feels like a satisfying payoff after playing the hand

**Scenario as Card:**
- Buyer scenario appears as an actual card (like the cards I play)
- Card border, art space, stats layout - looks like it belongs in a deckbuilder
- Immediately recognizable as "the challenge card I'm playing against"
- Heat threshold shown as a heat bar on the card itself

**Active Card Zones:**
- Product slot: "This is the drug I'm selling" - clear highlight when active
- Location slot: "This is where the deal is happening" - clear highlight when active
- Conviction slot: "This is the law enforcement threat" - clear when active
- Insurance slot: "This is my safety net" - clear when active
- Override feedback: When Buyer plays their Location, I see mine return to discard and theirs take its place

**Visual Heat Bar:**
- **Vertical bar** (positioned to the right of scenario card)
- Bar fills from bottom to top as heat accumulates
- Color transitions from green → yellow → red as approaching threshold
- Threshold marker clearly visible on the bar
- Current heat value and threshold displayed numerically (e.g., "65/100")

**Support Card Organization:**
- Cover/Modifier cards in a common zone (not scattered in hand)
- Clearly labeled "Support Cards" or similar
- Easy to see all active modifiers at a glance
- Discard pile visible for cards that have been played and removed

**Reduced Clutter:**
- Everything has a place and a purpose
- No cards floating in ambiguous zones
- Clear grouping: Challenge (scenario) → Active (product/location/conviction/insurance) → Support (cover/modifiers) → Hand
- Empty slots show ghosted placeholders ("No Product", "No Conviction", etc.)

### Specification Requirements

**Hand Resolution Overlay:**
- Modal overlay that appears when hand resolves
- Dims/blurs underlying game view
- Large, centered results panel with outcome breakdown
- Evidence, Cover, Heat displayed as visual bars/meters
- Profit/loss prominently displayed with color coding
- Conviction outcome (if applicable) with threshold comparison
- "Continue" or "Next Deal" button to dismiss

**Scenario Card Display:**
- Card-shaped container (not rectangular text box)
- Card border with persona-specific color theming
- Title area with buyer persona name
- Art/icon placeholder for scenario illustration
- Stats section showing:
  - Scenario name and description
  - Product requirements (icons + names)
  - Location preferences (icons + names)
  - Multiplier value
  - Heat bar (integrated into card)
- Visual heat bar showing current_heat / threshold
- Card dimensions consistent with other cards (scaled up for prominence)

**Active Card Slot System:**
- **Product Slot:**
  - Highlighted border when active
  - Shows active product card
  - Ghosted placeholder when empty ("No Product")
  - Override animation when Buyer replaces with their product
- **Location Slot:**
  - Highlighted border when active
  - Shows active location card
  - Ghosted placeholder when empty ("No Location")
  - Override animation when Buyer plays their location
- **Conviction Slot:**
  - Highlighted border when active
  - Shows active conviction card (RICO, Conspiracy, etc.)
  - Ghosted placeholder when empty ("No Conviction")
  - Grayed out if conviction threshold not met
- **Insurance Slot:**
  - Highlighted border when active
  - Shows active insurance card
  - Ghosted placeholder when empty ("No Insurance")
  - Triggers effect on bust/conviction

**Visual Heat Bar:**
- **Vertical bar** (positioned to the right of scenario card)
- Height: ~200-250px
- Current heat fills from bottom to top
- Background gradient: Green (0-50%) → Yellow (50-80%) → Red (80-100%)
- Threshold marker (horizontal line or notch)
- Numeric display at bottom: "65/100" format
- Pulsing/warning animation when within 5 of threshold (MVP: optional)

**Support Card Zone:**
- Dedicated area for Cover/Modifier cards
- Labeled "Support Cards" or with icon
- Horizontal layout showing all active support cards
- Cards display in compact form (smaller than hand cards)
- Tooltips on hover for full details

**Discard Pile Indicator:**
- Small stack visual showing discard pile count
- Click/hover to expand and view discarded cards
- Shows cards that have been replaced (via override) or removed
- Position near deck/hand area for logical grouping

**Card Replacement Animation:**
- When Buyer overrides Product/Location:
  - Player's card slides out of active slot
  - Visual effect (fade, slide) shows card moving to discard
  - Buyer's card slides into active slot
  - Brief highlight/glow to draw attention to the change

### UI Layout Design (from Figma mockup)

**Screen Layout (16:9 optimized):**

**Top Row (compact, no labels):**
- **Left side:** Active card slots (horizontal row)
  - Location slot (blue) - shows active location or dashed placeholder
  - Product slot (green) - shows active product or dashed placeholder
  - Conviction slot (red) - shows active conviction card or dashed placeholder
  - Insurance slot (purple) - shows active insurance or dashed placeholder
- **Right side:** Buyer scenario card (large, card-styled, orange/brown theme)
  - Icon/art area at top
  - "BUYER SCENARIO" label
  - Scenario title (e.g., "High Roller")
  - Description text
  - Profit indicator (green badge: "+200% profit")
  - Risk indicator (orange/red badge: "medium risk")
- **Far right:** Vertical heat bar (purple/magenta)
  - Shows current/max (e.g., "65/100")
  - Fills from bottom to top

**Middle Play Area:**
- **Left:** Narc section (red border)
  - Shows 3 card backs with "?" placeholders
- **Center-Left:** Evidence/Cover/Deal Mod pools
  - No "Additive Pools" label (removed for space)
  - Shows cards contributing to each pool
  - Played cards displayed horizontally
- **Center-Right:** Discard pile
  - Shows count and top card
- **Right:** Buyer deck panel (orange border)
  - Shows buyer's upcoming cards vertically
  - Card types visible (Insurance, Location, etc.)

**Bottom Row:**
- **Left:** Narc's visible hand (shows upcoming Narc cards)
- **Center:** Your Hand (Dealer) - main play area (teal/green panel)
  - Shows your cards with type icons and stats
  - Action buttons: "Play a card to end turn" (green), "Check (Pass)" (gray)
- **Right:** Buyer's visible hand (shows upcoming Buyer cards)

**Hand Resolution:**
- Modal overlay (appears on top of entire screen)
- Dims/blurs underlying game view
- Shows outcome breakdown, profit/loss, conviction results
- Dismissible with button press

**Design Principles:**
- No section labels where obvious (e.g., "Override Slots", "Additive Pools" removed)
- Dashed outlines for empty slots (ghosted placeholders)
- Strong color coding: Products (green), Locations (blue), Conviction (red), Insurance (purple)
- Symmetrical threat layout: Narc (left) vs Buyer (right), Player (center)
- Compact vertical spacing for 16:9 aspect ratio
- Clear visual hierarchy: Challenge (top) → Active Play (middle) → Your Hand (bottom)

---

### MVP Scope

**Phase 1 includes:**
- Complete layout restructure per Figma design (16:9 optimized)
- Hand resolution overlay with results breakdown
- Scenario card redesign (card-styled with vertical heat bar)
- Active slot system (Product, Location, Conviction, Insurance) - horizontal row, compact
- Vertical heat bar (fills bottom-to-top, positioned right of scenario card)
- Evidence/Cover/Deal Mod pools (no labels, clean presentation)
- Symmetrical hand display (Narc left, Player center, Buyer right)
- Ghosted placeholders for empty slots (dashed outlines)
- Buyer deck panel (right side, shows upcoming cards)
- Discard pile indicator
- Basic card replacement visual feedback

**Phase 1 excludes:**
- Animated transitions (instant state changes acceptable)
- Custom art/illustrations for scenario cards (placeholder icons)
- Expandable discard pile viewer (just show count)
- Tooltips for support cards (show basic stats inline)
- Heat bar pulsing/warning animations (color transitions only)
- Responsive layout (fixed sizes acceptable for MVP)

### Priority Justification

**HIGH PRIORITY** - Core usability issue affecting every hand

**Why HIGH:**
- Affects player understanding of game state (what's happening and why)
- Scenario display breaks deckbuilder thematic consistency
- Clutter actively harms player decision-making
- Resolution feedback is critical to learning game mechanics
- No workarounds available (can't play without understanding outcomes)

**Benefits:**
- **Clarity:** Players immediately understand game state and outcomes
- **Theme:** Scenario as card reinforces deckbuilder identity
- **Usability:** Active slots show exactly what's in play
- **Feedback:** Resolution overlay provides satisfying payoff
- **Scannability:** Visual heat bar replaces buried text information
- **Organization:** Support card zone eliminates clutter
- **Learnability:** Clear visual hierarchy teaches game mechanics

---

## Additional UI Improvements to Consider

Beyond the core requirements, here are additional improvements to clean up the "hellish display":

### 1. Color Palette Standardization
**Problem:** Hardcoded RGB tuples scattered throughout code
**Solution:** Centralized theme module with named color constants
- Persona colors (Frat Bro = vibrant green, Housewife = suburban beige, Wolf = corporate blue)
- Card type colors (Product, Location, Cover, etc. - consistent across all displays)
- State colors (active, inactive, ghosted, warning)
- Benefit: Consistent theming, easy to adjust, improves visual coherence

### 2. Card Component Reusability
**Problem:** Card styling logic duplicated in 3+ functions (player hand, play areas, buyer visible)
**Solution:** Single reusable CardDisplay component
- Parameterized size, state (active/inactive), interaction (clickable/static)
- Consistent stat formatting across all card displays
- Benefit: Reduces code duplication, ensures visual consistency

### 3. Layout Constants
**Problem:** Magic numbers (100px, 120px, 150px) hardcoded everywhere
**Solution:** Named sizing constants (CARD_WIDTH_SMALL, CARD_WIDTH_HAND, etc.)
- Easier to adjust proportions globally
- Self-documenting code
- Benefit: Faster iteration on layout, clearer intent

### 4. Buyer Visible Hand Integration
**Problem:** Buyer's next hand shown separately below main play area
**Solution:** Integrate into scenario card or adjacent panel
- Shows "What's coming next" in context
- Reduces vertical scrolling/scanning
- Benefit: Tighter visual grouping, less eye movement

### 5. Totals Bar Redesign
**Problem:** Status text scattered across top bar (Evidence, Cover, Heat all separate)
**Solution:** Visual meter panel
- Three horizontal bars side-by-side (Evidence, Cover, Heat)
- Each fills to current value with color coding
- Thresholds marked on bars (e.g., Evidence = Conviction threshold)
- Benefit: At-a-glance status, visual instead of numeric

### 6. Narc's Cards Visualization
**Problem:** "Narc's Cards" label with card backs in play area feels disconnected
**Solution:** Integrate into Conviction slot system
- Conviction slot shows "Narc has X cards"
- Expand on click to show card backs count
- On conviction resolution, reveal and animate into slot
- Benefit: Connects dealer threat to conviction mechanic

### 7. Betting Actions Repositioning
**Problem:** Betting buttons (Check/Raise/Fold) at bottom, disconnected from scenario
**Solution:** Position near scenario card or active slots
- Betting decisions relate to scenario/risk assessment
- Closer to relevant information (heat, multiplier, scenario requirements)
- Benefit: Reduces cognitive load, groups related actions

### 8. Hand Area Optimization
**Problem:** Player hand takes 200px height, cards may wrap awkwardly
**Solution:** Collapsible/expandable hand panel
- Show 5-6 cards prominently, collapse rest with indicator
- Click/hover to expand full hand
- Alternative: Horizontal scrollable hand area
- Benefit: More screen space for resolution/active play area

### 9. Empty State Design
**Problem:** Empty play areas show nothing (ambiguous if intentional or bug)
**Solution:** Ghosted placeholders with labels
- "Play a Product card"
- "Play a Location card"
- "No Conviction (Evidence < 80)"
- Benefit: Teaches mechanics, eliminates ambiguity

### 10. Multiplier Visibility
**Problem:** Multiplier shown only in scenario text, easy to miss
**Solution:** Large multiplier badge on scenario card
- "×2.5" in prominent display
- Color-coded (high multiplier = gold, low = gray)
- Shows reduced multiplier if demand not met
- Benefit: Critical decision factor highlighted

---

## Open Questions

1. Should hand resolution overlay be dismissible (click outside) or require button press?
2. Should scenario card show both scenarios (grayed out for inactive one) or only active scenario?
3. Should discard pile be always visible or expandable on demand?
4. Should heat bar animate when heat changes, or instant update?
5. Should active slots pulse/glow when empty to prompt player action?
6. Should support card zone have a limit (max N visible, overflow hidden/scrollable)?
7. Should Buyer visible hand integrate into scenario card, or remain separate panel?
8. Should resolution overlay show card-by-card breakdown (Product: +$50, Location: -10 Heat) or just totals?
9. Should conviction/insurance slots show "Not Active" vs "Empty" (distinguish between no card vs card present but not triggered)?
10. Should color palette support persona theming (different colors per Buyer) or unified theme?

---

## Feasibility Analysis

**Date:** 2025-11-15
**Reviewer:** ARCHITECT
**Status:** ✅ FEASIBLE - Proceed with scope refinement

### Technical Assessment

**Can we build this?** ✅ Yes

All RFC-011 requirements are achievable with Bevy 0.14.2's vanilla UI system. The codebase already demonstrates proven patterns for overlays, card displays, and reactive UI updates.

**Bevy Version:** 0.14.2
**Current UI Architecture:** Monolithic (5,704 lines in main.rs)
**Risk Level:** LOW-MEDIUM

---

### Answers to RFC Questions

**Q: Can Bevy UI support overlay/modal pattern without custom plugin?**
✅ **YES** - Display::Flex/None toggle pattern already used in 3+ places (line 1853). Perfect foundation for hand resolution modal.

**Q: Is card component reusability feasible with current entity-based UI approach?**
✅ **YES** - Card rendering logic duplicated 4 times currently. Extract to helper function `spawn_card_display()` eliminates ~200 lines of duplication.

**Q: Render performance impact of additional UI entities?**
✅ **NEGLIGIBLE** - ~50 additional entities for slots/bars/overlays. Current UI handles hundreds of entities without performance issues.

**Q: Should we refactor monolithic create_ui() before adding features, or incrementally?**
✅ **INCREMENTALLY** - Modularize DURING RFC-011 implementation (Option C). Extract theme constants and card helpers first, then implement new features in modular structure.

**Q: Does Bevy UI support gradient fills for heat bar?**
⚠️ **NO (native)** - Use stacked nodes workaround (3 colored segments: green/yellow/red). Simple, no shader required. Alternative: single-color node that changes color based on percentage.

**Q: How should we handle card replacement animation?**
✅ **Despawn/Respawn** - Existing pattern works. Add brief highlight effect via BackgroundColor transition. Can upgrade to custom animation component post-MVP.

**Q: Estimated scope: fits in one SOW (≤20 hours)?**
⚠️ **SPLIT RECOMMENDED** - Core features fit in 15-21 hours, but recommend splitting into two SOWs for quality:
- **SOW-011-A** (12-15h): Hand resolution overlay + scenario card redesign + theme extraction
- **SOW-011-B** (10-12h): Active slots + support zone + card replacement feedback

---

### System Integration

**Fits existing architecture:** ✅ Yes, with modularization improvements

**Integration Points:**

1. **Overlay System** - Reuse Display::Flex/None pattern from `toggle_game_state_ui_system` (line 1853)
2. **Card Rendering** - Extract helper from 4 existing implementations:
   - Player hand (line 854)
   - Played cards (line 1881)
   - Buyer visible hand (line 2104)
   - Deck builder (line 1700)
3. **Reactive Updates** - Continue using `Query<&T, Changed<T>>` pattern (proven, efficient)
4. **Theme System** - New `ui/theme.rs` module for color constants (eliminate hardcoded RGB tuples)
5. **Active Slots** - New component markers + reactivity system (follows existing pattern)

**No Breaking Changes:**
- Existing UI systems remain functional during refactor
- Incremental migration to modular structure
- All tests should pass throughout implementation

---

### Proposed Architecture

**Current State:**
```
src/main.rs (5,704 lines)
├── create_ui() - 370+ lines
├── 20+ UI update systems scattered
└── Hardcoded colors, duplicated card rendering
```

**Target State (Post-RFC-011):**
```
src/
├── main.rs (coordinator, ~500 lines)
├── game/
│   ├── state.rs (HandState, Card, etc.)
│   └── logic.rs (hand resolution, betting)
├── ui/
│   ├── mod.rs (plugin registration)
│   ├── creation.rs (create_ui, setup_deck_builder)
│   ├── systems.rs (update systems)
│   ├── components.rs (component markers)
│   ├── theme.rs (colors, sizes, constants)
│   └── helpers.rs (spawn_card_display, etc.)
└── ai/
    └── betting.rs (AI decision logic)
```

**Benefits:**
- Each module ~500-800 lines (manageable)
- UI concerns isolated and testable
- Color palette centralized
- Card rendering logic reusable
- Clear separation of concerns
- Easier to add future UI features

---

### Implementation Patterns

**Pattern 1: Hand Resolution Overlay**
```rust
// Modal overlay entity with Display::None initially
// On hand resolution: set Display::Flex, spawn results breakdown
// On dismiss: set Display::None, despawn children
// Proven pattern from existing game state toggles
```

**Pattern 2: Visual Heat Bar (Vertical)**
```rust
// Stacked nodes approach (no shader needed):
// Parent node (200-250px height, represents threshold)
//   ├── Red segment (80-100%) - top
//   ├── Yellow segment (50-80%) - middle
//   └── Green segment (0-50%) - bottom
// Update child heights based on current_heat percentage
// Fills from bottom to top
// Numeric display at bottom: "65/100"
// Color transitions handled by segment visibility
```

**Pattern 3: Active Slot System**
```rust
#[derive(Component)]
struct ActiveSlot {
    slot_type: SlotType, // Product/Location/Conviction/Insurance
}

// System: query HandState.cards_played, determine active card per slot
// Update border color: yellow if active, gray if empty
// Spawn card child if active, ghosted placeholder if not
```

**Pattern 4: Reusable Card Display**
```rust
fn spawn_card_display(
    parent: &mut ChildBuilder,
    card: &Card,
    size: CardSize, // Small/Hand/Large
    state: CardState, // Active/Inactive/Ghosted
    interactive: bool, // Clickable or static
) {
    // Centralized card styling logic
    // Eliminates ~200 lines of duplication
}
```

---

### Scope Assessment

**Core MVP Features (15-21 hours):**
- Hand resolution overlay: 2-3h
- Scenario card redesign (card-styled with heat bar): 3-4h
- Active slot system (4 slots): 4-5h
- Visual heat bar (stacked nodes): 2-3h
- Support card zone: 2-3h
- Card replacement feedback: 2-3h

**Architecture Improvements (8-10 hours):**
- Extract theme constants: 2-3h
- Card component helper: 2-3h
- Modularize UI systems: 4-6h

**Total Estimate: 22-28 hours** (updated based on Figma design)

---

### Recommended Split

**SOW-011-A: Core Layout & Foundation (14-18 hours)**
- Extract color palette to theme.rs
- Create reusable card display helper
- Complete layout restructure per Figma design (16:9 optimized)
- Scenario card redesign (card-styled, orange/brown theme)
- Vertical heat bar (fills bottom-to-top, positioned right of scenario)
- Active slot system (Product/Location/Conviction/Insurance) - horizontal row
- Ghosted placeholders (dashed outlines for empty slots)
- Evidence/Cover/Deal Mod pools (clean, no labels)

**SOW-011-B: Hand Resolution & Polish (8-10 hours)**
- Hand resolution overlay modal
- Symmetrical hand display (Narc left, Player center, Buyer right)
- Buyer deck panel (right side, vertical card display)
- Discard pile indicator
- Card replacement feedback (despawn/respawn with highlight)
- Profit/risk indicators on scenario card
- Modularize remaining UI systems

**Why Split:**
- SOW-011-A establishes foundation (theme, helpers, overlay pattern)
- SOW-011-B builds on proven foundation (slots, zones)
- Each SOW delivers user-visible value independently
- Easier to review and test in smaller chunks
- Lower risk per SOW

---

### Risks and Mitigation

| Risk | Level | Mitigation |
|------|-------|------------|
| File size navigation | MEDIUM | Modularize during implementation, not after |
| Color inconsistency | MEDIUM | Extract theme.rs first (SOW-011-A priority) |
| Entity hierarchy complexity | LOW | Use clear component markers (existing pattern) |
| Gradient limitation | LOW | Stacked nodes workaround (no shader needed) |
| Query efficiency | LOW | Continue using Changed<T> throughout |
| Performance impact | LOW | ~50 entities negligible vs current hundreds |

---

### Alternatives Considered

**Alternative 1: Keep Monolithic main.rs**
- Pro: Zero modularization overhead, faster initial implementation
- Con: File grows to 7,000+ lines, harder to maintain long-term
- **Rejected:** Technical debt compounds, future features harder to add

**Alternative 2: Full Refactor Before RFC-011**
- Pro: Clean slate for new UI features
- Con: 4-6 additional hours upfront, delays user-facing value
- **Rejected:** Unnecessary overhead, modularize incrementally instead

**Alternative 3: Sprite-Based Heat Bar**
- Pro: Could support true gradients and animations
- Con: Asset creation overhead, overkill for simple bar
- **Rejected:** Stacked nodes sufficient for MVP, can upgrade post-MVP

---

### Open Items for Discussion

**Q1: Heat Bar Visual Design** ✅ RESOLVED
- **Decision:** Vertical bar with stacked nodes (green/yellow/red segments)
- **Placement:** To the right of scenario card (not integrated)
- **Fills:** Bottom to top
- **Based on:** Figma mockup

**Q2: Heat Bar Orientation** ✅ RESOLVED
- **Decision:** Vertical orientation (not horizontal as originally proposed)
- **Height:** ~200-250px
- **Rationale:** Better visual clarity, saves horizontal space, fills intuitively upward
- **Based on:** Figma mockup

**Q3: Active Slot Empty State** ✅ RESOLVED
- **Decision:** Dashed outlines for empty slots (ghosted placeholders)
- **No labels needed** - type indicated by position and color
- **Based on:** Figma mockup showing dashed blue/green/red/purple outlines

**Q4: Card Replacement Animation Scope** ✅ RESOLVED
- **Decision:** Instant despawn/respawn with highlight (MVP)
- **Rationale:** Faster to implement, clear visual feedback, slide animation deferred to post-MVP
- **Based on:** PLAYER approval

**Q5: Modularization Timing** ✅ RESOLVED
- **Decision:** Modularize during SOW-011-A (theme + helpers first)
- **Rationale:** Establishes clean foundation, prevents new technical debt
- **Approach:** Incremental refactor alongside feature implementation

**Q6: Layout Restructure Scope** ✅ RESOLVED
- **Decision:** Complete layout restructure in SOW-011-A (14-18h)
- **Rationale:** Cohesive visual overhaul, easier to test as unit, avoids partial migration issues
- **Based on:** PLAYER approval

**All Open Items Resolved** - Ready for approval

---

### Recommendation

**✅ FEASIBLE - Approve with scope split**

**Status:** Ready for PLAYER/ARCHITECT iteration in Discussion section

**Proposed Path Forward:**
1. PLAYER reviews feasibility analysis and open items
2. ARCHITECT/PLAYER iterate on design choices (Q1-Q5)
3. Create SOW-011-A (Core Resolution UI)
4. Implement SOW-011-A, validate with PLAYER
5. Create SOW-011-B (Active Slots) based on learnings
6. Update feature matrix upon completion

**Architecture Quality:** RFC-011 implementation will IMPROVE codebase structure via incremental modularization. Post-implementation codebase will be more maintainable than current state.

---

## Discussion

### ARCHITECT Notes - Initial Review (2025-11-15)

**Strong Points:**
- Clear player need (visual clarity, deckbuilder identity)
- All features achievable with current tech stack
- Low technical risk (proven patterns)
- Architecture improvement opportunity (modularization)

**Critical Success Factors:**
1. Extract theme constants FIRST (prevents new hardcoded colors)
2. Create card helper EARLY (prevents new duplication)
3. Modularize DURING implementation (not after)
4. Split into two SOWs (manageable scope, independent value)

**Technical Highlights:**
- Bevy UI fully supports overlay/modal pattern (Display::Flex/None)
- Card rendering currently duplicated 4 times (~200 lines) - excellent refactoring opportunity
- Stacked nodes workaround for heat bar gradient (no shader needed)
- ~50 new UI entities negligible performance impact

**Scope Control (Updated with Figma Design):**
- SOW-011-A: 14-18 hours (complete layout restructure + theme foundation)
- SOW-011-B: 8-10 hours (hand resolution overlay + polish)
- Total: 22-28 hours across two SOWs (well-scoped)

**Design Decisions from Figma:**
- ✅ Vertical heat bar (not horizontal)
- ✅ Dashed outlines for empty slots (no labels needed)
- ✅ Compact top row (no "Override Slots" or "Additive Pools" labels)
- ✅ Symmetrical hand layout (Narc/Player/Buyer at bottom)
- ✅ 16:9 optimized vertical spacing

**All Design Decisions Resolved:**
- ✅ Q1: Vertical heat bar with stacked nodes (Figma)
- ✅ Q2: Heat bar positioned right of scenario (Figma)
- ✅ Q3: Dashed outlines for empty slots (Figma)
- ✅ Q4: Instant card replacement with highlight for MVP (PLAYER)
- ✅ Q5: Modularize during SOW-011-A (ARCHITECT)
- ✅ Q6: Complete layout restructure in SOW-011-A (PLAYER)

**Ready for final approval and SOW creation.**

---

## Approval

**Status:** ✅ **APPROVED** - Ready for SOW creation

**Approvers:**
- ARCHITECT: ✅ Approved (2025-11-15) - Feasible with scope split (SOW-011-A/B)
- PLAYER: ✅ Approved (2025-11-15) - Solves player need, Figma design validated

**Scope Constraint:** 22-28 hours across two SOWs:
- SOW-011-A: 14-18 hours (Core Layout & Foundation per Figma design)
- SOW-011-B: 8-10 hours (Hand Resolution & Polish)

**Dependencies:**
- None (self-contained UI improvements)

**Next Steps:**
1. ✅ ARCHITECT: Feasibility analysis complete
2. ✅ PLAYER: All open items resolved (Q1-Q6)
3. ✅ ARCHITECT/PLAYER: Design choices finalized
4. ⏳ ARCHITECT: Create SOW-011-A (next action)

**Date:** 2025-11-15
