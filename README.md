# Drug Dealer Roguelite Deckbuilder

**Status:** SOW-003 Complete - Insurance, Conviction, and Complete Cards

An unnamed drug dealer roguelite deckbuilder game built with Rust and Bevy. Currently implements multi-round hands with betting, AI opponents, insurance mechanics, and conviction system.

## Quick Start

### Prerequisites

- Rust (nightly toolchain configured via `rust-toolchain.toml`)
- Cargo

### Run the Game

```bash
cargo run
```

**What to expect:**
1. Window opens showing game UI with 3 rounds of play
2. **Betting Phase:** Each round, players bet by checking or playing cards (raise)
3. **Flip Phase:** Cards reveal, totals update with Evidence/Cover/Heat/Profit
4. **Decision Phase:** After rounds 1-2, choose Continue or Fold
5. **Resolution:** After round 3, check for bust (Evidence > Cover)
6. **Insurance:** If you have cash and insurance cards, can survive bust
7. **Conviction:** High heat blocks insurance at thresholds (Warrant 40, DA Approval 60)
8. **Multi-Hand Runs:** Safe → "NEXT HAND" (keep cash/heat), Bust → "NEW RUN" (reset)

### Run Tests

```bash
cargo test
```

**46 unit tests** covering:
- Card mechanics (override rules, additive stacking)
- Multi-round state machine and betting system
- AI decision making and initiative
- Insurance activation and affordability
- Conviction threshold checks and overrides
- Cash/heat accumulation across hands
- Face-down card handling in multi-round play

### Build

```bash
cargo build
```

## Current Features

### Card Collection (SOW-003)
- **Player Deck (15 cards):**
  - Products: Weed, Meth, Heroin, Cocaine
  - Locations: Safe House, School Zone, Warehouse
  - Cover: Alibi, Bribe
  - Evidence: Informant
  - Insurance: Plea Bargain ($1000 cost), Fake ID ($0 cost)
  - Conviction: Warrant (threshold 40), DA Approval (threshold 60)
  - Deal Modifier: Disguise
- **Narc Deck (15 cards):** 10× Donut Break, 3× Patrol, 2× Surveillance
- **Customer Deck (10 cards):** 5× Regular Order, 5× Haggling

### Multi-Round Betting System (SOW-002)
- **3 Rounds per hand:** Each round has Betting → Flip → Decision
- **Betting Actions:** Check (pass), Raise (play card), Fold (exit hand)
- **Initiative System:** First raiser gains initiative (controls turn order)
- **AI Opponents:** Narc and Customer with weighted decision-making

### Insurance & Conviction System (SOW-003)
- **Insurance Cards:** Save you from bust if you have cash
  - Act as Cover during hand (+15 to +20)
  - Activate on bust if affordable (pay cost, gain heat penalty)
  - Single-use (burn after activation)
- **Conviction Cards:** Override insurance at high heat
  - No effect on totals (only affects bust resolution)
  - Check heat threshold (40 for Warrant, 60 for DA Approval)
  - Block insurance if heat ≥ threshold
- **Multi-Hand Runs:** Cash and heat persist across hands until bust

### Card Mechanics
- **Override Rules:** Last Product/Location/Insurance/Conviction played = active
- **Additive Rules:** Evidence, Cover, and Deal Modifiers stack
- **Heat Calculation:** Sum all heat modifiers, accumulates across hands
- **Bust Resolution:**
  1. Evidence ≤ Cover → Safe
  2. Conviction active + heat ≥ threshold → Busted (overrides insurance)
  3. Insurance active + affordable → Pay cost, gain heat → Safe
  4. Otherwise → Busted

### UI
- Real-time totals (Evidence, Cover, Heat, Profit)
- Cash and cumulative heat display
- Insurance status (cost, heat penalty)
- Conviction warnings (threshold alerts)
- Dynamic button text (CHECK/CALL, NEXT HAND/NEW RUN)
- Color-coded cards by type

## Controls

- **During Betting:**
  - Click card to Raise/Call
  - Click CHECK to check (if no raises)
  - Click FOLD to fold hand
- **At Decision Point:** Click CONTINUE or FOLD
- **At Hand End:** Click NEXT HAND (safe) or NEW RUN (busted)
- **Close Window:** End game

## Card Data

Cards are currently hardcoded in `src/main.rs`:
- `create_narc_deck()` - 15 cards (shuffled)
- `create_customer_deck()` - 10 cards (shuffled)
- `create_player_deck()` - 15 cards (shuffled)

To modify card values, edit these functions and recompile.

**Note:** Hot-reloadable card data (RON files) is planned for a future iteration.

## Project Structure

```
.
├── src/
│   └── main.rs           # All game logic (single file for MVP)
├── docs/                 # Game design specs, RFCs, ADRs, SOWs
├── ROLES/                # Role-based development guidelines
├── Cargo.toml            # Rust dependencies
├── rust-toolchain.toml   # Rust nightly toolchain
└── README.md             # This file
```

## Development

- **TDD:** Unit tests written for all core mechanics
- **Bevy ECS:** Game state managed through Entity-Component-System architecture
- **Pure Functions:** Card logic extracted into testable pure functions

## Implementation Status

- ✅ **SOW-001:** Minimal playable hand (single round, basic mechanics)
- ✅ **SOW-002:** Betting system and AI opponents (3 rounds, betting, initiative)
- ✅ **SOW-003:** Insurance and conviction system (bust survival, heat management)

## Next Steps

See `docs/01-rfc/` for planned features and design documents.

## Documentation

- **SOWs:** `docs/03-sow/` (implementation plans)
  - SOW-001: Minimal playable hand (Merged)
  - SOW-002: Betting system and AI (Merged)
  - SOW-003: Insurance and conviction (Merged)
- **RFCs:** `docs/01-rfc/` (feature requirements)
- **ADRs:** `docs/02-adr/` (architectural decisions)
- **Specs:** `docs/00-spec/` (game design vision)
- **CLAUDE.md:** Instructions for Claude Code sessions

## License

TBD
