// SOW-AAA Phase 2/3: Buyer system models
// Extracted from main.rs (originally lines 2379-2420)

use super::card::Card;

/// Buyer demand specification - what Products/Locations satisfy this Buyer
#[derive(Debug, Clone)]
pub struct BuyerDemand {
    pub products: Vec<String>,      // e.g., ["Pills", "Weed"] - product names that satisfy demand
    pub locations: Vec<String>,     // e.g., ["Private Residence", "Warehouse"] - location names that satisfy demand
    pub description: String,        // Human-readable description for UI
}

/// SOW-010: Buyer scenario - specific motivation/context for this deal
#[derive(Debug, Clone)]
pub struct BuyerScenario {
    pub display_name: String,                // "Get Wild", "Rock Bottom"
    pub products: Vec<String>,               // ["Weed", "Coke"] - at least one required (OR logic)
    pub locations: Vec<String>,              // ["Frat House", "Locker Room"] - preferred locations (OR logic)
    pub heat_threshold: Option<u32>,         // Scenario-specific threshold (overrides persona default)
    pub description: String,                 // "Chaotic party energy, maximum wildness"
}

/// Buyer persona - merges Dealer scenario deck + Customer modifiers into one entity
#[derive(Debug, Clone)]
pub struct BuyerPersona {
    pub display_name: String,                // "Frat Bro", "Desperate Housewife"
    pub demand: BuyerDemand,                 // SOW-010: Deprecated - scenarios define demands now
    pub base_multiplier: f32,                // ×1.0 to ×3.0 range (when demand met)
    pub reduced_multiplier: f32,             // When demand not met (typically ×1.0)
    pub heat_threshold: Option<u32>,         // SOW-010: Deprecated - scenarios have own thresholds
    pub evidence_threshold: Option<u32>,     // Buyer bails if Evidence exceeds (None = never bails)
    pub reaction_deck: Vec<Card>,            // 7 cards unique to this persona
    pub scenarios: Vec<BuyerScenario>,       // SOW-010: 2 scenarios per Buyer
    pub active_scenario_index: Option<usize>, // Which scenario is active (set during Buyer selection)
}
