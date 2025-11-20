# Drug Dealer Roguelite Deckbuilder

**Status:** SOW-014 Complete - Dynamic Narrative Construction System

An unnamed drug dealer roguelite deckbuilder game built with Rust and Bevy. Features sequential turn-based play with Buyer personas, scenario-driven deals, active slot system, hand resolution overlay, and dynamic narrative generation with 666K+ unique story variations.

## Quick Start

### Prerequisites

- Rust (nightly toolchain configured via `rust-toolchain.toml`)
- Cargo

### Run the Game

```bash
cargo run
```

**What to expect:**
1. **Deck Builder:** Choose 10-20 cards from 20-card pool (or use preset)
2. **Start Run:** Click START RUN to begin with selected Buyer persona
3. **Sequential Play:** Narc → Player turn order, cards played face-up one at a time
4. **3 Rounds:** Each hand has 3 rounds (Draw → PlayerPhase → BuyerReveal)
5. **Active Slots:** Product, Location, Conviction, Insurance slots show what's in play
6. **Pass/Bail Out:** Check to skip playing a card, or fold to exit hand
7. **Buyer Scenarios:** 2 scenarios per Buyer with different product demands
8. **Hand Resolution:** Overlay shows outcome (Safe/Busted/BuyerBailed/etc.)
9. **Multi-Hand Runs:** NEW DEAL continues with same buyer, GO HOME returns to deck builder

### Run Tests

```bash
cargo test
```

**60+ unit tests** covering:
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

### Card Collection (SOW-010)
- **Player Deck Pool (20 cards - choose 10-20):**
  - Products (9): Weed, Ice (Meth), Heroin, Coke, Fentanyl, Codeine, Ecstasy, Shrooms, Acid
  - Locations (4): Safe House, Abandoned Warehouse, Storage Unit, Dead Drop
  - Cover (2): Alibi, Fake Receipts
  - Insurance (2): Bribed Witness, Clean Money
  - Deal Modifiers (3): Disguise, Lookout, etc.
- **Narc Deck (25 cards):** Evidence and Conviction cards
- **Buyer Deck (7 per persona):** 2 Locations + 5 Deal Modifiers (3 visible, random selection)

### Sequential Turn-Based Play (SOW-008, SOW-009)
- **3 Rounds per hand:** PlayerPhase (Narc → Player turns) → BuyerReveal
- **Player Actions:** Play card face-up, Pass (check), or Bail Out (fold)
- **Fixed Turn Order:** Narc always goes first, then Player
- **Buyer System:** 3 personas (Frat Bro, Desperate Housewife, Wall Street Wolf)
- **2 Scenarios per Buyer:** Different product demands, heat thresholds, multipliers

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

### UI (SOW-011)
- **16:9 Optimized Layout:** Active slots + scenario card + heat bar (top), played pool + player hand (bottom)
- **Active Slot System:** Visual Product/Location/Conviction/Insurance slots
- **Vertical Heat Bar:** Dynamic fill, color transitions (green/yellow/red)
- **Hand Resolution Overlay:** Modal with outcome-specific results and narrative story
- **Totals Bar:** Evidence, Cover, Multiplier displayed prominently
- **Discard Pile:** Vertical list of replaced cards
- **Buyer Scenario Card:** Shows scenario, demands, multipliers, heat limit
- **Two-tier Card Sizing:** Small (110x140) for visible hands/pool, Medium (120x152) for player hand/slots

### Dynamic Narrative System (SOW-012, SOW-014)
- **666K+ Unique Stories:** Grammatically-aware composition from card fragments
- **20+ Sentence Patterns:** Simple, compound, complex, multi-sentence structures
- **Element Reordering:** Location, evidence, and other elements vary in position
- **Context-Aware Conjunctions:** Positive outcomes use "and", negative use "but"
- **Grammatical Structure Filtering:** Full clauses vs. prepositional phrases
- **Card-Based Fragments:** Products, buyers, locations, and evidence provide narrative clauses
- **Optional Clauses:** Cards only specify relevant narrative elements (no empty arrays)
- **Zero Fallback Text:** All test variations produce grammatically correct stories

## Controls

- **Deck Builder:**
  - Click cards to select/deselect for your deck
  - Click preset buttons (Default/Aggro/Control) to load presets
  - Click START RUN when deck is valid (10-20 cards)
- **During Play:**
  - Click card in your hand to play it face-up
  - Click PASS to skip playing a card
  - Click BAIL OUT to fold and exit hand
- **Hand Resolution:**
  - Overlay appears automatically when hand completes
  - Click NEW DEAL to continue run with same deck
  - Click GO HOME to return to deck builder

## Asset System

**Asset Externalization (SOW-013):** All cards, buyers, and narrative content are defined in external RON files under `assets/`:
- `assets/cards/*.ron` - Card definitions (products, locations, evidence, modifiers, etc.)
- `assets/buyers.ron` - Buyer personas with scenarios and reaction decks
- `assets/narc_deck.ron` - Narc deck composition (references cards by ID)
- `assets/narrative_defaults.ron` - Default narrative fragments

**Card ID System:** Cards use semantic string IDs (`"weed"`, `"safe_house"`, `"patrol"`) for readability and maintainability.

**Deduplication:** Cards are defined once and referenced by ID in deck compositions, eliminating duplicates.

## Project Structure

```
.
├── src/
│   ├── main.rs           # Core game logic (~5600 lines)
│   └── ui/               # Modular UI system (SOW-011-A)
│       ├── mod.rs        # UI module entry point
│       ├── theme.rs      # Color palette and sizing constants
│       ├── components.rs # UI component markers
│       ├── helpers.rs    # Card display helpers
│       └── systems.rs    # UI update systems
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
- ✅ **SOW-002:** Betting system and AI opponents (3 rounds, sequential play)
- ✅ **SOW-003:** Insurance and conviction system (bust survival, heat management)
- ✅ **SOW-004:** Card retention between hands (persistent hands)
- ✅ **SOW-005:** Deck balance and card distribution (20-card pool)
- ✅ **SOW-006:** Run progression and deck building (meta systems)
- ✅ **SOW-008:** Sequential play with progressive reveals (turn-based)
- ✅ **SOW-009:** Buyer system (merged Dealer + Customer into Buyer personas)
- ✅ **SOW-010:** Buyer scenarios and product expansion (9 products, 2 scenarios/buyer)
- ✅ **SOW-011-A:** UI refactor - Core layout & foundation (modular UI, 16:9 layout)
- ✅ **SOW-011-B:** UI refactor - Hand resolution & polish (overlay, consistent sizing)
- ✅ **SOW-012:** Narrative generation system (phrasal fragments, pattern composition)
- ✅ **SOW-013:** Asset externalization (RON files, string IDs, card deduplication)
- ✅ **SOW-014:** Dynamic narrative construction (666K+ variations, grammatical awareness)

## Next Steps

See `docs/01-rfc/` for planned features and design documents.

## Documentation

- **SOWs:** `docs/03-sow/` - 14 statements of work (all complete and merged to main)
- **RFCs:** `docs/01-rfc/` - 14 feature requests (13 implemented, 1 rejected)
- **ADRs:** `docs/02-adr/` - 6 architectural decision records
- **Specs:** `docs/00-spec/` - Game design specifications with feature matrices
- **CLAUDE.md:** Instructions for Claude Code sessions
- **ROLES/:** Role-based development guidelines (DEVELOPER, ARCHITECT, PLAYER, DEBUGGER)

## License

TBD
