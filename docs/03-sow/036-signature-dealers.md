# SOW-036: Signature Dealers

## Status

**In Progress** - 2026-07-13

## References

- **Design:** Locked design, Reed 2026-07-13 (this SOW captures it directly - no RFC).
- **Builds on:** SOW-023 (Kingpin & Dealers), SOW-024 (Areas & Unlocks),
  SOW-025 (Street Cred & Stationing), SOW-029 (City Map), SOW-031 (Suppliers),
  SOW-033 (Zone Retheme / dealer-`<slug>`.png portraits).
- **Deferred to:** RFC-035 deck rework (per-dealer unique decks).
- **Branch:** `sow-036-signature-dealers`
- **Implementation Time:** ~half day

---

## Feature Summary

Each zone gets ONE **signature dealer** - a themed, named face you hire AT
that zone. The point: you don't hire generically from anywhere. You hire at a
location, and the hire lands stationed there.

- **Faces** (portraits already on disk as `dealer-<slug>.png`):
  - Trailer Park -> **Bubba**
  - Suburbia -> **Roxanne**
  - Red Light District -> **Marcus**
- **Unlock:** available once the zone is unlocked
  (`account.unlocked_locations` contains the area id). No cred gate.
- **Cost:** the existing hire ladder - `SaveData::next_hire_cost()` =
  `hire_cost(dealers.len())` = `500 * 2^(len-1)`. Same as a generic hire.
- **Hired AT the zone:** on hire, a `DealerState` is pushed with
  `station = <that area id>` (NOT `default_station`), name/portrait from the
  zone's authored signature def, `Available`, `is_kingpin: false`.
- **One per zone:** a zone's signature can only be hired once.
- **Unique deck per dealer is DEFERRED** to the RFC-035 deck rework. The
  signature dealer is a themed hire (name + face + stationed), nothing more
  for now.

Generic hiring is NOT retired. The generic `hire_dealer()` + roster-panel HIRE
button stay; only the generic portrait pool shrinks (the three signature faces
are reserved).

---

## Implementation Plan

### Phase 1: Content model + reservation

**Goal:** Author each zone's signature dealer and reserve its face from the
generic hire pool, with load-time validation.

**Deliverables:**
- `SignatureDealerDef { name, portrait }` on `ShopLocationDef`
  (`src/models/shop_location.rs`), mirroring `supplier: Option<SupplierDef>`.
- Authored per zone in `assets/data/shop_locations.ron`
  (Bubba / Roxanne / Marcus).
- `validate_shop_locations` fails loud on a missing signature or an
  empty name/portrait (authorability rule).
- `DEALER_PORTRAIT_POOL` (`src/save/types.rs`) shrinks from
  `[Marcus, Gladys, Bubba, Roxanne]` to `[Gladys]` - a generic `recruit()`
  can no longer grab a signature face.
- `load_actor_portraits` (`src/assets/loader.rs`) maps the signature
  portrait keys from the RON signature defs (like buyers/narcs), so the loud
  disk-existence check still covers all four `dealer-*.png` faces and the
  render finds them.

**Architectural Constraints:**
- All `dealer-*.png` faces still load; generic recruit only offers Gladys.
- Signature portrait key is a KEY into `GameAssets.actor_portraits`
  (e.g. "Bubba"), mapped to `dealer-bubba.png`, consistent with the pool.

**Success Criteria:**
- `validate_shop_locations` rejects a zone with no signature / empty fields.
- The game loads with all four dealer faces available; recruit only draws
  Gladys.

### Phase 2: Hire mechanic (save model)

**Goal:** A guarded `hire_signature_dealer` that pushes a pre-stationed dealer.

**Deliverables:**
- `signature_of: Option<String>` on `DealerState` (`#[serde(default)]`; the
  area id the dealer is the signature of; `None` for generic/kingpin).
- `DealerState::signature(area_id, def)` constructor (stationed at `area_id`).
- `SaveData::hire_signature_dealer(area_id, def) -> bool`: guards (zone
  unlocked + no existing signature for that zone + affordable via
  `next_hire_cost()`), spends, pushes.
- `SAVE_VERSION` bump 8 -> 9 (new `DealerState` field; serde-default keeps it
  back-compat, the bump wipes old saves to a fresh account per the SOW-021
  policy).

**Architectural Constraints:**
- Cost = `next_hire_cost()` (shared ladder). No cred gate.
- One signature per zone; a second attempt is a no-op returning false.

