// SOW-013-B Phase 2: Player deck from assets
// SOW-020: Filter by unlocked cards from AccountState

use crate::models::card::Card;
use crate::assets::GameAssets;
use std::collections::HashSet;

/// SOW-020: Create Player available cards filtered by unlocked card IDs
/// Only includes cards that:
/// 1. Have a shop_location (are player-purchasable cards, not buyer-only)
/// 2. Are in the unlocked_cards set
pub fn create_player_deck_filtered(assets: &GameAssets, unlocked_cards: &HashSet<String>) -> Vec<Card> {
    let mut deck = Vec::new();

    // Products - filter by shop_location presence and unlock status
    for card in assets.products.values() {
        if card.shop_location.is_some() && unlocked_cards.contains(&card.id) {
            deck.push(card.clone());
        }
    }

    // Locations - only player locations (have shop_location), filtered by unlock
    for card in assets.locations.values() {
        if card.shop_location.is_some() && unlocked_cards.contains(&card.id) {
            deck.push(card.clone());
        }
    }

    // Cover cards - filter by shop_location and unlock
    for card in &assets.cover {
        if card.shop_location.is_some() && unlocked_cards.contains(&card.id) {
            deck.push(card.clone());
        }
    }

    // Insurance cards - filter by shop_location and unlock
    for card in &assets.insurance {
        if card.shop_location.is_some() && unlocked_cards.contains(&card.id) {
            deck.push(card.clone());
        }
    }

    // Modifiers - only player modifiers (have shop_location), filtered by unlock
    for card in &assets.modifiers {
        if card.shop_location.is_some() && unlocked_cards.contains(&card.id) {
            deck.push(card.clone());
        }
    }

    deck
}

/// SOW-013-B: Create Player available cards from loaded assets (legacy, all cards)
/// Used for HandState initialization (full card pool for gameplay)
pub fn create_player_deck(assets: &GameAssets) -> Vec<Card> {
    let mut deck = Vec::new();

    // All products with shop_location (player-purchasable)
    for card in assets.products.values() {
        if card.shop_location.is_some() {
            deck.push(card.clone());
        }
    }

    // All player locations (have shop_location)
    for card in assets.locations.values() {
        if card.shop_location.is_some() {
            deck.push(card.clone());
        }
    }

    // All cover cards with shop_location
    for card in &assets.cover {
        if card.shop_location.is_some() {
            deck.push(card.clone());
        }
    }

    // All insurance cards with shop_location
    for card in &assets.insurance {
        if card.shop_location.is_some() {
            deck.push(card.clone());
        }
    }

    // All player modifiers (have shop_location)
    for card in &assets.modifiers {
        if card.shop_location.is_some() {
            deck.push(card.clone());
        }
    }

    deck
}
