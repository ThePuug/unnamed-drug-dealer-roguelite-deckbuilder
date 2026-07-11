# SOW-021: Stabilization — Defect Fixes and Data Integrity

## Status

**Merged** - 2026-07-12

## References

- **Review:** Design & Fun Review 2026-07 (`unnamed-indie-studio-internal` repo: `projects/drug-dealer-roguelite/review-2026-07.md`) — this SOW implements the review's "Phase 0 — Defuse"
- **Spec:** [Card System](../00-spec/card-system.md), [Bust & Insurance Mechanics](../00-spec/bust-insurance-mechanics.md), [Core Gameplay Loop](../00-spec/core-gameplay-loop.md)
- **RFC:** None — all items are defect corrections against already-approved specs, per process discussion (2026-07-11)
- **Branch:** sow-021-stabilization (merged)
- **Implementation Time:** ~6 hours (including adversarial review pass)

**Pre-approved decisions (from review Q&A, 2026-07-11):**
1. Upgrade thresholds: **spec values are canonical** (0/3/8/15/25/40 plays per `card-system.md:114`), superseding the `production: 5/12/25/50/100` code comments
2. Save compatibility: **wiping existing saves is acceptable** (version bump, no migration)
3. The Block unlock gating: **out of scope** — location unlocking is its own future SOW
4. Demand data: **keep RON demand lists authorable** (human-readable card names, NOT converted to IDs); kill the brittleness with load-time validation instead
5. Process: single SOW, no RFC

---

## Implementation Plan

### Phase 1: Core Rules Defects

**Goal:** Make the implemented rules match the approved specs and eliminate the latent character-deletion path.

**Deliverables:**
- Insurance heat penalty charged exactly once, at activation (currently double-charged: on play via `get_card_heat` in `state_machine.rs:236` AND on activation in `resolution.rs:114`)
- Upgrade tier thresholds set to spec values 0/3/8/15/25/40 (`UpgradeTier::from_play_count` and `next_threshold` in `save/types.rs`, currently TESTING 1/2/3/4/5); TESTING/production comment annotations removed
- `SAVE_VERSION` bumped 1 → 2; loading an older version follows the existing `UnsupportedVersion` path and gracefully starts a fresh account (no crash, no partial load)
- Deck exhaustion (< 3 cards) routed to a neutral session end equivalent to GO HOME, replacing the internal `HandOutcome::Busted` mapping (`state_machine.rs:82`) that currently sits behind a UI-only button guard

**Architectural Constraints:**
- Spec is authoritative: `bust-insurance-mechanics.md` states heat penalty is gained when insurance *activates*, not when played
- Insurance card UI must not imply an on-play heat cost after the fix (the displayed penalty is contingent on activation)
- Deck exhaustion must be safe even if reached through a non-UI path (internal state must never map exhaustion to character deletion); player sees an explanatory message, banked session state transfers exactly as GO HOME does
- Existing characters' derived tiers will shift under new thresholds — acceptable, saves are wiped by the version bump

**Success Criteria:**
- Playing Fake ID adds 0 heat; a bust saved by Fake ID adds exactly +40 heat, once
- A card with 3 plays is Tier 1; with 39 plays Tier 4; with 40 plays Tier 5
- A version-1 save file on disk produces a fresh account without error
- Forcing a hand start with < 3 deck cards in a test ends the session neutrally; character survives
- All existing tests updated and passing

**Duration:** 3-4 hours

---

### Phase 2: Demand Data Integrity

**Goal:** Every buyer demand string resolves to a real card, and future authoring mistakes surface at load time instead of silently killing payouts.

**Deliverables:**
- Load-time validation: every demand string (products AND locations, base demand AND scenario demand, in `assets/buyers.ron`) must resolve to a known card name; violations fail loudly in debug builds and log clearly in release
- Corrected demand data. Known-dead strings from audit (verify full list via the new validation):
  - Products: `"Pills"` — used in all three buyers' base demands, matches no card (candidates: Codeine for Desperate Housewife / Wall Street Wolf, Ecstasy for Frat Bro)
  - Locations: `"Park"` → `"At the Park"`; `"Warehouse"` → `"Abandoned Warehouse"`; `"Private Residence"` → `"Safe House"`; `"Dorm"`, `"Party"`, `"Office"` — no sensible existing target, remove (authoring new player-ownable locations is future roadmap work, not this SOW)
- Feature matrix updates for any spec claims corrected by this work

