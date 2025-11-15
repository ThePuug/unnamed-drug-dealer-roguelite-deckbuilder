// UI Module
// SOW-011-A Phase 1: Modular UI organization
// SOW-011-A Phase 2: Reusable card display helpers
// SOW-011-A Phase 4: Active slot and heat bar systems
// SOW-AAA: UI setup functions

pub mod theme;
pub mod components;
pub mod helpers;
pub mod systems;
pub mod setup;

// Re-exports for convenience
pub use helpers::*;
pub use systems::*;
