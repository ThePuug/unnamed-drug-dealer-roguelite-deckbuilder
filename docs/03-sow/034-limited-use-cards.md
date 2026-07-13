# SOW-034: Limited-Use Cards — Product as Consumable Stock

## Status

**Accepted** - 2026-07-13 (the "nothing wagered" fix; next after SOW-033).
All 6 phases landed on `sow-034-limited-use-cards`; adversarial review passed
(§8); merged `--no-ff` to `main`. 273 tests, zero warnings.

## References

- **Design:** studio repo `design-updates/2026-07-13-limited-use-cards.md`
  (forks locked by Reed: fixed batches; fronting is the out-of-stock
  floor; margins easy in the starter zone, tighter up the ladder).
- **Architecture map:** 5-agent code sweep (2026-07-13) — its full plan +
  touchpoint checklist are embedded in §4–§6 below. Refs are `file:line`
  at `main` post-SOW-033 (`SAVE_VERSION = 7`).
- **Reuses:** SOW-031 fronts (owed / window / muscle / souring), SOW-033
  zones (per-zone RON on `ShopLocationDef`).
- **Branch:** `sow-034-limited-use-cards`
- **Implementation Time:** 3-4 days (6 phases, each independently green).

---

## 1. Goal

Turn products from permanent unlocks into **consumable stock**, so every
deal wagers inventory you paid for — the direct fix for the "nothing
wagered" fun problem. **Unlock stays permanent access** (the cred+cash
ladder is untouched); what you *hold* is a batch of charges bought with
cash or fronted on credit. **Each play of a product burns one charge**
(fold-before-play free; a bust loses that one charge, not the batch). At 0
charges a product is **out of stock** — inert until restocked. **Fronting
reframes** from "front an unlock" to "front a batch of product."

**One-line architecture:** unlock stays a flag in `unlocked_cards`; stock
is a new consumable ledger `HashMap<product_id, u32>` on `AccountState`.
One `Card` per id flows through the deck unchanged; a charge burns at the
single player commit edge (`card_click_system` → `input.rs:555`); fronts
gain one field and reuse the entire SOW-031 escalation machine.

---

## 2. Resolved decisions (the map's 8 open items — coordinator's calls)

1. **Repossession seizure (souring):** seize `min(front.charges,
   stock[id])` — take back up to the delivered batch, capped by what
   remains. Access (`unlocked_cards`) is never revoked; only unsold
   product is repossessed.
2. **Fresh-account starting stock:** seed **one Weed batch**
   (`add_stock("weed", BATCH_SIZE)`) so turn 1 is playable without a
   forced front. (0-start is legal since fronting is the floor, but a
   seeded batch is friendlier.)
3. **Margin authoring:** per-zone **`restock_margin: f32`** on
   `ShopLocationDef`, RON-authored — exactly on-spec with "easy in the
   starter zone, tighter up the ladder." `restock_unit =
   round(Product.price × zone.restock_margin)`; `batch_cost = restock_unit
   × BATCH_SIZE`. **NOT `shop_price`** (that's a one-time unlock cost,
   ~10× payout base — using it per-unit runs every product underwater).
   Starting values (flagged for playtest tuning): trailer_park **0.35**,
   suburbia **0.50**, red_light_district **0.65**.
4. **Zero-total-stock run start:** **warn, don't block** — a stockless
   run is legal but unwinnable; surface a hint, let the player proceed.
5. **Persistence of the commit-time burn:** rely on the existing
   end-of-hand save (`save_after_resolution_system`, `save_integration.rs
   :220`), which runs after `card_click_system` each hand. No extra save
   call (a crash before resolve loses at most one in-flight hand —
   acceptable pre-release).
6. **`FrontState.card_id`:** keep the name (minimize churn); doc-clarify
   it now means "the product this batch is of."
7. **`BATCH_SIZE`:** global constant `= 4` for now; per-zone batch size is
   a deferred later knob.
8. **Override double-burn:** two Products played in one hand = two charges
   burned — intended per "each play burns one charge." Confirm no UI
   surprise (the hand shows remaining charges).

---

## 3. Deck representation (decided: one Card + save-side ledger)

**One product `Card` per id; charges live only in save.** N physical
copies is rejected — it breaks an id-unique invariant that pervades the
deck stack (selection toggle `input.rs:399-408`, pool build
`player_deck.rs:16-20`, resync `deck_builder.rs:44`, UI highlight
`ui_update.rs:986` all key on id), forces synthetic ids (`weed#1..#4`),
and *still* needs a save-side count because the run deck is rebuilt from
`selected_cards.clone()` each run (`input.rs:499`). A `charges` field on
the in-deck `Card` is also rejected (clobbered by that per-run rebuild).

