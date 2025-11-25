// SOW-AAA Phase 2/4: Deck builder model
// Extracted from main.rs (originally lines 48-85)

use bevy::prelude::Resource;
use super::card::Card;
use crate::data::{validate_deck, create_player_deck, create_default_deck};

/// Deck builder resource for managing card selection
#[derive(Resource)]
pub struct DeckBuilder {
    pub available_cards: Vec<Card>,  // All player cards
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

    /// SOW-013-B: Initialize from loaded assets with default deck selection
    pub fn from_assets(assets: &crate::assets::GameAssets) -> Self {
        let available = create_player_deck(assets);
        let selected = create_default_deck(&available);
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

    #[test]
    fn test_deck_builder_from_assets() {
        use crate::models::test_helpers::create_mock_game_assets;
        let assets = create_mock_game_assets();
        let builder = DeckBuilder::from_assets(&assets);

        // Should have 24 available cards (9 products + 4 locations + 4 cover + 2 insurance + 5 modifiers)
        assert_eq!(builder.available_cards.len(), 24);

        // from_assets() loads Default preset, so should have 20 selected cards
        assert_eq!(builder.selected_cards.len(), 20);
        assert!(builder.is_valid()); // Should be valid with default preset
    }
}