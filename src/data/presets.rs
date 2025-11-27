// SOW-AAA Phase 1: Deck validation and preset data creators
// Extracted from main.rs (originally line 4151-4289)
// SOW-020: Updated to support dynamic card availability

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

/// Helper to get card from available pool by name (returns Option for SOW-020 flexibility)
fn try_get_card(available: &[Card], name: &str) -> Option<Card> {
    available.iter()
        .find(|c| c.name == name)
        .cloned()
}

/// SOW-020: Create default deck from whatever cards are available
/// Selects a balanced mix based on what's unlocked, up to 20 cards
pub fn create_default_deck_from_available(available: &[Card]) -> Vec<Card> {
    let mut selected = Vec::new();

    // Sort by card type for balanced selection
    let products: Vec<_> = available.iter()
        .filter(|c| matches!(c.card_type, CardType::Product { .. }))
        .cloned()
        .collect();
    let locations: Vec<_> = available.iter()
        .filter(|c| matches!(c.card_type, CardType::Location { .. }))
        .cloned()
        .collect();
    let covers: Vec<_> = available.iter()
        .filter(|c| matches!(c.card_type, CardType::Cover { .. }))
        .cloned()
        .collect();
    let insurances: Vec<_> = available.iter()
        .filter(|c| matches!(c.card_type, CardType::Insurance { .. }))
        .cloned()
        .collect();
    let modifiers: Vec<_> = available.iter()
        .filter(|c| matches!(c.card_type, CardType::DealModifier { .. }))
        .cloned()
        .collect();

    // Target: 20 cards with balanced distribution
    // Products: up to 5
    selected.extend(products.into_iter().take(5));
    // Locations: up to 4
    selected.extend(locations.into_iter().take(4));
    // Cover: up to 4
    selected.extend(covers.into_iter().take(4));
    // Insurance: up to 2
    selected.extend(insurances.into_iter().take(2));
    // Modifiers: up to 5
    selected.extend(modifiers.into_iter().take(5));

    selected
}

/// Create default preset deck (legacy - uses hardcoded names)
/// Used when all cards are available
#[allow(dead_code)]
pub fn create_default_deck(available: &[Card]) -> Vec<Card> {
    // Preferred cards if available
    let preferred = [
        "Weed", "Codeine", "Ecstasy", "Coke", "Fentanyl",
        "Safe House", "Abandoned Warehouse", "Storage Unit", "Dead Drop",
        "Alibi", "Bribe", "Fake Receipts", "Bribed Witness",
        "Plea Bargain", "Fake ID",
        "Disguise", "Burner Phone", "Lookout", "Clean Money", "False Trail",
    ];

    let mut selected: Vec<Card> = preferred.iter()
        .filter_map(|name| try_get_card(available, name))
        .collect();

    // If we don't have enough, fall back to dynamic selection
    if selected.len() < 10 {
        return create_default_deck_from_available(available);
    }

    selected
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