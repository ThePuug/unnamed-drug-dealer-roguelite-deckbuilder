# Core Gameplay Loop - Feature Matrix

**Companion to:** [core-gameplay-loop.md](core-gameplay-loop.md)

**Last Updated:** 2025-11-15 (Updated to reflect RFC-010 Buyer Scenarios and RFC-011 UI Refactor)

**Overall Status:** Updated to reflect Buyer System (RFC-009) - Customer/Dealer replaced with unified Buyer entity

---

## Major Changes from Spec to Implementation

**RFC-005 (Deck Balance):**
- ✅ Player deck expanded to 20 cards (was 15)
- ✅ Evidence cards removed from player deck (moved to Narc)
- ✅ Conviction cards moved to Narc deck (was in player deck)
- ~~Customer deck now strategic~~ → **Superseded by RFC-009 Buyer System**

**RFC-006 (Deck Building):**
- ✅ Players now choose 10-20 cards from 20-card pool
- ✅ Deck builder UI implemented
- ✅ Presets available (Default, Aggro, Control)

**RFC-007 (Betting):**
- ❌ Rejected - betting currency mechanic didn't work

**RFC-008 (Sequential Play) - MAJOR CORE LOOP REWORK:**
- ✅ **Sequential play** (one card at a time, face-up) replaces simultaneous face-down play
- ~~Dealer deck with 3 community cards~~ → **Superseded by RFC-009 Buyer reaction deck**
- ~~Rotating turn order~~ → **Replaced by fixed Narc→Player order (RFC-009)**
- ✅ **Check action** (skip playing card)
- ✅ **Fold mechanic** after Dealer reveal (Rounds 1-2)
- ~~Customer can fold~~ → **Customer removed (RFC-009)**
- ✅ **Narc cannot fold** (always plays through)
- ✅ **Running totals update after each card** (progressive information)
- ❌ Initiative/raising mechanics removed (simplified)

**RFC-009 (Buyer System) - ENTITY SIMPLIFICATION:**
- ✅ **Buyer entity replaces Customer + Dealer** (3 players → 2 players)
- ✅ **3 Buyer personas** (Frat Bro, Desperate Housewife, Wall Street Wolf)
- ✅ **Buyer reaction deck** (7 cards per persona, 3 visible)
- ✅ **Demand satisfaction system** (Product + Location matching)
- ✅ **Buyer bail thresholds** (Heat/Evidence limits)
- ✅ **Profit multipliers** (base vs reduced based on demand)
- ✅ **Fixed turn order** (Narc → Player, no rotation)
- ✅ **Visible hand UI** (anticipation mechanic)
- ⏸️ **Session structure** (BuyerSelection, SessionDecision states - deferred)

**RFC-010 (Buyer Scenarios and Product Expansion) - IMPLEMENTED:**
- ✅ **2 scenarios per Buyer** (different motivations/contexts)
- ✅ **9 products total** (expanded from 5: added Codeine, Ecstasy, Shrooms, Acid)
- ✅ **Scenario-specific demands** (different products/locations per scenario)
- ✅ **Product/Location tags** (for future conditional logic)
- ✅ **Thematic coherence** (scenarios tell stories, locations make sense)

**RFC-011 (UI Refactor) - IMPLEMENTED:**
- ✅ **16:9 optimized layout** (Figma design)
- ✅ **Active slot system** (Product/Location/Conviction/Insurance slots)
- ✅ **Vertical heat bar** (dynamic fill, color transitions)
- ✅ **Hand resolution overlay** (modal with outcome-specific results)
- ✅ **Single shared played pool** (Evidence/Cover/DealMod for all players)
- ✅ **Modular UI architecture** (ui/theme, components, helpers, systems)
- ✅ **Consistent card sizing** (Small vs Medium two-tier system)
- ✅ **Discard pile** (vertical list of replaced cards)
- ✅ **Slot-based player hand** (preserves card positions)

**These changes represent a fundamental shift from "3-player complex AI" to "2-player with strategic Buyer personas"**

---

## Legend