**Success Criteria (unit-tested):**
- Hires when unlocked + affordable + not-yet-hired; stationed at the zone;
  `signature_of` set.
- No-ops (no mutation, no spend) when locked, already hired, or broke.

### Phase 3: Map UI

**Goal:** A "HIRE `<NAME>` - $X" button on the unlocked zone node, unit-testable
via the pure view-model.

**Deliverables:**
- `SignatureStatus` + `signature_status(area, save)` in
  `src/ui/map_view.rs`, surfaced as a field on `ZoneNodeView`
  (`Available { name, portrait, cost, affordable }` / `Hired { name }` /
  `Unavailable`).
- `MapSignatureHireButton` component (mirrors `ShopAreaUnlockButton`).
- `spawn_zone_node` (`src/systems/city_map.rs`): on an Unlocked node with no
  dealer armed for a move, render the signature action from the view-model.
- Commit path in `roster_button_system` (`src/systems/input.rs`, where the
  map's other action button - SEND / `RosterMoveButton` - is already handled):
  call `hire_signature_dealer`, save on success. The map rebuilds
  automatically (`populate_map_nodes_system` watches `SaveData` changes).

**Architectural Constraints:**
- Button state derives from the pure view-model so it stays unit-testable.
- The move flow (armed dealer -> SEND) takes precedence over the resting-state
  hire button; they never both occupy the action area.

**Success Criteria:**
- Unlocked + not hired + affordable -> enabled HIRE button.
- Not affordable -> disabled/greyed with the cost.
- Already hired -> "runs this zone" tag, no button.
- Locked zone -> no signature offer (UNLOCK button as before).

---

## Acceptance Criteria

**Functional:**
- Hiring a zone's signature adds a dealer stationed at that zone with the
  authored name + face, spending the shared hire-ladder cost.
- A zone's signature can be hired at most once.
- Generic hiring still works; generic recruits only ever wear Gladys.

**UX:**
- Signature hire button appears on the unlocked zone node; downstream (map
  chips, roster card, run routing, jail/kingpin) picks up the stationed dealer
  with no extra work.

**Code Quality:**
- Zero warnings on `cargo build` AND `cargo test`.
- New pure bits unit-tested: `signature_status` view-model +
  `hire_signature_dealer` guards + validation.

---

## Discussion

### Implementation Note: commit path lives in `roster_button_system`

The brief points at `area_unlock_button_system` as the commit-path template.
That system also rebuilds the shop's selector row explicitly - irrelevant for a
map action. The map's OTHER action button (SEND / `RosterMoveButton`) is
already handled inside `roster_button_system`, which mutates `SaveData`, marks
dirty, and saves; the map repopulates automatically because
`populate_map_nodes_system` watches `SaveData::is_changed()`. So the signature
hire is added as one more arm of `roster_button_system` - the closest existing
analog - rather than a standalone system. Same commit semantics, no duplicated
rebuild, no new system registration.

### Implementation Note: recruit() "skip signature portraits" is the pool shrink

The brief asks `recruit()`'s uniqueness scan to also skip signature portraits
(belt-and-suspenders). `recruit()` is a pure function over `existing:
&[DealerState]` and draws faces only from `DEALER_PORTRAIT_POOL`. Once the three
signature faces leave the pool (leaving `[Gladys]`), the scan literally cannot
reach a signature face - the pool shrink IS the skip. Threading the RON-authored
signature keys into `recruit()` would duplicate content into code (violating the
authorability rule) for no additional guarantee, so it is intentionally not
done.

### Implementation Note: signature portrait is a KEY, not a filename

`SignatureDealerDef.portrait` holds the portrait KEY ("Bubba"), matching how
`DealerState.portrait` and `DEALER_PORTRAIT_POOL` work (the render looks up
`actor_portraits[key]`). `load_actor_portraits` maps `key -> dealer-<slug>.png`,
so the loud disk-existence check and the render both resolve it.

### OPEN QUESTION (Reed to decide - not acted on here): retire generic hiring?

Face scarcity: 3 of 4 dealer faces are now reserved for signatures, so generic
hiring is effectively Gladys-only. Options: retire generic hiring entirely (the
roster-panel HIRE button), or add more dealer faces so generic hiring keeps
variety. Left as-is for now (generic `hire_dealer()` + roster HIRE button
unchanged, pool = `[Gladys]`). Reed to decide.

---

## Acceptance Review

*Populated by ARCHITECT after implementation.*

---

## Conclusion

*Populated at closeout.*
