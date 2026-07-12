# SOW-030: Kingpin Ledger

## Status

**In Progress** - 2026-07-12

## References

- **Design:** studio repo `design-updates/2026-07-12-kingpin-ledger.md`
  (three panels, derive-not-record rule, open questions)
- **Umbrella:** RFC-023 (EmpireEpitaph, fallen_empires, leaderboard_top
  already exist); P5 History pillar
- **Branch:** sow-030-kingpin-ledger
- **Implementation Time:** 1-2 days

---

## Implementation Plan

**Design rule (hard constraint): DERIVE, DON'T RECORD.** Every displayed
number comes from existing save state - zero schema changes, zero
migrations. If a stat can't be derived, it's out of scope (noted for a
future event-log SOW), not an excuse to add fields.

### Phase 1: Ledger view-model (pure, TDD)

**Deliverables:**
- `src/ui/ledger_view.rs` (mirrors map_view.rs): empire summary (lifetime
  revenue, cash, decks played, dealers hired, zones unlocked, roster
  convictions - the same numbers EmpireEpitaph::from_save freezes),
  per-dealer dossier rows (deals closed = Σ street_cred, cred by zone,
  decks played, priors, heat tier, story count; kingpin first + marked),
  fallen-empires board rows via leaderboard_top with the LIVING empire
  as an unranked IN PROGRESS row at its would-be rank
- Unit tests for every function (empty roster impossible - kingpin
  invariant; zero-cred dealers; tie ranks; living empire above/below/
  between epitaphs)

### Phase 2: Ledger overlay (UI)

**Deliverables:**
- LEDGER button in the hub tab row; full-screen overlay following the
  SOW-029 pattern EXACTLY - child of DeckBuilderRoot, opaque canvas,
  `FocusPolicy::Block` on the root (review lesson), `init_resource`'d
  ui state (review lesson), CLOSE exits
- Three panels: THE EMPIRE strip, THE ROSTER dossiers (click dealer →
  story feed, newest first, reusing the story history data), FALLEN
  EMPIRES board (click epitaph → its archived stories)
- Live refresh on save changes (existing Changed<SaveData> patterns)

### Phase 3: Map node history line + roster hygiene

**Deliverables:**
- City map node cards gain one derived line: total deals closed in the
  zone (Σ roster street_cred there) + best dealer name (SOW-029
  Acceptance confirmed placement)
- E1 from art-backlog.md: remove "Pimp" from DEALER_PORTRAIT_POOL, fix
  the stale "not used by any buyer persona" comment (the face belongs to
  the Strip's buyer since SOW-028)

### Phase 4: Verification

**Deliverables:**
- e2e on forged scenarios: hustler (rich roster history) - open ledger,
  panels populated, dealer story feed opens; mogul + a game-over path -
  fallen board shows epitaph + IN PROGRESS row ranks correctly;
  screenshots to out\sweep30\
- Harness upkeep if hub tab row shifts (playtest.ps1 header docs)
- Feature matrices, SOW Discussion, status → Review; roadmap Iteration 9
  entry is the coordinator's

---

## Acceptance Criteria

**Functional:** ledger shows live empire summary, per-dealer dossiers
with browsable stories, and the fallen-empires board browsable OUTSIDE
game over with the living empire ranked among the dead; map nodes carry
the per-zone history line.
**Integrity:** zero save-schema changes (serialized bytes identical
before/after for the same state); overlay blocks clicks (no fall-through
regressions).
**Code Quality:** zero warnings; view-model fully unit-tested; no
duplicated derivation between ledger_view and map_view (shared helpers
where the same number appears twice).

---

## Discussion

*Populated during implementation.*

---

## Acceptance Review

*Populated after implementation.*
