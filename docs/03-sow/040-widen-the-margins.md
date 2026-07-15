# SOW-040: Widen the Margins

## Status

**Review** - 2026-07-15 (implemented in an isolated worktree off `sow-040`;
unit-tested, zero warnings on `cargo build` AND `cargo test`; live shop drive
pending the human e2e run of the `widened` forge scenario).

## References

- **Design:** Locked design, Reed 2026-07-15 (grounded in the real code; no RFC).
- **Builds on:** SOW-034 (Limited-Use Cards - shipped the per-zone
  `restock_margin` ladder and the pure `restock_unit` / `batch_cost`
  derivations), SOW-025 (Street Cred & Stationing - the `best_cred(zone)`
  accessor and `DealerState.street_cred`), SOW-031 (Suppliers & Fronts - the
  FRONT batch the discount also flows through), SOW-038 (Unlockable Dealers -
  the precedent for deriving new behavior from existing save state with NO new
  field / NO version bump, reusing `signature_of`).
- **Branch:** `sow-040` (worktree off `main`).
- **Implementation Time:** ~half day.

---

## Feature Summary

**Restock gets progressively cheaper as the roster's best street cred in a zone
rises** - the earn-back reward for SOW-034's per-zone `restock_margin` ladder
(trailer_park 0.35 / suburbia 0.50 / red_light_district 0.65). SOW-034 made a
fresh zone's supply *tighter* the higher up the ladder you go; SOW-040 hands
that cost back as you build a reputation there.

The effective restock margin for a zone is the authored `restock_margin` scaled
by a cred-driven discount factor keyed on `best_cred(zone)`:

```
effective_margin(zone) = restock_margin(zone) x cred_margin_factor(best_cred(zone))

cred_margin_factor(best_cred):     // highest cleared threshold wins
    best_cred >= 10  -> 0.55       [TUNING]
    best_cred >=  6  -> 0.70       [TUNING]
    best_cred >=  3  -> 0.85       [TUNING]
    else             -> 1.00       // cred 0-2: authored margin untouched

restock_unit = max(1, round(base_sale_price * effective_margin))   // EXISTING fn
batch_cost   = restock_unit * BATCH_SIZE(=4)                        // EXISTING fn
```

The discount is **purely derived** from existing state (`DealerState.street_cred`,
already in the save since SOW-025) - exactly like SOW-038 reused `signature_of`.
So there is **NO new save field and NO `SAVE_VERSION` bump** (stays 11).

**cred 0 is a strict no-op** (factor 1.0): SOW-034's shipped economy is
unchanged and its existing tests still pass verbatim.

### Worked examples (proves the reward is visible and stays positive)

| product | base | zone margin | cred 0 | cred 3 | cred 6 | cred 10 |
|---------|-----:|------------:|-------:|-------:|-------:|--------:|
| weed | 30 | 0.35 (tp) | unit 11 / **$44** | unit 9 / $36 | unit 7 / $28 | unit 6 / **$24** |
| coke | 120 | 0.65 (rld) | unit 78 / **$312** | - | - | unit 43 / **$172** |

Every case keeps `effective_margin` in (0,1), so a batch of 4 base-price sales
always clears its restock (no product runs underwater) - SOW-034's invariant is
preserved. The existing `max(1)` floor keeps restock non-free even at the
deepest discount (weed stays $6/charge at cred 10).

---

## Implementation Plan

### Phase 1: Pure discount derivations (models, unit-tested, zero ECS)

**Goal:** Make the cred-discount ladder a pure, testable function next to the
existing SOW-034 derivations.

**Deliverables (`src/models/shop_location.rs`):**
- `const CRED_MARGIN_LADDER: [(u32, f32); 3] = [(10, 0.55), (6, 0.70), (3, 0.85)]`
  - a game-wide progression curve kept as a code const, matching the
  `hire_cost` / `bail_cost` / FRONT_VIG precedent (progression ladders live in
  code, not RON). Flagged `[TUNING]`.
- `pub fn cred_margin_factor(best_cred: u32) -> f32` - scans the ladder
  high->low, first cleared threshold wins, default 1.0.
- `pub fn effective_restock_margin(base_margin: f32, best_cred: u32) -> f32`
  = `base_margin * cred_margin_factor(best_cred)`.
- `restock_unit` / `batch_cost` are UNCHANGED - callers pass the effective
  margin.

**Success Criteria:** step boundaries exact; cred 0 a strict no-op; monotonic
non-increasing in cred; never free / never underwater. See Test Plan.

### Phase 2: The single derivation seam (systems)

**Goal:** Discount the margin at the ONE place shop.rs already derives it, so
buy, restock, the display label, and the front all discount consistently with
no duplicated logic.

**Deliverables (`src/systems/shop.rs`, `populate_shop_cards_system`):**
- Bind `base_margin` where `margin` was bound (the authored `restock_margin`
  with the defensive `DEFAULT_RESTOCK_MARGIN` fallback).
- Read `best_cred` from the already-computed `area_best_cred` (SOW-025, captured
  in this closure at the cred-gate site - no new plumbing).
- `let margin = effective_restock_margin(base_margin, best_cred);`

