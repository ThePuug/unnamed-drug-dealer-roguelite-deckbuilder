# Product Roadmap — Path to Playable

**Owner:** Claude (lead), directed by Reed
**Last Updated:** 2026-07-12
**Cadence:** playtest (e2e driver) → design update → SOW → implement + tests → repeat.
Each iteration appends to the Iteration Log below.

## Vision (from Reed, 2026-07-12)

You are the **kingpin**, not the dealer. You hire dealers who run sessions on
your behalf; they carry their own heat and decks, and can be **jailed** for a
period when busted. **Cash is global.** The product needs:

| Pillar | One-liner | Today's foundation |
|---|---|---|
| P1 Kingpin & dealers | Roster of hired dealers, each with own heat/deck/story; jail replaces permadeath; global cash | Single `CharacterState` + permadeath; `AccountState.cash_on_hand` already global |
| P2 Heat difficulty | Rising heat makes the world harder | RFC-018 narc tiers from career heat (works — baseline playtest busted hand 1 vs a high-tier narc) |
| P3 Progression & unlocks | Cards and capabilities unlock over time | RFC-017/019 upgrades, SOW-020 shops; **The Block is authored but permanently locked** (`unlock_location` has zero callers) |
| P4 Unlockable areas | New areas gate shops/customers/cards | `shop_locations.ron` (the_corner, the_block); buyers not yet area-gated |
| P5 History | Stories accumulate into a legacy | `story_history` per character + narrative engine; only visible in a deck-builder overlay |

## Sequencing (revised 2026-07-12 after the map/stationing design iteration)

1. ✅ **SOW-023 Kingpin & Dealers Foundation** (P1) — merged.
2. ✅ **SOW-024 Areas & Unlocks** (P3+P4) — merged; territories with interim
   random-area run selection.
3. ✅ **SOW-025 Street Cred & Stationing** — merged; stationed runs, cred
   accrual, cash+downtime moves, cred-gated shops with credit line.
4. ✅ **SOW-026 Content Authoring Pass** — merged; Weed-only lean start, full
   cash+cred shop ladder, measured pacing (Shrooms session 2-3).
5. ✅ **SOW-027 Heat Economy** — merged; per-area × per-tier narc
   compositions (multipliers retired), Lay Low + Crooked Lawyer, Heat
   upgrade wired, fresh floor ≤ Hot, ZERO-warnings baseline.
6. ✅ **SOW-028 Third Zone: The Strip + zone coherence** — merged; Pimp
   persona, Ecstasy/Ice re-zoned to the Strip, Housewife → Block first
   rung (Wolf ×2.8 gap closed), Corner fresh floor re-tuned to Cold.
7. ✅ **SOW-029 City Map Screen** — merged; map overlay with three live
   node cards, unlock + relocation from the map, two-click rotation.
8. ✅ **SOW-030 Kingpin Ledger** (P5) — merged; empire strip, dossiers
   with story feeds, browsable board with IN PROGRESS row, map zone
   history line, zero schema changes.
9. **SOW-031 Suppliers & Fronts** (Reed-confirmed v2) — named supplier NPC
   per zone; fronts: take product cards now, owe from proceeds, due in N
   runs on the run-ticker; escalation ladder on default (cut off → muscle →
   soured). Rationale: due-dates make unproductive runs cost something —
   run-quality pressure countering fold-early safety. Defaults: fronts
   against cards (own after payoff), 1 supplier/zone. Design: studio repo
   `2026-07-12-supplier-mechanic.md`.
10. **SOW-032 Starter/Tutorial Arc — "Road to Your First Dealer"** —
   OPTIONAL at empire start; skipping confers NO benefit (the arc earns
   exactly what ordinary play would); beats: first front → first payback →
   graduation at the first $500 hire.

**Closed threads (Reed, 2026-07-12):** dev save wipes are a non-concern for
the leaderboard; Lay Low stays committed (no cancel); heat stays global per
dealer. Original debt list fully absorbed: jail-as-wager shipped (023),
RFC-019 resolved (027), harness isolation/outcome-awareness shipped (023/024).

## Iteration Log

### Iteration 9 — 2026-07-12

