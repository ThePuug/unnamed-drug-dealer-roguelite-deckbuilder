# Narrative System Specification

**Last Updated:** 2025-11-27

## Overview

Generates thematic story text from card combinations and hand outcomes. Transforms abstract mechanics into memorable narratives displayed in the resolution overlay.

---

## Architecture

### Components

| Component | Purpose | Location |
|-----------|---------|----------|
| StoryComposer | Main entry point, pattern matching, assembly | `composer.rs` |
| DynamicPattern | Defines sentence structures per context | `patterns.rs` |
| NarrativeFragments | Tagged phrase collections | `fragments.rs` |
| SentenceStructure | Grammar tree for assembly | `builder.rs` |

### Flow

1. Hand resolves → `compose_story_from_hand()` called
2. Pattern matched based on Buyer, cards played, outcome
3. Structure built from pattern template
4. Slots filled with fragments (Buyer → Card → Default fallback)
5. Sentence assembled and finalized (capitalization, punctuation)
6. Story stored in `hand_state.hand_story`, displayed in overlay

---

## Fragment System

### Fragment Types

| Role | Source Priority | Example |
|------|-----------------|---------|
| BuyerSubject | Buyer scenario → Default | "A desperate housewife" |
| BuyerNeed | Buyer scenario → Default | "needed her fix" |
| Product | Product card → Default | "I had the stuff" |
| Location | Location card → Default | "at the park" |
| Evidence | Evidence card → Default | "the cops tapped my lines" |
| Resolution | Outcome-specific defaults | "the deal went smooth" |

### Fragment Tags

**ClauseRelation:** Semantic relationship (And, But, Because, When, Although, etc.)
**GrammaticalStructure:** FullClause, Prepositional, Passive, Gerund

Tags enable intelligent fragment selection based on sentence structure needs.

---

## Sentence Structures

| Structure | Example |
|-----------|---------|
| SubjectPredicate | "[Buyer] [needed something]" |
| Compound | "[clause] but [clause]" |
| Complex | "[main] when [subordinate]" |
| ReversedComplex | "Although [sub], [main]" |
| Concatenated | "[action] [at location]" |
| MultiSentence | "[Sentence 1]. [Sentence 2]." |

Patterns select appropriate structures based on available context (Buyer present, Evidence cards played, outcome type).

---

## Integration

**Input:**
- HandState (cards_played, buyer_persona, outcome)

**Output:**
- Story string stored in `hand_state.hand_story`
- Displayed in resolution overlay via `ResolutionStory` component

**Fallbacks:**
- Missing Buyer fragments → Default fragments
- Missing card fragments → Default fragments
- No matching pattern → Generic catch-all pattern

---

## Design Principles

1. **Never break:** Fallbacks ensure story always generates
2. **Grammatically correct:** Proper capitalization, punctuation, conjunctions
3. **Variety:** Multiple patterns and fragment options prevent repetition
4. **Outcome-appropriate:** Tone matches result (Safe = success, Busted = tension)
