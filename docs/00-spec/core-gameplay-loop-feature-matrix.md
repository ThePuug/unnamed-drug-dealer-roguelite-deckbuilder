# Core Gameplay Loop - Feature Matrix

**Companion to:** [core-gameplay-loop.md](core-gameplay-loop.md)

**Last Updated:** 2025-11-10 (Updated to reflect RFC-005, RFC-006, RFC-008 implementations)

**Overall Status:** Updated to reflect recent RFC implementations - significant changes to core loop

---

## Major Changes from Spec to Implementation

**RFC-005 (Deck Balance):**
- ✅ Player deck expanded to 20 cards (was 15)
- ✅ Evidence cards removed from player deck (moved to Narc)
- ✅ Conviction cards moved to Narc deck (was in player deck)
- ✅ Customer deck now strategic (Products, Locations, Modifiers instead of placeholder Evidence)

**RFC-006 (Deck Building):**
- ✅ Players now choose 10-20 cards from 20-card pool
- ✅ Deck builder UI implemented
- ✅ Presets available (Default, Aggro, Control)

**RFC-007 (Betting):**
- ❌ Rejected - betting currency mechanic didn't work

**RFC-008 (Sequential Play) - MAJOR CORE LOOP REWORK:**
- ✅ **Sequential play** (one card at a time, face-up) replaces simultaneous face-down play
- ✅ **Dealer deck with 3 community cards** revealed progressively (NEW mechanic)
- ✅ **Rotating turn order** per round (was fixed Narc→Customer→Player)
- ✅ **Check action** (skip playing card)
- ✅ **Fold mechanic** after Dealer reveal (Rounds 1-2)
- ✅ **Customer can fold** (removes cards from totals)
- ✅ **Narc cannot fold** (always plays through)
- ✅ **Running totals update after each card** (progressive information)
- ❌ Initiative/raising mechanics removed (simplified)

**These changes represent a fundamental shift from "poker-style betting" to "progressive information revelation"**

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

### Hand Flow - 10/14 complete (71%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| 3-round hand structure | ✅ | Player Phase → Dealer Reveal × 3 | RFC-008, SOW-008 |
| Rotating turn order | ✅ | Changes per round | RFC-008, SOW-008 |
| Sequential card play (face-up) | ✅ | **CHANGED: One at a time, immediate reveal** | RFC-008, SOW-008 |
| Check action | ✅ | Skip playing card | RFC-008, SOW-008 |
| Dealer card reveals | ✅ | **NEW: 3 community cards, one per round** | RFC-008, SOW-008 |
| Player fold after Dealer reveal | ✅ | **NEW: Fold option Rounds 1-2** | RFC-008, SOW-008 |
| Customer can fold | ✅ | **NEW: Removes cards from totals** | RFC-008, SOW-008 |
| Narc cannot fold | ✅ | Hardcoded behavior | RFC-008, SOW-008 |
| Running totals calculation | ✅ | After each card played | RFC-008, SOW-008 |
| End of hand resolution | ✅ | Calculate finals, bust check | SOW-001-008 |
| Dealer deck system (20 cards) | ✅ | **NEW: Separate deck with Locations/Modifiers/Wild** | RFC-008, SOW-008 |
| Card retention between hands | ✅ | Unplayed cards carry over | RFC-004, SOW-004 |
| Hand history/replay | ❌ | Review previous hands | Phase 3 |
| Undo last action | ❌ | Take back play | Phase 3 |

### Round Flow - 8/9 complete (89%) - **SIGNIFICANTLY CHANGED per RFC-008**

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| Sequential turn-based play | ✅ | **CHANGED: One player at a time** | RFC-008, SOW-008 |
| Play card face-up | ✅ | **CHANGED: Immediate reveal, no face-down** | RFC-008, SOW-008 |
| Check action (skip card) | ✅ | Play no card this turn | RFC-008, SOW-008 |
| Fold action | ✅ | Exit hand (after Dealer reveal) | RFC-008, SOW-008 |
| Cards visible immediately | ✅ | **CHANGED: No simultaneous flip** | RFC-008, SOW-008 |
| Running totals update per card | ✅ | **CHANGED: After each card, not per round** | RFC-008, SOW-008 |
| Dealer reveal after Player Phase | ✅ | **NEW: Community card reveal** | RFC-008, SOW-008 |
| Decision point after Dealer reveal | ✅ | "Fold or Continue?" | RFC-008, SOW-008 |
| Turn order indicator UI | ❌ | Show whose turn + order | Phase 2 polish |

**Note:** Initiative and raising mechanics removed in RFC-008 (betting system simplified)

---

## Player Feedback Systems

### Visual Indicators - 4/13 complete (31%)

| Feature | Status | Notes | RFC/ADR/SOW |
|---------|:------:|-------|-------------|
| Turn indicator | ❌ | Highlight active player | Phase 2 polish |
| Card count per player | ❌ | Show hand size | Phase 2 polish |
| Running totals display | ✅ | Evidence/Cover/Heat/Profit | SOW-001, SOW-002 |
| Color-coded safety | ❌ | Green/Yellow/Red zones | RFC-003 |
| Evidence gap display | ❌ | "Cover +20" or "Evidence +15" | RFC-003 |
| Heat accumulation | ✅ | "+45 Heat this hand" | SOW-001, SOW-002 |
| Initiative badge | ❌ | "X has initiative" | Phase 2 polish |
| Raises remaining | ❌ | "2/3 raises left" | Phase 2 polish |
| Active Product highlight | ✅ | Show which Product active | SOW-001, SOW-002 |
| Active Location highlight | ✅ | Show which Location active | SOW-001, SOW-002 |
| Bust warning | ❌ | "Evidence > Cover if flip now" | RFC-003 |
| Fold projection | ❌ | "If fold: Keep Heat +30" | RFC-003 |
| Continue projection | ❌ | "If continue: Need 25 Cover" | RFC-003 |

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

*None yet - MVP in progress*

---

## Notes

- **SOW-001 is technical validation only** - Single round, manual play, no betting
- **Fun validation happens in RFC-002** - 3-round structure, AI opponents, betting tension
- **Stakes validation happens in RFC-003** - Insurance clutch moments, conviction dread
- **Many features deferred to Phase 2** - Character persistence, Heat decay, Trust, meta-progression
