# Narrative System - Feature Matrix

**Spec:** [narrative-system.md](narrative-system.md)
**Last Updated:** 2025-11-27
**Overall Status:** 4/5 features complete (80%)

---

## Summary

| Category | Complete | Total | % |
|----------|:--------:|:-----:|:-:|
| Core System | 4 | 4 | 100% |
| Not Implemented | 0 | 1 | 0% |
| **Total** | **4** | **5** | **80%** |

---

## Core System - 4/4 (100%)

| Feature | Status | Notes |
|---------|:------:|-------|
| Narrative Engine | ✅ | StoryComposer with pattern matching |
| Fragment System | ✅ | TaggedFragment with ClauseRelation, GrammaticalStructure |
| Dynamic Sentence Construction | ✅ | SentenceStructure enum, DynamicPattern builder |
| Resolution Overlay Integration | ✅ | hand_story displayed via ResolutionStory component |

---

## Not Implemented - 0/1 (0%)

| Feature | Status | Notes |
|---------|:------:|-------|
| History Log | ❌ | View past stories from run |

---

## Scrapped Features

| Feature | Reason |
|---------|--------|
| Atomic Fragments | Word-level composition unnecessary; phrase-level sufficient |

---

## Implementation Notes

- Stories generated at hand resolution (`game_loop.rs`, `input.rs`)
- Fragments sourced from: Buyer scenario → Card data → Default fallbacks
- 6 sentence structure types support varied output
- Comprehensive test coverage in `story_test.rs`
