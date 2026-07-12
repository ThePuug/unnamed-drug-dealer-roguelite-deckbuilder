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

    /// SOW-031 review fix: re-sync the pool against CURRENT ownership
    /// without resetting the player's selection. Mid-hub unlocks (cash
    /// buys, fronts) must be playable immediately - a fronted card that
    /// waits for the next go-home rebuild burns a window tick before it
    /// can earn. The retain covers the other direction: a repossessed
    /// card leaves the selection too.
    pub fn resync_available(
        &mut self,
        assets: &crate::assets::GameAssets,
        unlocked_cards: &HashSet<String>,
    ) {
        self.available_cards = create_player_deck_filtered(assets, unlocked_cards);
        self.selected_cards.retain(|c| unlocked_cards.contains(&c.id));
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
    fn resync_adds_new_unlocks_and_evicts_repossessions() {
        use crate::models::test_helpers::create_mock_game_assets;
        let assets = create_mock_game_assets();

        let unlocked = AccountState::starting_collection();
        let mut builder = DeckBuilder::from_assets_filtered(&assets, &unlocked);
        let full: HashSet<String> = builder_all_ids(&assets);

        // Grow: a mid-hub unlock (buy or front) appears in the pool NOW,
        // and the existing selection is untouched
        let selected_before = builder.selected_cards.len();
        builder.resync_available(&assets, &full);
        assert!(builder.available_cards.len() > selected_before);
        assert_eq!(builder.selected_cards.len(), selected_before);

        // Shrink: repossess one selected card - it leaves BOTH lists
        let victim = builder.selected_cards[0].id.clone();
        let mut without: HashSet<String> = full.clone();
        without.remove(&victim);
        builder.resync_available(&assets, &without);
        assert!(builder.available_cards.iter().all(|c| c.id != victim));
        assert!(builder.selected_cards.iter().all(|c| c.id != victim));
    }

    fn builder_all_ids(assets: &crate::assets::GameAssets) -> HashSet<String> {
        DeckBuilder::from_assets_filtered(
            assets,
            &assets
                .products
                .values()
                .map(|c| c.id.clone())
                .chain(assets.locations.values().map(|c| c.id.clone()))
                .chain(assets.cover.iter().map(|c| c.id.clone()))
                .chain(assets.insurance.iter().map(|c| c.id.clone()))
                .chain(assets.modifiers.iter().map(|c| c.id.clone()))
                .collect(),
        )
        .available_cards
        .iter()
        .map(|c| c.id.clone())
        .collect()
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