# SOW-039: Retire Generic Hire Pool

## Status

**Review** - 2026-07-15 (implemented in an isolated worktree; unit-tested,
zero warnings on `cargo build` AND `cargo test`; live boot/map flow pending the
human e2e drive).

## References

- **Design:** Locked design, Reed 2026-07-14/15 (this SOW captures the
  scheduled retirement directly - no RFC).
- **Builds on / closes:** SOW-023 (Kingpin & Dealers - introduced the generic
  `hire_dealer()`/`recruit()` pool), SOW-036 (Signature Dealers - RESERVED
  three faces and scheduled the pool's retirement), SOW-038 (Unlockable Dealers
  - rehomed the pool's last surviving face, Gladys, into Trailer Park as a
  cred-gated unlockable, pre-clearing this SOW).
- **Branch:** `sow-039` (worktree off `main`, which already carries SOW-037 +
  SOW-038).
- **Implementation Time:** ~half day.

---

## Feature Summary

Retire the **generic dealer hire pool** end to end. Since SOW-036/038 the roster
is meant to grow ONLY through **authored zone dealers** - the per-zone signature
(SOW-036) and the cred-gated unlockables (SOW-038), both hired AT a zone on the
CITY MAP. The generic path (a random name + face from code-owned pools, hired
from a roster-panel HIRE card) is now dead weight and a second, off-theme way to
grow the roster. This SOW removes it entirely.

This is a **PURE REMOVAL** of the generic path. Everything SOW-036/038 shipped is
KEPT verbatim: `DealerState::zone_dealer`, `hire_signature_dealer`,
`hire_zone_dealer`(+`_core`), the map signature + unlockable input arms, the
signature + unlockable portrait-loader loops, and Trailer Park's Gladys
unlockable in `shop_locations.ron`.

### Removed (generic path only)

- `SaveData::hire_dealer()`.
- `DealerState::recruit()`, `DEALER_NAME_POOL`, `DEALER_PORTRAIT_POOL`.
- `RosterHireButton` component; the roster-panel HIRE `$500` card spawn in
  `populate_roster_panel_system` (replaced by a static
  "Hire dealers on the CITY MAP" hint node) and its now-unused `hire_cost` local.
- The generic-hire arm + `hire_query` param in `roster_button_system`.
- `theme::ROSTER_HIRE_BG`.
- The `DEALER_PORTRAIT_POOL` loop in `load_actor_portraits` (the `Silhouette`
  placeholder mapping is KEPT; Gladys' face keeps loading via the SOW-038
  unlockable loop - no orphan).
- `tools/e2e/playtest.ps1`: the `-Hire` switch + HIRE-card coordinates.

### Rerouted (reuse the existing zone dealer / kingpin - never a pool)

- `src/save/forge.rs`: every `DealerState::recruit(...)` in a dev scenario now
  builds `DealerState::zone_dealer(area, name, portrait)` with already-loaded
  authored faces (Bubba/Roxanne/Marcus/Gladys). Scenario counts/shapes are
  unchanged (pinned by the existing `*_scenario_shape` tests).
- `ensure_roster_on_run_start` (empty-roster recovery): pushes
  `DealerState::kingpin()` (the guaranteed starter dealer) instead of a generic
  recruit.

### Save format

- `SAVE_VERSION` **9 -> 10**. No serialized field changes, but a v9 save could
  hold a GENERICALLY-hired "Gladys" (the pool's last face, now an authored
  UNLOCKABLE with `signature_of` set). Per the SOW-021 version-bump policy the
  mismatch wipes such saves to a fresh account, so no pool-hired ghost survives
  the retirement (`io.rs` rejects the version mismatch -> fresh kingpin).

---

## Implementation Plan

### Phase 1: Remove the generic model + call sites

- Delete `hire_dealer()`, `recruit()`, `DEALER_NAME_POOL`,
  `DEALER_PORTRAIT_POOL` from `src/save/types.rs` (deletion over `#[allow]`, per
  GUIDANCE). Keep `next_hire_cost()` (the shared hire ladder both authored hires
  spend).
- Delete `RosterHireButton` (`src/ui/components.rs`) and `theme::ROSTER_HIRE_BG`.
- `populate_roster_panel_system` (`src/systems/ui_update.rs`): replace the HIRE
  card block with a static, non-interactive "Hire dealers on the CITY MAP" hint;
  drop the now-unused `hire_cost` local.
- `roster_button_system` (`src/systems/input.rs`): drop ONLY the generic-hire arm
  + `hire_query` param. Keep the SOW-036 signature arm, the SOW-038 unlockable
  arm, and the save-on-dirty flow.