- SOW-030 merged (225 tests, zero warnings, save byte-identical —
  derive-not-record held): LEDGER overlay with the empire strip (the six
  numbers the epitaph freezes), roster dossiers + story feeds, fallen-
  empires board browsable in play with the living empire slotted IN
  PROGRESS at its would-be rank (ties go to the dead), map zone history
  line, E1 portrait-pool fix.
- Adversarial review: 4 LOW findings, fixed same-day — panel caps with
  the IN PROGRESS row pinned (was clipping off a full board), epitaph
  feeds now honest archive-order (flat archives carry no global
  chronology — a real timeline needs the future event-log SOW), cap/tail
  logic moved into the tested view layer, harness -Hire learned 2-dealer
  scenarios. Trend: SOW-029 review caught 2 HIGH, this one only LOWs —
  the recorded lessons held by instruction.
- Open for Reed: arcade score formula (lifetime revenue for now);
  epitaph naming at game over (schema addition — awaiting nod).
- Next: SOW-031 Suppliers & Fronts (design confirmed: fronts against
  cards, 1 supplier/zone, due-dates on the run ticker, cut off → muscle
  → soured; zone strings move to RON per the SOW-029 carry).

### Iteration 8 — 2026-07-12

- SOW-029 merged (196 tests, zero warnings, +21 pure view-model tests):
  CITY MAP overlay with three node cards (status/price with live
  affordability, clientele + payout band, fiction-voice narc hints,
  stationed-dealer chips with heat/cred, best-cred ★), zone unlock and
  dealer relocation from the map through the existing SOW-024/025 code
  paths, two-click burn-then-cool rotation. e2e: full
  unlock→relocate→run-at-new-station loop verified live.
- Pre-merge adversarial review (16 agents) caught 2 HIGH defects before
  they shipped: a launch panic on pending-upgrade saves (bare ResMut on
  a conditionally-inserted resource) and overlay click fall-through
  (Node defaults FocusPolicy::Pass — the opaque canvas was visual only).
  Both fixed + re-verified live. Lesson recorded: overlays over a live
  screen need FocusPolicy::Block; state-wide resources need
  init_resource, not OnEnter inserts that can early-return.
- Process: all three repos pushed to GitHub (Reed caught local-only
  history); push is now part of every closeout. "Fork" jargon dropped.
- Art asks ledger created (studio repo art-backlog.md, full 4-way
  sweep): 13 items — notable finds beyond the known asks: the Kingpin
  wears barista.png, pimp.png double-booked buyer/dealer-pool (fix
  queued SOW-030), watermark retouch pass, per-card art as the largest
  untouched surface.
- Carried: StoryHistoryOverlay focus gap (pre-existing) → stabilization;
  zone strings → RON in SOW-031; Strip session-3 heat awaits Reed's
  playtest.
- Next: SOW-030 Kingpin Ledger (design: studio repo
  2026-07-12-kingpin-ledger.md — derive-not-record, summary-first,
  browsable fallen-empires board, map node history line).

### Iteration 7 — 2026-07-12

- SOW-028 merged (175 tests, zero warnings): The Strip (nightlife,
  $1,200) with the Pimp (×2.0, Night Shift / VIP Treatment), Back of the
  Club + VIP Room, Velvet Rope + crowd modifiers, vice-sweep narc texture
  (heat-noisy, conviction-light; tips Hot+ only, one per tier). Ecstasy/
  Ice re-zoned Block→Strip; Housewife → Block ×1.5 first rung (Wolf ×2.8
  flag closed — pinned by test and drawn live).
- Measured, one tuning iteration: Corner fresh floor was regressing to
  Scorching-135 (root cause ~90% buyer reaction cards) → tuned to Cold-0;
  Strip stays hottest per session BY DESIGN (heat tax, zero busts in six
  blind sessions — feeds the burn-then-cool rotation); Block busts at
  Blazing with 5-run sentences (contrast intact).
- Systems insight recorded: self-referential heat sources (tips→heat→
  tiers→tips) compound — ration them per tier when authoring narcs.
- For Reed: art asks (club back-alley, VIP lounge backgrounds); judgment
  call on Strip session-3 heat (+85 at Blazing) after a human playtest.
- Process: fork committed to local main by mistake; branch rebuilt, main
  reset with Reed's approval, merge --no-ff as usual. Nothing lost.
- Next: SOW-029 City Map Screen (three real nodes to show).

### Iteration 6 — 2026-07-12

