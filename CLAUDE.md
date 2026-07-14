# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Role Adoption

**You must adopt a role for each session.** The default role is **DEVELOPER** unless explicitly instructed otherwise.

### Available Roles

**Development Team Roles:**
- **DEVELOPER** (default): General development work, TDD, clean code, feature implementation (see `ROLES/DEVELOPER.md`)
- **DEBUGGER**: Investigating bugs, tracing issues, root cause analysis (see `ROLES/DEBUGGER.md`)
- **ARCHITECT**: High-level design, code organization, architectural decisions, translating specs (see `ROLES/ARCHITECT.md`)

**Product & Player Roles:**
- **PLAYER**: End-user perspective, fun factor, UX, roadmap priorities, voice of the customer (see `ROLES/PLAYER.md`)

### Role Guidelines

- **Switching roles**: User can request role changes at any time (e.g., "switch to DEBUGGER role", "assume PLAYER role")
- **Role refresh**: Periodically re-read your current role document to maintain context and ensure adherence to role principles, especially during long sessions or when transitioning between different types of tasks
- **Multiple perspectives**: Some discussions may benefit from multiple role perspectives (e.g., PLAYER feedback on ARCHITECT designs)

**At the start of each session, read and adopt the DEVELOPER role by default.**

## Project Overview

This is an unnamed drug dealer roguelite deckbuilder built in Rust with the Bevy 0.18 engine (package `drug-dealer-deckbuilder`, edition 2021). It is a mature, actively developed game — 36+ merged SOWs, a large unit-test suite (280+ tests, zero-warnings baseline), RON-driven content (zones, buyers, shops, cards), a kingpin/dealer roster loop, and versioned HMAC-signed saves (`SAVE_VERSION` currently 9). See [docs/00-spec/product-roadmap.md](docs/00-spec/product-roadmap.md) for current status.

## Commands

```bash
# Build (debug)
cargo build

# Run the game
cargo run

# Run the full unit-test suite (280+ tests; must stay zero-warnings)
cargo test

# Optimized release build
cargo build --release

# Dev tool: forge a signed save for an e2e playtest scenario, then exit
cargo run -- forge <scenario> [--dir <path>]
```

The e2e playtest driver lives at `tools/e2e/game-drive.ps1` (design-coordinate click/hover/screenshot), alongside `tools/e2e/playtest.ps1`; it pairs with the `forge` subcommand to boot the game into a known state.

## Code Organization

```
src/
├── main.rs         - App bootstrap: Bevy plugin/state wiring + the `forge` CLI subcommand
├── game_state.rs   - GameState enum (Bevy states) and shared resources
├── models/         - Pure, unit-tested domain logic (card, cards, hand_state state machine,
│                     buyer, deck_builder, shop_location, narrative engine)
├── systems/        - Bevy ECS systems that orchestrate models (game_loop, input, shop,
│                     city_map, kingpin_ledger, save_integration, ui_update, upgrade_choice)
├── ui/             - View layer: pure `*_view.rs` render fns (tested) + setup/components/theme
├── data/           - Built-in preset content (player_deck, narc_deck, buyer_personas, presets)
├── save/           - Versioned HMAC-signed saves (types w/ SAVE_VERSION, crypto, io, forge)
└── assets/         - Runtime RON asset loading (loader, registry)
```

Top-level `assets/` holds the RON content (cards, buyers, shop_locations, narc_deck), fonts, shaders, and generated art. Domain logic lives in `models/` as pure functions; `systems/` only orchestrates.

## Documentation Map

The repository uses a comprehensive documentation system. Understanding where to find information and when to update documentation is critical.

### Root-Level Documents

**[README.md](README.md)**
- User-facing overview of the game
- Current playable features
- Controls and what to expect
- Build instructions
- **Update when:** Adding major features, changing controls, or updating build instructions

**[GUIDANCE.md](GUIDANCE.md)** ⚠️ CRITICAL (if it exists)
- **ALWAYS read before making code changes**
- Core architecture patterns
- TDD workflow rules
- Common pitfalls and anti-patterns
- System execution order
- **Update when:** User confirms a solution works AND pattern should be documented for future reference
- **Never commit** - only update the file locally

**[CLAUDE.md](CLAUDE.md)** (this file)
- Instructions for Claude Code sessions
- Role adoption system
- Documentation map
- Commands and code organization
- **Update when:** Adding new documentation types, changing project structure, or updating Claude workflow

