# SOW-028: Third Zone — The Strip + Zone Coherence

## Status

**Review** - 2026-07-12 (all 4 phases implemented on `sow-028-the-strip`; 175 tests, zero warnings)

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

### The shipped zone matrix

| | The Corner (free) | The Strip ($1,200) | The Block ($2,000) |
|---|---|---|---|
| **Identity** | street craft — where everyone starts | crowd craft — clubs, cash, noise | money craft — private, premium, paranoid |
| **Clientele** | Frat Bro (×2.5) | Pimp (×2.0) | Desperate Housewife (×1.5, first rung), Wall Street Wolf (×2.8) |
| **Products** | Weed (start), Shrooms $100·c1, Codeine $250·c2, Acid $400·c3 | Ecstasy $1,600·c1, Ice $3,000·c3 (both re-zoned from the Block) | Coke $5,000·c4, Heroin $8,000·c5, Fentanyl $12,000·c6 |
| **Shop locations** | At the Park $250·c1, Storage Unit $1,500·c3 | Back of the Club $800·c1 | Safe House $2,500·c2, Abandoned Warehouse $3,500·c3 |
| **Shop modifiers** | (street tier) | Velvet Rope $1,200·c2 | (money tier) |
| **Buyer-only cards** | Locker Room, Frat House | VIP Room, Comp'd Bottles, Lost in the Crowd, Making a Scene | By the Pool, In a Limo |
| **Narc texture** | baseline ladder (the default IS the Corner) | vice sweep: heat-noisy, conviction-light — tips at Hot+ only, one per tier; zero busts in 6 blind sessions | conviction-heavy: warrants early, Blazing busts and jails |

### Measured pacing (blind harness, 2 hands/session, shared save)

| Zone (scenario) | Pre-tune heat/session → career | Post-tune heat/session → career |
|---|---|---|
| Corner (fresh, heat 0) | +25/+55/+55 → **135 Scorching** (breaks SOW-027 ≤ Hot bar) | +15/+0/−15 → **0 Cold** (bar passed with margin) |
| Strip (nightowl, heat 20) | +25/+85/+105 → 235 Inferno, 0 busts | +45/+50/+85 → 200, 0 busts |
| Block (hustler Ray, heat 30) | +30/+35 → 95 Blazing (Housewife draws) | +85 (Wolf draw), then Busted at Blazing → jailed 5 runs |

Banking under blind play: Corner $130 lifetime over 3 fresh sessions; Strip
+$69 over 2 sessions (Pimp pays reduced ×1 when blind play ignores demand).

**The one tuning iteration** (root causes measured from session logs — the
Corner's overage was ~90% buyer reaction cards, not narc or player cards):
1. Noise Complaint heat 20 → 10 (in BOTH Frat Bro and Pimp decks; the buyer
   plays it regardless of skill, and it drew twice per session).
2. Frat House heat 10 → 5 (the Frat Bro is now the Corner's only clientele,
   so his whole deck IS the fresh floor's buyer texture).
3. Strip ladder: anonymous tips only at Hot+ and exactly ONE per tier
   (draft had them at every tier, two at Hot+ — tips feed heat, heat feeds
   tiers, tiers feed tips: a +85/+105 compounding spiral).

**Post-tune reading.** The Corner floor is now heat-neutral under worst-case
play — blind sessions hover Cold. Real heat comes from products sold, which
is the right shape for a starting zone. The Strip stays the hottest zone
per session (+45-85) *by design* — it is a heat tax, not handcuffs (six
blind sessions, zero busts), and the counter-play is exactly the rotation
loop Reed described: burn on the Strip, cool on the Corner. The Block
completes the contrast: fewer heat points but Blazing-tier busts with
5-run sentences. One acceptance judgment call for Reed: whether Strip
session 3 (+85 at Blazing) should soften further — deliberately left after
the single allowed iteration.

**Variance note:** one discarded post-tune Corner session busted on hand 1
at Cold — Prior Conviction (+20 evidence, pre-existing Frat Bro card) plus
blind junk-play beat the fresh deck's cover. Pre-existing exposure, not a
tune effect; blind play folds no hands, so this is the floor's worst case.

### First-rung verification (the SOW-027 Wolf ×2.8 flag)

Shipped-content test `test_shipped_block_first_rung_pays_without_coke`
pins it: the Housewife (Block, ×1.5) is demand-satisfiable with
STARTING-collection Weed on her "In Denial" scenario once she plays her
own By the Pool — Block expansion pays before any Block product buy, and
her Codeine demand ($250 Corner rung) is the deliberate step up. Verified
live: Block runs draw her ("Run area: the_block - buyer: Desperate
Housewife").

### e2e verification (nightowl scenario)

- Shop shows three area tabs; Strip stock: Ecstasy $1,600 purchasable with
  "unlocked by The Kingpin" credit line, Ice locked "NEEDS CRED 3 (best:
  2)", Back of the Club $800, Velvet Rope $1,200.
- Live purchase: Ecstasy $1,600 → cash $2,500 → $900, card flips to OWNED.
- Persona draws per zone in run logs: Frat Bro (Corner ×6), Pimp (Strip
  ×6), Housewife + Wolf (Block).
- Strip cred accrues: forged 2 → CRED 4 after two Safe-deal sessions.

### Creative deviations from the design sketch (recorded rationale)

1. **Locker Room stays with the Frat Bro** (sketch suggested re-flavoring
   it into the Strip pool). Moving it would have broken the Frat Bro's
   shipped scenario location lists and his "Get Laid" demand; the Strip
   got purpose-built VIP Room + Back of the Club instead.
2. **Housewife demands are Codeine/Fentanyl, not Codeine/Ice** (sketch
   said "Codeine/Ice — relief, not glamour"). Ice moved to the Strip as
   party stock; giving her Ice would cross-zone her demand for no gain.
   Her shipped scenarios already carry Codeine (mid-tier) and Fentanyl
   (Block top-shelf) — relief-coded both.
3. **Strip narc texture tuned mid-SOW** (tips Hot+ only, one per tier) —
   the draft's "tips everywhere" reading of vice-sweep compounded; the
   shipped rule keeps the fiction ("nobody talks about a nobody") with
   measured pacing.
4. **Phases 1-3 in one commit**: the "every area has clientele" validation
   makes the Strip unshippable without the Pimp — content phases were
   inseparable (SOW-023/026 precedent).

### Art asks for Reed

- Dedicated **club back-alley** background (Back of the Club currently
  reuses `dead_drop.png` — the neon alley reads right, but it's shared).
- Dedicated **VIP lounge** background (VIP Room reuses `frat_house.png`,
  the PARTY ZONE lounge).
- The Pimp uses the previously unused portrait as planned.

---

## Acceptance Review

*Populated after implementation.*
