# SOW-029: City Map Screen

## Status

**In Progress** - 2026-07-12

## References

- **Design:** studio repo `design-updates/2026-07-12-city-map-screen.md`
  (node card contents, actions, open questions) building on
  `2026-07-12-world-map-and-areas.md`
- **Umbrella:** RFC-024/025 (territories, stationing) - UI surface over
  existing mechanics; no new RFC
- **Branch:** sow-029-city-map
- **Implementation Time:** 1-2 days (UI-heavy)

---

## Implementation Plan

### Phase 1: Map overlay scaffold

**Deliverables:**
- Full-screen MAP overlay reachable from the deck-builder hub (shop/
  operations overlay precedent; `GameState` untouched)
- Three zone nodes laid out as a city gradient (street -> nightlife ->
  uptown) in the 1920x1080 design space, SOW-022 theme palette
- Open/close plumbing; returns cleanly to the hub

### Phase 2: Node cards (read surface)

**Deliverables:**
- Pure view-model functions (TDD, no ECS in tests): per-zone node view
  from `SaveData` + `ShopLocationDef`s - unlock status, price +
  affordability, clientele names + payout band, native product family,
  narc texture hint (fiction voice: patrols / vice sweeps / task force),
  stationed dealer chips (name, heat tier, cred in that zone), best-cred
  dealer marker (mirrors the shop's "unlocked by" credit line)
- Locked zones render the same card grayed with price - sell the
  aspiration, keep the narc hint visible
- Live refresh when cash/stationing/heat change (existing Changed<T>
  gating patterns)

### Phase 3: Actions (write surface)

**Deliverables:**
- Zone unlock purchase from the node - SAME underlying purchase path as
  the current row button (one code path)
- Station/move from the map: select dealer chip -> destination node;
  confirmation shows move fee + 1-run downtime BEFORE committing (reuses
  `move_dealer`); jailed/relocating/broke states disabled with reason
- The flat area-selector row is REPLACED by a MAP button (deletion over
  duplication; recommendation from design doc - flag in Discussion if
  implementation finds a reason to keep it). Shop area tabs stay.

### Phase 4: Verification

**Deliverables:**
- Unit tests for every view-model function (unlock states, affordability,
  best-cred marker, move eligibility)
- e2e on a forged scenario: open map, purchase a zone from its node,
  relocate a dealer via the map, confirm the next run uses the new
  station; screenshots
- Harness upkeep: if hub layout changes, update `tools/e2e/playtest.ps1`
  reference clicks + header docs and re-verify one closed-loop session
- Feature matrices, SOW Discussion; roadmap Iteration 8 entry is the
  coordinator's

---

## Acceptance Criteria

**Functional:** map shows three live nodes (status, clientele, products,
narc hint, stationed dealers); zone unlock and dealer relocation both work
from the map; run flow unregressed.
**UX:** locked zones readable as aspiration; move confirm shows full cost
before commit; burn-then-cool rotation is a two-click loop.
**Code Quality:** zero warnings (baseline); view-model logic fully
unit-tested; no duplicated purchase/move code paths.

---

## Discussion

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
