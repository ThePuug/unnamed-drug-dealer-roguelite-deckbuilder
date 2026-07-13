// SOW-013-A Phase 3: Asset loader using direct RON loading at startup

use bevy::prelude::*;
use crate::models::card::{Card, CardType};
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

    // SOW-033: actor portraits load AFTER buyers + shop_locations (their
    // `portrait` / `narc_portrait` RON fields drive the map) - see below.

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

    // SOW-027: narc compositions are built AFTER shop locations load (their
    // area keys are validated against the loaded areas) - see below

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

    // SOW-024: Load areas (shop locations) - loaded before buyers so persona
    // area references can be validated against real area ids
    match load_shop_locations("assets/data/shop_locations.ron") {
        Ok(areas) => {
            crate::models::shop_location::validate_shop_locations(&areas)
                .expect("Area validation failed");

            // Every card's shop_location must be a real area id (same
            // fail-loud-in-debug treatment as SOW-021 demand strings)
            let area_ids: Vec<&str> = areas.iter().map(|a| a.id.as_str()).collect();
            for card in game_assets
                .products
                .values()
                .chain(game_assets.locations.values())
                .chain(game_assets.cover.iter())
                .chain(game_assets.insurance.iter())
                .chain(game_assets.modifiers.iter())
            {
                if let Some(loc) = &card.shop_location {
                    if !area_ids.contains(&loc.as_str()) {
                        #[cfg(debug_assertions)]
                        panic!("Card '{}' references unknown area '{}'", card.name, loc);
                        #[cfg(not(debug_assertions))]
                        error!("Card '{}' references unknown area '{}' (unpurchasable)", card.name, loc);
                    }
                }
            }

            info!("Loaded {} areas", areas.len());
            game_assets.shop_locations = areas;
        }
        Err(e) => {
            error!("Failed to load shop_locations.ron: {}", e);
            panic!("Critical asset loading failure - fix shop_locations.ron");
        }
    }

    // SOW-027: Build per-area, per-tier narc deck compositions.
    // Difficulty IS the composition (RFC-018 stat multipliers retired):
    // the run's narc deck = effective[dealer's station][dealer's heat tier].
    // The authored format is SPARSE: a `default` ladder plus per-area tier
    // OVERRIDES - inheritance is resolved here so a new area ships with zero
    // narc authoring and gets the baseline (Reed's authoring-burden concern).
    match load_narc_compositions("assets/narc_deck.ron") {
        Ok(raw) => {
            const TIER_KEYS: [&str; 6] = ["Cold", "Warm", "Hot", "Blazing", "Scorching", "Inferno"];
            let area_ids: Vec<&str> = game_assets.shop_locations.iter().map(|a| a.id.as_str()).collect();

            // Resolve card ids -> cards, panicking with an authoring-friendly location
            let resolve = |ids: &[String], whose: &str| -> Vec<crate::models::card::Card> {
                ids.iter()
                    .map(|id| {
                        evidence_defs
                            .get(id)
                            .or_else(|| conviction_defs.get(id))
                            .unwrap_or_else(|| {
                                panic!("narc_deck.ron: unknown card id '{}' in {}", id, whose)
                            })
                            .clone()
                    })
                    .collect()
            };

            // The default ladder must be complete - it's what new areas inherit
            for tier in TIER_KEYS {
                match raw.default.get(tier) {
                    None => panic!("narc_deck.ron: default ladder is missing tier '{}'", tier),
                    Some(ids) if ids.is_empty() => {
                        panic!("narc_deck.ron: default ladder tier '{}' is empty", tier)
                    }
                    _ => {}
                }
            }

            // Overrides may only name real areas and real tiers
            for (area, overrides) in &raw.areas {
                if !area_ids.contains(&area.as_str()) {
                    panic!("narc_deck.ron: unknown area '{}' (known: {:?})", area, area_ids);
                }
                for (tier, ids) in overrides {
                    if !TIER_KEYS.contains(&tier.as_str()) {
                        panic!("narc_deck.ron: area '{}' overrides unknown tier '{}'", area, tier);
                    }
                    if ids.is_empty() {
                        panic!("narc_deck.ron: area '{}' tier '{}' override is empty", area, tier);
                    }
                }
            }

            // Effective table: every purchasable area x tier, overrides beating default
            let mut compositions: HashMap<String, HashMap<String, Vec<crate::models::card::Card>>> =
                HashMap::new();
            for area in &area_ids {
                let overrides = raw.areas.get(*area);
                let mut tier_map = HashMap::new();
                for tier in TIER_KEYS {
                    let (ids, source) = match overrides.and_then(|o| o.get(tier)) {
                        Some(ids) => (ids, "override"),
                        None => (raw.default.get(tier).expect("validated above"), "default"),
                    };
                    let deck = resolve(ids, &format!("{}/{}", area, tier));
                    // Authors see what actually shipped (compact count summary)
                    #[cfg(debug_assertions)]
                    {
                        let mut counts: std::collections::BTreeMap<&str, u32> =
                            std::collections::BTreeMap::new();
                        for card in &deck {
                            *counts.entry(card.name.as_str()).or_insert(0) += 1;
                        }
                        let summary = counts
                            .iter()
                            .map(|(name, n)| format!("{n}x {name}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        debug!("narc effective {}/{} ({source}): {}", area, tier, summary);
                    }
                    tier_map.insert(tier.to_string(), deck);
                }
                compositions.insert(area.to_string(), tier_map);
            }

            // Authoring hygiene: a defined narc card that appears nowhere is
            // probably a mistake (warn, don't fail - it may be staged content)
            let used: std::collections::HashSet<&String> = raw
                .default
                .values()
                .chain(raw.areas.values().flat_map(|o| o.values()))
                .flatten()
                .collect();
            for id in evidence_defs.keys().chain(conviction_defs.keys()) {
                if !used.contains(id) {
                    warn!("narc card '{}' is defined but used in no composition", id);
                }
            }

            let deck_count: usize = compositions.values().map(|t| t.len()).sum();
            info!("Built {} narc area/tier deck compositions", deck_count);
            game_assets.narc_compositions = compositions;
        }
        Err(e) => {
            error!("Failed to load narc_deck.ron: {}", e);
            panic!("Critical asset loading failure - fix narc_deck.ron");
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

            // SOW-021: Every demand string must resolve to a real card name.
            // Fail loudly in debug; log an error (but keep running) in release.
            let product_names: Vec<&str> = game_assets.products.values().map(|c| c.name.as_str()).collect();
            let location_names: Vec<&str> = game_assets.locations.values().map(|c| c.name.as_str()).collect();
            for buyer in &buyers {
                if let Err(e) = validate_buyer_demand_strings(buyer, &product_names, &location_names) {
                    #[cfg(debug_assertions)]
                    panic!("Demand string validation failed: {}", e);
                    #[cfg(not(debug_assertions))]
                    error!("Demand string validation failed (demand cannot pay out): {}", e);
                }
            }

            // SOW-024: persona areas must be real, and every area must have
            // clientele (a run in an empty area would have no buyer to draw).
            // Same fail-loud-in-debug treatment as demand strings.
            if let Err(e) = validate_persona_areas(&buyers, &game_assets.shop_locations) {
                #[cfg(debug_assertions)]
                panic!("Persona area validation failed: {}", e);
                #[cfg(not(debug_assertions))]
                error!("Persona area validation failed: {}", e);
            }

            game_assets.buyers = buyers;
            info!("Loaded {} buyer personas", game_assets.buyers.len());
        }
        Err(e) => {
            error!("Failed to load buyers.ron: {}", e);
            panic!("Critical asset loading failure - fix buyers.ron");
        }
    }

    // SOW-033: portrait map is built from the loaded buyers' `portrait` and
    // the areas' `narc_portrait` RON fields (plus the dealer pool). Runs here
    // so both are populated; a missing mapped file panics loud.
    load_actor_portraits(&asset_server, &mut game_assets);

    // SOW-026: the lean starting collection must still build a legal deck
    // (>=1 Product, >=1 Location) - fail loudly in debug, error in release.
    let all_player_cards = collect_player_cards(&game_assets);
    if let Err(e) = validate_fresh_collection(&all_player_cards) {
        #[cfg(debug_assertions)]
        panic!("Fresh collection validation failed: {}", e);
        #[cfg(not(debug_assertions))]
        error!("Fresh collection validation failed: {}", e);
    }

    // SOW-026: every scenario must demand at least one product attainable
    // at-or-before its buyer's rung on the area ladder (warn-only - an
    // OR-demand with one reachable product still pays out).
    for warning in ladder_attainability_warnings(
        &game_assets.buyers,
        &all_player_cards,
        &game_assets.shop_locations,
    ) {
        warn!("Shop ladder: {}", warning);
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

/// SOW-027: sparse narc composition file - a complete `default` per-tier
/// ladder plus per-area tier overrides (inheritance resolved by the caller)
#[derive(serde::Deserialize)]
pub struct NarcCompositionsFile {
    pub default: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub areas: HashMap<String, HashMap<String, Vec<String>>>,
}

fn load_narc_compositions(path: &str) -> Result<NarcCompositionsFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    ron::from_str(&content)
        .map_err(|e| format!("Failed to parse {} - Check RON syntax:\n{}", path, e))
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

/// SOW-024: Every persona's area must exist, and every area must have at
/// least one persona (unlocking it must buy access to actual clientele)
fn validate_persona_areas(
    buyers: &[BuyerPersona],
    areas: &[crate::models::shop_location::ShopLocationDef],
) -> Result<(), String> {
    for buyer in buyers {
        if !areas.iter().any(|a| a.id == buyer.area) {
            return Err(format!(
                "persona '{}' lives in unknown area '{}'",
                buyer.display_name, buyer.area
            ));
        }
    }
    for area in areas {
        if !buyers.iter().any(|b| b.area == area.id) {
            return Err(format!(
                "area '{}' has no clientele - runs there would have no buyer",
                area.id
            ));
        }
    }
    Ok(())
}

/// SOW-026: every player-ownable card across the loaded pools
fn collect_player_cards(game_assets: &GameAssets) -> Vec<Card> {
    game_assets
        .products
        .values()
        .chain(game_assets.locations.values())
        .chain(game_assets.cover.iter())
        .chain(game_assets.insurance.iter())
        .chain(game_assets.modifiers.iter())
        .cloned()
        .collect()
}

/// SOW-026: the fresh (starting-collection) pool must reference real cards
/// and build a legal default deck - the lean start gates products hard, so
/// this is the guard that keeps "lean" from becoming "unplayable"
fn validate_fresh_collection(all_cards: &[Card]) -> Result<(), String> {
    let unlocked = crate::save::AccountState::starting_collection();

    for id in &unlocked {
        if !all_cards.iter().any(|c| &c.id == id) {
            return Err(format!(
                "starting collection references unknown card id '{id}'"
            ));
        }
    }

    let available: Vec<Card> = all_cards
        .iter()
        .filter(|c| unlocked.contains(&c.id))
        .cloned()
        .collect();
    let deck = crate::data::create_default_deck_from_available(&available);
    crate::data::validate_deck(&deck)
        .map_err(|e| format!("fresh starting collection cannot build a legal deck: {e}"))
}

/// SOW-026: warnings for scenarios whose demanded products are ALL gated
/// above the buyer's area on the shop ladder (rung = position in
/// shop_locations.ron; a demand no dealer could ever stock is a dead payout).
/// Starting-collection products are attainable everywhere.
fn ladder_attainability_warnings(
    buyers: &[BuyerPersona],
    all_cards: &[Card],
    areas: &[crate::models::shop_location::ShopLocationDef],
) -> Vec<String> {
    let rung = |area_id: &str| areas.iter().position(|a| a.id == area_id);
    let starting = crate::save::AccountState::starting_collection();
    let mut warnings = Vec::new();

    for buyer in buyers {
        let Some(buyer_rung) = rung(&buyer.area) else {
            continue; // unknown areas already fail validate_persona_areas
        };
        for scenario in &buyer.scenarios {
            if scenario.products.is_empty() {
                continue;
            }
            let attainable = scenario.products.iter().any(|name| {
                all_cards.iter().any(|c| {
                    &c.name == name
                        && matches!(c.card_type, CardType::Product { .. })
                        && (starting.contains(&c.id)
                            || c.shop_location
                                .as_deref()
                                .and_then(&rung)
                                .is_some_and(|r| r <= buyer_rung))
                })
            });
            if !attainable {
                warnings.push(format!(
                    "scenario '{}' of '{}' ({}) demands only products gated above that area",
                    scenario.display_name, buyer.display_name, buyer.area
                ));
            }
        }
    }
    warnings
}

/// SOW-024: Load areas (shop locations) from RON file
fn load_shop_locations(path: &str) -> Result<Vec<crate::models::shop_location::ShopLocationDef>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let areas: Vec<crate::models::shop_location::ShopLocationDef> = ron::from_str(&content)
        .map_err(|e| format!("Failed to parse {} - Check RON syntax:\n{}", path, e))?;

    if areas.is_empty() {
        return Err(format!("{} is empty - must have at least one area", path));
    }

    Ok(areas)
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

/// SOW-021: Validate that every demand string on a buyer (base demand and all
/// scenarios, products and locations) resolves to a real card name.
///
/// Demand matching is by exact card-name string (see is_demand_satisfied), so a
/// typo or a renamed card silently makes a demand impossible to satisfy - the
/// multiplier just never pays out. This check makes that an authoring-time error
/// instead. Demand lists deliberately stay human-readable names (not IDs) to
/// keep content authorable.
fn validate_buyer_demand_strings(
    buyer: &BuyerPersona,
    product_names: &[&str],
    location_names: &[&str],
) -> Result<(), String> {
    let check = |strings: &[String], known: &[&str], kind: &str, context: &str| -> Result<(), String> {
        for s in strings {
            if !known.contains(&s.as_str()) {
                return Err(format!(
                    "Buyer '{}' {} demands unknown {} '{}' - demand strings must exactly match a card name (known: {})",
                    buyer.display_name, context, kind, s, known.join(", ")
                ));
            }
        }
        Ok(())
    };

    check(&buyer.demand.products, product_names, "product", "base")?;
    check(&buyer.demand.locations, location_names, "location", "base")?;

    for scenario in &buyer.scenarios {
        let context = format!("scenario '{}'", scenario.display_name);
        check(&scenario.products, product_names, "product", &context)?;
        check(&scenario.locations, location_names, "location", &context)?;
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
        // All PNG (Reed's 2026-07-13 watermark-removal re-export). Backgrounds
        // are a silent fallback (unlike the loud portrait check) — keep aligned
        // to the files on disk. (E3 follow-up: a loud existence check here.)
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
        // SOW-028 Strip locations. VIP Room now has dedicated art (Reed,
        // 2026-07-13); Back of the Club still reuses the neon alley
        // (dead_drop) until its own club-alley background lands.
        ("Back of the Club", "dead_drop.png"),
        ("VIP Room", "vip_club_lounge.png"),
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

/// SOW-033: build the actor portrait map from RON (buyers.ron `portrait`,
/// shop_locations.ron `narc_portrait`) plus the code-owned dealer hire pool.
/// The hard-coded name->file HashMap is gone (art-backlog E3: new personas
/// silently got no art). A mapped portrait whose file is missing on disk is a
/// LOUD load error - Bevy's async asset_server.load would otherwise fail
/// silently. Must be called AFTER buyers + shop_locations are loaded.
fn load_actor_portraits(asset_server: &AssetServer, game_assets: &mut GameAssets) {
    // Runtime fallback narc face (also the Red Light District's authored art)
    const NARC_DEFAULT: &str = "narc-default.png";

    // (portrait-key, filename) pairs from every source; loaded through one
    // path with a loud file-existence check.
    let mut mapped: Vec<(String, String)> = Vec::new();

    // Buyers: keyed by display_name (the render lookup), file from RON
    for buyer in &game_assets.buyers {
        assert!(
            !buyer.portrait.trim().is_empty(),
            "buyer '{}' has no portrait in buyers.ron (SOW-033/E3 requires one)",
            buyer.display_name
        );
        mapped.push((buyer.display_name.clone(), buyer.portrait.clone()));
    }

    // Narcs: keyed by area id (ui_update looks up by the current run area),
    // file from RON narc_portrait, else the shared default. A dedicated
    // NARC_DEFAULT key backs the runtime fallback for any future area.
    mapped.push((NARC_DEFAULT.to_string(), NARC_DEFAULT.to_string()));
    for area in &game_assets.shop_locations {
        let file = area
            .narc_portrait
            .clone()
            .unwrap_or_else(|| NARC_DEFAULT.to_string());
        mapped.push((area.id.clone(), file));
    }

    // Dealer hire-pool faces + the kingpin placeholder. Code-owned pool;
    // filenames track the SOW-033 dealer-<slug>.png renames.
    mapped.push(("Silhouette".to_string(), "silhouette.png".to_string()));
    for key in crate::save::DEALER_PORTRAIT_POOL {
        let slug = key.to_lowercase().replace(' ', "-");
        mapped.push((key.to_string(), format!("dealer-{slug}.png")));
    }

    for (key, filename) in mapped {
        let disk_path = format!("assets/art/actors/{filename}");
        assert!(
            std::path::Path::new(&disk_path).exists(),
            "actor portrait '{key}' -> '{filename}' not found at {disk_path} \
             (SOW-033/E3: a mapped portrait must exist on disk)"
        );
        let handle = asset_server.load(format!("art/actors/{filename}"));
        game_assets.actor_portraits.insert(key, handle);
    }

    info!(
        "Loaded {} actor portraits (RON-mapped buyers/narcs + dealer pool)",
        game_assets.actor_portraits.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::buyer::{BuyerDemand, BuyerScenario};

    fn test_buyer(scenario_products: Vec<&str>, scenario_locations: Vec<&str>) -> BuyerPersona {
        BuyerPersona {
            area: "trailer_park".to_string(),
            portrait: String::new(),
            display_name: "Test Buyer".to_string(),
            demand: BuyerDemand {
                products: vec!["Weed".to_string()],
                locations: vec!["Safe House".to_string()],
                description: "test".to_string(),
            },
            base_multiplier: 2.0,
            reduced_multiplier: 1.0,
            evidence_threshold: None,
            reaction_deck_ids: vec![],
            reaction_deck: vec![],
            scenarios: vec![BuyerScenario {
                display_name: "Test Scenario".to_string(),
                products: scenario_products.into_iter().map(String::from).collect(),
                locations: scenario_locations.into_iter().map(String::from).collect(),
                heat_threshold: None,
                description: "test".to_string(),
                narrative_fragments: None,
            }],
            active_scenario_index: None,
        }
    }

    const PRODUCTS: &[&str] = &["Weed", "Coke"];
    const LOCATIONS: &[&str] = &["Safe House", "Frat House"];

    #[test]
    fn test_demand_validation_accepts_known_names() {
        let buyer = test_buyer(vec!["Weed", "Coke"], vec!["Frat House"]);
        assert!(validate_buyer_demand_strings(&buyer, PRODUCTS, LOCATIONS).is_ok());
    }

    #[test]
    fn test_demand_validation_rejects_unknown_product() {
        // "Pills" was a shipped dead string - no card has that name
        let buyer = test_buyer(vec!["Pills"], vec!["Frat House"]);
        let err = validate_buyer_demand_strings(&buyer, PRODUCTS, LOCATIONS).unwrap_err();
        assert!(err.contains("Pills"), "error should name the bad string: {}", err);
    }

    #[test]
    fn test_demand_validation_rejects_unknown_location() {
        // "Park" vs card "At the Park" was a shipped near-miss
        let buyer = test_buyer(vec!["Weed"], vec!["Park"]);
        let err = validate_buyer_demand_strings(&buyer, PRODUCTS, LOCATIONS).unwrap_err();
        assert!(err.contains("Park"), "error should name the bad string: {}", err);
    }

    #[test]
    fn test_demand_validation_checks_base_demand_too() {
        let mut buyer = test_buyer(vec!["Weed"], vec!["Safe House"]);
        buyer.demand.locations = vec!["Private Residence".to_string()];
        assert!(validate_buyer_demand_strings(&buyer, PRODUCTS, LOCATIONS).is_err());
    }

    #[test]
    fn test_persona_area_validation() {
        use crate::models::shop_location::ShopLocationDef;
        let area = |id: &str, unlocked: bool| ShopLocationDef {
            id: id.to_string(),
            name: id.to_string(),
            description: String::new(),
            unlocked,
            price: if unlocked { 0 } else { 2000 },
            identity: "CRAFT".to_string(),
            narc_hint: "eyes".to_string(),
            supplier: None,
            narc_portrait: None,
            restock_margin: 0.5,
        };

        // OK: one area, one persona living there (test_buyer defaults to trailer_park)
        let corner_only = vec![area("trailer_park", true)];
        let buyer = test_buyer(vec!["Weed"], vec!["Safe House"]);
        assert!(validate_persona_areas(&[buyer.clone()], &corner_only).is_ok());

        // Unknown persona area rejected
        let mut lost = buyer.clone();
        lost.area = "downtown".to_string();
        assert!(validate_persona_areas(&[lost], &corner_only)
            .unwrap_err()
            .contains("unknown area"));

        // Area without clientele rejected
        let with_block = vec![area("trailer_park", true), area("suburbia", false)];
        assert!(validate_persona_areas(&[buyer], &with_block)
            .unwrap_err()
            .contains("no clientele"));
    }

    #[test]
    fn test_shipped_persona_areas_all_resolve() {
        // SOW-024 acceptance criterion on the shipped content: every persona
        // area exists and every area has clientele
        let areas = load_shop_locations("assets/data/shop_locations.ron").expect("areas load");
        crate::models::shop_location::validate_shop_locations(&areas).expect("areas valid");
        let buyers = load_and_validate_buyers("assets/buyers.ron").expect("buyers load");
        validate_persona_areas(&buyers, &areas).expect("shipped persona areas resolve");
        // SOW-033: the Wall Street Wolf is shelved - no longer active clientele
        assert!(
            buyers.iter().all(|b| b.display_name != "Wall Street Wolf"),
            "Wolf should be shelved from the active roster"
        );
    }

    #[test]
    fn test_shipped_three_zone_coherence() {
        // SOW-033: the city is trailer_park -> suburbia -> red_light_district,
        // each with clientele; Red Light is the top (priciest) rung.
        let areas = load_shop_locations("assets/data/shop_locations.ron").expect("areas load");
        let ids: Vec<&str> = areas.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, vec!["trailer_park", "suburbia", "red_light_district"]);
        let suburbia = areas.iter().find(|a| a.id == "suburbia").unwrap();
        let red_light = areas.iter().find(|a| a.id == "red_light_district").unwrap();
        assert_eq!(suburbia.price, 1200);
        assert_eq!(red_light.price, 2500);
        assert!(red_light.price > suburbia.price, "top rung must out-price the middle");

        // SOW-031/033: every shipped zone carries its authored flavor + a named
        // supplier (validate_shop_locations enforces it; this pins the shipped
        // names so a content edit that renames one is deliberate)
        let suppliers: Vec<&str> = areas
            .iter()
            .map(|a| a.supplier.as_ref().expect("supplier").name.as_str())
            .collect();
        assert_eq!(suppliers, vec!["Lil Smoke", "Deb", "Miss Velvet"]);
        assert!(areas.iter().all(|a| !a.identity.is_empty() && !a.narc_hint.is_empty()));

        let buyers = load_and_validate_buyers("assets/buyers.ron").expect("buyers load");
        // SOW-033 headline: exactly 3 clientele per zone (9 personas total)
        assert_eq!(buyers.len(), 9);
        for id in ["trailer_park", "suburbia", "red_light_district"] {
            let count = buyers.iter().filter(|b| b.area == id).count();
            assert_eq!(count, 3, "zone {id} should have 3 buyers, has {count}");
        }
        let pimp = buyers.iter().find(|b| b.display_name == "Pimp").unwrap();
        assert_eq!(pimp.area, "red_light_district");
        // Zone coherence: the Housewife is Suburbia's clientele
        let housewife = buyers
            .iter()
            .find(|b| b.display_name == "Desperate Housewife")
            .unwrap();
        assert_eq!(housewife.area, "suburbia");
        assert_eq!(housewife.base_multiplier, 1.5);
        // SOW-033: the Frat Bro re-homed from the start to Suburbia
        let frat = buyers.iter().find(|b| b.display_name == "Frat Bro").unwrap();
        assert_eq!(frat.area, "suburbia");

        // Party economy lives in the Red Light District now
        let products =
            load_and_validate_cards("assets/cards/products.ron", "Product").expect("products");
        for name in ["Ecstasy", "Coke"] {
            let card = products.iter().find(|c| c.name == name).unwrap();
            assert_eq!(
                card.shop_location.as_deref(),
                Some("red_light_district"),
                "{name} should be Red Light stock"
            );
        }

        // SOW-033 headline: EXACTLY 2 shop products per zone (6 total). The
        // shelved premium tier (Acid/Ice/Heroin/Fentanyl) must carry no
        // shop_location - re-hooking one would otherwise silently break the
        // "2 products/zone" invariant while every other test stayed green.
        // (Guards the gap the SOW-033 adversarial review caught: the 3-buyers
        // /zone invariant was pinned above, the 2-products/zone one was not.)
        for id in ["trailer_park", "suburbia", "red_light_district"] {
            let count = products
                .iter()
                .filter(|c| c.shop_location.as_deref() == Some(id))
                .count();
            assert_eq!(count, 2, "zone {id} should stock exactly 2 products, has {count}");
        }
        let stocked = products.iter().filter(|c| c.shop_location.is_some()).count();
        assert_eq!(
            stocked, 6,
            "only the 6 zone products should have a shop_location; a shelved product got re-hooked"
        );
    }

    #[test]
    fn test_shipped_suburbia_first_rung_pays_without_coke() {
        // SOW-033 (carries SOW-028's reasoning): the Housewife (Suburbia, x1.5)
        // is satisfiable with STARTING-collection Weed once she brings her
        // own location - Suburbia expansion pays before any Suburbia product buy.
        let buyers = load_and_validate_buyers("assets/buyers.ron").expect("buyers load");
        let mut housewife = buyers
            .iter()
            .find(|b| b.display_name == "Desperate Housewife")
            .unwrap()
            .clone();
        let in_denial = housewife
            .scenarios
            .iter()
            .position(|s| s.display_name == "In Denial")
            .expect("In Denial scenario shipped");
        housewife.active_scenario_index = Some(in_denial);

        let mut hand_state = crate::models::hand_state::HandState::default();
        hand_state.buyer_persona = Some(housewife);
        hand_state
            .cards_played
            .push(crate::models::test_helpers::create_product("Weed", 30, 5));
        hand_state
            .cards_played
            .push(crate::models::test_helpers::create_buyer_location(
                "By the Pool",
                5,
                25,
                -10,
            ));

        assert!(hand_state.is_demand_satisfied());
        assert_eq!(hand_state.get_profit_multiplier(), 1.5);
    }

    fn load_all_shipped_player_cards() -> Vec<Card> {
        let mut all = Vec::new();
        for (path, kind) in [
            ("assets/cards/products.ron", "Product"),
            ("assets/cards/locations.ron", "Location"),
            ("assets/cards/cover.ron", "Cover"),
            ("assets/cards/insurance.ron", "Insurance"),
            ("assets/cards/modifiers.ron", "Modifier"),
        ] {
            all.extend(load_and_validate_cards(path, kind).unwrap_or_else(|e| panic!("{e}")));
        }
        all
    }

    #[test]
    fn test_shipped_fresh_collection_builds_valid_deck() {
        // SOW-026 acceptance criterion: the lean start must boot to a legal
        // deck, and Weed must be the ONLY starting product (Reed's gradient)
        let all = load_all_shipped_player_cards();
        validate_fresh_collection(&all).expect("fresh collection must build a legal deck");

        let starting = crate::save::AccountState::starting_collection();
        let starting_products: Vec<&str> = all
            .iter()
            .filter(|c| {
                starting.contains(&c.id) && matches!(c.card_type, CardType::Product { .. })
            })
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(starting_products, vec!["Weed"]);
    }

    #[test]
    fn test_shipped_ladder_leaves_no_card_orphaned() {
        // Everything trimmed from the start must be purchasable: every card
        // the lean pass removed from the collection is now shop-stocked with
        // a real price, and every shop-located card carries a price.
        // (Cards with NO shop_location are buyer-only by design - excluded.)
        let all = load_all_shipped_player_cards();
        for card in &all {
            if card.shop_location.is_some() {
                assert!(
                    card.shop_price.is_some(),
                    "shop card '{}' has a location but no price",
                    card.name
                );
            }
        }
        // The three cards the lean start trimmed are on the ladder
        for trimmed in ["shrooms", "codeine", "at_the_park"] {
            let card = all.iter().find(|c| c.id == trimmed).expect("trimmed card exists");
            assert!(
                card.shop_price.unwrap_or(0) > 0 && card.shop_cred_required.is_some(),
                "'{trimmed}' must be a priced, cred-gated shop unlock"
            );
        }
    }

    #[test]
    fn test_shipped_demands_attainable_on_ladder() {
        // SOW-026: zero dead payouts - every scenario can demand SOMETHING
        // attainable at-or-before its buyer's area
        let all = load_all_shipped_player_cards();
        let areas = load_shop_locations("assets/data/shop_locations.ron").expect("areas load");
        let buyers = load_and_validate_buyers("assets/buyers.ron").expect("buyers load");
        let warnings = ladder_attainability_warnings(&buyers, &all, &areas);
        assert!(warnings.is_empty(), "unattainable demands: {warnings:?}");
    }

    #[test]
    fn test_ladder_warning_when_all_demands_gated_above() {
        use crate::models::test_helpers::create_product;
        let mut premium = create_product("Fentanyl", 200, 50);
        premium.shop_location = Some("suburbia".to_string());
        let areas = vec![
            crate::models::shop_location::ShopLocationDef {
                id: "trailer_park".to_string(),
                name: "Trailer Park".to_string(),
                description: "test".to_string(),
                unlocked: true,
                price: 0,
                identity: "CRAFT".to_string(),
                narc_hint: "eyes".to_string(),
                supplier: None,
                narc_portrait: None,
                restock_margin: 0.5,
            },
            crate::models::shop_location::ShopLocationDef {
                id: "suburbia".to_string(),
                name: "Suburbia".to_string(),
                description: "test".to_string(),
                unlocked: false,
                price: 2000,
                identity: "CRAFT".to_string(),
                narc_hint: "eyes".to_string(),
                supplier: None,
                narc_portrait: None,
                restock_margin: 0.5,
            },
        ];
        // Corner buyer demanding a Block-gated product = dead payout -> warn
        let buyer = test_buyer(vec!["Fentanyl"], vec![]);
        let warnings = ladder_attainability_warnings(&[buyer], &[premium.clone()], &areas);
        assert_eq!(warnings.len(), 1);

        // The same demand from a Block buyer is fine
        let mut block_buyer = test_buyer(vec!["Fentanyl"], vec![]);
        block_buyer.area = "suburbia".to_string();
        let warnings = ladder_attainability_warnings(&[block_buyer], &[premium], &areas);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_shipped_demand_strings_all_resolve() {
        // SOW-021 acceptance criterion: zero unresolvable demand strings in
        // the shipped content. Loads the real RON files.
        let products = load_and_validate_cards("assets/cards/products.ron", "Product")
            .expect("products.ron should load");
        let locations = load_and_validate_cards("assets/cards/locations.ron", "Location")
            .expect("locations.ron should load");
        let buyers = load_and_validate_buyers("assets/buyers.ron")
            .expect("buyers.ron should load");

        let product_names: Vec<&str> = products.iter().map(|c| c.name.as_str()).collect();
        let location_names: Vec<&str> = locations.iter().map(|c| c.name.as_str()).collect();

        for buyer in &buyers {
            validate_buyer_demand_strings(buyer, &product_names, &location_names)
                .unwrap_or_else(|e| panic!("{}", e));
        }
    }

    #[test]
    fn test_shipped_narc_compositions_resolve_and_cover_all_tiers() {
        // SOW-027 acceptance criterion: the sparse composition file parses,
        // the default ladder covers all six heat tiers, and every card id in
        // every ladder resolves against shipped evidence/conviction cards.
        let file = load_narc_compositions("assets/narc_deck.ron")
            .expect("narc_deck.ron should parse");

        let evidence = load_and_validate_cards("assets/cards/evidence.ron", "Evidence")
            .expect("evidence.ron should load");
        let convictions = load_and_validate_cards("assets/cards/convictions.ron", "Conviction")
            .expect("convictions.ron should load");
        let known_ids: std::collections::HashSet<&str> = evidence
            .iter()
            .chain(convictions.iter())
            .map(|c| c.id.as_str())
            .collect();

        let all_tiers = ["Cold", "Warm", "Hot", "Blazing", "Scorching", "Inferno"];
        for tier in all_tiers {
            let ladder = file
                .default
                .get(tier)
                .unwrap_or_else(|| panic!("default ladder missing tier {}", tier));
            assert!(!ladder.is_empty(), "default {} ladder is empty", tier);
        }

        for (tier, ids) in file
            .default
            .iter()
            .chain(file.areas.values().flatten())
        {
            assert!(
                all_tiers.contains(&tier.as_str()),
                "unknown heat tier {:?} in narc_deck.ron",
                tier
            );
            for id in ids {
                assert!(
                    known_ids.contains(id.as_str()),
                    "narc_deck.ron references unknown card id {:?} at tier {}",
                    id,
                    tier
                );
            }
        }
    }

    #[test]
    fn test_shipped_trailer_park_cold_composition_is_gentle() {
        // SOW-027/033: the fresh-run floor fix. A fresh kingpin in Trailer Park
        // at Cold must face a mostly-donut deck (avg heat/card well under the
        // old flat deck) or the 3-blind-session floor regresses to Inferno.
        let file = load_narc_compositions("assets/narc_deck.ron")
            .expect("narc_deck.ron should parse");
        let evidence = load_and_validate_cards("assets/cards/evidence.ron", "Evidence")
            .expect("evidence.ron should load");
        let heat_by_id: HashMap<&str, i32> = evidence
            .iter()
            .filter_map(|c| match c.card_type {
                CardType::Evidence { heat, .. } => Some((c.id.as_str(), heat)),
                _ => None,
            })
            .collect();

        // trailer_park inherits default entirely, so Cold == default Cold
        let cold = file.default.get("Cold").expect("default has Cold");
        let total_heat: i32 = cold
            .iter()
            .map(|id| heat_by_id.get(id.as_str()).copied().unwrap_or(0))
            .sum();
        let avg = total_heat as f32 / cold.len() as f32;
        assert!(
            avg <= 3.0,
            "Trailer Park/Cold composition too hot: avg {:.1} heat/card (want <= 3.0)",
            avg
        );
    }

    #[test]
    fn test_shipped_narc_difficulty_climbs_with_the_ladder() {
        // SOW-033 (silent-risk flag 11): the narc override blocks were swapped
        // so difficulty climbs trailer_park < suburbia < red_light_district.
        // A find/replace that renamed keys without swapping contents would
        // leave suburbia harder than red_light - caught here by comparing
        // total Cold-tier evidence pressure (all three define Cold).
        let file = load_narc_compositions("assets/narc_deck.ron")
            .expect("narc_deck.ron should parse");
        let evidence = load_and_validate_cards("assets/cards/evidence.ron", "Evidence")
            .expect("evidence.ron should load");
        let evidence_by_id: HashMap<&str, u32> = evidence
            .iter()
            .filter_map(|c| match c.card_type {
                CardType::Evidence { evidence, .. } => Some((c.id.as_str(), evidence)),
                _ => None,
            })
            .collect();

        let cold_pressure = |ids: &[String]| -> u32 {
            ids.iter()
                .map(|id| evidence_by_id.get(id.as_str()).copied().unwrap_or(0))
                .sum()
        };

        // trailer_park inherits the default Cold; suburbia and red_light both
        // override it
        let trailer = cold_pressure(file.default.get("Cold").expect("default Cold"));
        let suburbia = cold_pressure(
            file.areas
                .get("suburbia")
                .and_then(|o| o.get("Cold"))
                .expect("suburbia Cold override"),
        );
        let red_light = cold_pressure(
            file.areas
                .get("red_light_district")
                .and_then(|o| o.get("Cold"))
                .expect("red_light_district Cold override"),
        );

        assert!(
            trailer < suburbia && suburbia < red_light,
            "narc Cold-tier evidence must climb with the ladder: trailer {trailer} < suburbia {suburbia} < red_light {red_light}"
        );
    }
}
