// SOW-AAA Phase 1: Player deck data creator
// Extracted from main.rs (originally line 3696-3853)

use crate::models::card::{Card, CardType};
use crate::models::narrative::NarrativeFragments; // SOW-012: Story fragments
use rand::seq::SliceRandom;

/// SOW-010: Create Player deck (20 cards - Dealer Theme)
pub fn create_player_deck() -> Vec<Card> {
    let mut deck = vec![
        // 9 Products (Budget → Premium, Low Heat → High Heat)
        Card {
            id: 10,
            name: "Weed".to_string(),
            card_type: CardType::Product { price: 30, heat: 5 },
            narrative_fragments: Some(NarrativeFragments {
                product_clauses: vec![
                    "I had the weed".to_string(),
                    "I brought the green".to_string(),
                    "I was holding some bud".to_string(),
                ],
                ..Default::default()
            }),
        },
        Card {
            id: 11,
            name: "Shrooms".to_string(),

            card_type: CardType::Product { price: 40, heat: 8 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 12,
            name: "Codeine".to_string(),
            card_type: CardType::Product { price: 50, heat: 10 },
            narrative_fragments: Some(NarrativeFragments {
                product_clauses: vec![
                    "I had the pills".to_string(),
                    "I brought codeine".to_string(),
                    "I was holding prescription meds".to_string(),
                ],
                ..Default::default()
            }),
        },
        Card {
            id: 13,
            name: "Acid".to_string(),

            card_type: CardType::Product { price: 60, heat: 12 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 14,
            name: "Ecstasy".to_string(),

            card_type: CardType::Product { price: 80, heat: 25 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 15,
            name: "Ice".to_string(),

            card_type: CardType::Product { price: 100, heat: 30 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 16,
            name: "Coke".to_string(),

            card_type: CardType::Product { price: 120, heat: 35 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 17,
            name: "Heroin".to_string(),

            card_type: CardType::Product { price: 150, heat: 45 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 18,
            name: "Fentanyl".to_string(),

            card_type: CardType::Product { price: 200, heat: 50 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        // 4 Locations
        Card {
            id: 19,
            name: "Safe House".to_string(),

            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 20,
            name: "Abandoned Warehouse".to_string(),

            card_type: CardType::Location { evidence: 15, cover: 25, heat: -10 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 21,
            name: "Storage Unit".to_string(),

            card_type: CardType::Location { evidence: 12, cover: 28, heat: -8 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 22,
            name: "Dead Drop".to_string(),

            card_type: CardType::Location { evidence: 8, cover: 20, heat: -5 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        // 4 Cover cards
        Card {
            id: 23,
            name: "Alibi".to_string(),

            card_type: CardType::Cover { cover: 30, heat: -5 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 24,
            name: "Bribe".to_string(),

            card_type: CardType::Cover { cover: 25, heat: 10 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 25,
            name: "Fake Receipts".to_string(),

            card_type: CardType::Cover { cover: 20, heat: 5 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 26,
            name: "Bribed Witness".to_string(),

            card_type: CardType::Cover { cover: 15, heat: -10 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        // 2 Insurance cards
        Card {
            id: 27,
            name: "Plea Bargain".to_string(),

            card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 28,
            name: "Fake ID".to_string(),

            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        // 5 Deal Modifiers (defensive focus)
        Card {
            id: 29,
            name: "Disguise".to_string(),

            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 30,
            name: "Burner Phone".to_string(),

            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 31,
            name: "Lookout".to_string(),

            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 32,
            name: "Clean Money".to_string(),

            card_type: CardType::DealModifier { price_multiplier: 0.9, evidence: 0, cover: 10, heat: -15 }, narrative_fragments: None, // SOW-012 Phase 1
        },
        Card {
            id: 33,
            name: "False Trail".to_string(),

            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: -10, cover: 15, heat: -5 }, narrative_fragments: None, // SOW-012 Phase 1
        },
    ];

    // Shuffle deck for variety
    deck.shuffle(&mut rand::thread_rng());
    deck
}
