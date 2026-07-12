# SOW-023: Kingpin & Dealers Foundation

## Status

**In Progress** - 2026-07-12 (Phases 1-2 complete on branch)

## References

- **RFC-023:** [Kingpin & Dealers](../01-rfc/023-kingpin-and-dealers.md)
- **Related:** SOW-021 (save conventions, SAVE_VERSION wipe), SOW-022 (screen + e2e driver), RFC-018 (per-heat narc tiers — now per-dealer)
- **Branch:** (proposed) sow-023-kingpin-dealers
- **Implementation Time:** 2-3 days

---

## Implementation Plan

### Phase 1: Dealer save model

**Goal:** The save file holds a roster of dealers instead of one optional character.

**Deliverables:** (REVISED 2026-07-12 - turn-based jail, kingpin-as-dealer)
- `DealerState` (wraps today's `CharacterState` record) with `name`, `portrait`,
  `is_kingpin`, `prior_convictions`, and
  `status: DealerStatus { Available, Jailed { runs_remaining, sentence_total, heat_at_bust } }`
- `SaveData.dealers: Vec<DealerState>` + `active_dealer: usize`; SAVE_VERSION bump;
  fresh saves start with the kingpin (`DealerState::kingpin()`)
- Name pool + portrait pool (the 9 unused actor portraits) for generated hires
- Pure functions: `jail_sentence_from_heat`, proportional `release`,
  `tick_sentence`, `bail_cost`, `hire_cost`

**Architectural Constraints:**
- Jail sentences are turn-based: ticked only when runs complete (no clocks)
- Roster invariant: at least one dealer always exists (fresh save = kingpin)
- All jail/bail/hire/release math is pure and unit-tested (no ECS in tests)
- Save wipe on version bump is acceptable (pre-release convention)

**Success Criteria:**
- Round-trip save/load of a roster with mixed Available/Jailed dealers
- Sentence scales with heat at bust; full serve releases at heat 0,
  bail releases at proportional heat; both scar `prior_convictions`
- Hire cost strictly increases with roster size

### Phase 2: Engine integration

**Goal:** Runs are executed BY the active dealer; busts jail instead of delete.

**Deliverables:**
- Run start reads the active dealer (heat → narc tier, upgrades, play counts)
- `save_after_resolution_system`: Busted → jail active dealer; KINGPIN bust →
  empire reset (the only permadeath)
- GO HOME transfers signed session heat + stories to the active dealer
  (skipped for busted runs - already priced at resolution) and ticks every
  other jailed dealer's sentence; bail logic (`SaveData::bail_out`) ready for Phase 3
- Decay applies per available dealer (jailed dealers skip decay; jail IS the reset)

**Architectural Constraints:**
- `HandState` remains dealer-agnostic (it already copies counts/upgrades in at
  run start and out at run end) — no dealer references inside the hand engine
- Existing tests for decay/upgrades/transfer keep passing re-pointed at dealers

**Success Criteria:**
- Bust → dealer jailed with countdown, save persists it, account cash untouched
- Two dealers with different heat produce different narc tiers on their runs

### Phase 3: Operations panel (deck-builder screen)

**Goal:** The player can see the roster, select who runs, and hire.

**Deliverables:**
- Roster panel: per dealer — portrait, name, heat tier chip, status
  (Available / Jailed with countdown), select-to-activate
- HIRE button with cost from global cash; disabled when unaffordable
- START RUN disabled when the active dealer is jailed

**Architectural Constraints:**
- Reuse card/chip styling from SOW-022 theme; colors as named constants
- Marker components + `Changed`/change-guarded updates per SOW-011/021 rules

**Success Criteria:**
- Full loop on screen: bust a dealer → see them jailed → hire a replacement →
  run with the new dealer

### Phase 4: Harness + docs

**Deliverables:**
- e2e driver/save isolation: game honors a save-path override (env var) so
  scripted playtests never touch the real save; driver sets it
- Scripted e2e: bust → jail → hire → new run (outcome-aware overlay buttons)
- Feature matrices + roadmap iteration log updated

---

## Acceptance Criteria

**Functional:** roster CRUD via play (hire/select/jail/release); no permadeath
path remains; cash global across dealers.
**UX:** player always knows who is out, who is jailed and for how long, and
what a hire costs.
**Performance:** lazy release checks only on load/screen entry.
**Code Quality:** jail/hire/release logic pure + tested; no new warnings.

---

## Discussion

### ANSWERED by Reed (2026-07-12) - design revision, implemented in Phases 1-2

1. **Jail duration:** turn-based, not wall-clock. `sentence = 1 + max(heat,0)/25`
   runs; every completed run by any OTHER dealer ticks all sentences.
2. **Hire cost curve:** confirmed $500 × 2^(len−1), kingpin counts in len.
3. **Jail release heat:** proportional to time served (full serve → 0); every
   release adds `prior_convictions += 1` (scar for future difficulty).
4. **Bail:** yes, in this SOW — $300 × runs_remaining from global cash; heat
   reduction only for time actually served. Logic in Phase 2, button Phase 3.
5. **The kingpin is a dealer** (`dealers[0]`, `is_kingpin`): starts every
   empire, always sendable, never jailed — but a kingpin bust is GAME OVER
   (full SaveData reset including the account; the only permadeath).

### Implementation Note: run-completion tick point (DEVELOPER, Phases 1-2)

"Session end" has exactly one choke point: `go_home_button_system` (both GO
HOME and END RUN land there), so sentence ticking lives there. The runner is
excluded from the tick so a dealer jailed by this very run doesn't start
serving on it. Busted runs skip the go-home heat transfer — the jail branch
(or empire reset) already priced the session at resolution; transferring
again would double-charge.

### Implementation Note: kingpin bust hygiene

`reset_empire()` also drops the `DeckBuilder` resource so the fresh empire
rebuilds its deck from the fresh account's starting collection instead of
inheriting the dead empire's selection. Phase 3 should add a proper
game-over moment on screen; today the fresh empire just appears at the
deck builder.

### Implementation Note: deferred UI reads (Phases 1-2)

Deck-builder heat/tier/story readouts point at the ACTIVE dealer; the roster
panel (select/hire/bail, jailed sentence display, kingpin badge) is Phase 3.
The Phase-3-consumed API (`hire_dealer`, `bail_out`, `next_hire_cost`,
`bail_cost`) carries `#[allow(dead_code)]` markers that Phase 3 removes.

---

## Acceptance Review

*Populated after implementation.*
