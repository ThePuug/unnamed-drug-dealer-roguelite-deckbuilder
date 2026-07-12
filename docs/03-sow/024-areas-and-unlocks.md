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

### Phase 2: Two-stage run selection (REFRAMED - territories)

**Goal:** Who you can sell to depends on WHERE the run happens.

**Deliverables:**
- `area` field on buyer personas (`buyers.ron`, serde default `the_corner`);
  Wall Street Wolf is Block clientele
- Two-stage run start: pick the run's AREA (INTERIM: random among unlocked -
  replaced by dealer stationing in a follow-up SOW), then draw the persona
  from that area's clientele ONLY (no pooled draw)
- Run area logged for e2e observability
- Load-time validation: persona areas must exist AND every area has clientele

**Architectural Constraints:**
- A fresh empire (Corner only) must always have >= 1 eligible persona
  (validation enforces)

**Success Criteria:**
- Fresh empire runs are always Corner-area (never the Wolf); after buying
  The Block, Block-area runs occur and draw the Wolf

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

### ANSWERED by Reed (2026-07-12) - territory reframe

Areas are territories on a (future) map with their own narc behavior,
customers, and products; unlocking buys ACCESS - customers don't relocate.
Run selection is two-stage (area first, then that area's clientele). The
random-among-unlocked area pick is INTERIM: dealer stationing (run area =
the active dealer's station, per-dealer-per-area street cred gating shop
unlocks) lands in the next SOW - see
`design-updates/2026-07-12-stationing-and-street-cred.md` in the studio
repo. Per-area narc decks, product pools, and the deck-power gradient are
explicitly out of this SOW.

### Implementation Note: shop_locations.ron becomes real (Phase 1)

The RON file existed since SOW-020 but was never loaded - the shop selector
was hard-coded. It is now the loaded, validated source of truth for areas
(ids, names, prices, starting unlock). The Block prices at $2,000: RFC-020
only recorded an "achievement unlock (future RFC)" placeholder, superseded
by RFC-024's cash purchase.

---

## Acceptance Review

*Populated after implementation.*
