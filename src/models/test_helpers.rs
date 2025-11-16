// SOW-012: Shared test helpers for creating Cards with narrative_fragments field
// Available to all test modules via models::test_helpers

#![cfg(test)]

use super::card::{Card, CardType};

/// Create a Product card for testing
pub fn create_product(name: &str, price: u32, heat: i32) -> Card {
    Card {
        id: rand::random(),
        name: name.to_string(),
        card_type: CardType::Product { price, heat },
        narrative_fragments: None,
    }
}

/// Create a Location card for testing
pub fn create_location(name: &str, evidence: u32, cover: u32, heat: i32) -> Card {
    Card {
        id: rand::random(),
        name: name.to_string(),
        card_type: CardType::Location { evidence, cover, heat },
        narrative_fragments: None,
    }
}

/// Create an Evidence card for testing
pub fn create_evidence(name: &str, evidence: u32, heat: i32) -> Card {
    Card {
        id: rand::random(),
        name: name.to_string(),
        card_type: CardType::Evidence { evidence, heat },
        narrative_fragments: None,
    }
}

/// Create a Cover card for testing
pub fn create_cover(name: &str, cover: u32, heat: i32) -> Card {
    Card {
        id: rand::random(),
        name: name.to_string(),
        card_type: CardType::Cover { cover, heat },
        narrative_fragments: None,
    }
}

/// Create a DealModifier card for testing
pub fn create_deal_modifier(name: &str, price_multiplier: f32, evidence: i32, cover: i32, heat: i32) -> Card {
    Card {
        id: rand::random(),
        name: name.to_string(),
        card_type: CardType::DealModifier { price_multiplier, evidence, cover, heat },
        narrative_fragments: None,
    }
}

/// Create an Insurance card for testing
pub fn create_insurance(name: &str, cover: u32, cost: u32, heat_penalty: i32) -> Card {
    Card {
        id: rand::random(),
        name: name.to_string(),
        card_type: CardType::Insurance { cover, cost, heat_penalty },
        narrative_fragments: None,
    }
}

/// Create a Conviction card for testing
pub fn create_conviction(name: &str, heat_threshold: u32) -> Card {
    Card {
        id: rand::random(),
        name: name.to_string(),
        card_type: CardType::Conviction { heat_threshold },
        narrative_fragments: None,
    }
}
