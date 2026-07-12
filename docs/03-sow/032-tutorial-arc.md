# SOW-032: Tutorial Arc — Road to Your First Dealer

## Status

**Planned** - 2026-07-13 (authored at the Iteration 10 pause; NOT started —
launch is the next session's first move)

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

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
