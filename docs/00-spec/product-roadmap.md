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
4. **SOW-026 Content Authoring Pass** — lean start (Weed-only), shop stock as
   the progression ladder with cash+cred requirements, per-area buyer demands
   (the difficulty gradient is authoring, per Reed).
5. **SOW-027 Heat Economy** (P2) — pressure: per-AREA narc deck composition
   (GDD scaling, territory-flavored) × per-dealer heat; relief: Lay Low +
   crooked lawyer (easy areas already serve as authored cooling venues);
   RFC-019 dead Heat-upgrade decision; Wolf ×2.8 balance check.
6. **SOW-028 City Map Screen** — area picker/preview + move UI once areas
   differ enough to preview.
7. **SOW-029 Kingpin Ledger** (P5) — empire/dealer history + full
   fallen-empires arcade board.

## Current-state debts the roadmap must absorb

- Fun-assessment root causes #1/#3 (nothing wagered / no goal): jail time IS
  the wager once dealers are assets; "grow the empire" is the goal scaffold.
- RFC-019 "Heat" upgrade stat is a no-op (flagged in SOW-022) — resolve in SOW-025.
- e2e harness: needs isolated save dir (baseline playtest permadeathed the
  live character) and outcome-aware overlay buttons.

## Iteration Log

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
