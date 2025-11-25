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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_helpers::*;

    #[test]
    fn test_validate_deck_missing_product() {
        let deck = vec![
            create_location("Safe House", 10, 30, -5),
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Product card");
    }

    #[test]
    fn test_validate_deck_missing_location() {
        let deck = vec![
            create_product("Weed", 30, 5),
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Location card");
    }

    #[test]
    fn test_validate_deck_valid() {
        let deck = vec![
            create_product("Weed", 30, 5),
            create_location("Safe House", 10, 30, -5),
        ];
        let result = validate_deck(&deck);
        assert!(result.is_ok());
    }
}