The in-deck product card is **always present and drawable**; only its
*committability* is gated by the ledger — so `validate_deck` (needs ≥1
Product) and the `deck.len() < 3` exhaustion guard never see a
product-less or short deck. An out-of-stock-but-unlocked product stays in
the pool as **inert**.

---

## 4. Implementation plan (6 phases, TDD, green at each boundary)

**Phase 0 — Save scaffolding (no behavior change).**
`AccountState.stock: HashMap<String,u32>` (`#[serde(default)]`, keyed by
`card.id`) beside `unlocked_cards`; methods `charges_in` / `has_stock` /
`add_stock` / `burn_charge` / `buy_batch` (next to `spend`); `BATCH_SIZE
= 4`; `SAVE_VERSION 7→8`; seed one Weed batch in `starting_collection`.
Unit-test the methods. Suite stays green (stock unused yet).

**Phase 1 — Consume hook (burn at commit + out-of-stock gate).**
Add `Option<ResMut<SaveData>>` to `card_click_system`; before committing a
Product extend the slot guard (`input.rs:545-550`) to reject when
`!has_stock(id)`; after `play_card(...)` returns `Ok` on a Product, call
`burn_charge(id)`. Tests: play→1 burn; fold-before-play→0; play-then-fold
→1; bust→1 (batch intact); two products→2; 0-stock product can't commit.
**Do NOT** put the burn where `increment_play_count` lives
(`save_integration.rs:132-156`) — that loop is `Safe`-gated and would skip
busts.

**Phase 2 — Shop batch-buy / restock.**
Route `shop_purchase_system` through `buy_batch(id, restock_unit,
BATCH_SIZE)` — first buy grants access + first batch, later buys restock.
`spawn_shop_card` "✓ OWNED" → **STOCK: k/N** + **RESTOCK** (or **OUT OF
STOCK** at 0); buy button → **BUY BATCH ($batch_cost)**. CutOff guard
(`shop.rs:566-573`) covers restock. Tests: batch buy spends
`restock_unit×N`, adds N, grants access; restock adds without re-unlock.

**Phase 3 — Front-a-batch reframe.**
`FrontState.charges: u32`; `take_front(batch_cost)` grants
`add_stock(id, BATCH_SIZE)` instead of `unlocked_cards.insert`, precondition
is **access** (delete the owned-guard; fronting is now repeatable for an
owned product); `tick_fronts` souring seizes `min(front.charges,
stock[id])` and keeps access; `pay_front` doc only; front label/gate use
`batch_cost`. Rewrite the SOW-031 tests that assert `unlocked_cards.
contains` after a front (`types.rs:1618-1820`, `forge.rs:151-165`) to
assert `charges_in`.

**Phase 4 — Per-zone margin economy.**
`restock_margin: f32` on `ShopLocationDef` + `shop_locations.ron` +
validation (`> 0.0`, `< 1.0`); `restock_unit = round(Product.price ×
margin)`; wire into buy/front cost + labels. Per-zone cost tests.

**Phase 5 — Hand UI + polish.**
`update_hand_ui_system` reads stock → **"[k left]"** badge (via
`UpgradeInfo`/`CardDisplayState`) and greys **OUT OF STOCK** at 0, without
adding cards to the 3-slot hand. Optional: deck-builder stock hint;
zero-total-stock run-start warning (§2.4). Manual/e2e via
`tools/e2e/game-drive.ps1`.

---

## 5. Touchpoint checklist (by file, from the architecture map)

**`src/save/types.rs`**
- `:19` SAVE_VERSION 7→8 (+comment). `:1050-1055` add `BATCH_SIZE`.
- `AccountState` (`:1333-1347`) add `stock` beside `unlocked_cards`; init
  in `new`. `:1416-1423` add the 5 stock methods. `starting_collection`
  (`:1366-1383`) seed weed batch.
- `FrontState` (`:99-110`) add `charges`. `take_front` (`:351-379`)
  param→batch cost, `:365` owned-guard→access precondition, `:377`
  insert→`add_stock`, store `charges`. `tick_fronts` (`:407-472`)
  repossession `:460-461` seize unsold charges, keep access. `pay_front`
  (`:385-398`) doc only. Rewrite front tests `:1618-1820`.

**`src/systems/input.rs`**
- `card_click_system` (`:526-561`) add `Option<ResMut<SaveData>>`; guard
  `:545-550` `has_stock` reject; `burn_charge` after `play_card` `Ok` for
  Products at `:555`. `:318-346` repoint post-tick re-snapshot from
  `unlocked_cards` to `stock`; deck-carry filter `:364` still keys
  `unlocked_cards` (access).

