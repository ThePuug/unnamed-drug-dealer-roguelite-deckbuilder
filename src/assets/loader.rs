// SOW-013-A Phase 3: Asset loader using direct RON loading at startup

use bevy::prelude::*;
use crate::models::card::Card;
use crate::models::buyer::BuyerPersona;
use crate::game_state::GameState;
use super::registry::GameAssets;
use std::fs;
use std::collections::HashMap;

/// Asset loader plugin
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(OnEnter(GameState::AssetLoading), load_game_assets)
            .add_systems(Update, check_assets_and_transition.run_if(in_state(GameState::AssetLoading)));
    }
}

/// Load all game assets from RON files at startup
fn load_game_assets(mut commands: Commands, asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAssets>) {
    info!("Loading game assets from RON files...");

    // Load narrative defaults first (includes resolution clauses)
    match load_narrative_defaults("assets/narrative_defaults.ron") {
        Ok(defaults) => {
            game_assets.narrative_defaults = defaults;
            info!("Loaded narrative defaults (with resolution clauses)");
        }
        Err(e) => {
            warn!("Failed to load narrative_defaults.ron: {} - using empty defaults", e);
        }
    }

    // Load products (no default merging - StoryComposer handles fallback)
    match load_and_validate_cards("assets/cards/products.ron", "Product") {
        Ok(cards) => {
            for card in &cards {
                validate_card(&card, "Product").expect("Product validation failed");

                // Debug: Check ALL products
                if let Some(frags) = &card.narrative_fragments {
                    eprintln!("DEBUG: {} has {} product_clauses", card.name, frags.product_clauses.len());
                } else {
                    eprintln!("DEBUG: {} narrative_fragments is NONE!", card.name);
                }

                game_assets.products.insert(card.name.clone(), card.clone());
            }
            info!("Loaded {} products", game_assets.products.len());
        }
        Err(e) => {
            error!("Failed to load products.ron: {}", e);
            panic!("Critical asset loading failure - fix products.ron");
        }
    }

    // Load locations (no default merging - StoryComposer handles fallback)
    match load_and_validate_cards("assets/cards/locations.ron", "Location") {
        Ok(cards) => {
            for card in &cards {
                validate_card(&card, "Location").expect("Location validation failed");
                game_assets.locations.insert(card.name.clone(), card.clone());
            }
            info!("Loaded {} locations", game_assets.locations.len());
        }
        Err(e) => {
            error!("Failed to load locations.ron: {}", e);
            panic!("Critical asset loading failure - fix locations.ron");
        }
    }

    // Load background images for locations
    load_background_images(&asset_server, &mut game_assets);

    // Load actor portraits
    load_actor_portraits(&asset_server, &mut game_assets);

    // Load card template
    game_assets.card_template = asset_server.load("art/card-template.png");
    info!("Loading card template image");

    // Load card placeholder
    game_assets.card_placeholder = asset_server.load("art/card-placeholder.png");
    info!("Loading card placeholder image");

    // Load card back
    game_assets.card_back = asset_server.load("art/card-back.png");
    info!("Loading card back image");

    // Load evidence card definitions
    let mut evidence_defs = HashMap::new();
    match load_and_validate_cards("assets/cards/evidence.ron", "Evidence") {
        Ok(cards) => {
            for card in cards {
                evidence_defs.insert(card.id.clone(), card);
            }
            info!("Loaded {} evidence card definitions", evidence_defs.len());
        }
        Err(e) => {
            error!("Failed to load evidence.ron: {}", e);
            panic!("Critical asset loading failure - fix evidence.ron");
        }
    }

    // Load conviction card definitions
    let mut conviction_defs = HashMap::new();
    match load_and_validate_cards("assets/cards/convictions.ron", "Conviction") {
        Ok(cards) => {
            for card in cards {
                conviction_defs.insert(card.id.clone(), card);
            }
            info!("Loaded {} conviction card definitions", conviction_defs.len());
        }
        Err(e) => {
            error!("Failed to load convictions.ron: {}", e);
            panic!("Critical asset loading failure - fix convictions.ron");
        }
    }

    // Build Narc deck from composition file
    match load_deck_composition("assets/narc_deck.ron") {
        Ok(ids) => {
            let mut narc_deck = Vec::new();
            for id in ids {
                // Try evidence first, then conviction
                if let Some(card) = evidence_defs.get(&id) {
                    narc_deck.push(card.clone());
                } else if let Some(card) = conviction_defs.get(&id) {
                    narc_deck.push(card.clone());
                } else {
                    panic!("Narc deck references unknown card ID: {}", id);
                }
            }
            game_assets.evidence = narc_deck;
            info!("Built Narc deck with {} cards from composition", game_assets.evidence.len());
        }
        Err(e) => {
            error!("Failed to load narc_deck.ron: {}", e);
            panic!("Critical asset loading failure - fix narc_deck.ron");
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

    // Load buyers and resolve their reaction deck IDs
    match load_and_validate_buyers("assets/buyers.ron") {
        Ok(mut buyers) => {
            // Resolve reaction_deck_ids to actual cards
            for buyer in &mut buyers {
                buyer.reaction_deck = buyer.reaction_deck_ids.iter()
                    .map(|id| {
                        // Try to find card by ID in locations (search values)
                        game_assets.locations.values()
                            .find(|c| &c.id == id)
                            .cloned()
                            .or_else(|| {
                                // Try modifiers
                                game_assets.modifiers.iter()
                                    .find(|c| &c.id == id)
                                    .cloned()
                            })
                            .unwrap_or_else(|| panic!("Buyer {} reaction_deck references unknown card ID: {}", buyer.display_name, id))
                    })
                    .collect();
            }

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

    // Create StoryComposer resource with full narrative defaults (handles fallback internally)
    let story_composer = crate::models::narrative::StoryComposer::new(game_assets.narrative_defaults.clone());
    commands.insert_resource(story_composer);
    info!("Created StoryComposer resource with defaults");

    game_assets.assets_loaded = true;
    info!("All game assets loaded successfully!");
}

/// Check if assets are loaded and transition to DeckBuilding state
fn check_assets_and_transition(
    game_assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_assets.assets_loaded {
        info!("Assets ready - transitioning to DeckBuilding");
        next_state.set(GameState::DeckBuilding);
    }
}

/// Load deck composition (list of card IDs) from RON file
fn load_deck_composition(path: &str) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let ids: Vec<String> = ron::from_str(&content)
        .map_err(|e| format!("Failed to parse {} - Check RON syntax:\n{}", path, e))?;

    if ids.is_empty() {
        return Err(format!("{} is empty - must have at least one card ID", path));
    }

    Ok(ids)
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
        CardType::Product { price, heat: _ } => {
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


/// Load narrative defaults from RON file
fn load_narrative_defaults(path: &str) -> Result<crate::models::narrative::NarrativeFragments, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    ron::from_str::<crate::models::narrative::NarrativeFragments>(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path, e))
}

/// Load background images for all locations
fn load_background_images(asset_server: &AssetServer, game_assets: &mut GameAssets) {
    // Map of location names to their background image filenames
    let background_files = HashMap::from([
        ("Safe House", "safe_house.png"),
        ("Abandoned Warehouse", "abandoned_warehouse.png"),
        ("Storage Unit", "storage_unit.png"),
        ("Dead Drop", "dead_drop.png"),
        ("Locker Room", "locker_room.png"),
        ("Frat House", "frat_house.png"),
        ("By the Pool", "by_the_pool.png"),
        ("At the Park", "at_the_park.png"),
        ("In a Limo", "in_a_limo.png"),
        ("Parking Lot", "parking_lot.png"),
    ]);

    let count = background_files.len();
    for (location_name, filename) in background_files {
        let path = format!("art/backgrounds/{}", filename);
        let handle = asset_server.load(&path);
        game_assets.background_images.insert(location_name.to_string(), handle);
        info!("Loading background image: {} -> {}", location_name, path);
    }

    info!("Initiated loading of {} background images", count);
}

/// Load actor portrait images
fn load_actor_portraits(asset_server: &AssetServer, game_assets: &mut GameAssets) {
    // Map of actor names to their portrait filenames
    let portrait_files = HashMap::from([
        ("Frat Bro", "frat-bro.png"),
        ("Desperate Housewife", "desperate-housewife.png"),
        ("Wall Street Wolf", "wall-street-wolf.png"),
        ("Narc", "narc.png"),
        ("Barista", "barista.png"),
        ("Displaced Patriot", "displaced-patriot.png"),
        ("Flower Child", "flower-child.png"),
        ("Hells Angel", "hells-angel.png"),
        ("Hippie", "hippie.png"),
        ("Pimp", "pimp.png"),
        ("Pretty Woman", "pretty-woman.png"),
        ("Street Walker", "street-walker.png"),
        ("Widow", "widow.png"),
    ]);

    let mut count = 0;
    for (actor_name, filename) in portrait_files.iter() {
        count += 1;
        let path = format!("art/actors/{}", filename);
        let handle = asset_server.load(&path);
        game_assets.actor_portraits.insert(actor_name.to_string(), handle);
        info!("Loading actor portrait: {} -> {}", actor_name, path);
    }

    info!("Initiated loading of {} actor portraits", count);
}
