// SOW-012: Shared test helpers for creating Cards with narrative_fragments field
// Available to all test modules via models::test_helpers

#![cfg(test)]

use super::card::{Card, CardType};
use super::buyer::BuyerPersona;
use crate::assets::GameAssets;
use std::collections::HashMap;

/// Create a Product card for testing
pub fn create_product(name: &str, price: u32, heat: i32) -> Card {
    Card {
        id: format!("test_{}", rand::random::<u32>()),
        name: name.to_string(),
        card_type: CardType::Product { price, heat },
        narrative_fragments: None,
    }
}

/// Create a Location card for testing
pub fn create_location(name: &str, evidence: u32, cover: u32, heat: i32) -> Card {
    Card {
        id: format!("test_{}", rand::random::<u32>()),
        name: name.to_string(),
        card_type: CardType::Location { evidence, cover, heat },
        narrative_fragments: None,
    }
}

/// Create an Evidence card for testing
pub fn create_evidence(name: &str, evidence: u32, heat: i32) -> Card {
    Card {
        id: format!("test_{}", rand::random::<u32>()),
        name: name.to_string(),
        card_type: CardType::Evidence { evidence, heat },
        narrative_fragments: None,
    }
}

/// Create a Cover card for testing
pub fn create_cover(name: &str, cover: u32, heat: i32) -> Card {
    Card {
        id: format!("test_{}", rand::random::<u32>()),
        name: name.to_string(),
        card_type: CardType::Cover { cover, heat },
        narrative_fragments: None,
    }
}

/// Create a DealModifier card for testing
pub fn create_deal_modifier(name: &str, price_multiplier: f32, evidence: i32, cover: i32, heat: i32) -> Card {
    Card {
        id: format!("test_{}", rand::random::<u32>()),
        name: name.to_string(),
        card_type: CardType::DealModifier { price_multiplier, evidence, cover, heat },
        narrative_fragments: None,
    }
}

/// Create an Insurance card for testing
pub fn create_insurance(name: &str, cover: u32, cost: u32, heat_penalty: i32) -> Card {
    Card {
        id: format!("test_{}", rand::random::<u32>()),
        name: name.to_string(),
        card_type: CardType::Insurance { cover, cost, heat_penalty },
        narrative_fragments: None,
    }
}

/// Create a Conviction card for testing
pub fn create_conviction(name: &str, heat_threshold: u32) -> Card {
    Card {
        id: format!("test_{}", rand::random::<u32>()),
        name: name.to_string(),
        card_type: CardType::Conviction { heat_threshold },
        narrative_fragments: None,
    }
}

/// Create mock GameAssets for testing (minimal but valid)
pub fn create_mock_game_assets() -> GameAssets {
    let mut assets = GameAssets::default();

    // Add all 9 products (player deck needs them)
    for (name, price, heat) in [
        ("Weed", 30, 5),
        ("Shrooms", 40, 8),
        ("Codeine", 50, 5),
        ("Acid", 60, 10),
        ("Ecstasy", 70, 12),
        ("Ice", 80, 15),
        ("Coke", 80, 15),
        ("Heroin", 90, 18),
        ("Fentanyl", 100, 20),
    ] {
        let card = create_product(name, price, heat);
        assets.products.insert(card.name.clone(), card);
    }

    // Add all 4 locations (player deck needs them)
    for (name, evidence, cover, heat) in [
        ("Safe House", 10, 30, -5),
        ("Abandoned Warehouse", 15, 25, -10),
        ("Storage Unit", 12, 28, -8),
        ("Dead Drop", 8, 20, -5),
    ] {
        let card = create_location(name, evidence, cover, heat);
        assets.locations.insert(card.name.clone(), card);
    }

    // Add evidence/conviction cards (Narc deck)
    assets.evidence = vec![
        create_evidence("Donut Break", 0, 0),
        create_evidence("Patrol", 5, 5),
        create_conviction("Warrant", 10),
    ];

    // Add 4 cover cards (player deck needs them)
    assets.cover = vec![
        create_cover("Alibi", 30, -5),
        create_cover("Bribe", 25, 10),
        create_cover("Fake Receipts", 20, 5),
        create_cover("Bribed Witness", 15, -10),
    ];

    // Add 2 insurance cards (player deck needs them)
    assets.insurance = vec![
        create_insurance("Plea Bargain", 20, 1000, 20),
        create_insurance("Fake ID", 15, 0, 40),
    ];

    // Add 5 modifier cards (player deck needs them - match preset names)
    assets.modifiers = vec![
        create_deal_modifier("Disguise", 1.0, 0, 10, -5),
        create_deal_modifier("Burner Phone", 1.0, 0, 10, -5),
        create_deal_modifier("Lookout", 1.0, -10, 15, 0),
        create_deal_modifier("Clean Money", 1.0, 5, 5, 0),
        create_deal_modifier("False Trail", 1.0, 5, 0, -5),
    ];

    // Add a basic buyer persona
    let buyer = create_mock_buyer_persona();
    assets.buyers = vec![buyer];

    assets.assets_loaded = true;
    assets
}

/// Create a mock buyer persona for testing
fn create_mock_buyer_persona() -> BuyerPersona {
    use super::buyer::{BuyerDemand, BuyerScenario};

    BuyerPersona {
        display_name: "Test Buyer".to_string(),
        demand: BuyerDemand {
            products: vec!["Weed".to_string()],
            locations: vec!["Safe House".to_string()],
            description: "Test buyer".to_string(),
        },
        base_multiplier: 1.0,
        reduced_multiplier: 0.5,
        heat_threshold: Some(50),
        evidence_threshold: None,
        reaction_deck_ids: vec![], // Empty for mock - not used in tests
        reaction_deck: vec![
            create_deal_modifier("Test Modifier 1", 1.0, 10, 5, 5),
            create_deal_modifier("Test Modifier 2", 1.0, 5, 10, 0),
            create_location("Test Location 1", 5, 20, -5),
            create_location("Test Location 2", 10, 15, 5),
            create_deal_modifier("Test Modifier 3", 1.5, 15, 0, 10),
            create_deal_modifier("Test Modifier 4", 0.8, 5, 15, -5),
            create_deal_modifier("Test Modifier 5", 1.0, 5, 0, 20),
        ],
        scenarios: vec![
            BuyerScenario {
                display_name: "Test Scenario".to_string(),
                products: vec!["Weed".to_string()],
                locations: vec!["Safe House".to_string()],
                heat_threshold: Some(40),
                description: "Test scenario".to_string(),
                narrative_fragments: None,
            },
        ],
        active_scenario_index: Some(0),
    }
}