- ✅ **Complete** - Fully implemented per spec
- 🔄 **In Progress** - Currently being developed (SOW active)
- 🚧 **Partial** - Partially implemented or MVP version
- ❌ **Not Started** - Planned but not implemented
- ⏸️ **Deferred** - Intentionally postponed to post-MVP
- 🎯 **Planned** - RFC approved, SOW created, ready for implementation

---

## Game Structure Hierarchy

### Run (Character Lifecycle) - 0/7 complete (0%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| Character persistence | ❌ | Permadeath system | Phase 2 |
| Total profit tracking | ❌ | Accumulate across decks | Phase 2 |
| Decks played counter | ❌ | Track survival time | Phase 2 |
| Heat persistence | ❌ | Persist between sessions | Phase 2 |
| Heat real-time decay | ❌ | Decay over real-world time | Phase 2 |
| Customer Trust persistence | ❌ | Persist between sessions | Phase 2 |
| Permadeath on bust | ❌ | Character deletion | Phase 2 |

### Deck (Session) - 2/8 complete (25%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| 20-card deck pool | ✅ | 20 cards per player deck | RFC-005, SOW-005 |
| Deck building UI (10-20 cards) | ✅ | Choose cards from pool | RFC-006, SOW-006 |
| Session play (3-5 hands) | 🚧 | Multiple hands possible, needs polish | SOW-001-008 |
| "Go Home" early option | ✅ | Exit between hands | Implemented |
| Deck exhaustion handling | 🚧 | Basic logic present | SOW-004 |
| Card counter display | ❌ | "X cards remaining" UI | Phase 2 |
| Post-session summary | ❌ | Profit banked, Heat delta | Phase 2 |
| Strategic deck building | ❌ | Heat-based recommendations | Phase 2 |

---

## Hand Structure

### Hand Flow - 11/14 complete (79%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| 3-round hand structure | ✅ | Player Phase → Buyer Reveal × 3 | RFC-008/009, SOW-008/009 |
| Turn order | ✅ | **CHANGED: Fixed Narc→Player (was rotating)** | RFC-009, SOW-009 |
| Sequential card play (face-up) | ✅ | One at a time, immediate reveal | RFC-008, SOW-008 |
| Check action | ✅ | Skip playing card | RFC-008, SOW-008 |
| Buyer card reveals | ✅ | **CHANGED: Random from 3 visible cards** | RFC-009, SOW-009 |
| Player fold on player's turn | ✅ | Fold option all rounds (1, 2, 3) during PlayerPhase | RFC-008, SOW-008 |
| Buyer cannot fold | ✅ | **CHANGED: Buyer plays via reaction deck** | RFC-009, SOW-009 |
| Narc cannot fold | ✅ | Hardcoded behavior | RFC-008, SOW-008 |
| Running totals calculation | ✅ | After each card played | RFC-008, SOW-008 |
| End of hand resolution | ✅ | Validity/Bail/Demand/Bust checks | RFC-009, SOW-009 |
| Buyer reaction deck | ✅ | **NEW: 7 cards per persona, 3 visible** | RFC-009, SOW-009 |
| Card retention between hands | ✅ | Unplayed cards carry over | RFC-004, SOW-004 |
| Hand history/replay | ❌ | Review previous hands | Phase 3 |
| Undo last action | ❌ | Take back play | Phase 3 |

