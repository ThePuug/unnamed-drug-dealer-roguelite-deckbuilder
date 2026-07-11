# SOW-021: Stabilization — Defect Fixes and Data Integrity

## Status

**Planned** - 2026-07-11

## References

- **Review:** Design & Fun Review 2026-07 (`unnamed-indie-studio-internal` repo: `projects/drug-dealer-roguelite/review-2026-07.md`) — this SOW implements the review's "Phase 0 — Defuse"
- **Spec:** [Card System](../00-spec/card-system.md), [Bust & Insurance Mechanics](../00-spec/bust-insurance-mechanics.md), [Core Gameplay Loop](../00-spec/core-gameplay-loop.md)
- **RFC:** None — all items are defect corrections against already-approved specs, per process discussion (2026-07-11)
- **Branch:** (proposed)
- **Implementation Time:** 7-10 hours

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

*This section is populated during implementation with questions, decisions, and deviations.*

---

## Acceptance Review

*This section is populated after implementation is complete.*
