# Drug Dealer Roguelite Deckbuilder

**Status:** actively developed through **SOW-036** (the "kingpin era"). See [`docs/00-spec/product-roadmap.md`](docs/00-spec/product-roadmap.md) for the live status and iteration log.

An unnamed drug-dealer roguelite deckbuilder built in **Rust** with the **Bevy** engine. You play the **kingpin**: hire dealers who run deals on your behalf across a city of unlockable neighborhoods, juggle heat, cash, and supply, and build a legacy that outlives any single bust.

## Quick Start

### Prerequisites
- Rust toolchain (pinned via `rust-toolchain.toml`)
- Cargo

### Build / Run / Test
```bash
cargo run                 # play
cargo test                # unit suite (280+ tests, zero-warnings baseline)
cargo build --release     # optimized build
```

Dev tool — forge a signed save into a known state for playtesting, then exit:
```bash
cargo run -- forge <scenario> [--dir <path>]
```

## The Game

You're not the dealer on the corner — you're the **kingpin**. The loop:

- **Hire dealers.** Each neighborhood offers a named **signature dealer** you hire on the city map; they run deals **stationed** in that zone, carrying their own deck, heat, and story.
- **Cash is global; jail replaces permadeath.** A busted dealer goes to **jail** for a sentence (post bail to spring them early); only a busted *kingpin* ends the empire. Cash pools across the whole roster.
- **Unlock the city.** Three neighborhoods — **Trailer Park** (free start) → **Suburbia** ($1,200) → **Red Light District** ($2,500) — each with its own clientele, narcs, supplier, and shop ladder.
- **Buy or front your stock.** Products are **limited-use consumable stock**: unlocking grants permanent access, but each is bought — or **fronted** on supplier credit — in batches, and every deal burns a charge. Run dry and you're out until you restock.
- **Heat scales the danger.** The more heat a dealer carries, the tougher the narcs they draw; **Lay Low** and a **Crooked Lawyer** are your release valves.
- **Build a legacy.** The **Kingpin Ledger** collects roster dossiers and stories; fallen empires land on a browsable arcade board.

### Clientele (9 buyer personas, 3 per neighborhood)
- **Trailer Park:** Biker, Tweaker, Deadbeat
- **Suburbia:** Frat Bro, Desperate Housewife, Widow
- **Red Light District:** Pimp, Working Girl, John

### Products
Six products stock the shops across the three zones — **Weed, Shrooms** (Trailer Park), **Codeine, Xanax** (Suburbia), **Ecstasy, Coke** (Red Light District) — with a premium tier (Acid, Ice, Heroin, Fentanyl) shelved for a future arc.

## A Deal (the hand)

Each deal is a sequential, turn-based hand against the **narc**:
- Cards play face-up one at a time; the narc acts, then you, across the rounds of a hand.
- You stack **Evidence** against **Cover** — if Evidence ≤ Cover the deal is **Safe**, otherwise you risk a **Bust**.
- **Insurance** can save you from a bust if you can pay the cost; **Conviction** cards override insurance once heat clears their threshold.
- Outcomes (Safe / Busted / Buyer Bailed / …) resolve in an overlay with a **dynamically generated narrative** — grammar-aware composition from card fragments, hundreds of thousands of variations.

## Controls (mouse-driven)

- **Hub:** manage your roster, open the **City Map** or **Kingpin Ledger**, and **START RUN**.
- **City Map:** **UNLOCK** a new zone, **HIRE** a zone's dealer, or **SEND** a stationed dealer to another zone.
- **Shop (per zone):** **BUY BATCH / RESTOCK** stock, **FRONT** on supplier credit, **PAY** down what you owe; cred-gated items show their requirement.
- **In a deal:** click a card to play it, **PASS** to check, **BAIL OUT** to fold; on resolution, **NEW DEAL** to continue or **GO HOME** to return to the hub.

## Content (RON-authored)

All content is defined in human-readable **RON** files under `assets/` and validated at load:
- `assets/cards/*.ron` — products, locations, cover, insurance, modifiers, convictions, evidence
- `assets/buyers.ron` — buyer personas (area-gated), scenarios, reaction decks
- `assets/data/shop_locations.ron` — zones: unlock ladder, shop stock, signature dealer, supplier, narc mix
- `assets/narc_deck.ron`, `assets/narrative_defaults.ron` — narc composition & default narrative fragments

## Project Structure

```
src/
├── main.rs         # App bootstrap: Bevy plugin/state wiring + `forge` CLI
├── game_state.rs   # GameState (Bevy states) + shared resources
├── models/         # Pure, unit-tested domain logic (cards, hand state machine,
│                   # buyer, deck_builder, shop_location, narrative engine)
├── systems/        # Bevy ECS systems (game_loop, input, shop, city_map,
│                   # kingpin_ledger, save_integration, ui_update, upgrade_choice)
├── ui/             # View layer: pure *_view.rs render fns + setup/components/theme
├── data/           # Built-in preset content
├── save/           # Versioned, HMAC-signed saves (types w/ SAVE_VERSION, crypto, io, forge)
└── assets/         # Runtime RON asset loading (loader, registry)
```
Top-level `assets/` holds the RON content, fonts, shaders, and generated art.

## Development

- **Bevy ECS**, state-driven; domain logic lives in `models/` as **pure functions**, `systems/` only orchestrate.
- **TDD**; view logic is tested as pure `*_view.rs` functions; **zero warnings** required on both build and test.
- **Versioned saves:** HMAC-signed with serde-default migration (`SAVE_VERSION` currently 9); a schema bump wipes old saves (pre-release convention).
- End-to-end playtests drive the real window via `tools/e2e/game-drive.ps1`.

## Documentation

The repo uses a numbered documentation system under `docs/` — see [`docs/README.md`](docs/README.md):
- **Specs & feature matrices:** `docs/00-spec/` (start with `product-roadmap.md`)
- **RFCs:** `docs/01-rfc/` · **ADRs:** `docs/02-adr/` · **SOWs:** `docs/03-sow/` (each folder has a `README.md` index)
- **CLAUDE.md** — guidance for Claude Code sessions · **ROLES/** — role playbooks (DEVELOPER, ARCHITECT, DEBUGGER, PLAYER)

## License

TBD
