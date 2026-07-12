# SOW-023: Kingpin & Dealers Foundation

## Status

**Planned** - 2026-07-12

## References

- **RFC-023:** [Kingpin & Dealers](../01-rfc/023-kingpin-and-dealers.md)
- **Related:** SOW-021 (save conventions, SAVE_VERSION wipe), SOW-022 (screen + e2e driver), RFC-018 (per-heat narc tiers — now per-dealer)
- **Branch:** (proposed) sow-023-kingpin-dealers
- **Implementation Time:** 2-3 days

---

## Implementation Plan

### Phase 1: Dealer save model

**Goal:** The save file holds a roster of dealers instead of one optional character.

**Deliverables:**
- `DealerState` (extends today's `CharacterState` fields) with `name`,
  `portrait`, `status: DealerStatus { Available, Jailed { until: u64 } }`
- `SaveData.dealers: Vec<DealerState>` + `active_dealer: usize`; SAVE_VERSION bump
- Name pool + portrait pool (the 9 unused actor portraits) for generated hires
- Pure functions: jail term from bust heat, release check, hire cost curve

**Architectural Constraints:**
- Jail time is real-clock (same timestamp source as heat decay) and evaluated
  lazily — no background timers
- Roster invariant: at least one dealer always exists (a fresh save creates one)
- All jail/hire/release math is pure and unit-tested (no ECS in tests)
- Save wipe on version bump is acceptable (pre-release convention)

**Success Criteria:**
- Round-trip save/load of a roster with mixed Available/Jailed dealers
- Jail term scales with heat at bust; release resets heat to 0
- Hire cost strictly increases with roster size

### Phase 2: Engine integration

**Goal:** Runs are executed BY the active dealer; busts jail instead of delete.

**Deliverables:**
- Run start reads the active dealer (heat → narc tier, upgrades, play counts)
- `save_after_resolution_system`: Busted → jail active dealer (permadeath path removed)
- GO HOME transfers signed session heat + stories to the active dealer
- Decay applies per dealer (jailed dealers skip decay; jail IS the reset)

**Architectural Constraints:**
- `HandState` remains dealer-agnostic (it already copies counts/upgrades in at
  run start and out at run end) — no dealer references inside the hand engine
- Existing tests for decay/upgrades/transfer keep passing re-pointed at dealers

**Success Criteria:**
- Bust → dealer jailed with countdown, save persists it, account cash untouched
- Two dealers with different heat produce different narc tiers on their runs

### Phase 3: Operations panel (deck-builder screen)

**Goal:** The player can see the roster, select who runs, and hire.

**Deliverables:**
- Roster panel: per dealer — portrait, name, heat tier chip, status
  (Available / Jailed with countdown), select-to-activate
- HIRE button with cost from global cash; disabled when unaffordable
- START RUN disabled when the active dealer is jailed

**Architectural Constraints:**
- Reuse card/chip styling from SOW-022 theme; colors as named constants
- Marker components + `Changed`/change-guarded updates per SOW-011/021 rules

**Success Criteria:**
- Full loop on screen: bust a dealer → see them jailed → hire a replacement →
  run with the new dealer

### Phase 4: Harness + docs

**Deliverables:**
- e2e driver/save isolation: game honors a save-path override (env var) so
  scripted playtests never touch the real save; driver sets it
- Scripted e2e: bust → jail → hire → new run (outcome-aware overlay buttons)
- Feature matrices + roadmap iteration log updated

---

## Acceptance Criteria

**Functional:** roster CRUD via play (hire/select/jail/release); no permadeath
path remains; cash global across dealers.
**UX:** player always knows who is out, who is jailed and for how long, and
what a hire costs.
**Performance:** lazy release checks only on load/screen entry.
**Code Quality:** jail/hire/release logic pure + tested; no new warnings.

---

## Discussion

### Open questions for Reed (direction wanted, not blocking - defaults chosen)

1. **Jail duration feel:** default = `30min + 1min × heat-at-bust` real time
   (a hot bust ≈ 1-2h bench). Alternative: session-count based ("out for the
   next N runs"). Which feel do you want?
2. **Hire cost curve:** default = $500 × 2^(roster-1) ($500, $1000, $2000...).
3. **Jail release heat:** default = reset to 0 (jail as the heat valve).
   Alternative: halved, or +1 permanent "prior conviction" as a scar that
   feeds SOW-025 difficulty. Default keeps a future scar hook but doesn't add it yet.
4. **All-jailed + broke:** default = you wait (empire idles). Alternative:
   a bail-out mechanic (pay to release early). Bail feels right thematically -
   include in this SOW or defer?

*Defaults will ship unless redirected; changing any is a small tune.*

---

## Acceptance Review

*Populated after implementation.*
