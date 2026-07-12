# RFC-027: Heat Economy — Pressure & Relief

## Status

**Proposed** — 2026-07-12 (design basis: studio repo
`design-updates/2026-07-12-heat-economy.md` and
`2026-07-12-world-map-and-areas.md`; measured targets from SOW-026)

## Problem

Two measured failures share one root:
1. **Pressure is shapeless.** Narc difficulty is a stat multiplier
   (RFC-018) applied to one fixed deck — every area feels the same, card
   faces get scaled numbers, and the GDD's intended composition scaling
   ("hotter = more wiretaps and warrants, not bigger numbers") never shipped.
2. **Relief has no floor.** SOW-026's pacing run put a fresh kingpin at
   **184 heat (Inferno) in 3 bad sessions** with wall-clock decay (~184h) as
   the only exit. Sustained play needs the harder in-game cooling paths Reed
   asked for; the fresh-player floor needs covering.

## Design

### Pressure: per-area narc deck COMPOSITION (retiring stat multipliers)

- `narc_deck.ron` becomes per-area, per-heat-tier **deck compositions** (GDD
  model): the Corner at Cold is mostly Donut Breaks and light Patrols — dead
  hands cost little; the Corner at Inferno brings warrants; the Block's
  baseline starts where the Corner's mid-tiers end. Authored content,
  validated at load (authorability rule).
- The narc deck for a run = composition(station area, dealer's heat tier).
- **RFC-018's stat multipliers retire** — composition is the difficulty.
  Card faces stop showing scaled numbers (kills the rounding/truncation
  display-vs-engine legacy and the ⚖ badge). PENDING REED: flagged for
  approval; default proceeds as composition-primary.

### Relief: two mechanical coolers (geographic cooling comes free)

- **Lay Low** (roster action): a fixed package per use — bench the dealer
  for 2 runs, pay upkeep, cool a capped amount (default $200 / −40 heat /
  2 runs; committed once chosen). Reuses relocation's ticking; never cools
  below 0. Priced so decay stays the efficient path (guardrail from the
  heat-economy design note).
- **Crooked Lawyer** (cash action, immediate): chunked cooling, default
  −25 heat for $625 ($25/heat), usable anytime. The expensive "right now"
  button.
- **Geographic cooling** emerges from composition: station a hot dealer on
  the Corner and its gentle low-tier decks + decay do the rest (Reed's
  intended "smart" play — no new mechanic).
- **RFC-019 "Heat" upgrade finally wired**: the upgrade multiplier applies
  in `get_card_heat` to POSITIVE-heat player cards only (never worsens
  negative-heat cards — the objection that killed the naive wiring).

### Measured acceptance targets

- Fresh-floor: 3 blind Corner sessions from a fresh empire end ≤ Hot
  (was: Inferno 184).
- Target-play pacing from SOW-026 (Shrooms session 2-3) must not regress.
- Wolf ×2.8 vs the Block's composition gets a first real balance read.