**Architectural Constraints:**
- Demand lists stay human-readable display names in RON (authorability decision — do NOT convert to card IDs)
- Validation lives in the asset loading path so it covers all future content, not just buyers.ron's current contents
- After fixes, every scenario must retain ≥ 1 achievable product and ≥ 1 achievable location (achievable = player-ownable OR present in that buyer's reaction deck)
- Note: `"Private Residence"` → `"Safe House"` leaves that demand behind The Block shop until location unlocking ships — accepted, documented here intentionally

**Success Criteria:**
- An intentionally bad demand string in a test asset triggers the validation failure
- All shipped demand strings resolve to real cards
- Each of the 6 scenarios has at least one demand combination the player can actually satisfy or watch the buyer satisfy

**Duration:** 2-3 hours

---

### Phase 3: UX Corrections

**Goal:** The player always knows where they are in a hand, and upgrade rewards stop arriving as modal spam.

**Deliverables:**
- Round and turn indicator visible during play: current round (1-3) plus current actor state (Narc acting / your turn / Buyer reacting)
- Pending card upgrades presented in a single batched screen instead of one full-screen modal per card; choices deferrable (skipping keeps the upgrade pending)
- Correction of the core-gameplay-loop feature matrix entry that claims a turn indicator already exists

**Architectural Constraints:**
- Indicator must be driven by existing `HandState` phase/turn data (no new state tracking)
- Batched upgrade screen preserves the existing 2-option stat choice per card and the pending-upgrades queue semantics
- No regression of the existing upgrade star/foil feedback on cards

**Success Criteria:**
- At any point mid-hand, the UI states the round number and whose action is in progress
- Returning home with 3 pending upgrades shows one screen, not three sequential modals; skipping preserves all 3 as pending
- Feature matrix accurately reflects shipped turn-indicator behavior

**Duration:** 2-3 hours

---

## Out of Scope (explicitly)

- **The Block unlock gating** (`unlock_location` has zero callers) — location unlocking is its own future SOW
- **Escrow/pot banking, turn-order change, Narc AI, audio/juice** — review roadmap Phase 1+, separate SOWs
- **Authoring new player-ownable demand locations** (Dorm, Office, Private Residence as real cards) — future content work
- **Stale doc sweep beyond touched matrices** (e.g., `narc_deck.ron` threshold comments may be corrected opportunistically if touched)

---

## Acceptance Criteria

**Functional:**
- Insurance heat single-charge verified by test
- Spec upgrade thresholds live; TESTING values gone
- Old saves rejected gracefully into a fresh account
- Deck exhaustion cannot delete a character through any path
- Zero unresolvable demand strings; validation active at load

**UX:**
- Round/turn state always visible during a hand
- Upgrade choices batched and deferrable
- No regressions in existing flows (deck builder, shop, resolution overlay)

**Code Quality:**
- All fixes covered by unit tests (TDD per repo guidance)
- Feature matrices updated where claims changed
- No new warnings; existing test suite passes

---

## Discussion

### Implementation Note: Deck exhaustion modeled as "no outcome change", not a new HandOutcome variant

Considered adding a `HandOutcome::DeckExhausted` variant, but that ripples through every outcome match (resolution, save integration, UI overlay, narrative patterns) for a state the UI already prevents. Chose instead: `start_next_hand` returns `false` and leaves the prior (Safe/Folded) outcome untouched, so the resolution overlay simply persists with NEW DEAL disabled — and the disabled button now reads "OUT OF CARDS" so the player knows why. The permadeath precondition (`outcome == Busted`) can no longer be fabricated by exhaustion through any path.

### Implementation Note: Demand string fix choices

"Pills" mapped per persona flavor: Ecstasy (Frat Bro), Codeine (Desperate Housewife), Coke (Wall Street Wolf). "Park" → "At the Park", "Warehouse" → "Abandoned Warehouse", "Private Residence" → "Safe House". "Dorm" and "Office" removed (no card exists; authoring those as player-ownable cards is future content work). Two demand targets remain gated behind the locked Block shop — Safe House (Housewife) and Ice/Coke (Wolf "Adrenaline Junkie", pre-existing) — accepted here; both resolve when location unlocking ships.

### Implementation Note: Audit found more dead strings than the review listed

The review named "Private Residence", "Dorm", "Office", "Park". The load-time validator also caught "Pills" (in all three buyers' base demands) and "Warehouse" (≠ "Abandoned Warehouse"), and Frat Bro's base demand locations were 100% dead ("Dorm", "Party", "Park"). This is exactly the failure class the validator now prevents — a name-based system with no integrity check.

### Implementation Note: Adversarial review pass (pre-merge)

A 3-lens review (correctness / Bevy systems / spec compliance) with per-finding adversarial verification ran against the branch diff. 16 findings raised, 9 confirmed real (rest refuted as unreachable), all fixed before merge. Notable confirmed items: DECIDE LATER soft-locked the game (`setup_deck_builder` was a third pending-upgrades gate not threaded with the deferral flag); the resolution overlay's old "Deck Exhausted" branch would have mislabeled every genuine late-run bust after Phase 1 removed the exhaustion→Busted mapping; the UI's raw `deck.len()` exhaustion check contradicted the engine's deck+unplayed-hand check (could force-end runs 1-2 hands early — now unified via `playable_cards_remaining()`); and the HeatPenalty upgrade stat had no effect anywhere once the double-charge was fixed (now applied at activation). Also fixed: `apply_decay` re-applied the same decay on every DeckBuilding re-entry (now consumes its window).

### Implementation Note: Wolf base demand product

The SOW's candidate list suggested Codeine for Wall Street Wolf; implementation chose **Coke** (premium fits the persona and matches his "Adrenaline Junkie" scenario products). Note his base and Adrenaline Junkie demands are Block-shop products, so like Safe House they stay luck-gated until location unlocking ships.

### Implementation Note: Batched upgrade screen API

`CharacterState::apply_upgrade_choice` (head-of-queue only) replaced by `apply_upgrade_choice_for(card_name, stat)` so rows on the batched screen resolve in any order. Keyboard shortcuts 1/2 resolve the *first* pending upgrade's two options. DECIDE LATER sets an `UpgradeChoiceDeferred` resource consumed by `check_pending_upgrades_system` and `initialize_deck_builder_from_assets`; the flag clears on entering InRun so deferred upgrades re-prompt after the next run.

---

## Acceptance Review

### Scope Completion: 100%

**Phases Complete:**
- ✅ Phase 1: Core Rules Defects
- ✅ Phase 2: Demand Data Integrity
- ✅ Phase 3: UX Corrections
- ✅ Review-findings fix pass (9 confirmed findings, all resolved)

### Architectural Compliance

All pre-approved decisions honored: spec thresholds (0/3/8/15/25/40) live, save wipe via version bump, The Block untouched, demand lists remain human-readable names with load-time validation, single SOW without RFC. Deck exhaustion can no longer reach the permadeath path through any caller; the UI and engine now share one exhaustion predicate (`playable_cards_remaining`). One scope addition beyond the plan, driven by review: the HeatPenalty upgrade multiplier is now applied at insurance activation (the stat was otherwise inert — a latent RFC-019 gap surfaced by the single-charge fix).

### Player Experience Validation (PLAYER role)

The player can now always answer "whose turn is it and which round am I in"; upgrade rewards arrive as one decision screen instead of modal spam and can be deferred; a disabled NEW DEAL explains itself ("OUT OF CARDS") and no longer lies about when the deck is actually exhausted; a genuine bust is reported as a bust. Demand multipliers can no longer silently reference nonexistent cards.

### Performance

No new per-frame allocations (button label writes now change-guarded); no measurable load-time impact from demand validation (string comparisons over ~30 cards at startup).

### Test Coverage

107 tests passing (was 99 pre-SOW): +8 covering insurance single-charge, spec tier boundaries, version rejection, exhaustion neutrality, demand validation (4 unit + 1 shipped-content integration), out-of-order upgrade application, decay idempotency, and HeatPenalty-at-activation.

---

## Conclusion

All seven verified defects from the July 2026 design review's "Phase 0 — Defuse" are fixed, plus five more surfaced during implementation (Pills/Warehouse/Party dead strings, decay re-application, inert HeatPenalty stat, exhaustion predicate mismatch, stale bust overlay branch). The game's data layer now self-validates at load. Next up per the review roadmap: the tension milestone (escrow pot, commit-before-reveal, Narc policy, visible payout).

---

## Sign-Off

**Reviewed By:** ARCHITECT Role (3-lens adversarial review, per-finding verification)
**Date:** 2026-07-12
**Decision:** ✅ **ACCEPTED**
**Status:** Merged to main
