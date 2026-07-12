# SOW-027: Heat Economy — Pressure & Relief

## Status

**Merged** - 2026-07-12

## References

- **RFC-027:** [Heat Economy](../01-rfc/027-heat-economy.md)
- **Related:** SOW-026 (measured floor: Inferno-in-3-blind-sessions),
  RFC-018 (stat multipliers - retired here, ANSWERED below),
  RFC-019 (Heat upgrade - wired here), GDD narc composition tables
- **Branch:** sow-027-heat-economy
- **Implementation Time:** 2 days

---

## Implementation Plan

### Phase 1: Per-area narc compositions

**Deliverables:**
- `narc_deck.ron` restructured: per-area, per-tier deck compositions (counts
  of each narc card), Corner tiers gentle at the bottom (Cold: mostly
  Donut Break/Patrol, no convictions), Block starting sharper; validated at
  load (cards exist, counts sane, every area x tier defined or inherits)
- Narc deck construction: composition(dealer station, dealer heat tier)
- Retire narc stat multipliers: `narc_upgrade_tier` scaling out of
  calculate_totals/get_card_heat/display; ⚖ badge removed; conviction ticks
  and card faces now always show authored numbers
- Composition tables documented in the SOW Discussion

**Success Criteria:**
- Corner/Cold blind dead-hands accrue little heat; Block/Hot brings warrants
- No scaled-number displays remain anywhere

### Phase 2: Relief mechanics

**Deliverables:**
- Lay Low: roster action, fixed package (const-tuned: $200, 2 runs bench,
  -40 heat, floor 0, committed); reuses relocation ticking + guards
- Crooked Lawyer: immediate chunk (-25 heat for $625), roster button,
  disabled when unaffordable/heat too low to matter
- RFC-019 Heat upgrade wired in get_card_heat (positive-heat player cards
  only); upgrade choice screen unchanged
- All math pure + tested (incl. "upgrades never worsen negative-heat cards")

**Success Criteria:**
- Both coolers charge/cool exactly as configured and respect guards
- Heat upgrade measurably reduces accrued heat for upgraded cards

### Phase 3: UI

**Deliverables:**
- Roster card actions: LAY LOW and LAWYER buttons beside MOVE (consistent
  chip styling, disabled states with reasons)
- Laying-low status ("LAYING LOW · N RUNS") on cards + START RUN guard label

**Success Criteria:**
- Full loop visible: hot dealer -> lawyer chunk -> lay low -> back cooler

### Phase 4: Pacing verification + docs

**Deliverables:**
- Re-run the SOW-026 pacing scripts: fresh-floor (3 blind sessions <= Hot),
  target-play (Shrooms session 2-3 unregressed); Block/Wolf balance read
  (target-play sessions in the Block at mid heat; report survivability and
  payout feel vs the ~x2.8)
- Feature matrices (heat-system, card-system - badge removal), roadmap
  Iteration 6 entry is the coordinator's

---

## Acceptance Criteria

**Functional:** compositions load/validate; coolers work with guards; Heat
upgrade live; no scaled displays.
**Pacing (measured):** fresh floor <= Hot after 3 blind sessions; SOW-026
target-play pacing unregressed.
**Code Quality:** pure+tested math; zero new warnings.

---

## Discussion

### ANSWERED (Reed, 2026-07-12): retiring RFC-018 stat multipliers

Composition-primary difficulty replaces the ⚖ multiplier system - CONFIRMED.
Multipliers, `narc_upgrade_tier`, the ⚖ badge, and every scaled display are
gone; telegraph == card face == totals == resolution by construction.

### Composition format: sparse + inheritable (Reed's authoring concern)

Reed flagged that per-area x per-tier tables could become an authoring
burden as areas grow. Mitigation shipped in the format itself:

