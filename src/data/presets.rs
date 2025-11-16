// SOW-AAA Phase 1: Deck validation and preset data creators
// Extracted from main.rs (originally line 4151-4289)

use crate::models::card::{Card, CardType};

/// SOW-006: Validate deck meets all constraints
pub fn validate_deck(deck: &[Card]) -> Result<(), String> {
    // Check required card types (gameplay requires at least one of each)
    let has_product = deck.iter().any(|c| matches!(c.card_type, CardType::Product { .. }));
    let has_location = deck.iter().any(|c| matches!(c.card_type, CardType::Location { .. }));

    if !has_product {
        return Err("Need at least 1 Product card".to_string());
    }
    if !has_location {
        return Err("Need at least 1 Location card".to_string());
    }

    Ok(())
}

/// Create default preset deck (20 cards: balanced selection from 24-card pool)
pub fn create_default_deck() -> Vec<Card> {
    vec![
        // 5 products (mix of tiers, exclude 4 products)
        Card { id: 10, name: "Weed".to_string(),            card_type: CardType::Product { price: 30, heat: 5 }, narrative_fragments: None },
        Card { id: 12, name: "Codeine".to_string(),            card_type: CardType::Product { price: 50, heat: 10 }, narrative_fragments: None },
        Card { id: 14, name: "Ecstasy".to_string(),            card_type: CardType::Product { price: 80, heat: 25 }, narrative_fragments: None },
        Card { id: 16, name: "Coke".to_string(),            card_type: CardType::Product { price: 120, heat: 35 }, narrative_fragments: None },
        Card { id: 18, name: "Fentanyl".to_string(),            card_type: CardType::Product { price: 200, heat: 50 }, narrative_fragments: None },
        // 4 locations (all of them - dealer safe)
        Card { id: 19, name: "Safe House".to_string(),            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 }, narrative_fragments: None },
        Card { id: 20, name: "Abandoned Warehouse".to_string(),            card_type: CardType::Location { evidence: 15, cover: 25, heat: -10 }, narrative_fragments: None },
        Card { id: 21, name: "Storage Unit".to_string(),            card_type: CardType::Location { evidence: 12, cover: 28, heat: -8 }, narrative_fragments: None },
        Card { id: 22, name: "Dead Drop".to_string(),            card_type: CardType::Location { evidence: 8, cover: 20, heat: -5 }, narrative_fragments: None },
        // 4 Cover cards (all of them)
        Card { id: 23, name: "Alibi".to_string(),            card_type: CardType::Cover { cover: 30, heat: -5 }, narrative_fragments: None },
        Card { id: 24, name: "Bribe".to_string(),            card_type: CardType::Cover { cover: 25, heat: 10 }, narrative_fragments: None },
        Card { id: 25, name: "Fake Receipts".to_string(),            card_type: CardType::Cover { cover: 20, heat: 5 }, narrative_fragments: None },
        Card { id: 26, name: "Bribed Witness".to_string(),            card_type: CardType::Cover { cover: 15, heat: -10 }, narrative_fragments: None },
        // 2 Insurance (all of them)
        Card { id: 27, name: "Plea Bargain".to_string(),            card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 }, narrative_fragments: None },
        Card { id: 28, name: "Fake ID".to_string(),            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 }, narrative_fragments: None },
        // 5 modifiers (all of them)
        Card { id: 29, name: "Disguise".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 }, narrative_fragments: None },
        Card { id: 30, name: "Burner Phone".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 }, narrative_fragments: None },
        Card { id: 31, name: "Lookout".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 }, narrative_fragments: None },
        Card { id: 32, name: "Clean Money".to_string(),            card_type: CardType::DealModifier { price_multiplier: 0.9, evidence: 0, cover: 10, heat: -15 }, narrative_fragments: None },
        Card { id: 33, name: "False Trail".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: -10, cover: 15, heat: -5 }, narrative_fragments: None },
    ]
}

/// Create aggro preset deck (12-14 cards: high-profit, risky, minimal defense)
pub fn create_aggro_deck() -> Vec<Card> {
    vec![
        // High-profit products (5)
        Card { id: 10, name: "Weed".to_string(),            card_type: CardType::Product { price: 30, heat: 5 }, narrative_fragments: None },
        Card { id: 11, name: "Meth".to_string(),            card_type: CardType::Product { price: 100, heat: 30 }, narrative_fragments: None },
        Card { id: 12, name: "Heroin".to_string(),            card_type: CardType::Product { price: 150, heat: 45 }, narrative_fragments: None },
        Card { id: 13, name: "Cocaine".to_string(),            card_type: CardType::Product { price: 120, heat: 35 }, narrative_fragments: None },
        Card { id: 14, name: "Fentanyl".to_string(),            card_type: CardType::Product { price: 200, heat: 50 }, narrative_fragments: None },
        // Risky locations (2)
        Card { id: 16, name: "School Zone".to_string(),            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 }, narrative_fragments: None },
        Card { id: 18, name: "Back Alley".to_string(),            card_type: CardType::Location { evidence: 25, cover: 20, heat: 0 }, narrative_fragments: None },
        // Minimal defense - just 1 Cover
        Card { id: 19, name: "Alibi".to_string(),            card_type: CardType::Cover { cover: 30, heat: -5 }, narrative_fragments: None },
        // 2 modifiers (to reach minimum 10 cards)
        Card { id: 25, name: "Disguise".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 }, narrative_fragments: None },
        Card { id: 27, name: "Lookout".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 }, narrative_fragments: None },
    ]
}