Everything downstream (`ProductStock.unit_price` / `batch_cost`, the
RESTOCK / BUY BATCH label, and `spawn_front_button`'s `batch`) reads this single
`margin`, so all four discount consistently. Click handlers (`buy_batch`,
`take_front`) consume the values stored on the button components at spawn time -
no click-path change.

### Phase 3: Live-verify handle + docs

**Deliverables:**
- `src/save/forge.rs`: a `widened` scenario - kingpin stationed in trailer_park
  with 10 cred (deepest tier) and $200 to restock. Added to the roundtrip
  enumeration and given a `widened_scenario_shape` test.
- This SOW + README index row.

---

## Acceptance Criteria

**Functional:**
- `effective_restock_margin(0.35, 0) == 0.35` exactly - a fresh (cred 0-2) zone
  charges the authored margin, so restock_unit(30)=11 / batch=$44 is identical
  to SOW-034's shipped_zone_economy pin. **No economy regression.**
- A high-cred zone costs strictly less: weed batch $24 < $44 at cred 10, coke
  batch $172 < $312 at cred 10.
- The discount lands exactly at thresholds (weed batch $44 @ cred 2 -> $36 @
  cred 3 -> $28 @ cred 6 -> $24 @ cred 10).
- `restock_unit` is monotonic non-increasing in cred for every shipped product
  (never rises as a zone is built up).
- Never free / never underwater: `max(1)` floor holds at the deepest factor;
  `effective_margin` stays in (0,1) for every shipped (margin, cred) combo.
- Discount logic lives in ONE seam; buy_batch, restock cost, the shop label, and
  take_front all read the same discounted `margin`.

**Save:**
- `SAVE_VERSION == 11` (no bump). The effective margin is derived at read-time
  from `best_cred(zone)`; nothing new is persisted. Existing saved fronts keep
  their already-locked-in `owed` (the debt was incurred at the price of the day).

**Code Quality:**
- Zero warnings on `cargo build` AND `cargo test`.

---

## Discussion

### One seam, no duplicated discount

The design's key rule: derive the effective margin from `best_cred` in ONE place
so buy, restock, the display label, and the front stay consistent. shop.rs
already funnels all four through a single `margin` local
(`populate_shop_cards_system` L241-244); discounting `margin` there propagates
everywhere for free. Making fronts diverge would require re-deriving a SEPARATE
undiscounted `batch_cost` just for the front button - strictly MORE code for a
worse outcome.

### Fronts get the same discount (by construction)

`spawn_front_button` is passed the exact `batch` variable RESTOCK uses, so the
front's base batch cost is discounted too; the front vig still applies on top
(`front_owed = discounted_batch * 1.25`). Thematically coherent: your
relationships lower your cost of goods whether you pay cash or on credit. It also
self-corrects the lifeline - the front button only appears while `cash < batch`,
so a cheaper batch surfaces the front LESS often for established players
(graduating them off supplier credit), while a fresh zone (cred 0, factor 1.0)
still sees the front at its full SOW-034 price.

### Keyed on best_cred (not the stationed dealer)

Reuses the exact `best_cred(zone)` accessor already driving the shop's cred
gates and SOW-038 hires - "any dealer's reputation opens doors" is the
established SOW-025 rule.

### Why the ladder is a code const, not RON

For MVP the ladder is a game-wide progression curve, matching the
`hire_cost` / `bail_cost` / FRONT_VIG precedent (all code-side). Per-zone
authorable ladders (an optional `#[serde(default)] cred_discount_ladder` on
`ShopLocationDef` with a monotonic-non-increasing validate check) are a clean
additive follow-up if a zone ever needs a distinct curve - explicitly deferred,
no RON change this SOW.

### Deviation: none

The implementation follows the locked design exactly. No open question was
resolved differently from its proposed default.

---

## Test Plan (all pure, no ECS)

`src/models/shop_location.rs`:
1. `cred_margin_factor_step_boundaries` - factor(0)=factor(2)=1.0;
   factor(3)=factor(5)=0.85; factor(6)=factor(9)=0.70; factor(10)=factor(100)=0.55.
2. `cred_zero_is_a_strict_no_op` - `effective_restock_margin(0.35, 0) == 0.35`;
   restock_unit(30)=11, batch=$44 (identical to SOW-034's pin).
3. `high_cred_zone_costs_less` - weed batch $24 < $44, coke batch $172 < $312.
4. `discount_lands_exactly_at_thresholds` - weed batch $44/$36/$28/$24 at cred
   2/3/6/10 (asserts the step values, not just "less").
5. `restock_unit_is_monotonic_non_increasing_in_cred` - across cred 0..=15 for
   all 6 shipped products (guards a rounding-induced bump).
6. `never_free_and_never_underwater_at_deepest_discount` - `max(1)` floor holds;
   effective margin in (0,1) and a batch clears under 4 base sales for every
   shipped (margin, cred) combo.

`src/save/forge.rs`: `widened_scenario_shape` (10 cred in trailer_park,
best_cred == Some((0, 10)), $200 cash) + `widened` in the roundtrip enumeration.

---

## Live Verify Plan

1. `cargo run -- forge widened`; drive `tools/e2e/game-drive.ps1` to the shop,
   Trailer Park tab, screenshot the weed card - assert its button reads
   `RESTOCK $24` (or `BUY BATCH $24`), BELOW the cred-0 baseline of $44.
2. Cross-check the baseline against a low-cred forge (`fronted` = 1 cred, or a
   fresh new game) whose Trailer Park weed shows `$44`. The $44 -> $24 delta is
   direct visual proof the discount fired end-to-end through the real UI.

---

## Conclusion

SOW-040 turns SOW-034's zone-margin ladder into an earn-back curve: as the
roster's best cred in a zone climbs past 3 / 6 / 10, its restock margin is
discounted 0.85 / 0.70 / 0.55, making buy, restock, the shop label, and the
front progressively cheaper - all from a single derivation seam in shop.rs and
two pure functions in `shop_location.rs`. cred 0-2 is a strict no-op, so the
shipped economy is untouched and every SOW-034 test still passes. The discount
is purely derived from `street_cred`, so there is no new save field and no
`SAVE_VERSION` bump (stays 11). Unit-tested (7 new tests, 333 total), zero
warnings. Live shop drive of the `widened` scenario pending the human e2e run.
