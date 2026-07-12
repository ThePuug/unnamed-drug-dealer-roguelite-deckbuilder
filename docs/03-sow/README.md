# Statements of Work (SOWs)

SOWs are work orders for developers. Each SOW defines what needs to be built, why it matters, and the constraints the implementation must satisfy - but NOT how to implement it.

## SOW Index

| # | Title | Category | Status | Created |
|---|-------|----------|--------|---------|
| 001 | Minimal Playable Hand | 🛠️ Foundation | ✅ Merged | 2025-11-09 |
| 002 | Betting System and AI | 🃏 Cards | ✅ Merged | 2025-11-09 |
| 003 | Insurance and Complete Cards | 🃏 Cards | ✅ Merged | 2025-11-09 |
| 004 | Card Retention Between Hands | 🃏 Cards | ✅ Merged | 2025-11-09 |
| 005 | Deck Balance and Card Distribution | 🃏 Cards | ✅ Merged | 2025-11-09 |
| 006 | Run Progression and Meta Systems | 🎲 Roguelite | ✅ Merged | 2025-11-09 |
| 007 | Meaningful Betting Decisions | 🎲 Roguelite | ❌ Rejected | 2025-11-10 |
| 008 | Sequential Play with Progressive Dealer Reveals | 🎲 Roguelite | ✅ Merged | 2025-11-10 |
| 009 | Buyer System (Merged Dealer + Customer) | 🎲 Roguelite | ✅ Merged | 2025-11-11 |
| 010 | Buyer Scenarios and Product/Location Expansion | 🃏 Cards | ✅ Merged | 2025-11-15 |
| 011-A | UI Refactor - Core Layout & Foundation | 🎨 UI | ✅ Merged | 2025-11-15 |
| 011-B | UI Refactor - Hand Resolution & Polish | 🎨 UI | ✅ Merged | 2025-11-15 |
| 012 | Narrative Generation System | 🎨 UI | ✅ Merged | 2025-11-16 |
| 013 | Asset Externalization | 🛠️ Dev Tools | ✅ Merged | 2025-11-18 |
| 014 | Dynamic Narrative Construction | 🎨 UI | ✅ Merged | 2025-11-19 |
| 015 | Heat & Character Persistence | 🎲 Roguelite | ✅ Merged | 2025-11-25 |
| 016 | Account Cash System | 💰 Economy | ✅ Merged | 2025-11-25 |
| 017 | Player Card Upgrades | 🃏 Cards | ✅ Merged | 2025-11-25 |
| 018 | Narc Difficulty Scaling | 🎲 Roguelite | ✅ Merged | 2025-11-26 |
| 019 | Upgrade Stat Choice | 🃏 Cards | ✅ Merged | 2025-11-27 |
| 020 | Location Card Shops | 💰 Economy | ✅ Merged | 2025-11-27 |
| 021 | Stabilization - Defect Fixes and Data Integrity | 🛠️ Foundation | ✅ Merged | 2026-07-11 |
| 022 | Game Play Screen v2 | 🎨 UI | ✅ Merged | 2026-07-12 |
| 023 | Kingpin & Dealers Foundation | 🎲 Roguelite | ✅ Merged | 2026-07-12 |
| 024 | Areas & Unlocks | 💰 Economy | ✅ Merged | 2026-07-12 |
| 025 | Street Cred & Stationing | 🎲 Roguelite | ✅ Merged | 2026-07-12 |
| 026 | Content Authoring Pass | 🃏 Cards | ✅ Merged | 2026-07-12 |
| 027 | Heat Economy | 🎲 Roguelite | ✅ Merged | 2026-07-12 |
| AAA | Code Organization - main.rs Modularization | 🛠️ Dev Tools | ✅ Merged | 2025-11-15 |

**Legend:**
- **Status:** ✅ Accepted/Merged | 🔄 In Progress/Review | 📝 Planned/Proposed | ❌ Rejected
- **Category:** 🃏 Cards | 💰 Economy | 🎲 Roguelite | 🗺️ Map/Events | 🧪 Substances | ⚔️ Combat | 🎨 UI | 🛠️ Dev Tools

---

## SOW Template

