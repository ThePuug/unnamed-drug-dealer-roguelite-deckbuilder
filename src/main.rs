mod ui;
mod assets; // SOW-013-A: Asset loading system
mod data;
mod models;
mod systems;
mod game_state;

use bevy::prelude::*;
use ui::setup::*;
use models::card::*;
use models::deck_builder::*;
use models::hand_state::*; // SOW-AAA Phase 5
use models::fonts::EmojiFont;
use systems::*;
use game_state::{GameState, AiActionTimer}; // SOW-AAA Phase 8

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(assets::AssetLoaderPlugin) // SOW-013-A: Load game assets
        .init_state::<GameState>()  // SOW-006: Add state management (starts in AssetLoading)
        .insert_resource(AiActionTimer::default())  // SOW-008: AI pacing timer
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::DeckBuilding), initialize_deck_builder_from_assets) // SOW-013-B: Init when entering DeckBuilding
        .add_systems(OnEnter(GameState::DeckBuilding), setup_deck_builder)  // SOW-006: Setup deck builder UI
        .add_systems(OnExit(GameState::DeckBuilding), cleanup_deck_builder_ui);  // SOW-013-B: Cleanup UI when leaving

    app
        .add_systems(Update, toggle_game_state_ui_system)
        .add_systems(Update, (
            ai_betting_system,
            auto_flip_system,
            betting_button_system,
            restart_button_system,
            go_home_button_system,
            update_betting_button_states,
            update_restart_button_states,
        ).chain())
        .add_systems(Update, (
            update_played_cards_display_system,
            render_buyer_visible_hand_system,
            render_narc_visible_hand_system,
            recreate_hand_display_system,
            ui_update_system,
            ui::update_active_slots_system,  // SOW-011-A Phase 4
            ui::update_heat_bar_system,      // SOW-011-A Phase 4
            ui::update_resolution_overlay_system, // SOW-011-B Phase 1
        ).chain())
        .add_systems(Update, (
            card_click_system,
        ).run_if(in_state(GameState::InRun)))
        .add_systems(Update, (
            deck_builder_card_click_system,
            preset_button_system,
            start_run_button_system,
            update_deck_builder_ui_system,
            populate_deck_builder_cards_system,
        ).chain().run_if(in_state(GameState::DeckBuilding)))
        .run();
}

// SOW-013-B: Initialize DeckBuilder from loaded assets (OnEnter DeckBuilding state)
// Only runs if DeckBuilder doesn't exist (first time entering DeckBuilding)
fn initialize_deck_builder_from_assets(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    existing_deck_builder: Option<Res<DeckBuilder>>,
) {
    // Only initialize if DeckBuilder doesn't exist yet
    if existing_deck_builder.is_none() {
        let deck_builder = DeckBuilder::from_assets(&game_assets);
        let card_count = deck_builder.available_cards.len();
        commands.insert_resource(deck_builder);
        info!("DeckBuilder initialized from assets with {} cards", card_count);
    } else {
        info!("DeckBuilder already exists - preserving current deck selection");
    }
}

// SOW-013-B: Cleanup deck builder UI when leaving DeckBuilding state
fn cleanup_deck_builder_ui(
    mut commands: Commands,
    deck_builder_root_query: Query<Entity, With<ui::components::DeckBuilderRoot>>,
) {
    for entity in deck_builder_root_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
