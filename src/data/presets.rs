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

/// Helper to get card from available pool by name
fn get_card(available: &[Card], name: &str) -> Card {
    available.iter()
        .find(|c| c.name == name)
        .expect(&format!("Card '{}' not found in available pool", name))
        .clone()
}

/// Create default preset deck (20 cards: balanced selection from 24-card pool)
pub fn create_default_deck(available: &[Card]) -> Vec<Card> {
    vec![
        // 5 products (mix of tiers, exclude 4 products)
        get_card(available, "Weed"),
        get_card(available, "Codeine"),
        get_card(available, "Ecstasy"),
        get_card(available, "Coke"),
        get_card(available, "Fentanyl"),
        // 4 locations (all of them - dealer safe)
        get_card(available, "Safe House"),
        get_card(available, "Abandoned Warehouse"),
        get_card(available, "Storage Unit"),
        get_card(available, "Dead Drop"),
        // 4 Cover cards (all of them)
        get_card(available, "Alibi"),
        get_card(available, "Bribe"),
        get_card(available, "Fake Receipts"),
        get_card(available, "Bribed Witness"),
        // 2 Insurance (all of them)
        get_card(available, "Plea Bargain"),
        get_card(available, "Fake ID"),
        // 5 modifiers (all of them)
        get_card(available, "Disguise"),
        get_card(available, "Burner Phone"),
        get_card(available, "Lookout"),
        get_card(available, "Clean Money"),
        get_card(available, "False Trail"),
    ]
}

/// Create aggro preset deck (12-14 cards: high-profit, risky, minimal defense)
pub fn create_aggro_deck(available: &[Card]) -> Vec<Card> {
    vec![
        // High-profit products (5)
        get_card(available, "Weed"),
        get_card(available, "Ice"),
        get_card(available, "Heroin"),
        get_card(available, "Coke"),
        get_card(available, "Fentanyl"),
        // Minimal defense - just 1 Cover
        get_card(available, "Alibi"),
        // 2 modifiers (to reach minimum)
        get_card(available, "Disguise"),
        get_card(available, "Lookout"),
    ]
}

/// Create control preset deck (15-18 cards: heavy defense, safe locations)
pub fn create_control_deck(available: &[Card]) -> Vec<Card> {
    vec![
        // Conservative products (2)
        get_card(available, "Weed"),
        get_card(available, "Ice"),
        // Safe locations (all)
        get_card(available, "Safe House"),
        get_card(available, "Abandoned Warehouse"),
        get_card(available, "Storage Unit"),
        get_card(available, "Dead Drop"),
        // All Cover cards (4)
        get_card(available, "Alibi"),
        get_card(available, "Bribe"),
        get_card(available, "Fake Receipts"),
        get_card(available, "Bribed Witness"),
        // All Insurance (2)
        get_card(available, "Plea Bargain"),
        get_card(available, "Fake ID"),
        // All defensive modifiers (5)
        get_card(available, "Disguise"),
        get_card(available, "Burner Phone"),
        get_card(available, "Lookout"),
        get_card(available, "Clean Money"),
        get_card(available, "False Trail"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_deck_missing_product() {
        let deck = vec![
            Card { id: 15, name: "Safe House".to_string(), card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 }, narrative_fragments: None },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Product card");
    }

    #[test]
    fn test_validate_deck_missing_location() {
        let deck = vec![
            Card { id: 10, name: "Weed".to_string(), card_type: CardType::Product { price: 30, heat: 5 }, narrative_fragments: None },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Location card");
    }
}