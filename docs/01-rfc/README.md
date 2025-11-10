# Requests for Comments (RFCs)

RFCs bridge player needs and technical reality. Each RFC starts with a player-facing problem, explores feasibility, and results in an approved plan for implementation.

## RFC Index

| # | Title | Category | Status | Created | Has ADR |
|---|-------|----------|--------|---------|---------|
| 001 | MVP Core Prototype | 🛠️ Foundation | ✅ Split Approved (superseded) | 2025-11-09 | N/A |
| 001-revised | Minimal Playable Hand | 🛠️ Foundation | ✅ Implemented (SOW-001 merged) | 2025-11-09 | ADR-001 |
| 002 | Betting System and AI Opponents | 🎲 Roguelite | ✅ Implemented (SOW-002 merged) | 2025-11-09 | ADR-002/004/005 |
| 003 | Insurance and Complete Cards | 🃏 Cards | ✅ Implemented (SOW-003 merged) | 2025-11-09 | ADR-001/003 |
| 004 | Card Retention Between Hands | 🃏 Cards | ✅ Implemented (SOW-004 merged) | 2025-11-09 | N/A |
| 005 | Deck Balance and Card Distribution | 🃏 Cards | ✅ Implemented (SOW-005 merged) | 2025-11-09 | N/A |
| 006 | Run Progression and Meta Systems | 🎲 Roguelite | ✅ Implemented (SOW-006 merged) | 2025-11-09 | N/A |
| 007 | Meaningful Betting Decisions | 🎲 Roguelite | ❌ Rejected | 2025-11-10 | N/A |

**Legend:**
- **Status:** ✅ Implemented (merged to main) | ✅ Approved (ready for implementation) | 🔄 Under Review | 📝 Draft/Proposed | ❌ Rejected
- **Category:** 🃏 Cards | 💰 Economy | 🎲 Roguelite | 🗺️ Map/Events | 🧪 Substances | ⚔️ Combat | 🎨 UI | 🛠️ Dev Tools

---

## RFC Template

```markdown
# RFC-NNN: [Feature Name]

## Status

**[Draft / Under Review / Approved]** - YYYY-MM-DD

## Feature Request

### Player Need

From player perspective: **[One sentence summary]** - [Describe the problem from player's viewpoint]

**Current Problem:**
Without [feature]:
- [Specific pain point 1]
- [Specific pain point 2]
- [Specific pain point 3]

**We need a system that:**
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

### Desired Experience

Players should experience:
- **[Aspect 1]:** [Description]
- **[Aspect 2]:** [Description]
- **[Aspect 3]:** [Description]

### Specification Requirements

**[Feature Component 1]:**
- [Specific requirement]
- [Specific requirement]

**[Feature Component 2]:**
- [Specific requirement]
- [Specific requirement]

### MVP Scope

**Phase 1 includes:**
- [What's in scope]

**Phase 1 excludes:**
- [What's deferred]

### Priority Justification

**[HIGH / MEDIUM / LOW] PRIORITY** - [One line reason]

**Why [priority]:**
- [Justification 1]
- [Justification 2]

**Benefits:**
- [Benefit 1]
- [Benefit 2]

---

## Feasibility Analysis

### Technical Assessment

**Proposed Solution: [Solution Name]**

#### Core Mechanism

[Describe how it works - formulas, data flow, key components]

#### Performance Projections

[Overhead estimates, development time]

#### Technical Risks

**1. [Risk Name]**
- *Risk:* [Description]
- *Mitigation:* [How to handle]
- *Impact:* [Severity assessment]

### System Integration

**Affected Systems:**
- [System 1]
- [System 2]

**Compatibility:**
- ✅ [Integration point 1]
- ✅ [Integration point 2]

### Alternatives Considered

#### Alternative 1: [Name]

[Description]

**Rejected because:**
- [Reason 1]
- [Reason 2]

---

## Discussion

### ARCHITECT Notes

[Key architectural insights, extensibility, technical observations]

### PLAYER Validation

[How this meets player needs, success criteria from spec]

---

## Approval

**Status:** [Approved / Needs Changes / Rejected]

**Approvers:**
- ARCHITECT: [✅/❌] [Comments]
- PLAYER: [✅/❌] [Comments]

**Scope Constraint:** Fits in one SOW ([X] hours/days)

**Dependencies:**
- [Dependency 1]
- [Dependency 2]

**Next Steps:**
1. [Action 1]
2. [Action 2]

**Date:** YYYY-MM-DD
```

---

## RFC Lifecycle

