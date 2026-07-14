# SOW-038: Unlockable Dealers Per Area

## Status

**Review** - 2026-07-14 (implemented in an isolated worktree; unit-tested,
zero warnings; live map HIRE flow pending human e2e drive).

## References

- **Design:** Locked design, Reed 2026-07-14 (this SOW captures it directly - no RFC).
- **Builds on:** SOW-023 (Kingpin & Dealers), SOW-024 (Areas & Unlocks),
  SOW-025 (Street Cred & Stationing / `best_cred`), SOW-029 (City Map),
  SOW-033 (Zone Retheme / `dealer-<slug>.png` portraits),
  SOW-036 (Signature Dealers - the one-per-zone hire this extends).
- **Branch:** `wf_3203694f-20d-1` (worktree off `main`)
- **Implementation Time:** ~half day

---

## Feature Summary

Extend SOW-036's one-signature-per-zone hiring so each zone ALSO offers
**additional named dealers that unlock at street-cred thresholds**. Purely
**ADDITIVE** over the signature model - the signature dealer stays exactly as
it was; each zone gains a list of cred-gated unlockables alongside it.

- **Unlock condition:** a **street-cred threshold**, reusing SOW-025's
  `SaveData::best_cred(area)` (any dealer's rep in the zone opens the door).
- **Model:** additive - `signature_dealer: Option<SignatureDealerDef>` is kept,
  and a new `unlockable_dealers: Vec<AreaDealerDef>` is added to
  `ShopLocationDef`.
- **Cred-locked display:** a cred-locked offer renders as
  `<NAME> — NEEDS CRED N (have M)` on the map node (no button).
- **Cost:** the shared `next_hire_cost()` ladder - the same cost as a signature
  or generic hire. No per-def premium.
- **Hired AT the zone:** on hire, a `DealerState` is pushed stationed at the
  zone, with `signature_of = Some(area_id)` reused as the zone marker. **No new
  `DealerState` field; `SAVE_VERSION` stays 9.**
- **Hire-once identity:** the `(area, name)` pair. A hired unlockable does not
  block the signature and vice-versa (`has_zone_dealer(area, name)` keys on
  both).

### Pilot content

- **Trailer Park** authors ONE unlockable: **Gladys** (`portrait: "Gladys"`,
  `cred_required: 5`). `dealer-gladys.png` already exists on disk. This
  deliberately **rehomes Gladys from the generic hire pool into Trailer Park**,
  pre-clearing the follow-up SOW (SOW-036's RESOLVED "retire the generic pool").
- **Suburbia** and **Red Light District** author an empty
  `unlockable_dealers: []` (pending new authored faces).

Note: this SOW only *authors* Gladys as an unlockable; `DEALER_PORTRAIT_POOL`
still contains `["Gladys"]` (the generic pool is retired in the follow-up SOW,
not here). Gladys mapping to `dealer-gladys.png` from both the pool loop and the
new unlockable loop is idempotent (same key -> same file).

---

## Implementation Plan

### Phase 1: Content model + validation + portrait loading

- `AreaDealerDef { name, portrait, cred_required }` in
  `src/models/shop_location.rs` (`Serialize/Deserialize/Clone/PartialEq`;
  `portrait` is a KEY into `actor_portraits`, same convention as
  `SignatureDealerDef`).
- `#[serde(default)] unlockable_dealers: Vec<AreaDealerDef>` on
  `ShopLocationDef`.
- `validate_shop_locations`: each unlockable needs a non-empty name AND
  portrait; every dealer NAME within a zone (signature + all unlockables) must
  be UNIQUE - the `(area, name)` pair is the hire-once identity, so a collision
  fails loud at load.
- `load_actor_portraits` (`src/assets/loader.rs`): a loop over each area's
  `unlockable_dealers` mapping `portrait -> dealer-<slug>.png` into the mapped
  set (parallel to the SOW-036 signature loop), so the loud disk-existence
  assert covers the new faces.
- `assets/data/shop_locations.ron`: the pilot content above.

### Phase 2: Hire mechanic (save model)

- Rename `has_signature_dealer(area)` -> `has_zone_dealer(area, name)`, matching
  on BOTH `signature_of == Some(area)` AND `name`. Callers updated
  (`signature_status` in `map_view.rs`; the signature test).
- `hire_zone_dealer(area, def: &AreaDealerDef) -> bool` with guards: (a) zone
  unlocked; (b) `best_cred(area) >= def.cred_required`; (c)
  `!has_zone_dealer(area, name)`; (d) affordable via `next_hire_cost()`. On
  pass: spend and push a `DealerState` stationed at the zone with
  `signature_of = Some(area)`, name/portrait from the def.
- A private `hire_zone_dealer_core(area, name, portrait, cred_required)` is the
  shared core; `hire_signature_dealer` routes through it with `cred_required =
  0` (signature behavior unchanged). `DealerState::signature` retired in favor
  of the generalized `DealerState::zone_dealer(area, name, portrait)`
  constructor (deletion over `#[allow]`, per GUIDANCE).
- No new `DealerState` field; `SAVE_VERSION` stays **9**.

### Phase 3: Map view-model + UI + commit path

- `AreaDealerOffer { name, portrait, cred_required, state }` +
  `AreaDealerOfferState { Available { cost, affordable } | Locked { cred_have }
  | Hired }`; pure `area_dealer_offers(area, save) -> Vec<AreaDealerOffer>`
  mirroring `hire_zone_dealer`'s guards (empty on a locked zone).
  `unlockable_dealers: Vec<AreaDealerOffer>` added to `ZoneNodeView`, filled in
  `zone_node_view`.
- `MapAreaDealerHireButton { area_id, name, portrait, cred_required }` in
  `src/ui/components.rs` (mirrors `MapSignatureHireButton` plus the cred payload
  for a server-side re-check).
- `spawn_zone_node` (`src/systems/city_map.rs`, unlocked resting-state branch,
  after `spawn_signature_action`): spawn each offer - Available -> a HIRE
  button; Locked -> a `NEEDS CRED N (have M)` row; Hired -> a `<NAME> RUNS THIS
  ZONE` tag. Armed-move SEND keeps precedence over the whole resting block.
- `roster_button_system` (`src/systems/input.rs`): a `MapAreaDealerHireButton`
  query arm (alongside the SOW-036 signature arm) rebuilds an `AreaDealerDef`
  from the payload, calls `hire_zone_dealer`, sets dirty, logs - same
  save-on-dirty flow.

---

## Acceptance Criteria

**Functional:**
- Hiring a zone's unlockable adds a dealer stationed at that zone with the
  authored name + face, gated on cred and spending the shared hire-ladder cost.
- The `(area, name)` slot is hire-once; the signature stays independently
  hireable.
- Signature behavior is unchanged (routed through the same core, cred 0).

**UX:**
- Below the signature action on an unlocked node: a HIRE button when cred is met
  and the slot is open; a `NEEDS CRED N (have M)` row when cred-locked; a `<NAME>
  RUNS THIS ZONE` tag once hired.

**Code Quality:**
- Zero warnings on `cargo build` AND `cargo test`.
- Pure bits unit-tested: `area_dealer_offers` view-model, `hire_zone_dealer`
  guards, `has_zone_dealer` `(area, name)` distinction, validation, and the
  shipped RON loading the Gladys pilot. Existing SOW-036 signature tests stay
  green.

---

## Discussion

### Additive over the signature model (not a replacement)

The signature dealer is left untouched: `signature_dealer` and its offer/hire
path stay. Unlockables are a *second* list on the zone. The only shared change
is the hire-once check moving from "any signature at this zone"
(`has_signature_dealer(area)`) to the `(area, name)` pair
(`has_zone_dealer(area, name)`) - required so a hired unlockable (which also
carries `signature_of = Some(area)`, reused as the zone marker) does not block
the signature, and vice-versa.

### Cred threshold as the unlock (default, decided)

The unlock condition is a street-cred threshold, reusing SOW-025's `best_cred`
(any dealer's rep in the zone counts). This is consistent with how the shop
already gates cred-locked products and keeps the mechanic legible: build rep in
a zone, unlock its deeper roster.

### Reusing `signature_of` - no `SAVE_VERSION` bump

An unlockable hire is a zone dealer just like a signature: stationed AT the
zone, marked with `signature_of`. Reusing that field (rather than adding a new
one) keeps the save format at v9 - no migration, no wipe. `signature_of` is now
better read as "the zone this dealer belongs to" than strictly "signature of".

### Gladys rehome - pre-clearing the generic-pool retirement

SOW-036's RESOLVED note scheduled retiring the generic hire pool (which had
shrunk to Gladys-only) as a follow-up SOW. Authoring Gladys as Trailer Park's
first unlockable rehomes her into the zone-hire model, pre-clearing that
follow-up. This SOW does NOT yet remove Gladys from `DEALER_PORTRAIT_POOL` or
retire `hire_dealer()` - that stays the follow-up's job; the two Gladys ->
`dealer-gladys.png` mappings are idempotent in the meantime.

---

## Conclusion

Shipped additively across all three phases: content model + validation + face
loading, a guarded `hire_zone_dealer` routed through a shared core (signature
unchanged, `SAVE_VERSION` still 9), and the map HIRE / NEEDS-CRED / RUNS-THIS-ZONE
rows driven by the pure `area_dealer_offers` view-model. Trailer Park pilots
Gladys (cred 5); the other two zones author empty lists pending new faces.
Unit-tested, zero warnings. Live map HIRE flow pending the human e2e drive.
