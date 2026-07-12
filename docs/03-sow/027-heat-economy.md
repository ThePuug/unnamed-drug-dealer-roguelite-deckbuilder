# SOW-027: Heat Economy — Pressure & Relief

## Status

**Planned** - 2026-07-12

## References

- **RFC-027:** [Heat Economy](../01-rfc/027-heat-economy.md)
- **Related:** SOW-026 (measured floor: Inferno-in-3-blind-sessions),
  RFC-018 (stat multipliers - retired here, PENDING Reed's confirmation),
  RFC-019 (Heat upgrade - wired here), GDD narc composition tables
- **Branch:** (proposed) sow-027-heat-economy
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

### PENDING Reed: retiring RFC-018 stat multipliers

Composition-primary difficulty replaces the ⚖ multiplier system. Flagged
before merge; implementation proceeds on the default (retire) - trivially
revertible to hybrid if Reed prefers.

---

## Acceptance Review

*Populated after implementation.*
