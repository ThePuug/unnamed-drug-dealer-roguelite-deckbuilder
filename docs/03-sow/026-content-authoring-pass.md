# SOW-026: Content Authoring Pass — The Progression Ladder

## Status

**Planned** - 2026-07-12

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

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