/// Create control preset deck (15-18 cards: heavy defense, safe locations)
pub fn create_control_deck() -> Vec<Card> {
    vec![
        // Conservative products (2)
        Card { id: 10, name: "Weed".to_string(),            card_type: CardType::Product { price: 30, heat: 5 }, narrative_fragments: None },
        Card { id: 11, name: "Meth".to_string(),            card_type: CardType::Product { price: 100, heat: 30 }, narrative_fragments: None },
        // Safe locations (3)
        Card { id: 15, name: "Safe House".to_string(),            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 }, narrative_fragments: None },
        Card { id: 17, name: "Warehouse".to_string(),            card_type: CardType::Location { evidence: 15, cover: 25, heat: -10 }, narrative_fragments: None },
        Card { id: 18, name: "Back Alley".to_string(),            card_type: CardType::Location { evidence: 25, cover: 20, heat: 0 }, narrative_fragments: None },
        // All Cover cards (4)
        Card { id: 19, name: "Alibi".to_string(),            card_type: CardType::Cover { cover: 30, heat: -5 }, narrative_fragments: None },
        Card { id: 20, name: "Bribe".to_string(),            card_type: CardType::Cover { cover: 25, heat: 10 }, narrative_fragments: None },
        Card { id: 21, name: "Fake Receipts".to_string(),            card_type: CardType::Cover { cover: 20, heat: 5 }, narrative_fragments: None },
        Card { id: 22, name: "Bribed Witness".to_string(),            card_type: CardType::Cover { cover: 15, heat: -10 }, narrative_fragments: None },
        // All Insurance (2)
        Card { id: 23, name: "Plea Bargain".to_string(),            card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 }, narrative_fragments: None },
        Card { id: 24, name: "Fake ID".to_string(),            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 }, narrative_fragments: None },
        // All defensive modifiers (5)
        Card { id: 25, name: "Disguise".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 }, narrative_fragments: None },
        Card { id: 26, name: "Burner Phone".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 }, narrative_fragments: None },
        Card { id: 27, name: "Lookout".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 }, narrative_fragments: None },
        Card { id: 28, name: "Clean Money".to_string(),            card_type: CardType::DealModifier { price_multiplier: 0.9, evidence: 0, cover: 10, heat: -15 }, narrative_fragments: None },
        Card { id: 29, name: "False Trail".to_string(),            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: -10, cover: 15, heat: -5 }, narrative_fragments: None },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_deck_missing_product() {
        let deck = vec![
            Card { id: 15, name: "Safe House".to_string(), card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 }, narrative_fragments: None },
            Card { id: 19, name: "Alibi".to_string(), card_type: CardType::Cover { cover: 30, heat: -5 }, narrative_fragments: None },
            Card { id: 20, name: "Bribe".to_string(), card_type: CardType::Cover { cover: 25, heat: 10 }, narrative_fragments: None },
            Card { id: 21, name: "Fake Receipts".to_string(), card_type: CardType::Cover { cover: 20, heat: 5 }, narrative_fragments: None },
            Card { id: 22, name: "Bribed Witness".to_string(), card_type: CardType::Cover { cover: 15, heat: -10 }, narrative_fragments: None },
            Card { id: 23, name: "Plea Bargain".to_string(), card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 }, narrative_fragments: None },
            Card { id: 24, name: "Fake ID".to_string(), card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 }, narrative_fragments: None },
            Card { id: 25, name: "Disguise".to_string(), card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 }, narrative_fragments: None },
            Card { id: 26, name: "Burner Phone".to_string(), card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 }, narrative_fragments: None },
            Card { id: 27, name: "Lookout".to_string(), card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 }, narrative_fragments: None },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Product card");
    }

    #[test]
    fn test_validate_deck_missing_location() {
        let deck = vec![
            Card { id: 10, name: "Weed".to_string(), card_type: CardType::Product { price: 30, heat: 5 }, narrative_fragments: None },
            Card { id: 11, name: "Meth".to_string(), card_type: CardType::Product { price: 100, heat: 30 }, narrative_fragments: None },
            Card { id: 12, name: "Heroin".to_string(), card_type: CardType::Product { price: 150, heat: 45 }, narrative_fragments: None },
            Card { id: 13, name: "Cocaine".to_string(), card_type: CardType::Product { price: 120, heat: 35 }, narrative_fragments: None },
            Card { id: 14, name: "Fentanyl".to_string(), card_type: CardType::Product { price: 200, heat: 50 }, narrative_fragments: None },
            Card { id: 19, name: "Alibi".to_string(), card_type: CardType::Cover { cover: 30, heat: -5 }, narrative_fragments: None },
            Card { id: 20, name: "Bribe".to_string(), card_type: CardType::Cover { cover: 25, heat: 10 }, narrative_fragments: None },
            Card { id: 21, name: "Fake Receipts".to_string(), card_type: CardType::Cover { cover: 20, heat: 5 }, narrative_fragments: None },
            Card { id: 22, name: "Bribed Witness".to_string(), card_type: CardType::Cover { cover: 15, heat: -10 }, narrative_fragments: None },
            Card { id: 23, name: "Plea Bargain".to_string(), card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 }, narrative_fragments: None },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Location card");
    }
}