mod ui;
mod data;
mod models;
mod systems;
mod game_state;

use bevy::prelude::*;
use bevy::asset::load_internal_binary_asset;
use ui::setup::*;
use models::card::*;
use models::deck_builder::*;
use models::hand_state::*; // SOW-AAA Phase 5
use systems::*;
use game_state::{GameState, AiActionTimer}; // SOW-AAA Phase 8

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_state::<GameState>()  // SOW-006: Add state management
        .insert_resource(DeckBuilder::new())  // SOW-006: Initialize deck builder
        .insert_resource(AiActionTimer::default())  // SOW-008: AI pacing timer
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_deck_builder);  // SOW-006: Setup deck builder UI

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
            deck_builder_card_click_system,
            preset_button_system,
            start_run_button_system,
            update_deck_builder_ui_system,
            populate_deck_builder_cards_system,
        ).chain())
        .run();
}
