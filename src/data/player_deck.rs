// SOW-013-B Phase 2: Player deck from assets
// Loads cards from GameAssets registry

use crate::models::card::Card;
use crate::assets::GameAssets;

/// SOW-013-B: Create Player available cards from loaded assets (24 cards total for deck building)
pub fn create_player_deck(assets: &GameAssets) -> Vec<Card> {
    let mut deck = Vec::new();

    // 9 Products
    deck.push(assets.products.get("Weed").expect("Weed not found").clone());
    deck.push(assets.products.get("Shrooms").expect("Shrooms not found").clone());
    deck.push(assets.products.get("Codeine").expect("Codeine not found").clone());
    deck.push(assets.products.get("Acid").expect("Acid not found").clone());
    deck.push(assets.products.get("Ecstasy").expect("Ecstasy not found").clone());
    deck.push(assets.products.get("Ice").expect("Ice not found").clone());
    deck.push(assets.products.get("Coke").expect("Coke not found").clone());
    deck.push(assets.products.get("Heroin").expect("Heroin not found").clone());
    deck.push(assets.products.get("Fentanyl").expect("Fentanyl not found").clone());

    // 4 Locations
    deck.push(assets.locations.get("Safe House").expect("Safe House not found").clone());
    deck.push(assets.locations.get("Abandoned Warehouse").expect("Abandoned Warehouse not found").clone());
    deck.push(assets.locations.get("Storage Unit").expect("Storage Unit not found").clone());
    deck.push(assets.locations.get("Dead Drop").expect("Dead Drop not found").clone());

    // 4 Cover cards
    deck.extend(assets.cover.clone());

    // 2 Insurance cards
    deck.extend(assets.insurance.clone());

    // 5 Modifiers (ALL available for deck building)
    deck.extend(assets.modifiers.clone());

    // Total: 9 + 4 + 4 + 2 + 5 = 24 cards
    deck
}
