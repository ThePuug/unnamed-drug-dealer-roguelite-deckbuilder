# SOW-032: Tutorial Arc — Road to Your First Dealer

## Status

**Review** - 2026-07-15 (implemented on the CURRENT main base =
SOW-037/038/039 merged, SAVE_VERSION 10 → 11; reconciled to the settled loop —
see Discussion)

## References

- **Design:** studio repo `design-updates/2026-07-13-tutorial-arc.md`
- **Locked constraints (Reed, 2026-07-12):** OPTIONAL at empire start;
  skipping confers NO benefit — the arc earns exactly what ordinary play
  would. "No rest for the weary."
- **Branch:** (proposed) sow-032-tutorial-arc
- **Implementation Time:** 1-2 days

---

## Implementation Plan

**Design consequence of no-skip-benefit (hard constraint):** the tutorial
is GUIDED PLAY, not a mode. Same economy, heat, narcs, RNG. It adds only
DIRECTION: a goal strip on the hub showing one beat at a time. No beat
grants anything beyond what the action itself earns.

### Phase 1: Beat model (pure, TDD)

**Deliverables:**
- Beat definitions + completion DETECTION from existing save state
  (derive, don't record): (1) first demand-matched deal, (2) first
  banked session with heat transfer, (3) first front taken, (4) first
  front resolved (payback OR the default-escalation lesson — both
  teach), (5) first cash+cred shop unlock, (6) graduation: first $500
  hire
- Save addition: tutorial progress (offered/declined/current beat/done)
  — SAVE_VERSION bump per SOW-021 policy
- Pure view-model for the goal strip line + hint per beat (house voice)

### Phase 2: Offer + goal strip (UI)

**Deliverables:**
- One-time offer at empire start: TAKE THE GUIDED START / I KNOW THE
  STREETS (skip); re-offered on new empires after game over
- Goal strip on the hub (near the tab row): current beat + hint,
  dismissible any time ("SKIP THE LESSONS" — always free, the arc never
  held anything back); retires at graduation with the closing beat
  ("You're not a dealer anymore. You're a kingpin.")
- Beat-completion toast/advance on the run-completion + hub choke points
- Bevy lessons apply: init_resource for any new UI state; FocusPolicy
  only if anything overlays; presentation derivation in the _view module

### Phase 3: Verification

**Deliverables:**
- Unit tests per beat detector (incl. out-of-order completion: a player
  who hires before ever fronting completes beat 6 and the arc
  fast-forwards past satisfied beats)
- e2e: fresh empire → accept → walk beats 1-3 live (deal, bank, front);
  forge mid-arc scenario for beats 4-6; skip path (decline → no strip,
  nothing different afterward); screenshots to out\sweep32\
- Feature matrices, SOW Discussion; roadmap Iteration 11 entry is the
  coordinator's

---

## Acceptance Criteria

**Functional:** offer, six detected beats, dismiss-any-time, graduation
retirement; declining produces an experience identical to pre-SOW play.
**No-benefit invariant (testable):** for identical action sequences, a
tutorial save and a skipped save hold identical cash/heat/cred/cards.
**Code Quality:** zero warnings; beat detection fully unit-tested; save
version bump handled per policy.

---

## Discussion

### Reconciliation to the CURRENT loop (2026-07-15)

The authored plan predated SOW-034 (consumable stock), SOW-036/038 (zone
signature + unlockable dealers), and SOW-039 (generic-hire retirement). The
beats were re-grounded on the loop as it exists on THIS base, as PURE
predicates over EXISTING save fields (no new "record" fields — derive, don't
record). Each authored beat mapped to a real current field:

| # | Authored beat | Predicate over CURRENT SaveData |
|---|---|---|
| 1 | First deal | `account.hands_completed >= 1` |
| 2 | Go home hot | `Σ dealers[*].character.decks_played >= 1` |
| 3 | First front | `!fronts.is_empty()` (latched by the cursor) |
| 4 | First payback | `fronts.is_empty()` **again** — reachable only past beat 3, so an empty ledger here means PAID or SOURED (both teach), never "never fronted" |
| 5 | First rung → **restock** | `account.unlocked_cards != AccountState::starting_collection()` (a `buy_batch` grows the collection) |
| 6 | Graduation (first $500 hire) | `dealers.len() >= 2` — ALSO short-circuits the whole arc to Graduated (hire-first fast-forwards past unwalked beats) |

**Settled-loop reconciliations (the two that moved):**

- **Map-only hiring (SOW-039).** The roster HIRE button is gone; the hub roster
  shows a "Hire dealers on the CITY MAP" hint. The first hire is the zone's
  SIGNATURE dealer (Bubba @ Trailer Park, $500, no cred gate) on the CITY MAP;
  cred-gated unlockables (Gladys) need cred. Beat 6 therefore teaches MAP
  hiring — its hint is "Open the CITY MAP and hire the zone's dealer", not a
  roster button. Authored beat 5 ("first rung": a cash+cred shop unlock)
  became **restock** because permanent unlocks were re-laddered into consumable
  stock: the ladder the player now feels is buying the next batch.

- **Products-only stock (SOW-034 / SOW-040 reversal).** Products are consumable
  charges (buy/front a batch, each play burns one); UTILITY cards are NOT
  consumable (Reed reversed SOW-040). The restock beat is therefore
  products-only, and the front beats (3/4) ride the same batch/charge economy
  the rest of the game uses.

**The cursor.** `TutorialState { status, cursor }` — `cursor` is the latched
high-water mark of walked beats (0..=6, an index into `Beat::ORDER`). It NEVER
decrements; `advance(&SaveData)` walks it forward one beat at a time in order
(the sequential gate is what gives beat 4 its "PAID" meaning) and flips to
Graduated the moment `dealers.len() >= 2`. Only an Accepted run advances:
Declined stays retired forever (resurrecting a skipped run's strip would break
the no-benefit invariant), Graduated is terminal.

**No-benefit invariant (hard constraint, testable).** The arc is purely
presentational — it never touches cash/heat/cred/unlocked_cards/stock/roster.
`tutorial_confers_no_gameplay_benefit` runs two identical action-histories
(Accepted vs Declined) and asserts every economy field is identical while the
tutorial state itself diverges.

**Graduation display.** The strip retires WITH its closing beat: while
Graduated it shows "You're not a dealer anymore. You're a kingpin." and drops
its dismiss button (nothing left to skip). Declined hides the strip entirely.
Offered hides the strip and raises the Block offer overlay instead.

**Bevy lessons applied.** The offer overlay carries `FocusPolicy::Block`
(GUIDANCE lesson 1) and is spawned only while `status == Offered`, AFTER the
pending-upgrade early-return guard in `setup_deck_builder` (GUIDANCE lesson 2).
No NEW state-wide resource was needed — the tutorial's state rides in SaveData
(already a resource), so there is no bare `ResMut` on a conditionally-inserted
resource. The three Update systems query for the strip/overlay entities, so the
pending-upgrade frame (where the strip is not spawned) is a clean no-op.

### Code map

- `src/ui/tutorial_view.rs` (NEW, pure): `Beat`, `beat_satisfied`,
  `derive_view` → `GoalStripView`, house-voice copy constants, graduation
  short-circuit. Unit-tested without ECS.
- `src/save/types.rs`: `TutorialStatus`, `TutorialState` (+ `advance`), the
  `SaveData.tutorial` field, `SaveData::new` init, SAVE_VERSION 10 → 11.
- `src/systems/tutorial.rs` (NEW): `tutorial_offer_button_system`,
  `tutorial_progress_system`, `populate_goal_strip_system`,
  `spawn_tutorial_offer_overlay`.
- `src/ui/setup.rs`: goal strip (under the tabs) + conditional offer overlay.
- `src/ui/components.rs`: the six markers.
- `src/save/forge.rs`: `tut_offer` / `tut_front` / `tut_restock` / `tut_hire`
  scenarios + the no-benefit invariant test.

---

## Acceptance Review

*Populated after implementation.*
