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

This is an unnamed drug dealer roguelite deckbuilder game. The project is in early development.

## Commands

_To be defined as the project develops. Update this section with build, test, and run commands._

```bash
# Example commands (update as project develops)
# Build
# cargo build / npm run build / etc.

# Run
# cargo run / npm start / etc.

# Tests
# cargo test / npm test / etc.
```

## Code Organization

_To be defined as the project structure emerges. Update this section with key directories and their purposes._

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

_To be created as the game design emerges._

**Purpose:** Define ideal game systems (aspirational, not necessarily implemented)
**Update when:** Major design decisions or feature scope changes (rare)

### Feature Matrices (`docs/00-spec/`)

Living documents tracking **spec vs. implementation** for each specification:

_To be created alongside specifications._

**See detailed [Feature Matrices](#feature-matrices) section below for when/how to update.**

### Architecture Decision Records (`docs/adr/`)

Documents recording **implementation decisions** and their rationale:

**Pattern:** `NNN-title.md` (e.g., `001-card-effect-system.md`)

_No ADRs exist yet._

**Purpose:** Record architectural decisions, context, consequences, and implementation details

**Update when:** Making significant architectural decisions (create new ADR via ARCHITECT role)

### Acceptance Documents (`docs/adr/`)

ARCHITECT role reviews of completed ADRs:

**Pattern:** `NNN-acceptance.md` (e.g., `001-acceptance.md`)

**Contents:**
- Implementation status by phase
- Architectural assessment
- Code quality review
- Outstanding items
- Final accept/reject recommendation

**Purpose:** Verify ADR implementation meets requirements before merging to main

**Update when:** ADR implementation is complete and ready for review (ARCHITECT role creates)

### Player Feedback Documents (`docs/adr/`)

PLAYER role perspectives on implemented features:

**Pattern:** `NNN-player-feedback.md` (e.g., `001-player-feedback.md`)

**Contents:**
- UX assessment
- Fun factor analysis
- Confusion points
- Improvement suggestions

**Purpose:** Evaluate features from end-user perspective

**Update when:** Feature is playable and PLAYER role can provide feedback

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

_No feature matrices exist yet._

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

_To be filled in as the project develops and patterns emerge._

### Architecture
- _TBD: Core architectural patterns (e.g., ECS, data-driven cards, etc.)_

### Testing Strategy
- _TBD: Unit test conventions, integration test approach_

### Code Style
- _TBD: Naming conventions, file organization_

---

## Getting Started

This project is in early development. When beginning work:

1. **Adopt DEVELOPER role** (or specified role)
2. **Read the docs/README.md** to understand the documentation system
3. **Check for existing specs** in `docs/00-spec/` to understand intended game design
4. **Check for existing RFCs** in `docs/01-rfc/` for approved features
5. **Check for existing ADRs** in `docs/02-adr/` for architectural patterns
6. **Create GUIDANCE.md** if project-specific patterns emerge that should be documented

---

## Questions?

For questions about:
- **The documentation system**: See `docs/README.md`
- **RFC process**: See `docs/01-rfc/README.md`
- **ADR format**: See `docs/02-adr/README.md`
- **SOW workflow**: See `docs/03-sow/README.md`
- **Role guidelines**: See `ROLES/[ROLE].md`
