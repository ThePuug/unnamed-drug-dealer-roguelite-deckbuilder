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

### Areas are TERRITORIES (reframed per Reed, 2026-07-12)

An **area** (today: `the_corner`, `the_block`; more later) is a territory on
a (future) city map, with its own narc behavior, its own customers, and its
own products. **Unlocking an area buys ACCESS to what is already there** —
customers don't relocate when you expand. Wall Street Wolf doesn't "move to"
the Block; he IS Block clientele you can now reach (his ×2.8 payout is the
pull that makes expansion worth the price).

Each area gates:
1. **A shop** — already true (`shop_locations.ron`, SOW-020; this RFC makes
   that file the loaded source of truth with prices).
2. **Customers** — each persona belongs to an area (`area` field in
   `buyers.ron`, default `the_corner`). **Run selection is two-stage**: pick
   the run's area first, then draw the persona from that area's clientele
   only. INTERIM: the area is picked randomly among unlocked areas — dealer
   stationing (run area = the active dealer's station, with per-dealer
   street cred) replaces this in a follow-up SOW (see the stationing design
   update in the studio repo).
3. **Cards** — already true via shop stock (block-priced cards).

**Out of scope here, reserved for the map/stationing SOWs:** per-area narc
deck profiles, per-area product pools, and the deck-power gradient that makes
under-equipped dealers struggle in richer territories.

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