### Role Documents (`ROLES/`)

Define different perspectives for development work:
- **[DEVELOPER.md](ROLES/DEVELOPER.md)** - Default role: TDD, clean code, feature implementation
- **[ARCHITECT.md](ROLES/ARCHITECT.md)** - High-level design, code organization, architectural decisions
- **[DEBUGGER.md](ROLES/DEBUGGER.md)** - Bug investigation, tracing issues, root cause analysis
- **[PLAYER.md](ROLES/PLAYER.md)** - End-user perspective, UX, fun factor, roadmap priorities

**Update when:** Refining role principles or adding new specialized roles

### Specifications (`docs/00-spec/`)

High-level game design documents describing **what the game should be**:

Existing specs: `core-gameplay-loop`, `card-system`, `heat-system`, `narrative-system`, `bust-insurance-mechanics`, `progression-meta`, and the living `product-roadmap.md` (the freshest source of truth for current status).

**Purpose:** Define ideal game systems (aspirational, not necessarily implemented)
**Update when:** Major design decisions or feature scope changes (rare)

### Feature Matrices (`docs/00-spec/`)

Living documents tracking **spec vs. implementation** for each specification:

Six matrices exist (one per spec): `core-gameplay-loop`, `card-system`, `heat-system`, `narrative-system`, `bust-insurance-mechanics`, `progression-meta` — see `docs/00-spec/*-feature-matrix.md`.

