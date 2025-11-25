mod ui;
mod assets;
mod data;
mod models;
mod systems;
mod game_state;
mod save;

use bevy::prelude::*;
use bevy::asset::load_internal_binary_asset;
use ui::setup::*;
use models::card::*;
use models::deck_builder::*;
use models::hand_state::*;
use models::fonts::EmojiFont;
use systems::*;
use game_state::{GameState, AiActionTimer};
use save::SavePlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Drug Dealer Deckbuilder".to_string(),
            resolution: (1920, 1080).into(),
            ..default()
        }),
        ..default()
    }));

    // Set DejaVuSans as default font (has filled star U+2605 and good Unicode coverage)
    load_internal_binary_asset!(
        app,
        bevy::text::TextFont::default().font,
        "../assets/fonts/DejaVuSans.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app
        .add_plugins(assets::AssetLoaderPlugin)
        .add_plugins(SavePlugin)
        .add_plugins(ui::FoilMaterialPlugin)
        .init_state::<GameState>()
        .insert_resource(AiActionTimer::default())
        .init_resource::<CharacterLoaded>()
        .init_resource::<DecayInfo>()
        .add_systems(Startup, setup)
        // Character persistence systems
        .add_systems(OnEnter(GameState::DeckBuilding), (
            load_character_system,
            apply_decay_system,
            initialize_deck_builder_from_assets,
            setup_deck_builder,
        ).chain())
        .add_systems(OnEnter(GameState::InRun), ensure_character_on_run_start)
        .add_systems(OnExit(GameState::DeckBuilding), cleanup_deck_builder_ui)
        .add_systems(OnExit(GameState::InRun), mark_deck_completed_system);

    app
        .add_systems(Startup, ui::scale_ui_to_fit_system)  // Initial UI scaling
        .add_systems(Update, ui::scale_ui_to_fit_system)  // UI scaling for any window size
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
            update_actor_portraits_system,
            recreate_hand_display_system,
            ui_update_system,
            ui::update_active_slots_system,
            ui::update_heat_bar_system,
            ui::update_resolution_overlay_system,
            ui::update_background_system,
            update_character_heat_display_system,
            update_decay_display_system,
            clear_decay_display_system,
            update_account_cash_display_system,
        ).chain())
        .add_systems(Update, (
            card_click_system,
            save_after_resolution_system,
        ).run_if(in_state(GameState::InRun)))
        .add_systems(Update, (
            deck_builder_card_click_system,
            start_run_button_system,
            update_deck_builder_ui_system,
            populate_deck_builder_cards_system,
            ui::ui_scroll_system, // Bevy 0.17: Manual scroll handling
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
        commands.entity(entity).despawn();
    }
}
