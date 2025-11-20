// SOW-012: Narrative Generation System
// Modular structure for story composition from card combinations

pub mod fragments;
pub mod patterns;
pub mod composer;
pub mod builder;

#[cfg(test)]
mod story_test; // Comprehensive story generation test

// Re-export key types for external use
pub use fragments::NarrativeFragments;
pub use composer::StoryComposer;