**See detailed [Feature Matrices](#feature-matrices) section below for when/how to update.**

### RFCs (`docs/01-rfc/`)

Requests for Comments — feature proposals with feasibility and approval discussion:

**Pattern:** `NNN-feature.md` (matching the SOW number). Frozen once Approved. See `docs/01-rfc/README.md` for the index (RFCs run to 035).

### Architecture Decision Records (`docs/02-adr/`)

Documents recording **implementation decisions** and their rationale:

**Pattern:** `NNN-title.md` (e.g., `001-card-type-system-and-interaction-rules.md`)

Six ADRs exist (001–006); see `docs/02-adr/README.md` for the index and status legend.

**Purpose:** Record architectural decisions, context, consequences, and implementation details

**Update when:** Making significant architectural decisions (create new ADR via ARCHITECT role)

### SOWs (`docs/03-sow/`)

Statements of Work — implementation plans (`NNN-feature.md`, matching RFC numbers):

**Pattern:** each SOW carries its Implementation Plan, a Discussion log, and an in-document **Acceptance Review** section. 36 merged to date. See `docs/03-sow/README.md` for the index and the full SOW lifecycle.

**Update when:** creating/implementing/accepting a SOW (status transitions Planned → In Progress → Merged).

### Acceptance & Player Feedback

Acceptance review is **not** a separate document type: it lives inside each SOW's *Acceptance Review* section (see the SOW lifecycle in `docs/03-sow/README.md`). Standalone `*-acceptance.md` / `*-player-feedback.md` docs and a `docs/adr/` folder are **not** used — the repository uses the numbered `00-spec` / `01-rfc` / `02-adr` / `03-sow` taxonomy.

---

## Documentation Workflow

### When Starting Work
1. **Read role document** (default: DEVELOPER)
2. **Read GUIDANCE.md** (if it exists - architectural patterns)
3. **Check feature matrix** for relevant spec (implementation status)
4. **Review related ADRs** (implementation decisions)

### During Development
1. **Follow TDD** (if GUIDANCE.md exists and specifies it)
2. **Write tests first**
3. **Update feature matrix** when completing features
4. **Consult specs** for design intent

### After Completing Feature
1. **Update feature matrix** (mark ✅, add ADR references, recalculate totals)
2. **Create/update ADR** if architectural decision made
3. **Update GUIDANCE.md** only after user confirms solution works

### When Creating New Systems
1. **ARCHITECT role** creates ADR documenting decision
2. **DEVELOPER role** implements per ADR
3. **ARCHITECT role** creates acceptance document when complete
4. **PLAYER role** creates feedback document when playable
5. **Update feature matrix** throughout

---

## Feature Matrices

**Each specification has a companion feature matrix** that tracks implementation status against the spec. These living documents help maintain alignment between design and implementation.

### Location Pattern

```
docs/00-spec/
├── [spec-name].md
└── [spec-name]-feature-matrix.md
```

Six feature matrices exist: `core-gameplay-loop`, `card-system`, `heat-system`, `narrative-system`, `bust-insurance-mechanics`, `progression-meta` (each `docs/00-spec/[spec]-feature-matrix.md`).

### When to Consult Feature Matrices

**ALWAYS consult the relevant feature matrix when:**
- Starting work on a new feature
- Planning implementation for a spec requirement
- Prioritizing which features to build next
- Reviewing what's already been completed
- Identifying gaps between spec and implementation

### When to Update Feature Matrices

**ALWAYS update the relevant feature matrix when:**
- Completing a feature (change status from ❌/🚧 to ✅)
- Accepting an ADR that implements spec features
- Starting work on a feature (change status to 🔄 In Progress)
- Making an intentional deviation from spec (add to "Implementation Deviations")
- Deferring a planned feature (change status to ⏸️ Deferred with rationale)

### Update Process

1. **Locate the matrix:** Find `docs/00-spec/[spec-name]-feature-matrix.md`
2. **Update feature status:** Change status symbols (❌ → ✅ or 🔄)
3. **Add ADR references:** Link to relevant ADR documents
4. **Update category totals:** Recalculate "X/Y complete" for each category
5. **Update overall status:** Recalculate total completion percentage
6. **Update "Last Updated" date:** Set to current date
7. **Document deviations:** If implementation differs from spec, add to "Implementation Deviations" section with rationale

### Status Symbols

- ✅ **Complete** - Fully implemented per spec
- 🚧 **Partial** - Partially implemented or MVP version
- ❌ **Not Started** - Planned but not implemented
- ⏸️ **Deferred** - Intentionally postponed to post-MVP
- 🔄 **In Progress** - Currently being developed

### Example Workflow

```
User: "Implement card draw mechanic"

DEVELOPER:
1. Reads card-system-feature-matrix.md (if it exists)
2. Sees "Card draw: ❌ Not Started"
3. Implements the feature
4. Updates matrix:
   - Changes status to ✅ Complete
   - Adds ADR reference (if applicable)
   - Updates "Card Mechanics: 3/10 complete (30%)"
   - Updates overall percentage
   - Sets "Last Updated: 2025-11-09"
```

---

## Project-Specific Patterns

> Also read **GUIDANCE.md** before code changes — it carries the recorded pitfalls and execution-order rules.

### Architecture
- Bevy 0.18 ECS, state-driven via the `GameState` enum. Domain logic lives in `models/` as **pure functions**; `systems/` only orchestrate them.
- Content is **RON-driven** and loaded at runtime — authorability over IDs: keep content files human-readable and validate at load time (loud panic on missing/invalid content).
- Saves are versioned + HMAC-signed (`SAVE_VERSION` currently 9) with serde-default field migration; a version bump wipes old saves (pre-release wipe convention, SOW-021).

### Testing Strategy
- Large unit-test suite (280+ tests); **zero warnings** required on both `cargo build` and `cargo test`.
- View logic is tested as **pure functions** in `src/ui/*_view.rs` (no ECS in tests).
- TDD per `ROLES/DEVELOPER.md`. Live behavior is verified with the e2e driver (`tools/e2e/game-drive.ps1`).

### Code Style
- Pure view functions in `*_view.rs`; prefer deletion over `#[allow(...)]`; content authorability over hardcoded IDs.

---

## Getting Started

This project is an established Rust/Bevy game. When beginning work:

1. **Adopt DEVELOPER role** (or specified role)
2. **Read the docs/README.md** to understand the documentation system
3. **Check for existing specs** in `docs/00-spec/` to understand intended game design
4. **Check for existing RFCs** in `docs/01-rfc/` for approved features
5. **Check for existing ADRs** in `docs/02-adr/` for architectural patterns
6. **Read GUIDANCE.md** (already present) for project-specific patterns before making code changes

---

## Questions?

For questions about:
- **The documentation system**: See `docs/README.md`
- **RFC process**: See `docs/01-rfc/README.md`
- **ADR format**: See `docs/02-adr/README.md`
- **SOW workflow**: See `docs/03-sow/README.md`
- **Role guidelines**: See `ROLES/[ROLE].md`
