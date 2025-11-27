// SOW-AAA Phase 2/4: Deck builder model
// Extracted from main.rs (originally lines 48-85)
// SOW-020: Filter by unlocked cards from AccountState

use bevy::prelude::Resource;
use super::card::Card;
use crate::data::{validate_deck, create_player_deck_filtered, create_default_deck_from_available};
use std::collections::HashSet;

/// Deck builder resource for managing card selection
#[derive(Resource)]
pub struct DeckBuilder {
    pub available_cards: Vec<Card>,  // Unlocked player cards
    pub selected_cards: Vec<Card>,   // Chosen cards (10-20)
}

impl DeckBuilder {
    pub fn new() -> Self {
        // SOW-013-B: Empty until populated from assets
        Self {
            available_cards: Vec::new(),
            selected_cards: Vec::new(),
        }
    }

    /// SOW-020: Initialize from loaded assets filtered by unlocked cards
    pub fn from_assets_filtered(assets: &crate::assets::GameAssets, unlocked_cards: &HashSet<String>) -> Self {
        let available = create_player_deck_filtered(assets, unlocked_cards);
        let selected = create_default_deck_from_available(&available);
        Self {
            available_cards: available,
            selected_cards: selected,
        }
    }

    pub fn is_valid(&self) -> bool {
        validate_deck(&self.selected_cards).is_ok()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::AccountState;

    #[test]
    fn test_deck_builder_from_assets_filtered() {
        use crate::models::test_helpers::create_mock_game_assets;
        let assets = create_mock_game_assets();

        // Use starting collection (11 cards unlocked by default)
        let unlocked = AccountState::starting_collection();
        let builder = DeckBuilder::from_assets_filtered(&assets, &unlocked);

        // Starting collection has 11 cards (3 products + 3 locations + 2 cover + 1 insurance + 2 modifiers)
        // But mock assets may not have all these, so just check it's not empty and valid
        assert!(!builder.available_cards.is_empty(), "Should have some available cards");
        assert!(!builder.selected_cards.is_empty(), "Should have some selected cards");
        assert!(builder.is_valid(), "Default deck should be valid");
    }

    #[test]
    fn test_deck_builder_with_all_unlocked() {
        use crate::models::test_helpers::create_mock_game_assets;
        let assets = create_mock_game_assets();

        // Unlock all cards by their IDs
        let mut all_unlocked = HashSet::new();
        for card in assets.products.values() {
            if card.shop_location.is_some() {
                all_unlocked.insert(card.id.clone());
            }
        }
        for card in assets.locations.values() {
            if card.shop_location.is_some() {
                all_unlocked.insert(card.id.clone());
            }
        }
        for card in &assets.cover {
            if card.shop_location.is_some() {
                all_unlocked.insert(card.id.clone());
            }
        }
        for card in &assets.insurance {
            if card.shop_location.is_some() {
                all_unlocked.insert(card.id.clone());
            }
        }
        for card in &assets.modifiers {
            if card.shop_location.is_some() {
                all_unlocked.insert(card.id.clone());
            }
        }

        let builder = DeckBuilder::from_assets_filtered(&assets, &all_unlocked);

        // With all unlocked, we should have all player-purchasable cards
        assert!(!builder.available_cards.is_empty());
    }
}