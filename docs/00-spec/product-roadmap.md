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

## Sequencing

1. **SOW-023 Kingpin & Dealers Foundation** (P1) — the structural change every
   other pillar hangs off (dealers own heat → P2; dealers jailed → roster
   pressure → P3 spending; per-dealer stories → P5 ledger).
2. **SOW-024 Areas & Unlocks** (P3+P4) — revive The Block via
   `unlock_location`, area-gate buyers, area purchase from global cash.
3. **SOW-025 Heat Pressure Tuning** (P2) — narc deck scaling beyond stat
   multipliers (deck composition by tier), conviction pressure, and the
   dead RFC-019 "Heat" upgrade decision.
4. **SOW-026 Kingpin Ledger** (P5) — surfacing per-dealer and empire history.

## Current-state debts the roadmap must absorb

- Fun-assessment root causes #1/#3 (nothing wagered / no goal): jail time IS
  the wager once dealers are assets; "grow the empire" is the goal scaffold.
- RFC-019 "Heat" upgrade stat is a no-op (flagged in SOW-022) — resolve in SOW-025.
- e2e harness: needs isolated save dir (baseline playtest permadeathed the
  live character) and outcome-aware overlay buttons.

## Iteration Log

### Iteration 1 — 2026-07-12
- SOW-022 (Game Play v2) accepted + merged after playtest-directed iterations.
- Baseline scripted playtest: blind 3-slot play busts on hand 1
  (Evidence 64 > Cover 45) vs a career-heat-scaled narc; BUSTED offers only
  END RUN (permadeath). Confirms: difficulty-by-heat works, but permadeath
  makes it a wall, not a wager → P1 is correctly first.
- RFC-023 + SOW-023 authored (kingpin & dealers foundation).
- Open questions logged in SOW-023 Discussion.
