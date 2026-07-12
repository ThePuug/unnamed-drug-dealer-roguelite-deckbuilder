# SOW-028: Third Zone — The Strip + Zone Coherence

## Status

**Planned** - 2026-07-12

## References

- **Design:** studio repo `design-updates/2026-07-12-three-zones.md` (the
  zone matrix; Reed granted creative freedom incl. adjusting zones 1-2)
- **Umbrella:** RFC-024/025/027 (territories, cred gating, compositions) -
  content + one new persona under existing mechanics; no new RFC
- **Branch:** (proposed) sow-028-the-strip
- **Implementation Time:** 1-2 days (content-heavy)

---

## Implementation Plan

### Phase 1: The Strip exists

**Deliverables:**
- `shop_locations.ron`: the_strip (unlock ~$1,200, between Corner and Block)
- Strip narc composition override: "vice sweep" texture - heat-noisy,
  conviction-light (between Corner and Block pressure)
- New locations: Back of the Club, VIP Room (reuse authored backgrounds
  sensibly - locker_room/frat_house read nightlife; NEW background art is a
  Reed ask, flag what you'd want); Locker Room re-flavored into the Strip pool
- Strip shop stock: Ecstasy + Ice move here from the Block; ladder rungs
  (cash + cred) spliced between Corner top and Block entry

### Phase 2: The Pimp (new persona)

**Deliverables:**
- Pimp persona (unused portrait exists): area the_strip, base/reduced
  multipliers in the x1.5-2.2 band, 2 scenarios ("Night Shift" steady
  volume w/ generous heat cap; "VIP Treatment" high multiplier, tight cap),
  7-card reaction deck (crowd-flavored: cover-from-crowds, heat-from-scenes),
  narrative fragments in house voice
- Demand strings validated (SOW-021 pattern) and attainable at-or-before
  the Strip's ladder position

### Phase 3: Zone coherence pass (adjusting zones 1-2)

**Deliverables:**
- Desperate Housewife: area -> the_block, demands re-tuned mid-tier
  (Codeine/Ice - relief, not glamour) = the Block's first-rung clientele
  (closes the SOW-027 Wolf x2.8 flag)
- Frat Bro stays Corner; "Get Wild" may prefer Strip locations (cross-zone
  location preference is legal - demands are card names)
- Block shop keeps pure premium (Coke/Heroin/Fentanyl); modifiers grouped
  by zone identity (street craft / crowd craft / money craft)
- Every zone: >= 1 persona, coherent products/locations/modifiers; all
  validations green

### Phase 4: Tuning pass + verification

**Deliverables:**
- 3-zone pacing sweep with the harness: Corner->Strip unlock timing,
  Strip->Block, first x2.8-eligible session; sentence/bail/move/cooler
  interactions at Strip heat levels; ONE tuning iteration, re-measure
- e2e: Strip purchase, Pimp draws in Strip runs, Housewife draws in Block
  runs, re-zoned shop stock in the right shops
- Feature matrices, SOW status/Discussion (document the final zone matrix),
  roadmap Iteration 7 entry is the coordinator's

---

## Acceptance Criteria

**Functional:** three purchasable zones each with coherent clientele/
products/locations/narc texture; all load-time validations green.
**Pacing (measured):** ladder timings reported; Block has a reachable first
rung (Housewife sessions pay at her multiplier without Coke).
**Code Quality:** zero warnings (baseline); content pins updated
deliberately.

---

## Discussion

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