```markdown
# SOW-NNN: [Feature Name]

## Status

**[Planned / In Progress / Review / Approved / Merged]** - YYYY-MM-DD

## References

- **RFC-NNN:** [Feature Name](../01-rfc/NNN-feature.md)
- **ADR-NNN:** [Decision Name](../02-adr/NNN-decision.md) (if applicable)
- **Spec:** [Spec Reference](../00-spec/system.md) (if applicable)
- **Branch:** [branch-name / (proposed) / (merged)]
- **Implementation Time:** [X-Y hours/days]

---

## Implementation Plan

### Phase 1: [Phase Name]

**Goal:** [One sentence describing what this phase achieves]

**Deliverables:**
- [Specific file/component 1]
- [Specific file/component 2]
- [Specific file/component 3]

**Architectural Constraints:**
- [Constraint 1 - WHAT must be true, not HOW to do it]
- [Constraint 2 - Performance/integration requirements]
- [Constraint 3 - System boundaries/interfaces]
- [Constraint 4 - Data structures/formats]

**Success Criteria:**
- [Testable outcome 1]
- [Testable outcome 2]
- [Testable outcome 3]

**Duration:** [X hours/days]

---

### Phase 2: [Phase Name]

[Repeat structure for each phase]

---

## Acceptance Criteria

**Functional:**
- [All features work as specified]
- [Edge cases handled correctly]

**UX:**
- [Player-facing quality metrics]
- [No regressions in existing features]

**Performance:**
- [Overhead measurements]
- [Scalability requirements]

**Code Quality:**
- [Test coverage requirements]
- [Documentation completeness]
- [Code organization standards]

---

## Discussion

*This section is populated during implementation with questions, decisions, and deviations.*

### Implementation Note: [Topic]

[Document decisions made during implementation, deviations from plan, and rationale]

---

## Acceptance Review

*This section is populated after implementation is complete.*

### Scope Completion: [X%]

**Phases Complete:**
- ✅ Phase 1: [Name]
- ✅ Phase 2: [Name]
- ⏸️ Phase 3: [Name] (deferred to post-MVP)

### Architectural Compliance

[Assessment of whether implementation follows ADR specifications]

### Player Experience Validation

[Assessment from PLAYER role perspective]

### Performance

[Actual measurements vs. targets]

---

## Conclusion

[Summary of what was achieved, impact, and next steps]

---

## Sign-Off

**Reviewed By:** [ARCHITECT Role]
**Date:** YYYY-MM-DD
**Decision:** ✅ **[ACCEPTED / NEEDS CHANGES / REJECTED]**
**Status:** [Merged to main / Needs revision]
```

---

## SOW Lifecycle

### 1. Creation (Planned Status)

**Who:** ARCHITECT role creates from approved RFC

**Output:** `03-sow/NNN-[feature].md` matching RFC number

