# SOW-030: Kingpin Ledger

## Status

**Accepted** - 2026-07-12 (all 4 phases on `sow-030-kingpin-ledger`; 225 tests, zero warnings; save-integrity byte-identical; adversarial review findings fixed)

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

### Shipped UI structure

- **LEDGER tab** joins the hub row after CITY MAP (design (575, 40), gold
  `LEDGER_TAB_BG`); existing tab positions unchanged, so no prior harness
  coordinates moved.
- **LedgerOverlay** mirrors the map overlay exactly: absolute full-size
  child of `DeckBuilderRoot`, opaque `LEDGER_CANVAS_BG`,
  `FocusPolicy::Block` on the root, `GlobalZIndex(92)` (above the map's
  90 for determinism), CLOSE at the map's geometry so (1843, 79) works on
  both overlays.
- **Three panels** under a full-width EMPIRE strip:
  - THE EMPIRE: lifetime revenue (gold - the arcade score), cash, decks,
    hires, zones, convictions (red) - the exact numbers
    `EmpireEpitaph::from_save` freezes, pinned by test.
  - THE ROSTER (560px): dossier buttons - name (+" · BOSS" gold for the
    kingpin), heat tier colored, "STATION · n DEALS · n DECKS · n PRIORS
    · n STORIES" line, gold CRED line per zone in canonical order,
    status note (JAILED/MOVING/LAYING LOW) shared with map chips via the
    now-pub `map_view::chip_status_note`.
  - FALLEN EMPIRES (560px): every epitaph ranked by lifetime revenue;
    the living empire is a NON-button row with a gold border slotted at
    its would-be rank ("— $2,000 · IN PROGRESS"); ties go to the dead
    (the record stands until strictly beaten - pinned by test).
  - STORIES (flex): idle hint → feed on dossier/epitaph click, newest
    first, capped at 15 with an "… n earlier stories" tail.
- **View-model** is pure (`src/ui/ledger_view.rs`, 20 tests):
  empire/dossier/board/story derivations + the shared zone-history
  helpers. `systems/kingpin_ledger.rs` only orchestrates.

### Derive-don't-record held

Zero save-schema changes. `street_cred` doubles as the deals-closed
counter (+1 per Safe deal by construction), `EmpireEpitaph` already
archived stories, `leaderboard_top` already ranked. **Save integrity
verified live: SHA256 of save.dat byte-identical before/after a full
ledger session** (open, dossier feed, epitaph feed, dead-zone click,
close). "Biggest single deal" and named epitaphs stay out of scope until
an event-log SOW.

### Map node history line

"3 deals closed · best: The Kingpin (3)" in gold under each unlocked
zone's chips - placed BELOW the chips deliberately so the harness's
chip-y reference coordinates stay valid. Derivation shared with the
ledger (`ledger_view::zone_history_line`, best-dealer consistency with
the shop credit line pinned by test).

### E1 closed

