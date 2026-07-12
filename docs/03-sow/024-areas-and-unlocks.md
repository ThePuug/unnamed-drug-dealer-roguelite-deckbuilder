# SOW-024: Areas & Unlocks

## Status

**Planned** - 2026-07-12

## References

- **RFC-024:** [Areas & Unlocks](../01-rfc/024-areas-and-unlocks.md)
- **Related:** SOW-020 (shops, locked-content groundwork), SOW-021 (The Block
  unlock deliberately deferred to its own SOW - this is that SOW), SOW-023
  (global cash sinks, e2e harness)
- **Branch:** (proposed) sow-024-areas-unlocks
- **Implementation Time:** 1-2 days

---

## Implementation Plan

### Phase 1: Area purchase model

**Goal:** Areas are purchasable with global cash through existing account APIs.

**Deliverables:**
- Area price in `shop_locations.ron` (The Block: $2,000 unless SOW-020/RFC-020
  recorded different intent - check and honor it)
- Pure purchase fn wiring `AccountState::{is_location_unlocked, unlock_location, spend}`
  (finally giving unlock_location its caller); persists via save
- Load-time validation: area prices present, warn on unknown area ids

**Architectural Constraints:**
- Content stays human-readable RON validated at load (authorability rule)
- Purchase math pure + unit-tested; no ECS in tests

**Success Criteria:**
- Buying The Block: cash decreases, unlock persists across save/load,
  double-purchase impossible

### Phase 2: Buyer area-gating

**Goal:** Who you can sell to depends on where you operate.

**Deliverables:**
- `area` field on buyer personas (`buyers.ron`, serde default `the_corner`);
  Wall Street Wolf moves to `the_block`
- Run-start persona selection filters to unlocked areas
- Load-time validation: persona areas must exist in shop_locations (SOW-021
  demand-string pattern)

**Architectural Constraints:**
- A fresh empire (Corner only) must always have >= 1 eligible persona
  (validation enforces)

**Success Criteria:**
- Fresh empire never draws the Wolf; after buying The Block the Wolf appears
  in the rotation

### Phase 3: Shop UI purchase flow

**Goal:** Expansion is visible and buyable where shops already live.

**Deliverables:**
- Locked areas render in the shop location selector as "THE BLOCK - $2,000"
  (disabled-styled when unaffordable, per button discipline)
- Click purchase -> unlock + open that shop; one-line feedback ("New turf:
  The Block") in the shop area
- Roster/operations screen untouched

**Success Criteria:**
- Full loop on screen: earn -> buy Block -> browse Block shop -> next run can
  draw the Wolf

### Phase 4: Harness + docs

**Deliverables:**
- playtest.ps1: outcome-aware overlay button Y (the fallen-empires board
  moved NEW EMPIRE below the scripted click - known papercut from SOW-023)
- Forge scenario `mogul` (kingpin, $3,000, block still locked) for the
  purchase e2e; scripted e2e: buy Block -> verify Wolf eligibility + shop
- Feature matrices (progression-meta, card-system), roadmap Iteration 3 entry

---

## Acceptance Criteria

**Functional:** area purchase persists; buyer pool respects unlocks; no
regression to Corner-only fresh empires.
**UX:** locked areas advertise their price; unaffordable is visibly disabled.
**Code Quality:** purchase/gating logic pure + tested; zero new warnings.

---

## Discussion

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
