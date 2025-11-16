// SOW-012: Narrative Generation System
// Modular structure for story composition from card combinations

pub mod fragments;
pub mod patterns;
pub mod composer;

// Re-export key types for external use
pub use fragments::NarrativeFragments;
pub use patterns::{StoryPattern, PatternType, NarrativeRole};
pub use composer::StoryComposer;