`DEALER_PORTRAIT_POOL` dropped "Pimp" (9 → 8 faces) with the comment
rewritten to state the rule (a hire must never wear a buyer's face);
recruit()'s skip-used/modulo-wrap logic needed no change.

### Deviations (rationale)

1. **Ledger systems in their own chained group** - the hub Update chain
   sits exactly at Bevy's 20-system tuple limit; the ledger is
   self-contained (reads SaveData + its own resource, spawns only under
   LedgerBody), so cross-group ordering is immaterial.
2. **Story feed caps at 15** (view returns all; UI truncates with a
   tail count) - keeps the panel inside 1080 design height without
   pulling scroll machinery into the overlay. Revisit if feeds grow.
3. **New forge scenario `legacy`** (not in the SOW's list): two fallen
   empires bracketing the living one + stories on both dealers - makes
   the board's rank-slotting e2e deterministic instead of grinding a
   live game-over. Shape pinned by test.
4. **IN PROGRESS row is not clickable** - the living empire's stories
   live in the roster dossiers; duplicating them behind the board row
   would be two surfaces for one record.

### e2e evidence (out\sweep30\, legacy scenario, isolated save dir)

- `01-ledger-open.png`: three panels populated - empire strip correct
  ($2,000 gold / $800 / 9 / 1 / 2 / 1 red), both dossiers with cred
  lines, board showing 1. $5,000 → gold "— $2,000 · IN PROGRESS" →
  2. $900 (exact designed bracket).
- `02-dossier-stories.png`: Kingpin dossier focused (green border),
  "STORIES — THE KINGPIN" newest-first (3 stories).
- `03-epitaph-stories.png`: $5,000 epitaph focused, "STORIES — FALLEN
  EMPIRE ($5,000)" archive newest-first; focus swapped cleanly off the
  dossier.
- Dead-zone click at the old START RUN spot (1798, 987) with the ledger
  open: inert (no run started, log silent) - FocusPolicy::Block verified
  on the new overlay.
- CLOSE → hub live (CITY MAP tab worked immediately after);
  `04-map-history-line.png`: Corner "3 deals closed · best: The Kingpin
  (3)", Strip "2 deals closed · best: Ray (2)", locked Block shows no
  history line; chips unshifted.
- Save integrity: SHA256 identical before/after the whole session.

### Adversarial review (pre-merge, coordinator) — 4 findings, fixed 3d23634

A 19-agent review panel (4 dimensions, 3 skeptics per finding) sustained
four LOW-severity findings; a fifth (LEDGER tab shifting the shop row)
was refuted 0/3. All four fixed:

1. **Roster/board panels rendered unbounded rows** into a fixed-height
   layout with no scroll and no tail — after ~13 game-overs the living
   empire's gold IN PROGRESS row clipped off-screen (the review noted
   the story panel had a cap for exactly this reason, but the other two
   panels didn't). Fixed: caps as pure `ledger_view` functions (roster
   8, board 10) with the IN PROGRESS row PINNED below the fold and
   truthful tails.
2. **epitaph_stories claimed newest-first** but the archive flat-maps
   dealers in roster order — a hire's old stories rendered above the
   kingpin's newest. No global chronology is derivable, so the feed is
   now honest ARCHIVE order (per dealer, oldest first) — reads like a
   case file.
3. **"… 1 earlier stories"** plural bug in cap/tail logic living
   untested in the ECS layer, violating the module's own stated rule.
   Fixed by moving it into ledger_view (plural-correct, 8 new tests).
4. **playtest.ps1 -Hire** dealer-count switch didn't know the 2-dealer
   scenarios — `-Scenario legacy -Hire` would click Ray's card instead
   of HIRE (silently changing the active runner). Fixed for legacy AND
   the pre-existing hustler gap.

### For Reed

- **Score formula**: lifetime revenue is the arcade score (already
  ranked by `leaderboard_top`). Alternatives (runs survived, zones ×
  revenue) are a tuning conversation when the board has real entries.
- **"Name your fallen empire"** at game over would complete the arcade
  fantasy but needs a schema addition (epitaph name field) - waiting on
  your nod; if wanted, a tombstone/board-frame illustration becomes an
  art ask for art-backlog.md.
- No NEW art asks from this SOW (the ledger is text + palette by
  design; portraits on dossiers could ride a later polish pass using
  the existing actor art).

---

## Acceptance Review

**Reviewer:** ARCHITECT role (coordinator) — 2026-07-12
**Verdict: ACCEPT**

### Verification performed

- **Build/test:** 225 passed / 0 failed / 0 warnings on `3d23634`
  (+21 from the implementation: 20 ledger_view + 1 forge shape; +8 from
  the review fixes: panel caps, board pinning, archive order, plurals).
- **Branch hygiene:** all commits on `sow-030-kingpin-ledger`, main and
  assets untouched, Reed's local asset edits intact.
- **Derive-don't-record held:** zero save-schema changes; SHA256 of
  save.dat byte-identical across a full live ledger session (implementer-
  verified, method recorded in Discussion).
- **Adversarial review:** 19-agent panel; 4 LOW findings sustained, all
  fixed same-day (see Discussion); 1 finding refuted 0/3. Trend note:
  SOW-029's review caught 2 HIGH; this one only LOWs — the SOW-029
  lessons (init_resource, FocusPolicy::Block) were applied by
  instruction and verified in-diff, which is the process working.
- **Acceptance criteria:** met. Ledger live with dossiers + browsable
  board + IN PROGRESS row; map nodes carry the zone history line; E1
  portrait-pool fix in; no duplicated derivation (shared zone-history
  helpers pinned by test).

### Assessment

- The epitaph-ordering finding is the deepest lesson: **a flat archive
  cannot be re-sorted into chronology after the fact** — the fix is
  honest presentation now, and IF Reed ever wants a true cross-empire
  timeline, that's the event-log SOW (with timestamps), not a sort.
- Panel capping as pure functions continues the view-model discipline;
  the "presentation derivation must live in the _view module" rule is
  now explicit in both ledger_view and the review record.
- Deviations all sound (own system group at the 20-tuple limit; legacy
  forge scenario; non-clickable IN PROGRESS row).

### Carried forward

- **Reed judgment (open):** arcade score formula (lifetime revenue for
  now); "name your fallen empire" epitaph naming — schema addition
  awaiting his nod (+ tombstone frame art ask if wanted).
- Story feeds have no scroll — caps + tails are the mechanism until a
  scroll pass is justified.
- StoryHistoryOverlay focus gap (pre-SOW-029) still queued for
  stabilization.