- `narc_deck.ron` has a single `default` per-tier ladder authored once
  (it IS the Corner's ladder - the starting area defines "normal").
- `areas` entries override ONLY the tiers that differ. An area absent from
  the file (or present with an empty `{}` map) inherits the default
  entirely - a new area can ship with zero narc authoring.
- Validation resolves inheritance at load, fails loud on unknown areas/
  tiers/card ids and missing default tiers, and logs the EFFECTIVE table
  per area x tier in debug builds so authors see what actually shipped.

Authoring example (this is the shipped `the_corner` entry - inherits all
six tiers from the default):

```ron
areas: {
    "the_corner": {},            // inherits the default ladder entirely
    "the_block": {
        "Cold": [ /* only tiers that differ need authoring */ ],
        ...
    },
}
```

### Shipped compositions (post-tune)

Default ladder (= the Corner). Card stats: donut_break 0e/0h · patrol 5e/5h
· suspect_identified 10e/10h · probable_cause 15e/10h · surveillance 20e/5h
· stakeout 30e/15h · anonymous_tip 5e/20h · undercover_op 30e/0h ·
tapped_lines 35e/0h · warrant@30 · caught_red_handed@60 · random_search@90.

| Tier | Composition |
|---|---|
| Cold | 10x donut_break, 2x patrol (ambient) |
| Warm | 8x donut_break, 3x patrol, 1x suspect_identified |
| Hot | 5x donut_break, 4x patrol, 2x suspect_identified, 1x probable_cause, 1x surveillance, 1x warrant |
| Blazing | 2x donut, 2x patrol, 2x suspect, 3x probable_cause, 2x surveillance, 1x stakeout, warrant, caught_red_handed |
| Scorching | 1x donut, 1x patrol, 2x suspect, anonymous_tip, 3x probable_cause, 3x surveillance, 2x stakeout, warrant, caught_red_handed |
| Inferno | suspect, anonymous_tip, 2x probable_cause, 2x surveillance, 3x stakeout, 2x undercover_op, tapped_lines, warrant, 2x caught_red_handed, random_search |

The Block overrides all six tiers to start roughly where the Corner's
Warm/Hot ends (its Cold ≈ the Corner's old Warm/Hot band; its Inferno is
the harshest table in the game).

**Tune applied (the one allowed):** the first-draft ladder (Cold: 8 donut/
3 patrol/1 suspect) still let the all-dead-hands blind floor cross Hot
(career 110 = Blazing after two sessions) because the tier ramp compounds
the player's own dead-hand heat. Cold/Warm/Hot each softened one notch
(Cold -> ambient, Warm -> old Cold, Hot -> old Warm); Blazing+ untouched.

### Pacing measurements (Phase 4)

Blind-play floor (closed-loop script, 2 hands/session, 3 sessions/save):

- **SOW-026 baseline (multipliers):** 184 heat = Inferno; separate fresh
  sessions could die at Base narc.
- **Pre-tune compositions:** m1: 0 / 75 / kingpin BUSTED at Hot;
  m2: 10 / kingpin BUSTED at Cold; m3: 45 / 110 (Blazing) / BUSTED.
- **Post-tune (shipped):** sweep A: 0 / 0 / **35 (Warm)**; sweep B:
  45 / 60 / **85 (Hot)**. No busts in either sweep. **Acceptance met.**

Residual blind-floor heat is dominated by the player's own dead-hand cards
(+15-40/session), not narc cards (~2-6/session at Cold/Warm) - composition
can't reduce it further; the coolers are the designed answer.

Target-play regression: payouts untouched by this SOW; blind sweeps banked
$19-$75 per Safe deal (SOW-026's target-play band was $36-75/session), so
Shrooms-at-$100 in session 2-3 is unregressed.

Block/Wolf x2.8 read (hustler forge, Ray at the Block, heat 30): blind
off-demand play banks $30/session (Wolf pays reduced x1.0 without Coke +
private location) while heat runs +45..+85/session (30 -> 115 -> 160 over
two sessions; no busts - dead hands can't conviction out). Reading: the
x2.8 headline never applies without demand investment (Coke is a $5,000
unlock), so the Block is pure heat-pressure until mid-game - which is the
intended gate, and exactly the profile the coolers monetize ($200/-40
lay-low between Block sessions). Flag for a future content pass: consider
a mid-price Block demand product so the x2.8 has a reachable first rung.

### Cooler decisions

- Both coolers are Available-only AND require heat > 0: jailed heat settles
  via release()/bail (heat_at_bust bookkeeping would overwrite a lawyer's
  reduction), and committed (relocating/laying-low) dealers stay committed.
  Ineligible buttons render disabled with the reason (NO HEAT).
- SAVE_VERSION stays at 5: `DealerStatus::LayingLow` is purely additive;
  old saves deserialize unchanged (verified by a live save roundtrip).
- Constants live beside MOVE_FEE in save/types.rs: LAY_LOW_COST=200,
  LAY_LOW_RUNS=2, LAY_LOW_COOLING=40, LAWYER_COST=625, LAWYER_COOLING=25.

### Zero-warnings baseline (Reed, 2026-07-12, scope addition)

Repo-wide sweep to zero compiler warnings for both `cargo build` and
`cargo test`, deletion over `#[allow]` (no exceptions were needed), as a
dedicated final commit. 4 tests that pinned dead methods (add_heat x3,
danger_name) were deleted with their methods. Future SOWs hold this
baseline: a PR adding warnings is a PR that isn't done.

### UI note (found during Phase 4 e2e)

The roster action stack (MOVE/LAY LOW/LAWYER) initially overflowed the
fixed 250px dealer card and slid UNDER the neighboring card, which stole
its clicks. Fixed by letting the identity column shrink (min_width 0 +
clip). The e2e harness reference clicks for roster buttons in
tools/e2e/playtest.ps1's header comment predate the stack; rows are still
clickable at the documented coordinates but per-button coordinates must be
measured from a screenshot (game-drive echoes the fit/dpi mapping).

---

## Acceptance Review

### Scope Completion: 100% (+ zero-warnings scope addition)

All 4 phases plus the repo-wide warnings sweep (39 → 0 for build AND test,
deletion over allow, no exceptions needed).

### Measured acceptance: MET

- Fresh floor: 3 blind sessions end at **Warm (35)** / **Hot (85)** across
  two sweeps, no busts — down from Inferno-184 with fresh GAME OVERs.
- Target-play pacing unregressed (payouts untouched; blind Safe-deal banking
  $19-75 brackets the SOW-026 band).
- Wolf ×2.8 read delivered: headline payout is unreachable without the
  $5,000 Coke demand investment — the intended mid-game gate; the Block is
  pure heat pressure until then, which the coolers monetize. Content flag
  recorded: a mid-price Block demand product would give ×2.8 a first rung.

### Architectural Compliance

172 tests (delta explained: 4 dead-method pins deleted with their methods,
new composition/cooler/upgrade tests added). Telegraph == card face ==
totals == resolution by construction (multiplier retirement). Sparse
inheritable composition format with effective-table debug logging answers
Reed's authoring concern. Zero-warnings is the repo baseline going forward.

### Notable fix

Roster action stack overflow (buttons sliding under the neighboring card,
stealing clicks) — identity column now shrinks+clips.

---

## Sign-Off

**Reviewed By:** ARCHITECT review + measured pacing evidence
**Date:** 2026-07-12
**Decision:** ✅ **ACCEPTED**
**Status:** Merged to main
