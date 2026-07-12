# SOW-026: Content Authoring Pass — The Progression Ladder

## Status

**Review** - 2026-07-12 (all 3 phases complete on branch; pacing measured)

## References

- **Design:** studio repo `design-updates/2026-07-12-world-map-and-areas.md`
  ("the gradient is primarily an AUTHORING thing" - Reed) and
  `2026-07-12-stationing-and-street-cred.md` (cred requirements)
- **Umbrella RFCs:** RFC-024 (territories), RFC-025 (cred gating) - no new
  RFC; this SOW is content under existing mechanics
- **Branch:** (proposed) sow-026-authoring-pass
- **Implementation Time:** 1 day (mostly RON + validation + tuning)

---

## Implementation Plan

### Phase 1: Lean start

**Goal:** A fresh empire starts small enough that every unlock is felt.

**Deliverables:**
- Starting collection trimmed: **Weed is the only starting product** (Reed's
  example); minimal viable locations/covers/modifiers/insurance so the
  default deck still validates (>=1 Product, >=1 Location, deck-size minimum)
- Everything trimmed moves INTO shop stock (nothing deleted - re-laddered)
- Fresh-empire default deck preset updated; load-time validation extended:
  starting collection must produce a legal deck (fail loudly in debug)

**Success Criteria:**
- Fresh empire boots to a valid Weed-only-product deck and can play hand 1

### Phase 2: The ladder

**Goal:** Shop stock across areas forms a visible cash+cred progression.

**Deliverables:**
- Corner shop: early products (Shrooms as the FIRST unlockable per Reed -
  low price + low cred requirement; then Codeine/Acid tiers), support cards
  laddered behind modest cred
- Block shop: premium products (Ecstasy/Ice/Coke/Heroin/Fentanyl) with
  steeper cash + Block-cred requirements; Heroin/Fentanyl at the top
- Every shop item: price + cred_required tuned as a coherent curve
  (document the intended ladder in the SOW Discussion as a table)
- Buyer demands per area re-checked: each area's clientele demands products
  attainable at-or-before that area (validation already enforces
  demand-string existence; extend to warn if a demanded product is gated
  ABOVE its buyer's area ladder position)

**Success Criteria:**
- Ladder table in Discussion matches shipped RON; validation green

### Phase 3: Playtest + tune

**Goal:** The first hour feels like progression, not paperwork.

**Deliverables:**
- Closed-loop scripted sessions from `fresh`: measure hands-to-first-unlock
  (Shrooms), hands-to-Block-access at current prices/cred rates; adjust
  prices/thresholds once and re-measure
- Screenshots: fresh shop (locked ladder visible), first unlock moment
- Feature matrices (card-system, progression-meta), roadmap Iteration 5 entry

**Success Criteria:**
- First unlock reachable inside ~2-3 sessions of ordinary play; Block access
  a meaningful mid-term goal (report the measured numbers either way)

---

## Acceptance Criteria

**Functional:** fresh empire playable start to first unlock; nothing
orphaned (all cards reachable somewhere on the ladder); validations green.
**UX:** the shop reads as a ladder - locked items visibly show their price +
cred requirement.
**Code Quality:** content changes validated at load; zero new warnings;
tests updated where content literals are pinned.

---

## Discussion

### The shipped ladder (Phase 2)

Starting collection (8 cards, SAVE_VERSION 5): Weed, Dead Drop, Parking Lot,
Alibi, Fake Receipts, Fake ID, Burner Phone, Lookout.

| THE CORNER | Type | Price | Cred |
|---|---|---|---|
| Shrooms | Product | $100 (tuned from $150) | 1 |
| At the Park | Location | $250 | 1 |
| Codeine | Product | $400 | 2 |
| False Trail | Modifier | $600 (repriced from $1200) | 2 |
| Acid | Product | $1000 | 3 |
| Storage Unit | Location | $1500 | 3 (SOW-025 pilot) |

| THE BLOCK | Type | Price | Cred |
|---|---|---|---|
| Ecstasy | Product | $2000 | 1 |
| Disguise | Modifier | $1800 | 1 |
| Bribe | Cover | $2000 | 1 |
| Safe House | Location | $2500 | 2 |
| Clean Money | Modifier | $2500 | 2 |
| Ice | Product | $3000 | 2 |
| Bribed Witness | Cover | $3000 | 3 |
| Abandoned Warehouse | Location | $3500 | 3 |
| Plea Bargain | Insurance | $4000 | 3 |
| Coke | Product | $5000 | 4 |
| Heroin | Product | $8000 | 5 (SOW-025 pilot) |
| Fentanyl | Product | $12000 | 6 |

### Implementation Note: phases 1-2 share a code commit

The collection trim and the re-laddered prices are one content change - a
split would not build. SOW-023 precedent.

### Pacing measurements (Phase 3)

Blind-play floor (closed-loop script, no strategy), 3 sessions on one fresh
save: outcomes Safe/InvalidDeal, InvalidDeal/BuyerBailed, InvalidDeal/
BuyerBailed -> **$30 banked, 1 Corner cred, kingpin at 184 heat (Inferno)**.
Post-tune fresh session: InvalidDeal -> kingpin BUSTED (GAME OVER) - the
blind floor can die at Base narc.

Reading: the lean 1-product start means a productless hand is a dead hand
(expected - the design), but blind play pays full heat for dead hands. A
target-play session (play Weed when drawn, bail dead hands, GO HOME after
the Safe deal) banks $36-75 + 1 cred per short session -> **Shrooms at $100
lands in session 2-3** (SOW target met); Block access ($2,000 + $2,000
entry product) projects to a 10-20 session mid-term goal.

**Tune applied (the one allowed):** Shrooms $150 -> $100 - price was the
binding constraint (the cred gate is met in session 1).

**Flags for SOW-027 (heat economy):**
1. Dead hands accumulate full narc heat with zero income - the fresh
   kingpin hit Inferno in 3 bad sessions and has NO cooling path except
   wall-clock decay (184 hours!). Active cooling (Lay Low/lawyer) and/or
   per-area narc softness must cover the fresh-player floor.
2. The kingpin's own bust being GAME OVER + the heat spiral means weak
   early play is punishing; consider whether the Corner's narc baseline
   should be gentler (per-area narc decks are SOW-027 scope).

### Harness addition (Phase 3)

playtest.ps1 gained `-NoForge` and `-SaveDir` for multi-session pacing runs
on a persistent save.

### e2e evidence (Phase 3)

Verified on screen (hustler): the full Corner ladder renders with prices,
cred credit lines ("unlocked by The Kingpin") on every gated item, Shrooms
at the tuned $100, Weed the only OWNED product, lean 8/20 deck VALID.
Post-session roster screenshot after 3 blind sessions confirmed cash/cred/
heat bookkeeping ($30 / CRED 1 / 184 Inferno).

---

## Acceptance Review

*Populated after implementation.*