**`src/systems/shop.rs`**
- `:222-240` read `charges_in` beside `is_unlocked`. `spawn_shop_card`
  (`:352-524`) STOCK/RESTOCK/OUT OF STOCK, BUY BATCH label `:479`, FRONT
  gate `:489-496` `cash < batch_cost`, front button `:516`.
  `shop_purchase_system` (`:527-604`) route through `buy_batch`; CutOff
  guard `:566-573` covers restock. `front_take_system` (`:609-649`) pass
  batch cost to `take_front`.

**`src/models/shop_location.rs`** — `:18-42` add `#[serde(default)]
restock_margin: f32`; validate `:51-81`.
**`src/systems/ui_update.rs`** — `update_hand_ui_system` (`:38-108`) add
`Res<SaveData>`; "[k left]" via `UpgradeInfo`/`CardDisplayState` `:77-96`.
**`src/ui/front_view.rs`** — `front_button_label` (`:33-39`)
`front_owed(batch_cost)`.
**`src/ui/components.rs`** — `ShopPurchaseButton` (`:410-413`) /
`FrontTakeButton` (`:435-439`) carry batch total.
**`assets/data/shop_locations.ron`** — `restock_margin` per zone.
**`src/save/forge.rs`** — `:151-165` fixtures → `charges_in`.

**No change:** `card.rs` (Card/CardType), `deck_builder.rs`
(`resync_available` — id still in `unlocked_cards`), `player_deck.rs`,
`presets.rs`, `state_machine.rs` (`play_card` stays pure),
`resolution.rs`. **No burn in `save_integration.rs`** (Safe-gated).

---

## 6. Acceptance Criteria

**Functional:** products are consumable (buy/front a batch of `BATCH_SIZE`;
each play burns a charge; fold-before-play free; bust loses one charge not
the batch; out of stock = inert but deck stays legal). Unlock stays
permanent access. Fronting grants a batch; souring seizes unsold charges,
not access. Per-zone margins make Trailer Park forgiving and Red Light
tight. Fresh account starts with one Weed batch; `SAVE_VERSION 8`.
**Economy:** every zone's `batch_cost` is comfortably below four base-price
sales in the starter zone and thin (but positive) up the ladder; no
product runs underwater.
**Code Quality:** zero warnings on `cargo build` AND `cargo test`;
deletion over `#[allow]`; the consume-hook rules and stock methods fully
unit-tested; view logic pure in `_view` modules.

---

## 7. Discussion

*Populated during implementation (2026-07-13).*

### Shipped economy (per-zone restock unit / batch cost)

`restock_unit = round(Product.base_price × zone.restock_margin)`,
`batch_cost = restock_unit × BATCH_SIZE (4)`. Restock is priced off the
Product card's **base sale price** (its `CardType` price), NOT `shop_price`
(the one-time unlock, ~10× payout base — using it per-unit runs every product
underwater). Margins shipped: trailer_park **0.35**, suburbia **0.50**,
red_light_district **0.65** (tuning starting-points, flagged for playtest).

| Product | Zone | Base | Margin | restock_unit | batch_cost | 4 base sales | Batch profit |
|---|---|---|---|---|---|---|---|
| Weed | trailer_park | 30 | 0.35 | 11 | **44** | 120 | +76 |
| Shrooms | trailer_park | 40 | 0.35 | 14 | **56** | 160 | +104 |
| Codeine | suburbia | 50 | 0.50 | 25 | **100** | 200 | +100 |
| Xanax | suburbia | 55 | 0.50 | 28 | **112** | 220 | +108 |
| Ecstasy | red_light_district | 80 | 0.65 | 52 | **208** | 320 | +112 |
| Coke | red_light_district | 120 | 0.65 | 78 | **312** | 480 | +168 |

**No product runs underwater** — every batch_cost is below four base-price
sales (margin < 1.0 guarantees it; pinned by
`shipped_zone_economy_is_positive_and_laddered`). Trailer Park is forgiving
(batch ≈ 35% of four sales); Red Light is thin but positive (≈ 65%).

### Deviations / decisions

- **Phase commits merged 0+1 and 2+3.** The zero-warning gate (a *binary*
  crate flags any `pub` method unused in the non-test build) means a stock
  method must land in the same commit as its consumer. Phase 0's
  `charges_in`/`has_stock`/`burn_charge` are consumed by Phase 1's hook, so
  they committed together; `buy_batch` moved to its Phase 2 consumer. Phases 2
  and 3 are functionally coupled — the shop's FRONT button needs the reframed
  `take_front` (grants a batch, not an unlock), so shipping Phase 2 alone would
  ship a broken FRONT. Committed as Phase 0+1, Phase 2+3, Phase 4, Phase 5.
- **Fronting requires ACCESS now.** SOW-031 fronted an *unlock*; SOW-034 fronts
  a *batch of an already-unlocked product* (`take_front` precondition flipped
  from "reject if owned" to "require access", new error `"no access yet"`).
  First acquisition of a product is the cash+cred ladder (`buy_batch` grants
  access); fronting is the repeatable out-of-stock floor. The shop's FRONT
  offer is gated to accessed products you can't afford to restock in cash.
