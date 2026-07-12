# SOW-023: Kingpin & Dealers Foundation

## Status

**Merged** - 2026-07-12

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

### ANSWERED by Reed (2026-07-12) - arcade leaderboard addendum

Kingpin game-over feeds an arcade board of fallen empires: summary stats
displayed (top-3 by lifetime revenue on the GAME OVER overlay, "← THIS RUN"
marker), the full story ledger ARCHIVED in each `EmpireEpitaph` but not
displayed - presentation deferred to SOW-026. `fallen_empires` survives
`reset_empire` (append-then-reset-then-restore).

### Implementation Note: forge is a main-binary subcommand (Phase 4)

The plan said `src/bin/save-forge.rs`, but this is a binary crate with no lib
target - a second bin can't reach the save/crypto modules without a large
lib refactor. `cargo run -- forge <scenario> [--dir]` (early-exit before the
Bevy App) gives the same capability with zero restructuring. Scenarios:
fresh / funded / roster / hot, all validated + roundtrip-tested. Signing uses
the real HMAC key per Reed's explicit approval.

### Implementation Note: e2e verification (Phase 4)

`tools/e2e/playtest.ps1` is closed-loop: it forges a scenario into an
isolated `DDD_SAVE_DIR`, tails the game log for "Resolution outcome:" lines,
and clicks the right overlay button per outcome (the old script busted on
hand 1 without noticing). Verified live:
- roster scenario: selected Ray, played 2 hands (Safe, InvalidDeal), GO HOME
  → Ray's heat 45→52 on his card, Slim's sentence ticked 2→1 RUNS with bail
  $600→$300, cash banked globally ($1,230)
- funded scenario: HIRE clicked → Slim joined at $500, cash $5000→$4500,
  next hire doubled to $1000
Roster cards are FIXED 250px wide so the harness can target rows
deterministically. PS 5.1 gotcha baked into the script: native stderr must
not enter the pipeline (forge runs via cmd redirect).

### Implementation Note: stats-block dedup (Reed UI feedback, 2026-07-12)

The lower-left "Heat: N [Tier]" line duplicated the roster panel and was
removed (markers, spawn, and its update system pruned; deck count / Cash /
Lifetime stay). The decay callout is now the ONLY decay surface and names
the dealer it applies to: "While you were away: Ray cooled off by N" -
kept in the stats block, since it's account-level news you read on arrival.
Bonus pruning: the never-constructed CharacterHeatDisplay marker and the
unused SaveManager::reset_empire wrapper (its test now mirrors the real
kingpin-bust path and additionally asserts the board survives).

### Implementation Note: nested bail button

The BAIL button nests inside the dealer-row Button; bevy picking gives the
inner button the interaction, so paying bail doesn't re-select the runner.

---

## Acceptance Review

### Scope Completion: 100%

- ✅ Phase 1: Dealer save model (roster, turn-based jail, scars, bail/hire math)
- ✅ Phase 2: Engine integration (per-dealer runs, jail-not-delete, sentence
  ticking, empire reset with surviving fallen-empires board)
- ✅ Phase 3: Operations roster panel + arcade GAME OVER moment
- ✅ Phase 4: Harness (save forge via real HMAC key, DDD_SAVE_DIR isolation,
  closed-loop playtest script)

### Architectural Compliance

152 tests passing (17 new across jail/bail/hire/epitaph/forge); zero new
warnings (two pre-existing removed). Jail/bail/hire/epitaph logic pure and
unit-tested; HandState remains dealer-agnostic; theme/marker conventions held.

### Player Experience Validation

Reed reviewed the roster panels live ("the new dealer panels are good") and
directed the duplicate heat-line removal (applied). e2e-verified on forged
scenarios: dealer select, jail sentence ticking (2→1 runs with bail
$600→$300), per-dealer heat transfer, global cash banking, HIRE cost
doubling, and the kingpin GAME OVER with the FALLEN EMPIRES board ("← THIS
RUN" marker) rendering live in the `hot` scenario.

### Known papercut (harness, not game)

The fallen-empires board lengthens the game-over panel, moving NEW EMPIRE
below playtest.ps1's scripted click position — script needs an outcome-aware
button Y (queued for the next harness touch).

---

## Sign-Off

**Reviewed By:** User playtest + ARCHITECT review
**Date:** 2026-07-12
**Decision:** ✅ **ACCEPTED**
**Status:** Merged to main
