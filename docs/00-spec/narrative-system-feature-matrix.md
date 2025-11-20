# Narrative System Feature Matrix

| Feature | Status | RFC | SOW | Notes |
|---------|--------|-----|-----|-------|
| **Core Narrative Engine** | In Progress | [RFC-012](../01-rfc/012-narrative-generation-system.md) | [SOW-012](../03-sow/012-narrative-generation-system.md) | Basic pattern matching and composition |
| **Fragment System** | In Progress | [RFC-012](../01-rfc/012-narrative-generation-system.md) | [SOW-012](../03-sow/012-narrative-generation-system.md) | Phrasal fragments implemented |
| **Dynamic Sentence Construction** | Implemented | [RFC-014](../01-rfc/014-dynamic-narrative-construction.md) | [SOW-014](../03-sow/014-dynamic-narrative-construction.md) | Dynamic structure generation (no hardcoded variants), implemented in `builder.rs`, `patterns.rs` |
| **Resolution Overlay Integration** | Planned | [RFC-011](../01-rfc/011-ui-refactor.md) | - | Display story in UI |
| **History Log** | Future | - | - | View past stories |
| **Atomic Fragments** | Future | - | - | Word-level composition |
