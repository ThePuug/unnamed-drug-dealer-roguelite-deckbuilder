# SOW-031: Suppliers & Fronts

## Status

**Review** - 2026-07-13 (all 4 phases on `sow-031-suppliers-fronts`; 253 tests, zero warnings; full lifecycle e2e-verified live)

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

### Shipped mechanics (final numbers)

- **Vig 25%** (`front_owed`: $100 → $125, $500 → $625, $1,000 → $1,250);
  **window 4 runs** (`FRONT_WINDOW_RUNS`); **muscle seizure 20%** of cash
  on hand (`MUSCLE_SEIZURE_PCT`), or the active dealer benched 1 run
  (via `Relocating`, not jail - the kingpin invariant holds) when 20%
  rounds to zero; an unavailable dealer is spared the beating but the
  repossession and souring land regardless.
- **The tick includes the runner's own run** - unlike jail. That IS the
  mechanic: three consecutive unproductive runs (measured live: blind
  InvalidDeals) burned a full window and triggered CutOff.
- **Escalation:** Good → (expiry) CutOff [stock locked, debt keeps a
  fresh window] → (expiry) muscle + card repossessed + Soured
  [permanent; cash purchases fine, fronts never again].
- **PAY any time** from cash; settling during CutOff restores Good -
  Soured is the only permanent mark. One front per supplier; fronting
  requires Good standing AND the item's cred gate (rep gates trust).
- **Suppliers authored** in shop_locations.ron: Lil Smoke (Corner),
  Miss Velvet (Strip), The Broker (Block), each with a voice line on
  the shop header. Zone `identity` + `narc_hint` moved from map_view
  code into the same RON entries, required at load (SOW-029 carry
  closed; E3's spirit for zone flavor).

### Save-schema note (deviation from the SOW text)

The SOW said "old saves load clean (serde defaults)". The persistence
layer is bincode, which has NO field-level migration - serde defaults
can't help a binary format. The shipped handling is the established
SOW-021 policy: **SAVE_VERSION bumped 5 → 6**, version-mismatched saves
fall back to a fresh account (Reed: dev save wipes are a non-concern
pre-release). The serde defaults still stand for in-code construction.
Also folded per the SOW-021 pattern: a load-time `normalize()` hook
(currently: legacy kingpins wearing "Barista" get the Silhouette).

### Reed art drop (mid-flight fold-in)

Five dealer portraits renamed + wired (julie, marcus, gladys, bubba,
roxanne): `DEALER_PORTRAIT_POOL` 8 → 13 appended in order - 13 faces >=
12 names, no face duplication until roster 14. The kingpin now wears
Reed's **silhouette.png** ("Silhouette" key - the generic
no-chosen-face-yet placeholder) instead of borrowing barista.png;
Barista returned to the hire pool (the first hire now draws it - no
test pinned the old order). Character customization later replaces the
normalize() line.

### Measured pacing (blind + targeted, isolated saves)

- **Blind play is the floor:** three blind Corner runs on the `fronted`
  save produced $0 (InvalidDeals - blind clicking rarely completes
  product+location) and burned the whole window → CutOff. The pressure
  is real and visible (clock counted 3 → 2 → 1 on the hub).
- **Targeted play services the bar:** one targeted run (2 hands) earned
  $80 from the starting deck at Cold-Warm heat; the $125 Shrooms debt
  cleared inside the CutOff grace window. Scaled: a $500-class front
  (Acid $400 → owed $500, $125/run over the window) needs roughly one
  demand-matched deal per run - exactly Reed's serviceability bar with
  the deck the front itself provides (fronted Shrooms deals $100 at
  Frat ×2.5).
- **Muscle stings without death-spiraling:** measured seizure $13 off
  $65 cash; the real cost is the repossession (deck 9 → 8) + the
  permanent supply scar. A fresh empire survives it intact.
