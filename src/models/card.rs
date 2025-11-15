// SOW-AAA Phase 2: Card data models
// Extracted from main.rs (originally lines 2338-2373, 3408-3416)

use bevy::prelude::Component;

/// Who owns this card (SOW-009: Narc, Player, and Buyer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    Narc,
    Player,
    Buyer,  // SOW-009: Buyer reaction cards
}

/// Card types with their specific values (Extended in SOW-002/003/008)
#[derive(Debug, Clone)]
pub enum CardType {
    Product { price: u32, heat: i32 },
    Location { evidence: u32, cover: u32, heat: i32 },  // SOW-009: Used by both Player and Buyer (override rule)
    Evidence { evidence: u32, heat: i32 },
    Cover { cover: u32, heat: i32 },
    // SOW-002 Phase 4: Deal Modifiers (multiplicative price, additive Evidence/Cover/Heat)
    // SOW-009: Used by both Player and Buyer (price_multiplier defaults to 1.0 for non-price cards)
    DealModifier { price_multiplier: f32, evidence: i32, cover: i32, heat: i32 },
    // SOW-003 Phase 1: Insurance (Cover + bust activation)
    Insurance { cover: u32, cost: u32, heat_penalty: i32 },
    // SOW-003 Phase 2: Conviction (Heat threshold, overrides insurance)
    Conviction { heat_threshold: u32 },
    // SOW-009: DealerLocation removed (merged into Location)
    // SOW-009: DealerModifier removed (merged into DealModifier)
}

/// Card instance
#[derive(Component, Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub card_type: CardType,
    // RFC-010: Tags will be added when implementing scenarios
}

/// Totals calculated from all played cards
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Totals {
    pub evidence: u32,
    pub cover: u32,
    pub heat: i32,
    pub profit: u32,
}
