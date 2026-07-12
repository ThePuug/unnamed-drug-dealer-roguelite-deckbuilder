# SOW-025: Street Cred & Stationing

## Status

**Planned** - 2026-07-12

## References

- **RFC-025:** [Street Cred & Stationing](../01-rfc/025-street-cred-and-stationing.md)
- **Related:** SOW-023 (roster, sentence ticking), SOW-024 (areas, two-stage
  selection - the random area pick this SOW replaces), studio repo
  `design-updates/2026-07-12-stationing-and-street-cred.md` (locked decisions)
- **Branch:** (proposed) sow-025-cred-stationing
- **Implementation Time:** 1-2 days

---

## Implementation Plan

### Phase 1: Model

**Goal:** Dealers are placed and reputed.

**Deliverables:**
- `DealerState.station: String` (area id; fresh empire: kingpin stationed at
  the starting area) + `street_cred: HashMap<String, u32>`
- `DealerStatus::Relocating { runs_remaining }` (or equivalent) reusing the
  sentence-tick machinery; move fn: charge flat fee (tunable const) + set
  downtime 1 run; station validated against loaded areas
- Cred helpers: `add_cred(area)`, `best_cred(dealers, area) -> Option<(&Dealer, u32)>`
- Shop item cred requirements: `cred_required: Option<u32>` in shop stock RON
  (validated at load); pilot content: ONE Corner item (Shrooms if it exists in
  Corner stock, else the cheapest gated sensibly) and ONE Block item carry
  requirements
- SAVE_VERSION bump (wipe OK)

**Success Criteria:**
- Move: cash decreases, dealer unavailable exactly 1 run, station changed
- Cred accrues only via the API; save round-trips stations/cred/relocating

### Phase 2: Engine

**Goal:** Runs happen where the dealer stands; success builds rep there.

**Deliverables:**
- Run area = active dealer's station (replaces random pick; keep the
  clientele-from-area draw); HandState records run area
- Safe resolution: +1 cred to the runner in the run area
- Relocating ticks alongside jail on run completion; relocating/jailed
  dealers can't be sent out (extend the SOW-023 guard)
- Kingpin busts/game-over unchanged; empire reset wipes stations to default

**Success Criteria:**
- Two Safe hands in the Block = 2 Block cred on the runner, 0 elsewhere
- A dealer mid-relocation cannot start a run; arrival completes after 1 run

### Phase 3: UI

**Goal:** Placement and reputation are visible where decisions happen.

**Deliverables:**
- Dealer cards (operations roster): station name + cred-in-station
  ("THE BLOCK · CRED 5"), Relocating status with countdown
- MOVE button per dealer (with only 2 areas: "MOVE TO <other> - $250";
  disabled when unaffordable/jailed/relocating)
- Shop: cred-gated items show requirement + lock state; met requirements show
  the credit line "unlocked by <highest-cred dealer>" (Reed's visibility ask)
- START RUN guard extended to Relocating

**Success Criteria:**
- Full loop on screen: move a dealer to the Block -> wait out arrival -> deal
  Safe -> watch cred tick -> cred-gated item unlocks with credit line

### Phase 4: Harness + docs

**Deliverables:**
- Forge scenarios: `hustler` (kingpin stationed Corner with 4 Corner cred,
  Block unlocked, hired dealer stationed Block with 2 cred, $1,500)
- e2e: move flow + cred accrual + gated purchase with credit line visible
- Feature matrices, SOW/RFC statuses, roadmap Iteration 4 entry

---

## Acceptance Criteria

**Functional:** station-based runs; cred accrual/persistence; cred+cash shop
gating; move cost+downtime; no regressions to jail/bail/hire flows.
**UX:** station, cred, and relocation state visible on dealer cards; gated
shop items explain what's missing; credit line names the unlocking dealer.
**Code Quality:** all placement/cred/move logic pure + tested; zero new
warnings.

---

## Discussion

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
