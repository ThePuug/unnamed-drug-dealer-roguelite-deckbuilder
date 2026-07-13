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
pub mod foil_material;
pub mod view; // SOW-022: pure view-model logic for the gameplay screen
pub mod map_view; // SOW-029: pure view-model logic for the city map overlay
pub mod ledger_view; // SOW-030: pure view-model logic for the kingpin ledger
pub mod front_view; // SOW-031: pure view-model logic for supplier fronts
pub mod stock_view; // SOW-034: pure view-model logic for consumable product stock

// Re-exports for convenience
pub use helpers::*;
pub use systems::*;
pub use foil_material::FoilMaterialPlugin;