**Contains:**
- Implementation Plan (phases with deliverables)
- Architectural Constraints (what/why/constraints, NOT how)
- Acceptance Criteria (how we know it's done)

**Key Principle:** SOWs are **descriptive** (what to build), not **prescriptive** (how to build it)

### 2. Implementation Begins (Planned → In Progress)

**Who:** DEVELOPER role starts work

**Process:**
- Developer reads SOW phases sequentially
- Works through deliverables autonomously
- Has freedom to choose implementation approaches within constraints

**Status Update:** Change to "In Progress" when first commit made

**Feature Matrix Update:** Mark feature as "In Progress"

### 3. Discussion Updates (During In Progress)

**Who:** DEVELOPER role documents as work proceeds

**Adds to SOW:**
- Implementation questions and answers
- Decisions made during development
- Deviations from plan with rationale
- Discoveries that affect approach

**Location:** Discussion section of SOW

**Example:**
```markdown
### Implementation Note: Card Effect Timing

Initially planned card effects to resolve immediately, but discovered
this prevents counterplay. Changed to queued resolution system, allowing
opponents to respond. This aligns with ADR-XXX effect stack design.
```

### 4. Implementation Complete (In Progress → Review)

**Who:** DEVELOPER role finishes all phases

**Triggers:**
- All deliverables implemented
- Tests passing
- Branch ready for review

**Status Update:** Change to "Review"

**Next Step:** ARCHITECT reviews implementation

### 5. Acceptance Review (Review → Approved/Needs Changes)

**Who:** ARCHITECT role reviews against acceptance criteria

**Adds to SOW:**
- Scope Completion assessment
- Architectural Compliance check
- Player Experience Validation (PLAYER role input)
- Performance measurements
- Final sign-off decision

**Location:** Acceptance Review section of SOW

**Outcomes:**
- ✅ **Approved:** Ready to merge
- 🔄 **Needs Changes:** Specific revisions required
- ❌ **RFC Revision Required:** Implementation revealed RFC was infeasible

### 6. Merge and Lockdown (Approved → Merged)

**Who:** ARCHITECT or DEVELOPER merges branch

**Process:**
1. Merge branch to main
2. Update SOW status to "Merged"
3. Update feature matrix (mark complete, link to SOW)
4. SOW is now frozen (historical record)

**Status Update:** Change to "Merged", add merge date

**Feature Matrix Update:** Mark feature as "Complete"

### 7. Post-Merge (Merged Status)

**SOW is locked:** No further changes (historical record)

**If issues found:**
- Bug fixes: Direct commits to main (no SOW update)
- Design changes: Create new RFC + SOW
- Spec deviations: Document in feature matrix

---

## Writing Tips

### Implementation Plan

**Good Phase Structure:**
- Clear goal (one sentence)
- Specific deliverables (files, components)
- **Constraints, not instructions** (what must be true, not how)
- Testable success criteria

**Example - Descriptive (Good):**
```markdown
**Architectural Constraints:**
- Card effects: Support targeting (self/enemy/all), cost calculation, validation
- Effect resolution: Must integrate with game state manager
- Data format: JSON-serializable for save/load compatibility
```

**Example - Prescriptive (Bad):**
```markdown
**Implementation Steps:**
1. Create CardEffect struct with these fields...
2. In card.rs, add this code: `impl CardEffect {...}`
3. Loop through all targets and apply...
```

**Why Descriptive Wins:**
- Developer has autonomy (can choose best approach)
- Constraints define correctness (implementation is flexible)
- Easier to maintain (doesn't prescribe every detail)

### Architectural Constraints

**Focus on:**
- Performance requirements ("< 10ms per card resolution")
- Integration points ("Uses existing GameState from ADR-XXX")
- Data formats ("Card { id: u32, cost: i32, effects: Vec<Effect> }")
- System boundaries ("Effect resolution synchronous, animation async")
- Timing/ordering ("Validation BEFORE cost payment, effects AFTER")

**Avoid:**
- Code snippets (let developer write it)
- Step-by-step instructions (trust developer autonomy)
- Implementation details ("use Vec" vs "ordered collection required")

### Success Criteria

**Make it testable:**
- ✅ "10-cost card fails validation when player has 9 resources"
- ❌ "Cards have correct cost"

**Be specific:**
- ✅ "Drawing from empty deck applies 5 damage penalty"
- ❌ "Deck exhaustion works correctly"

**Cover edge cases:**
- ✅ "Effect targeting fails gracefully when target invalid"
- ❌ "Targeting system works"

### Discussion Section

**Document during implementation:**
- Design decisions made on-the-fly
- Trade-offs discovered during coding
- Deviations from original plan
- Bugs found and how they were fixed

**Format:**
```markdown
### Implementation Note: [Topic]

[Context: What we discovered]
[Decision: What we chose to do]
[Rationale: Why we made that choice]
```

---

## Common Patterns

### When to Update SOW During Implementation

**Update Discussion section when:**
- ✅ You discover a better approach than planned
- ✅ You find a constraint that wasn't documented
- ✅ You deviate from the plan (with good reason)
- ✅ You make a design decision the next developer should know

**Don't update for:**
- ❌ Implementation details (code-level choices)
- ❌ Bug fixes during development (expected)
- ❌ Refactoring within same phase (internal)

### When to Split Phases

**Split phases when:**
- Dependencies exist (Phase 2 needs Phase 1 complete)
- Natural checkpoints (infrastructure → feature → polish)
- Duration > 2 days (break into smaller units)

**Keep together when:**
- Tightly coupled (can't test one without other)
- Short duration (< 4 hours total)

### When to Defer to Post-MVP

Use "excludes" in Implementation Plan:
```markdown
**Phase 1 excludes:**
- Animated card effects (static images only - Phase 2)
- Combo detection (single card effects only - Phase 2)
```

Then document in Acceptance Review if actually deferred.

---

## Questions?

- **Can I deviate from the SOW?** Yes, document in Discussion with rationale
- **What if I find a better approach?** Use it, explain why in Discussion
- **How detailed should phases be?** Enough for another dev to understand constraints
- **When do I update feature matrix?** At status transitions (Planned → In Progress → Complete)
- **Can SOWs change after merge?** No - they're historical records
