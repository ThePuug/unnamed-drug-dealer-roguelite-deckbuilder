# SOW-029: City Map Screen

## Status

**Review** - 2026-07-12 (all 4 phases implemented on `sow-029-city-map`; 196 tests, zero warnings)

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

### Shipped UI structure

- **CITY MAP button** joins the hub tab row (design (415, 40)), always
  visible - the map is a management surface, not a shop mode.
- **MapOverlay** is an absolute full-size child of `DeckBuilderRoot`
  (opaque canvas, GlobalZIndex 90): it inherits the 1920x1080 design-space
  scaling AND the on-exit cleanup for free, and e2e clicks stay
  deterministic. Header = "THE CITY" + hint line + CLOSE; body = three
  node cards in `shop_locations.ron` definition order (centers x
  450/960/1470).
- **Node card** (480x680): name (+price when locked), identity line,
  fiction-voice narc hint (full brightness even when locked - risk is
  part of the pitch), clientele with payout band (derived min-max of the
  area personas' base multipliers), native products cheapest-first,
  stationed-dealer chips (heat tier colored, per-zone cred, ★ marks the
  roster's best cred - the same dealer the shop credits), and the action
  slot: UNLOCK button (locked) or SEND button (move flow armed).
- **View-model** is pure (`src/ui/map_view.rs`, mirrors view.rs): 21 unit
  tests cover unlock states, affordability, band formatting, product
  ordering, chip selectability/status notes, best-cred marker, move
  eligibility (jailed/relocating/broke/same-area/missing), and the hint
  line. `systems/city_map.rs` only orchestrates.

### One code path per action (acceptance requirement)

- Zone unlock: map nodes spawn the SOW-024 `ShopAreaUnlockButton` -
  `area_unlock_button_system` and `update_area_unlock_button_visuals`
  handle the map surface unchanged.
- Relocation: SEND buttons spawn the SOW-025 `RosterMoveButton` -
  `roster_button_system` commits via `move_dealer`. The map's
  `move_eligibility` view-fn mirrors the model's guards so the button
  never promises what the model refuses.

### Move flow (two clicks, cost visible before commit)

Click a dealer chip -> hint arms ("SENDING RAY - pick a destination ·
$250 + 1 RUN OUT") and every other unlocked node grows a SEND button
carrying the full cost on its face; the dealer's own node shows
"STATIONED HERE". The SEND click IS the confirm. Selection clears on
close, on commit (the dealer stops being available), and on any stale
state.

### Deviations (rationale)

1. **The shop's area-selector row survives** but lists UNLOCKED areas
   only (it still picks which area's stock the shop browses - that's a
   shop concern, not a map concern). What moved to the map is the
   locked-area PURCHASE buttons - zone unlocking now has exactly one
   surface. This is the "replace" the SOW asked for; deleting the row
   entirely would have coupled shop browsing to the map.
2. **Zone identity lines + narc hints are code-side** (`zone_identity`/
   `narc_hint` with fallbacks for unknown areas). Authoring them as
   `shop_locations.ron` fields is the right home eventually, but this SOW
   was scoped to zero content changes; flagged for the next content pass.
3. **Roster MOVE quick-button kept** - it's a one-click shortcut to "the
   first other unlocked area" and shares the commit path; the map is the
   full picker. Removing it would have regressed the SOW-025 e2e flows.

### e2e evidence (out\sweep29\)

- `hustler` scenario: map opened (3 nodes, Strip locked+affordable green),
  **Strip unlocked from its node** (log: "Unlocked area The Strip for
  $1200"; cash 1500->300; node flipped live), **Ray chip armed the move**
  (hint + SEND buttons on both other nodes, "RAY IS STATIONED HERE" on
  the Block), **SEND committed** (log: "Ray is relocating to the_strip";
  cash 300->50; chip moved to Strip with "MOVING · 1 RUN"), kingpin run
  ticked the relocation ("Back in action: Ray"), and **Ray's next run
  logged "Run area: the_strip - buyer: Pimp"**. Post-session map shows
  Ray ★ CRED 1 on the Strip (cred earned there, best-cred marker
  followed him).
- Affordability live-updates confirmed on the `mogul` closed-loop run:
  after buying the Block ($2,000 of $3,000), the Strip's UNLOCK button
  rendered disabled-dark ($1,000 < $1,200).
- Harness upkeep: `playtest.ps1 -BuyArea` now purchases through the map
  (CITY MAP tab -> node UNLOCK -> CLOSE); header reference-click docs
  updated; closed-loop re-verified (`mogul -BuyArea the_block -Hands 1`:
  unlock + full hand + GO HOME all landed).

### For Reed

- The three nodes are text + palette; per-zone node illustrations remain
  the standing art ask (design doc "Open for Reed").
- SOW-030 (ledger) gets an obvious home: a per-zone history line on the
  node card. SOW-031's supplier NPC likewise (one face per node).

---

## Acceptance Review

*Populated after implementation.*