- `load_actor_portraits` (`src/assets/loader.rs`): delete ONLY the
  `DEALER_PORTRAIT_POOL` loop. Keep the `Silhouette` mapping and the SOW-036
  signature + SOW-038 unlockable loops.

### Phase 2: Reroute + save bump

- `forge.rs`: `recruit(...)` -> `zone_dealer(area, name, authored-face)`.
- `ensure_roster_on_run_start`: `recruit(&[])` -> `kingpin()`.
- `SAVE_VERSION` 9 -> 10 with a doc comment recording the policy wipe.

### Phase 3: Tests + tooling

- Delete generic-hire tests (`test_hire_dealer_*`,
  `test_first_hire_gets_first_pool_face`, `generic_pool_reserves_signature_faces`).
- Reroute incidental multi-dealer tests that used `hire_dealer()`/`recruit()` to
  `hire_signature_dealer(...)` or a direct `zone_dealer(...)` push (in
  `types.rs`, `save/mod.rs`, `ui/map_view.rs`, `ui/ledger_view.rs`).
- Add `roster_grows_only_through_authored_zone_hires`: cash alone hires nobody;
  the roster grows only via `hire_signature_dealer` / `hire_zone_dealer`, and
  every non-kingpin ends up an authored zone dealer (`signature_of.is_some()`).
- Leave `io::test_old_version_rejected` untouched (version-relative).
- `tools/e2e/playtest.ps1`: drop the `-Hire` switch + HIRE coords.

---

## Acceptance Criteria

**Functional:**
- The roster grows ONLY through `hire_signature_dealer` / `hire_zone_dealer`;
  there is no cash-only generic hire. `grep src/` finds zero production hits for
  `hire_dealer` / `recruit` / `DEALER_PORTRAIT_POOL` / `DEALER_NAME_POOL` /
  `RosterHireButton` / `ROSTER_HIRE_BG` (comments excepted).
- SOW-036 signature + SOW-038 unlockable hires are UNCHANGED and their tests
  stay green.
- `SAVE_VERSION == 10`; a pre-v10 save loads as a fresh kingpin.

**UX:**
- The deck-builder roster panel shows NO HIRE card - a static
  "Hire dealers on the CITY MAP" hint stands where it was.
- The CITY MAP still offers the signature HIRE (e.g. Bubba at Trailer Park) and
  the cred-gated unlockable (Gladys, `NEEDS CRED 5`).

**Code Quality:**
- Zero warnings on `cargo build` AND `cargo test`.

---

## Discussion

### Pure removal, not a redesign

SOW-038 already did the hard part - rehoming Gladys (the pool's last face) into
Trailer Park as an unlockable - specifically to pre-clear this retirement. So
SOW-039 only deletes the now-redundant generic path and reroutes the handful of
dev-scenario and test call sites onto the authored `zone_dealer` constructor.
No new mechanics; the map hire flow is the single way to grow the roster.

### Why a `SAVE_VERSION` bump with no field change

bincode carries no field-level migration, and a v9 save could contain a dealer
named "Gladys" that was hired GENERICALLY (`signature_of == None`, stationed at
`DEFAULT_STATION`) rather than as the authored Trailer Park unlockable
(`signature_of == Some("trailer_park")`, stationed there). Rather than write a
data-scrubbing migration for that one ghost, the SOW-021 version-bump policy
applies: bump to v10 and let `io.rs` wipe the mismatched save to a fresh
kingpin. No serialized field changed; the bump is purely the wipe trigger.

### Empty-roster recovery restores the kingpin

`ensure_roster_on_run_start` is the defensive guard for an impossible-by-invariant
empty roster. With the generic recruit gone, the correct replacement is the
KINGPIN - the guaranteed starter dealer every fresh empire begins with - not a
fabricated faceless hire.

---

## Conclusion

The generic hire pool is retired end to end: model (`hire_dealer`/`recruit` +
both pools), UI (`RosterHireButton`, the roster HIRE card, `ROSTER_HIRE_BG`),
input arm, the portrait-loader pool loop, and the e2e `-Hire` switch are all
gone. Dev scenarios and incidental multi-dealer tests reroute onto the authored
`zone_dealer` constructor; the empty-roster guard restores the kingpin.
`SAVE_VERSION` bumps 9 -> 10 to wipe any pre-v10 save that could hold a
generically-hired Gladys. Everything SOW-036/038 shipped - `zone_dealer`,
`hire_signature_dealer`, `hire_zone_dealer`(+`_core`), the map signature +
unlockable input arms, the signature + unlockable loader loops, and the Gladys
pilot - is KEPT verbatim. Unit-tested, zero warnings. Live boot/map flow pending
the human e2e drive.
