// SOW-013-B Phase 2: Narc deck from assets
// SOW-027: per-area, per-tier compositions - difficulty IS the deck

use crate::models::card::Card;
use crate::assets::GameAssets;
use rand::prelude::*;

/// SOW-027: Build the narc deck for a run from the authored composition for
/// (station area, dealer heat tier). Load-time validation guarantees every
/// purchasable area has all six tiers; the fallback below is defense in
/// depth, not an expected path.
pub fn create_narc_deck(assets: &GameAssets, area: &str, tier: crate::save::HeatTier) -> Vec<Card> {
    let composition = assets
        .narc_compositions
        .get(area)
        .or_else(|| {
            bevy::log::warn!(
                "no narc compositions for area '{area}' - falling back to {}",
                crate::save::DEFAULT_STATION
            );
            assets.narc_compositions.get(crate::save::DEFAULT_STATION)
        })
        .and_then(|tiers| tiers.get(tier.name()));

    let mut deck = composition.cloned().unwrap_or_default();
    deck.shuffle(&mut rand::rng());
    deck
}
