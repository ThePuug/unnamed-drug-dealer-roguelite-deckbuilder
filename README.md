# Drug Dealer Roguelite Deckbuilder

**Status:** SOW-001 Complete - Minimal Playable Hand (Technical Validation)

An unnamed drug dealer roguelite deckbuilder game built with Rust and Bevy. Currently implements a single-round prototype to validate core card mechanics.

## Quick Start

### Prerequisites

- Rust (nightly toolchain configured via `rust-toolchain.toml`)
- Cargo

### Run the Game

```bash
cargo run
```

**What to expect:**
1. Window opens showing game UI
2. Narc automatically plays an Evidence card (Patrol or Surveillance)
3. Your turn: Click any card from your hand to play it
4. Totals update in real-time (Evidence, Cover, Heat, Profit)
5. Result: "SAFE!" (green) if Evidence ≤ Cover, or "BUSTED!" (red) if Evidence > Cover

### Run Tests

```bash
cargo test
```

**18 unit tests** covering:
- Card data model and state machine
- Override rules (Product/Location replacement)
- Additive rules (Evidence/Cover stacking)
- Bust check logic (Evidence vs Cover comparison)

### Build

```bash
cargo build
```

## Current Features (SOW-001)

### 8-Card MVP Collection
- **Narc (2 cards):** Patrol, Surveillance (Evidence cards)
- **Player (6 cards):**
  - Products: Weed ($30), Meth ($100), Heroin ($150)
  - Locations: Safe House, School Zone
  - Cover: Alibi

### Card Mechanics
- **Override Rules:** Last Product/Location played replaces previous
- **Additive Rules:** Evidence and Cover cards stack with Location base
- **Heat Calculation:** Sum all heat modifiers from played cards
- **Bust Check:** Evidence > Cover → Busted, Evidence ≤ Cover → Safe (tie goes to player)

### UI
- Real-time totals display (Evidence, Cover, Heat, Profit)
- Color-coded status (green = your turn, red = busted, green = safe)
- Clickable card buttons (color-coded by type)
- 3 play zones (Narc, Customer, Player)

## Controls

- **Mouse:** Click cards in your hand to play them
- **Close Window:** End game

## Card Data

Cards are currently hardcoded in `src/main.rs`:
- `create_narc_deck()` - Lines ~690-710
- `create_player_deck()` - Lines ~720-800

To modify card values, edit these functions and recompile.

**Note:** Hot-reloadable card data (RON files) is planned for a future iteration but not required for MVP technical validation.

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

## Next Steps (RFC-002)

After SOW-001 ARCHITECT review and approval:
- Add 3-round hand structure
- Implement betting mechanics (Check/Raise/Fold)
- Add basic AI opponents with static decks
- Expand to 15-card collection
- Add Deal Modifiers

## Documentation

- **SOW-001:** `docs/03-sow/001-minimal-playable-hand.md` (implementation details)
- **RFC-001-revised:** `docs/01-rfc/001-revised-minimal-playable-hand.md` (requirements)
- **Specs:** `docs/00-spec/` (game design vision)
- **CLAUDE.md:** Instructions for Claude Code sessions

## License

TBD
