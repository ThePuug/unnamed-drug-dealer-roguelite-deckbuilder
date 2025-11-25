// SOW-013-B Phase 2: Narc deck from assets
// Loads cards from GameAssets registry

use crate::models::card::Card;
use crate::assets::GameAssets;
use rand::prelude::*;

/// SOW-013-B: Create Narc deck from loaded assets (25 cards - Law Enforcement Theme)
pub fn create_narc_deck(assets: &GameAssets) -> Vec<Card> {
    let mut deck = assets.evidence.clone();

    // Shuffle deck for variety
    deck.shuffle(&mut rand::rng());
    deck
}