- **NO tuning iteration spent** - the measured numbers land on the
  design intent (tight for waste, serviceable for focus). Flagged for
  Reed's human playtest instead: if the 4-run window feels brutal with
  fold-heavy play, widen `FRONT_WINDOW_RUNS` to 5 before touching the
  vig.

### e2e evidence (out\sweep31\, isolated DDD_SAVE_DIR per flow)

- **Take (A1-A2, legacy):** supplier header quiet on Good; FRONT $1250
  button ONLY on the unaffordable product (Acid $1,000 vs $800 cash;
  affordable Codeine shows none); click → log `Fronted acid in
  the_corner: $1250 due in 4 runs`, card flips OWNED, cash untouched,
  header gains DUE line + PAY, hub clock appears next to START RUN.
- **Ledger + map (A3-A4):** OWED TO SUPPLIERS $1,250 (red, 7th strip
  stat, hidden at $0); Corner node "SUPPLIER: LIL SMOKE · DUE IN 4 RUNS
  — $1250" below the chips (harness chip-y contract intact), Strip
  "SUPPLIER: MISS VELVET", locked Block pitches THE BROKER.
- **Tick + CutOff (pt-fronted, B1):** clock 3 → 2 → 1 across blind runs
  (`FRONT DUE IN 2 RUNS — $125 TO LIL SMOKE`); third dud →
  `supplier cut you off - one more window`; shop shows red CUT OFF
  header + every purchasable card locked ("CUT OFF settle your debt");
  PAY still offered.
- **Redemption (B5-B11):** targeted hands earned $52 + $28 (the second
  playing the FRONTED Shrooms card - fronted cards play normally); GO
  HOME ticked the CutOff window 4 → 3; PAY click → `Front settled in
  the_corner (cash: $15)` → Shrooms OWNED forever, header quiet (Good
  restored), stock unlocked with fresh FRONT offers, hub clock gone.
- **Muscle + soured (pt-strapped, C1):** one run at CutOff/1-run-left →
  `Muscle visited over the the_corner front: seized $13` +
  `Supplier in the_corner soured: shrooms repossessed, no more fronts
  there`; shop shows SOURED — cash only (purchases open, zero FRONT
  buttons), Shrooms back on the shelf at $100, deck 8/20.
- Kingpin silhouette + new hire faces confirmed rendering on roster
  cards throughout.

### Deviations (rationale)

1. **Save-version bump instead of field migration** (above) - bincode.
2. **CutOff lock renders behind the cred gate on cred-locked items**
   (spawn order: cred check first). Both read as locks; the cred gate
   is the more informative of the two for those items.
3. **`strapped` scenario** instead of the SOW's "broke variant" name -
   it forges CutOff + 1 run + $40 so ONE run demonstrates muscle
   (waiting through a second full window in e2e would be pure padding).
4. **Muscle benches via `Relocating`** - reuses the tick machinery with
   zero new states; the chip note reads "MOVING · 1 RUN" which is
   tonally off for a beating. Cosmetic; flagged if Reed wants a
   dedicated status later.
5. **playtest.ps1 -Hire untouched** - both new scenarios are
   single-dealer (kingpin only), so the dealer-count switch is correct
   as-is.

### For Reed

- **Window feel** is the one open tuning knob (4 runs, own runs count).
  Blind/wasteful play gets punished fast BY DESIGN - confirm it feels
  like pressure rather than punishment in real play; widen to 5 before
  touching the 25% vig if not.
- **Muscle bench flavor**: "MOVING · 1 RUN" on a beaten dealer - want a
  dedicated "ROUGHED UP" status note someday?
- **No new art asks** beyond what's already ledgered (supplier
  portraits, #9) - the header is text + voice by design until those
  land. The art drop CLOSES ledger #4/#5 (pool expansion + Pimp slot);
  the silhouette closes the kingpin-face gap (#3) with an explicit
  placeholder until customization.

---

## Acceptance Review

*Populated after implementation.*