- **Souring seizes stock, not access.** `tick_fronts` on the second blown
  window seizes `min(front.charges, on-hand stock)` unsold charges and never
  touches `unlocked_cards`. The go-home post-tick re-snapshot of `unlocked_cards`
  (a SOW-031 defense against repossessed *access*) was dropped as dead — access
  is stable across the tick now.
- **Consume-hook timing tested via a pure function, not ECS.** `resolve_slot_click`
  (Inert / Commit{burn}) captures the rule and is unit-tested across all
  branches; the burn lives at the single `card_click_system` commit edge (never
  resolution, never the Safe-gated `increment_play_count` loop), so a bust or a
  fold-after-play structurally keeps the spent charge. Per DEVELOPER role this
  is preferred over a brittle full-hand ECS integration test.
- **Hand stock badge is a plain overlay** ("`k LEFT`" green / greyed "OUT OF
  STOCK") added to the fan wrapper, plus `CardDisplayState::Inactive` greying at
  0 charges. It does not add cards to the 3-slot hand; an out-of-stock product
  stays drawable but inert, so `validate_deck` and the `deck.len() < 3`
  exhaustion guard never see a product-less or short deck.
- **Shop stock line reads "IN STOCK: k"** (not the `STOCK: k/N` sketched in §4)
  — once you can stack batches, `k` can exceed `N`, so a `/N` denominator
  misreads. Same OUT-OF-STOCK wording as the hand badge.
- **Zero-total-stock run start warns via log** (`start_run_button_system`), not
  a UI element — non-blocking per §2.4. A visible hub hint is deferred (cheap
  follow-up).

### Verification

- `cargo build` + `cargo test` green, **zero warnings** on both; 273 tests
  pass (up from 263 pre-SOW).
- Forge smoke: `fresh`/`funded`/`fronted`/`strapped` all validate and
  round-trip under `SAVE_VERSION 8`.
- App launch smoke (isolated `DDD_SAVE_DIR`): no panic; `shop_locations.ron`
  with `restock_margin` loaded and passed `validate_shop_locations` ("Loaded 3
  areas"). The interactive window flow (clicking through a hand to watch a
  charge burn and the badge tick) was NOT driven — flagged for the coordinator.

---

## 8. Acceptance Review

**Reviewed 2026-07-13 (coordinator).** Adversarial review — 28 agents, 4
dimensions (consume-hook / fronts-reframe / stock-economy-save /
integration-softlock-tests), each finding hit by 3 refutation skeptics:
**8 candidates → 0 sustained.** The review positively confirmed the key
safety property (an all-out-of-stock hand is never a dead state; the front
floor is always reachable) plus the shop label, hand badge, and resync
behaviors.

**Applied anyway (coordinator judgment, `restock_unit` floor):** a dismissed
finding noted `restock_unit = round(base × margin)` can round to **0** for a
cheap product in a low-margin zone → a $0 batch that hands out the product
AND permanent access for free. Doesn't touch the shipped 6, but it's a
latent footgun, so floored `restock_unit` at 1 (`shop_location.rs:117`) +
pinned it (`restock_unit(1, 0.35) == 1`).

**Accepted as designed / documented (not blocking):**
- **Souring seizes pooled stock, not provenance-tracked fronted units.** The
  `stock` ledger is a flat per-id count, so `min(front.charges, on-hand)` on
  souring can seize cash-bought charges once fronted and bought stock mix.
  This is the deliberate §2.1 simplification (flat ledger; per-batch
  provenance was rejected as extra state). Narratively coherent — the muscle
  takes a batch's worth of product from your stash when you default. Revisit
  only if it feels unfair in playtest.
- **Margin near 1.0 is break-even, not "comfortably below."** The `(0,1)`
  bound + shipped margins (0.35/0.50/0.65) are fine; a future author picking
  ~0.99 gets break-even restock. Soft economy concern, caught by any
  attainability/feel playtest, not a correctness bug.
- **ECS-wiring coverage gap.** The three seams SOW-034 adds (burn in
  `card_click_system`, `buy_batch` in `shop_purchase_system`, `take_front` in
  `front_take_system`) rest on tested *pure helpers* (`resolve_slot_click`,
  `restock_unit`, `buy_batch`) — the thin ECS wiring itself has no driven
  test (per DEVELOPER role: pure logic tested, systems kept thin). The
  natural verification is a **playtest** — watching a charge burn and the
  badge tick down. Flagged for Reed.

**Verdict: ACCEPT.** Zero warnings on build + test, 273 green, forge
scenarios round-trip at `SAVE_VERSION 8`, no soft-lock, economy above water
with the free-product footgun now guarded.
