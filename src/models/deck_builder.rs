// SOW-AAA Phase 2/4: Deck builder model
// Extracted from main.rs (originally lines 48-85)

use bevy::prelude::Resource;
use super::card::Card;
use crate::data::{validate_deck, create_player_deck, create_default_deck, create_aggro_deck, create_control_deck};

/// Deck builder resource for managing card selection
#[derive(Resource)]
pub struct DeckBuilder {
    pub available_cards: Vec<Card>,  // All 20 player cards
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

    /// SOW-013-B: Initialize from loaded assets
    pub fn from_assets(assets: &crate::assets::GameAssets) -> Self {
        let available = create_player_deck(assets);
        let mut deck_builder = Self {
            available_cards: available,
            selected_cards: Vec::new(),
        };
        deck_builder.load_preset(DeckPreset::Default);
        deck_builder
    }

    pub fn is_valid(&self) -> bool {
        validate_deck(&self.selected_cards).is_ok()
    }

    pub fn load_preset(&mut self, preset: DeckPreset) {
        self.selected_cards = match preset {
            DeckPreset::Default => create_default_deck(&self.available_cards),
            DeckPreset::Aggro => create_aggro_deck(&self.available_cards),
            DeckPreset::Control => create_control_deck(&self.available_cards),
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DeckPreset {
    Default,
    Aggro,
    Control,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_deck_valid() {
        let deck = create_player_deck(&crate::assets::GameAssets::default()); // Default 20-card deck
        assert!(validate_deck(&deck).is_ok());
    }

    #[test]
    fn test_deck_builder_load_presets() {
        let assets = crate::assets::GameAssets::default();
        let mut builder = DeckBuilder::from_assets(&assets);

        // All presets should be valid (actual counts may vary with product expansion)
        builder.load_preset(DeckPreset::Aggro);
        assert!(builder.is_valid());

        builder.load_preset(DeckPreset::Control);
        assert!(builder.is_valid());

        builder.load_preset(DeckPreset::Default);
        assert!(builder.is_valid());
    }
}