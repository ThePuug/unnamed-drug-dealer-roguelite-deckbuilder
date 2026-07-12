# SOW-031: Suppliers & Fronts

## Status

**In Progress** - 2026-07-12

## References

- **Design:** studio repo `design-updates/2026-07-12-supplier-mechanic.md`
  (v2 CONFIRMED by Reed with rationale: due-dates turn run COUNT into run
  QUALITY pressure - the counterweight to free fold-early exits)
- **Locked defaults (Reed):** fronts against CARDS (own forever once paid
  off); ONE supplier per zone; due-dates tick at the run-completion choke
  (same machinery as jail/relocation); escalation cut off -> muscle ->
  soured
- **Branch:** sow-031-suppliers-fronts
- **Implementation Time:** 2 days (model + UI + content)

**Schema note:** unlike SOW-030, this SOW legitimately ADDS save state
(active fronts, supplier standing) - additive, serde-defaulted,
migration-safe. Old saves load with no fronts and clean standings.

---

## Implementation Plan

### Phase 1: Model (pure, TDD)

**Deliverables:**
- `SupplierDef` authored per zone in `shop_locations.ron` (name, voice
  line, standing thresholds if needed) - and fold in the SOW-029 carry:
  zone `identity` + `narc_hint` strings move from code into the same RON
  entries (load-time validated; the map's code-side fallbacks retire)
- Save additions (serde defaults): active fronts (card name, amount owed,
  runs remaining, zone), supplier standing per zone
  (Good / CutOff / Soured) + muscle-pending flag
- Pure logic + unit tests: take_front (vig ~25% over shop price, due in
  3-4 runs - tune in Phase 4), pay_front (any time, cash only), the
  run-completion tick (shared choke with jail/relocation, runner's own
  run counts), escalation ladder on expiry: CutOff (that supplier's
  stock locked until settled) -> Muscle (one-time cash seizure ~20% or
  active dealer benched 1 run if broke) -> Soured (that supplier never
  fronts again; a supply-side prior_convictions scar)
- Guards: one active front per supplier; can't front while CutOff/Soured
  with that supplier; kingpin invariant untouched

### Phase 2: Shop + hub surfaces

**Deliverables:**
- Supplier header on each zone's shop tab: name, voice line, standing,
  active front status ("DUE IN 2 RUNS - $625") - v1 fiction and v2
  mechanics in one surface
- FRONT button on product cards the player can't afford in cash (shows
  owed amount + due window before commit; same one-code-path discipline
  as unlocks/moves); card plays normally while fronted, becomes owned on
  payoff
- PAY button wherever the debt is visible (shop header + hub indicator)
- Hub pressure indicator near START RUN when a front is live: "FRONT DUE
  IN N RUNS - $X TO <SUPPLIER>" (the whole point is feeling the clock)

### Phase 3: Map + ledger integration

**Deliverables:**
- Map node card: supplier line (name + standing + due counter when a
  front is live) - the SOW-029-confirmed placement
- Ledger empire strip: outstanding debt figure when nonzero (derived
  from active fronts); soured suppliers surface on the zone history line

### Phase 4: Content, pacing + verification

**Deliverables:**
- Three suppliers authored with distinct voices (creative license:
  Corner street plug, Strip club connect, Block importer archetypes)
- Forge scenario `fronted` (live front mid-window) + `broke` variant
  (front + no cash, one run to due)
- Measured pacing: a $500-class front must be serviceable by roughly one
  Safe deal per window (Reed's tuning bar); muscle seizure must sting
  but not death-spiral a fresh empire; ONE tuning iteration, re-measure
- e2e: take front -> run -> pay -> card owned; default -> cut off (stock
  locked) -> second expiry -> muscle fires -> soured blocks future
  fronts; hub indicator counts down; screenshots to out\sweep31\
- Feature matrices, SOW Discussion (+ new art asks for the coordinator
  to harvest), status -> Review; roadmap Iteration 10 entry is the
  coordinator's

---

## Acceptance Criteria

**Functional:** full front lifecycle (take, service, own / default,
escalate, sour) live across shop, hub, map, ledger; zone strings authored
in RON with validations green.
**Pressure (measured):** an unproductive run visibly costs a tick; the
window is serviceable by target play per Reed's bar.
**Integrity:** old saves load clean (serde defaults); zero warnings;
model logic fully unit-tested; overlay/init_resource lessons applied
where any new UI state appears.

---

## Discussion

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
