# RFC-024: Areas & Unlocks

## Status

**Proposed** — 2026-07-12

## Problem

Progression pillar P3/P4: the game has exactly one effective area. The Block
exists — authored shop stock worth ~$49k of content — but
`AccountState::unlock_location` has zero callers (SOW-021 finding), so it is
permanently locked. Buyers are not area-gated, so there is nothing new to
meet. The empire needs somewhere to expand INTO, and global cash needs
something to buy besides dealers and bail.

## Design

### Areas gate three things

An **area** (today: `the_corner`, `the_block`; more later) gates:
1. **A shop** — already true (`shop_locations.ron`, SOW-020).
2. **Buyers** — new: each persona belongs to an area (`area` field in
   `buyers.ron`, defaulting to `the_corner`). Run-start persona selection
   draws only from unlocked areas. Initial split: Frat Bro + Desperate
   Housewife on the Corner; **Wall Street Wolf moves to the Block** — his
   ×2.8 payout becomes the pull that makes expansion worth the price.
3. **Cards** — already true via shop stock (block-priced cards).

### Buying an area

- Locked areas appear in the shop's location selector showing their price
  (e.g. "THE BLOCK — $2,000"); clicking with sufficient global cash calls the
  existing `unlock_location` + `spend`, persists, and opens the shop tab for
  it. Unaffordable = disabled with price shown (SOW-011 button discipline).
- Prices: content-defined in `shop_locations.ron` (authorability rule —
  human-readable data, validated at load). The Block defaults to $2,000
  unless SOW-020 docs recorded a different intent.

### Interplay with existing systems

- Cash sinks now compete: hires ($500 doubling), bail, areas — the empire
  wallet has real decisions.
- Area-gated buyers give SOW-025 a home for cool-down-contract personas
  (cooling through play) without new systems.
- Fallen-empire resets wipe unlocks with the account (a new empire re-expands)
  — consistent with the arcade framing.

## Consequences

- `buyers.ron` gains an `area` field (serde default keeps old files loading);
  load-time validation warns on unknown areas (SOW-021 pattern).
- Persona selection in `start_run_button_system` filters by unlocked areas.
- Shop UI: locked-location buttons render with price + purchase flow.
- Feature matrices: progression-meta and card-system matrices gain area rows.