- SOW-027 merged (172 tests, ZERO warnings — all 41 pre-existing removed,
  −355 lines dead code): compositions with sparse inheritance, multipliers/⚖
  retired, Lay Low ($200/2 runs/−40) + Lawyer ($625/−25), Heat upgrade
  wired positive-only.
- Measured: fresh blind floor Warm-35/Hot-85, no busts (was Inferno-184 with
  fresh GAME OVERs); target-play unregressed; Wolf ×2.8 confirmed gated
  behind Coke — first-rung fix designed into SOW-028 zone coherence.
- Reed: creative freedom on zone three (+ adjust zones 1-2), tutorial arc
  onto the roadmap (optional, no skip benefit), three threads closed.
  Three-zone city designed (Corner / Strip / Block).
- Next: SOW-028 The Strip.

### Iteration 5 — 2026-07-12

- SOW-026 merged (174 tests): 8-card Weed-only start, full cash+cred ladder
  (Shrooms $100·1 → Fentanyl $12k·6), pacing measured (target-play reaches
  Shrooms session 2-3; Block 10-20 sessions), one tune (Shrooms $150→$100).
- Exposed the fresh-floor problem (Inferno-184 in 3 blind sessions) —
  carried as SOW-027's measured acceptance bar.

### Iteration 4 — 2026-07-12

- SOW-025 merged (170 tests): dealers stationed per area, +1 cred per Safe
  deal (never decays), moves at $250 + 1-run downtime via the sentence
  ticker, cred-gated shop items with "unlocked by <dealer>" credit line and
  "NEEDS CRED N (best: M)" locked states — all verified live on the hustler
  scenario, including a user-clicked double relocation during acceptance.
- Pilot gates: Storage Unit (3 Corner cred), Heroin (5 Block cred); Shrooms
  couldn't take the pilot (it's starting collection) — becomes real in the
  SOW-026 re-laddering.
- Tuning flags carried: move fee vs hire vs bail feel, cred thresholds,
  sentence constant, Wolf ×2.8.
- Next: SOW-026 Content Authoring Pass.

### Iteration 3 — 2026-07-12

- SOW-024 merged: The Block purchasable ($2,000), its ~$49k of authored stock
  revived, two-stage territory run selection (Wolf e2e-confirmed as Block
  clientele), shop_locations.ron promoted to validated source of truth,
  harness tab/overlay coordinates fixed.
- Design iterated with Reed mid-flight: areas are TERRITORIES; the difficulty
  gradient is authoring-first (lean start, shop ladder); dealers get
  stationing + per-area street cred (decisions locked in the studio repo).
  Roadmap resequenced above.
- Next: SOW-025 Street Cred & Stationing.

### Iteration 2 — 2026-07-12

- SOW-023 all phases complete on `sow-023-kingpin-dealers` (Review):
  dealer roster + turn-based jail + bail + kingpin-as-dealer (Phases 1-2),
  operations panel + arcade game-over board (Phase 3), forge/isolated-save/
  closed-loop playtest harness (Phase 4).
- Reed's design answers folded in mid-flight: turn-based sentences scaling
  with heat, proportional time-served heat reduction, prior-conviction scars,
  bail tradeoff, kingpin game-over feeding a fallen-empires leaderboard
  (stats shown, stories archived for SOW-026).
- e2e-verified live: dealer select, jail tick on run completion, bail cost
  decay, per-dealer heat transfer, global cash, HIRE cost doubling.
- SOW-023 accepted (user playtest + hot-scenario e2e of the GAME OVER board) and merged to main.

### Iteration 1 — 2026-07-12
- SOW-022 (Game Play v2) accepted + merged after playtest-directed iterations.
- Baseline scripted playtest: blind 3-slot play busts on hand 1
  (Evidence 64 > Cover 45) vs a career-heat-scaled narc; BUSTED offers only
  END RUN (permadeath). Confirms: difficulty-by-heat works, but permadeath
  makes it a wall, not a wager → P1 is correctly first.
- RFC-023 + SOW-023 authored (kingpin & dealers foundation).
- Open questions logged in SOW-023 Discussion.

## Backlog (post-core)

- **Starter/Tutorial arc — "Road to Your First Dealer"** (Reed, 2026-07-12):
  onboarding whose graduation is affording the first $500 hire; teaches
  deal → heat → bank → cred along the way. After the map screen + ledger.
