// SOW-013-A Phase 3: Asset loader using direct RON loading at startup

use bevy::prelude::*;
use crate::models::card::Card;
use crate::models::buyer::BuyerPersona;
use super::registry::GameAssets;
use std::fs;

/// Asset loader plugin
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(PreStartup, load_game_assets);
    }
}

/// Load all game assets from RON files at startup
fn load_game_assets(mut game_assets: ResMut<GameAssets>) {
    info!("Loading game assets from RON files...");

    // Load products
    match load_and_validate_cards("assets/cards/products.ron", "Product") {
        Ok(cards) => {
            for card in cards {
                validate_card(&card, "Product").expect("Product validation failed");
                game_assets.products.insert(card.name.clone(), card);
            }
            info!("Loaded {} products", game_assets.products.len());
        }
        Err(e) => {
            error!("Failed to load products.ron: {}", e);
            panic!("Critical asset loading failure - fix products.ron");
        }
    }

    // Load locations
    match load_and_validate_cards("assets/cards/locations.ron", "Location") {
        Ok(cards) => {
            for card in cards {
                validate_card(&card, "Location").expect("Location validation failed");
                game_assets.locations.insert(card.name.clone(), card);
            }
            info!("Loaded {} locations", game_assets.locations.len());
        }
        Err(e) => {
            error!("Failed to load locations.ron: {}", e);
            panic!("Critical asset loading failure - fix locations.ron");
        }
    }

    // Load evidence (Narc deck)
    match load_and_validate_cards("assets/cards/evidence.ron", "Evidence") {
        Ok(cards) => {
            game_assets.evidence = cards;
            info!("Loaded {} evidence cards", game_assets.evidence.len());
        }
        Err(e) => {
            error!("Failed to load evidence.ron: {}", e);
            panic!("Critical asset loading failure - fix evidence.ron");
        }
    }

    // Load cover
    match load_and_validate_cards("assets/cards/cover.ron", "Cover") {
        Ok(cards) => {
            game_assets.cover = cards;
            info!("Loaded {} cover cards", game_assets.cover.len());
        }
        Err(e) => {
            error!("Failed to load cover.ron: {}", e);
            panic!("Critical asset loading failure - fix cover.ron");
        }
    }

    // Load insurance
    match load_and_validate_cards("assets/cards/insurance.ron", "Insurance") {
        Ok(cards) => {
            game_assets.insurance = cards;
            info!("Loaded {} insurance cards", game_assets.insurance.len());
        }
        Err(e) => {
            error!("Failed to load insurance.ron: {}", e);
            panic!("Critical asset loading failure - fix insurance.ron");
        }
    }

    // Load modifiers
    match load_and_validate_cards("assets/cards/modifiers.ron", "Modifier") {
        Ok(cards) => {
            game_assets.modifiers = cards;
            info!("Loaded {} modifier cards", game_assets.modifiers.len());
        }
        Err(e) => {
            error!("Failed to load modifiers.ron: {}", e);
            panic!("Critical asset loading failure - fix modifiers.ron");
        }
    }

    // Load buyers
    match load_and_validate_buyers("assets/buyers.ron") {
        Ok(buyers) => {
            for buyer in &buyers {
                validate_buyer(buyer).expect("Buyer validation failed");
            }
            game_assets.buyers = buyers;
            info!("Loaded {} buyer personas", game_assets.buyers.len());
        }
        Err(e) => {
            error!("Failed to load buyers.ron: {}", e);
            panic!("Critical asset loading failure - fix buyers.ron");
        }
    }

    game_assets.assets_loaded = true;
    info!("All game assets loaded successfully!");
}

/// Load and validate card list from RON file
fn load_and_validate_cards(path: &str, card_type_name: &str) -> Result<Vec<Card>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let cards: Vec<Card> = ron::from_str(&content)
        .map_err(|e| format!("Failed to parse {} - Check RON syntax:\n{}", path, e))?;

    if cards.is_empty() {
        return Err(format!("{} is empty - must have at least one card", path));
    }

    info!("Parsed {} {} cards from {}", cards.len(), card_type_name, path);
    Ok(cards)
}

/// Load and validate buyer personas from RON file
fn load_and_validate_buyers(path: &str) -> Result<Vec<BuyerPersona>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let buyers: Vec<BuyerPersona> = ron::from_str(&content)
        .map_err(|e| format!("Failed to parse {} - Check RON syntax:\n{}", path, e))?;

    if buyers.is_empty() {
        return Err(format!("{} is empty - must have at least one buyer", path));
    }

    info!("Parsed {} buyer personas from {}", buyers.len(), path);
    Ok(buyers)
}

/// Validate a card's values
fn validate_card(card: &Card, expected_type: &str) -> Result<(), String> {
    use crate::models::card::CardType;

    // Validate name
    if card.name.is_empty() {
        return Err(format!("Card id={} has empty name", card.id));
    }

    // Validate card type specific values
    match &card.card_type {
        CardType::Product { price, heat } => {
            if expected_type != "Product" {
                return Err(format!("Card '{}' is Product but loaded from {}", card.name, expected_type));
            }
            if *price == 0 {
                return Err(format!("Product '{}' has price=0 (must be > 0)", card.name));
            }
        }
        CardType::Location { evidence, cover, .. } => {
            if expected_type != "Location" {
                return Err(format!("Card '{}' is Location but loaded from {}", card.name, expected_type));
            }
            if *evidence == 0 && *cover == 0 {
                return Err(format!("Location '{}' has evidence=0 and cover=0 (must have some effect)", card.name));
            }
        }
        _ => {} // Other types validated as needed
    }

    Ok(())
}

/// Validate a buyer persona
fn validate_buyer(buyer: &BuyerPersona) -> Result<(), String> {
    // Validate name
    if buyer.display_name.is_empty() {
        return Err("Buyer has empty display_name".to_string());
    }

    // Validate scenarios
    if buyer.scenarios.is_empty() {
        return Err(format!("Buyer '{}' has no scenarios (must have at least 1)", buyer.display_name));
    }

    // Validate multipliers
    if buyer.base_multiplier <= 0.0 {
        return Err(format!("Buyer '{}' has base_multiplier <= 0", buyer.display_name));
    }

    // Validate reaction deck
    if buyer.reaction_deck.len() != 7 {
        return Err(format!("Buyer '{}' reaction_deck has {} cards (must have exactly 7)",
            buyer.display_name, buyer.reaction_deck.len()));
    }

    Ok(())
}