### Round Flow - 8/9 complete (89%) - **SIGNIFICANTLY CHANGED per RFC-008/009**

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| Sequential turn-based play | ✅ | **Fixed order: Narc→Player** | RFC-009, SOW-009 |
| Play card face-up | ✅ | Immediate reveal, no face-down | RFC-008, SOW-008 |
| Check action (skip card) | ✅ | Play no card this turn | RFC-008, SOW-008 |
| Fold action | ✅ | Exit hand (on player's turn, any round) | RFC-008, SOW-008 |
| Cards visible immediately | ✅ | No simultaneous flip | RFC-008, SOW-008 |
| Running totals update per card | ✅ | After each card, not per round | RFC-008, SOW-008 |
| Buyer reveal after Player Phase | ✅ | **Random card from 3 visible** | RFC-009, SOW-009 |
| Fold available during player turn | ✅ | Available alongside Play/Check actions | RFC-008, SOW-008 |
| Turn order indicator UI | ❌ | Show whose turn + order | Phase 2 polish |

**Note:** Initiative and raising mechanics removed in RFC-008 (betting system simplified)
**Note:** Customer removed, Dealer scenario deck replaced with Buyer reaction deck (RFC-009)

---

## Player Feedback Systems

### Visual Indicators - 10/13 complete (77%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| Turn indicator | ✅ | "Turn: Player" in status | RFC-011, SOW-011-B |
| Card count per player | ✅ | Visible hand displays | RFC-009, SOW-009 |
| Running totals display | ✅ | Evidence/Cover/Multiplier bar | RFC-011, SOW-011-A |
| Color-coded safety | ✅ | Heat bar green/yellow/red | RFC-011, SOW-011-A |
| Evidence gap display | ✅ | Shown in resolution overlay | RFC-011, SOW-011-B |
| Heat accumulation | ✅ | Heat bar with current/threshold | RFC-011, SOW-011-A |
| Initiative badge | ❌ | N/A (initiative removed) | - |
| Raises remaining | ❌ | N/A (raising removed) | - |
| Active Product highlight | ✅ | Active slot system | RFC-011, SOW-011-A |
| Active Location highlight | ✅ | Active slot system | RFC-011, SOW-011-A |
| Bust warning | ✅ | Real-time totals comparison | RFC-011, SOW-011-A |
| Fold projection | ❌ | Not implemented | Phase 2 polish |
| Continue projection | ❌ | Not implemented | Phase 2 polish |

### Decision Support - 0/4 complete (0%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| Risk assessment display | ❌ | Evidence gap, Cover left | Phase 2 |
| Reward evaluation | ❌ | Current profit, Heat cost | Phase 2 |
| Card management | ❌ | Cards left, hands remaining | Phase 2 |
| Strategic position | ❌ | Heat level, Trust level | Phase 2 |

---

## Edge Cases and Special Scenarios

### Special Conditions - 0/9 complete (0%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| "Go Home" early | ❌ | Exit between hands | RFC-002+ |
| "Go Home" projection | ❌ | Show stats if exit | RFC-002+ |
| All-in trigger | ❌ | Last card played | RFC-002 |
| All-in effects | ❌ | Betting ends | RFC-002 |
| Deck exhaustion detection | ❌ | Can't draw when empty | RFC-002 |
| Deck exhaustion warning | ❌ | "Last hand" alert | RFC-002 |
| All players fold | ❌ | Hand ends early | RFC-002 |
| Fold preserves cards | ❌ | Keep unplayed cards | RFC-002 |
| Fold loses profit | ❌ | No banking on fold | RFC-002 |

### Balance Targets - 0/4 complete (0%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| 15-minute session length | ❌ | 3-5 hands target | Validation in RFC-002 |
| 3-4 minute hand length | ❌ | 1 min/round × 3 + 30s | Validation in RFC-002 |
| 20-30% fold rate | ❌ | Desired player behavior | Validation in RFC-002 |
| 3-5 hands per deck | ❌ | Card consumption rate | Validation in RFC-002 |

---

## Implementation Status by RFC/SOW

### SOW-001: Minimal Playable Hand (~4h actual) - ✅ Complete

**Status:** Approved - Ready to Merge

**Features Delivered:**
- ✅ Hand state machine (single round flow)
- ✅ Draw phase (all players draw to 3 cards)
- ✅ Flip phase (simultaneous card reveal)
- ✅ Card interactions (override for Product/Location, additive for Evidence/Cover)
- ✅ Running totals calculation (Evidence, Cover, Heat, Profit)
- ✅ Bust check (Evidence > Cover → game over)
- ✅ Active Product/Location highlights
- ✅ Heat accumulation display
- ✅ 8 cards implemented (3 Products, 2 Locations, 2 Evidence, 1 Cover)

**Scope Exclusions (delivered in SOW-002):**
- 3-round structure (was single round only)
- Betting mechanics (was manual play)
- AI opponents
- Decision points

### SOW-002: Betting System and AI Opponents (~4h actual) - ✅ Complete

**Status:** Review - Implementation Complete, Awaiting Playtest

**Features Delivered:**
- ✅ 3-round hand structure (extended from single round)
- ✅ Betting phase (Check/Raise/Fold actions)
- ✅ Initiative system (first to raise gains control)
- ✅ Max 3 raises per round (prevents infinite loops)
- ✅ All-in mechanic (last card ends betting)
- ✅ Decision points (Continue/Fold between rounds)
- ✅ AI opponents (Narc and Customer with static decks)
- ✅ Deal Modifiers (4 types: multiplicative modifiers)
- ✅ Expanded to 15 cards total

**Scope Exclusions (Phase 2 or RFC-003):**
- Turn indicator/Initiative badge UI (polish deferred)
- Session play/deck exhaustion (deferred)
- Fold projections (deferred to RFC-003)

### RFC-003: Insurance and Complete Cards (14-18h) - Draft

**Planned Scope:**
- Get Out of Jail cards
- Make It Stick cards
- Insurance activation
- Conviction override
- Color-coded safety warnings
- Bust warnings
- Complete 20-card collection

**Features to Deliver:**
- Bust warnings
- Color-coded safety
- Evidence gap display

### Phase 2: Persistence and Meta

**Deferred Features:**
- Run (Character Lifecycle) - All 7 features
- Deck building UI
- "Go Home" early
- Post-session summary
- Scenario card mechanics
- Risk assessment tools
- Heat decay
- Trust system

---

## Related Documents

- **Spec:** [core-gameplay-loop.md](core-gameplay-loop.md)
- **RFC-001-revised:** [Minimal Playable Hand](../01-rfc/001-revised-minimal-playable-hand.md)
- **RFC-002:** [Betting System and AI](../01-rfc/002-betting-system-and-ai.md)
- **RFC-003:** [Insurance and Complete Cards](../01-rfc/003-insurance-and-complete-cards.md)
- **ADR-001:** [Card Type System](../02-adr/001-card-type-system-and-interaction-rules.md)
- **ADR-002:** [Betting System](../02-adr/002-betting-system-and-hand-structure.md)
- **ADR-004:** [Hand State Machine](../02-adr/004-hand-state-machine.md)
- **ADR-005:** [Initiative System](../02-adr/005-initiative-system.md)
- **SOW-001:** [Minimal Playable Hand](../03-sow/001-minimal-playable-hand.md)

---

## Implementation Deviations

### RFC-009: Buyer System (Supersedes Customer + Dealer)

**Status:** ✅ Implemented and merged (2025-11-14)

**Changes to Spec:**
- **Customer entity removed** - Previously 3 players (Narc, Customer, Player), now 2 players (Narc, Player)
- **Dealer scenario deck removed** - Replaced with Buyer reaction deck system
- **Rotating turn order removed** - Now fixed Narc → Player order
- **Customer AI removed** - No longer needed with 2-player structure

**New Systems Added:**
- **Buyer personas** - 3 distinct personas with different demands, multipliers, thresholds
- **Buyer reaction deck** - 7 cards per persona, 3 shown face-up (anticipation mechanic)
- **Demand satisfaction** - Product + Location matching affects profit multiplier
- **Buyer bail thresholds** - Heat/Evidence limits cause deal failure
- **Enhanced resolution** - Validity checks, bail checks, demand satisfaction

**Rationale:**
- Simplifies game from 3-player to 2-player structure
- Reduces AI complexity (only Narc AI needed)
- Adds strategic depth through Buyer personas and visible reaction deck
- Improves player clarity ("who am I dealing with?")
- Creates anticipation without frustration (visible hand mechanic)

**Documentation:**
- RFC: [docs/01-rfc/009-buyer-system.md](../01-rfc/009-buyer-system.md)
- SOW: [docs/03-sow/009-buyer-system.md](../03-sow/009-buyer-system.md)

---

## Notes

- **SOW-001 is technical validation only** - Single round, manual play, no betting
- **Fun validation happens in RFC-002** - 3-round structure, AI opponents, betting tension
- **Stakes validation happens in RFC-003** - Insurance clutch moments, conviction dread
- **Many features deferred to Phase 2** - Character persistence, Heat decay, Trust, meta-progression