### 1. Creation (Draft Status)

**Who:** PLAYER role identifies a feature need

**Output:** `01-rfc/NNN-[feature].md` with Status: Draft

**Contains:**
- Feature Request section (player perspective)
- Specification Requirements (what we want)
- Priority Justification (why now)

### 2. Feasibility Analysis (Draft → Under Review)

**Who:** ARCHITECT role evaluates technical feasibility

**Adds to RFC:**
- Technical Assessment (can we build this?)
- System Integration (how does it fit?)
- Alternatives Considered (what else did we explore?)
- Risks and unknowns

**Output:** RFC updated with Status: Under Review

### 3. Discussion and Iteration (Under Review)

**Who:** PLAYER and ARCHITECT collaborate

**Happens in:** RFC's Discussion section

**Process:**
- PLAYER raises concerns about player experience
- ARCHITECT proposes solutions or adjustments
- Both refine until consensus reached

**Duration:** As long as needed (multiple rounds possible)

### 4. Approval (Under Review → Approved)

**Who:** Both PLAYER and ARCHITECT must approve

**Approval Criteria:**
- ✅ PLAYER: Solves the player need
- ✅ ARCHITECT: Feasible and maintainable
- ✅ Scope: Fits in one SOW (≤20 hours)
- ✅ No unresolved conflicts

**Output:** RFC updated with Status: Approved, frozen from further changes

### 5. ADR Extraction (If Applicable)

**Who:** ARCHITECT role

**Decision:** Does this RFC contain significant architectural decisions?

**Examples:**
- ✅ **Yes:** Card effect system (scripting vs hardcoded)
- ✅ **Yes:** Deck composition (build-time vs runtime)
- ❌ **No:** Starting card pool (just design choices, no ADR)
- ❌ **No:** Balance tuning (just scope, no ADR)

**If Yes:** Create `02-adr/NNN-[decision].md` documenting the architectural choice

**If No:** Proceed directly to SOW creation

### 6. SOW Creation

**Who:** ARCHITECT role

**Output:** `03-sow/NNN-[feature].md` matching RFC number

**Contains:**
- Implementation plan (phases, deliverables)
- Acceptance criteria (how we know it's done)
- References to RFC (and ADR if applicable)

### 7. Lockdown (Approved Status)

**When:** Once approved, RFCs are frozen

**Why:** Preserve historical record of what was decided and why

**Changes After Approval:**
- RFC remains unchanged (locked)
- Implementation deviations documented in SOW Discussion section
- If major changes needed, create new RFC

---

## Writing Tips

### Feature Request Section

- **Start with player pain:** What's frustrating right now?
- **Use player language:** Avoid technical jargon in "Player Need"
- **Be specific:** "No meaningful choices in deck building" not "needs variety"
- **Quantify when possible:** "Only 10 cards available" not "limited options"

### Feasibility Analysis Section

- **Be honest:** If it's hard, say so and explain risks
- **Show your work:** Include formulas, data flow, key decisions
- **Estimate realistically:** Development time, performance overhead
- **Consider alternatives:** Why did we reject other approaches?

### Discussion Section

- **Document iteration:** Show how the design evolved
- **Capture insights:** Key realizations during analysis
- **Note trade-offs:** What did we sacrifice and why?

### Approval Section

- **Scope constraint is critical:** Must fit in one SOW (≤20 hours)
- **Dependencies matter:** What must exist first?
- **Clear next steps:** Who does what next?

---

## Common Patterns

### When to Split an RFC

If your RFC describes multiple independent features, split it:
- ❌ **Too broad:** "Card System" (effects, targeting, deck building, rendering)
- ✅ **Well-scoped:** "Card Effect Pipeline" (just effect resolution)

### When to Defer Features

Use "MVP Scope" section to defer complexity:
- **Phase 1 includes:** Core mechanic (must have)
- **Phase 1 excludes:** Polish, advanced features (nice to have)

### When ADR is Needed

Create ADR when RFC makes architectural choices:
- ✅ **Architecture:** Card effects as data (not hardcoded)
- ✅ **Architecture:** Deck composition validation strategy
- ❌ **Not architecture:** Which cards to implement first
- ❌ **Not architecture:** Starting deck balance numbers

---

## Questions?

- **Where do RFCs come from?** Player pain points, spec gaps, technical debt
- **Who can create RFCs?** Anyone, but PLAYER role formalizes them
- **How long does review take?** As long as needed for consensus
- **Can approved RFCs change?** No - create new RFC instead
- **What if implementation deviates?** Document in SOW Discussion section
