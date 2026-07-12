# SOW-025: Street Cred & Stationing

## Status

**Merged** - 2026-07-12

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

### Implementation Note: pilot item substitution (Phase 1)

The SOW named Shrooms as the Corner pilot for cred gating, but Shrooms is
STARTING COLLECTION (shop_price 0, pre-unlocked) - it cannot be shop-gated
as authored today. The Corner's only real shop item is **Storage Unit
($1,500)**, which took the pilot requirement (3 Corner cred); **Heroin
($8,000, Block)** carries the Block pilot (5 Block cred). Reed's
"Shrooms as an unlockable" example becomes real in the SOW-026 authoring
pass when the starting collection gets leaner and Shrooms moves to shop
stock.

### Implementation Note: MOVE button target (Phase 3)

With two areas the MOVE button targets the first unlocked area that isn't
the dealer's station ("MOVE TO THE BLOCK - $250"). A real area picker
belongs to the map screen SOW. Relocating dealers show "MOVING · N RUNS"
and START RUN reads "MOVING" (vs "JAILED") when the active dealer is
mid-move.

### Implementation Note: tick semantics (Phase 2)

`tick_sentence` now serves BOTH jail sentences and relocations at the same
choke point (run completion via GO HOME/END RUN), runner excluded. The
go-home log line became "Back in action: ..." accordingly. Pre-existing
subtlety worth knowing (not introduced here): decay skips unavailable
dealers per-call, but `last_played` isn't advanced while benched, so the
first decay after release counts the benched wall-clock hours too -
harmless today (release usually zeroes heat; relocation is 1 run) but worth
a look if decay semantics ever tighten.

### Tuning candidates (flagged for the next playtest)

- Move fee $250 vs first hire $500 vs bail $300/run: moving is currently
  the cheapest roster action; with only 2 areas that's probably right (it
  buys access, not relief - no heat effects), but revisit when the map
  grows or if move-spam appears.
- Cred pilot thresholds (3 / 5) are placeholders; the real ladder is
  SOW-026 authoring-pass work.
- Sentence constant (1 + heat/25) unchanged from SOW-023 and still untested
  against real pacing.

### e2e evidence (Phase 4)

`hustler` forge scenario (isolated save): roster panel verified on screen -
kingpin "THE CORNER · CRED 4 / READY / MOVE TO THE BLOCK $250" with active
border, Ray "THE BLOCK · CRED 2 / READY / MOVE TO THE CORNER $250", HIRE
$1,000 (roster-of-2 doubling), cash $1,500, and the SOW-023 heat line
correctly absent from the stats block. Shop cred-lock text, the
"unlocked by <name>" credit line, and a live move click could NOT be
visually verified - the desktop was contested during the run window (the
driver's foreground guard aborted clicks, four attempts over ~90s). Those
paths are covered by unit tests (best_cred, move_dealer, gate math) and
the server-side purchase guard; one uncontested e2e pass at acceptance
should eyeball the shop states.

---

## Acceptance Review

### Scope Completion: 100%

- ✅ Phase 1: Stationing + cred model (pilot gates: Storage Unit 3 Corner
  cred, Heroin 5 Block cred)
- ✅ Phase 2: Station-based runs, cred accrual, relocation ticking
- ✅ Phase 3: Roster placement UI, MOVE buttons, shop gate states + credit line
- ✅ Phase 4: `hustler` forge scenario, e2e, docs

### Architectural Compliance

170 tests (8 new); zero new warnings; all staged APIs consumed. Reuses the
sentence-tick choke point for relocations; purchase re-checks cred
server-side.

### Player Experience Validation

Verified live on the `hustler` scenario: roster cards with station/cred/
status and both MOVE buttons; **"unlocked by The Kingpin" credit line** on
the cred-met Storage Unit; **"NEEDS CRED 5 (best: 2)"** locked state on
Heroin; and a live double relocation (user-clicked during the acceptance
window): both dealers MOVING · 1 RUN, stations swapped, fees charged
($1,500 → $1,000), per-station cred display correct, START RUN guard
reading "MOVING".

### Tuning flags carried to SOW-026/027

Move fee $250 (cheapest roster action - fine at 2 areas), pilot cred
thresholds 3/5 placeholder, sentence constant unfelt, Wolf ×2.8 unchecked.

---

## Sign-Off

**Reviewed By:** ARCHITECT review + live acceptance e2e (user hands-on
during the window)
**Date:** 2026-07-12
**Decision:** ✅ **ACCEPTED**
**Status:** Merged to main
