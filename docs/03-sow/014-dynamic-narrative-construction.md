# SOW-014: Dynamic Narrative Construction

## Status

**In Progress** - 2025-11-19

## References

- **RFC-014:** [Dynamic Narrative Construction](../01-rfc/014-dynamic-narrative-construction.md)
- **Spec:** [Narrative System](../00-spec/narrative-system.md)

## Implementation Plan

### Phase 1: Core Logic (The Builder)

**Goal:** Implement the `SentenceBuilder` that can dynamically assemble a sentence tree.

**Deliverables:**
- `src/models/narrative/builder.rs`: New module.
- `SentenceBuilder` struct.
- `DynamicPattern` struct (replacing the static `StoryPattern` structure).
- Unit tests verifying dynamic assembly.

**Architectural Constraints:**
- Must use existing `SentenceStructure` enum (Simple, Compound, etc.) as the output format.
- Must support "Core" + "Satellite" architecture.

**Duration:** 4 hours

### Phase 2: Integration

**Goal:** Replace the old pattern matching logic in `StoryComposer` with the new dynamic system.

**Deliverables:**
- Update `src/models/narrative/composer.rs` to use `DynamicPattern`.
- Refactor `src/models/narrative/patterns.rs` to define patterns using the new dynamic syntax.
- Remove hardcoded `SentenceStructure` variants.

**Duration:** 3 hours

### Phase 3: Verification

**Goal:** Ensure stories are generated correctly and vary as expected.

**Deliverables:**
- Run game and verify console output.
- Add regression tests to ensure no "None" or broken sentences.

**Duration:** 1 hour

## Acceptance Criteria

- [x] `StoryComposer` no longer uses hardcoded `variant_a`, `variant_b`, etc.
- [x] Stories include optional elements (Location, Evidence) in varying positions.
- [x] Existing tests pass.
- [x] Stories generate as natural multi-sentence narratives (not run-on sentences)
- [x] Proper conjunction usage based on outcome (and/but)
- [x] Grammatically correct capitalization and punctuation

## Implementation Discussion

The initial implementation from Gemini required significant refinements to produce natural, grammatically correct stories. Key adjustments included: (1) Converting from single run-on sentences to multi-sentence narratives (sentence 1: subject+need, sentence 2: product+resolution), (2) Context-aware conjunctions where positive outcomes use "and" and negative outcomes use "but", (3) Proper capitalization for prepended clauses (lowercase after commas/subordinators), and (4) Semantic clarity by renaming "complication_clauses" to "evidence_clauses" since only Evidence cards (not Convictions/Insurance) generate these narrative elements. The system now generates 666K+ unique grammatically correct stories from test variations with zero fallback text usage.

During implementation, we also refactored the asset system to improve authoring: Card IDs were converted from numeric (10, 19, 2000) to semantic snake_case strings ("weed", "safe_house", "prior_conviction"). This makes assets more readable and maintainable. Evidence/Conviction narrative fragments were cleaned up - Conviction cards now have no evidence_clauses since they will use different narrative handling in future iterations.
