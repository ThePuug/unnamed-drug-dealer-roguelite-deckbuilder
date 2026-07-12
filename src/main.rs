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
    // SOW-023: dev subcommand - `cargo run -- forge <scenario> [--dir <path>]`
    // writes a crafted, signed save for e2e playtests and exits (no App)
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.get(1).map(String::as_str) == Some("forge") {
        save::forge::run_cli(&cli_args[2..]);
        return;
    }

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
        .init_resource::<shop::ShopState>() // SOW-020: Shop state for deck builder
        .init_resource::<UpgradeChoiceDeferred>() // SOW-021: DECIDE LATER flag
        .add_systems(Startup, setup)
        // Character persistence systems
        .add_systems(OnEnter(GameState::DeckBuilding), (
            load_character_system,
            apply_decay_system,
            check_pending_upgrades_system, // RFC-019: Redirect to UpgradeChoice if pending
            initialize_deck_builder_from_assets,
            setup_deck_builder,
        ).chain())
        .add_systems(OnEnter(GameState::InRun), (
            ensure_roster_on_run_start, // RFC-023: defensive roster invariant
            clear_upgrade_deferral, // SOW-021: re-prompt pending upgrades after next run
        ))
        .add_systems(OnExit(GameState::DeckBuilding), cleanup_deck_builder_ui)
        .add_systems(OnExit(GameState::InRun), mark_deck_completed_system)
        // RFC-019: Upgrade Choice state
        .add_systems(OnEnter(GameState::UpgradeChoice), setup_upgrade_choice_ui)
        .add_systems(OnExit(GameState::UpgradeChoice), cleanup_upgrade_choice_ui)
        .add_systems(Update, (
            upgrade_option_click_system,
            ui::ui_scroll_system, // SOW-021: scroll the batched upgrade list
        ).run_if(in_state(GameState::UpgradeChoice)));

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
            update_actor_portraits_system,
            recreate_hand_display_system,
            hand_hover_system,                // SOW-022: hand fan hover lift
            update_deck_discard_system,       // SOW-022: deck/discard stacks
            update_narc_intent_system,        // SOW-022: intent telegraph
            update_buyer_panel_system,        // SOW-022: wants bubble + chips
            update_buyer_played_bubble_system, // SOW-022: buyer reaction callout
            buyer_bubble_hover_system,        // SOW-022: hover detail
            update_spotlights_system,         // SOW-022: actor spotlights
            ui::update_active_slots_system,
            ui::update_standing_panel_system, // SOW-022: cash + heat panel
            ui::update_balance_bar_system,    // SOW-022: evidence vs cover
            ui::update_turn_display_system,   // SOW-021/022: round + actor pill
            ui::update_resolution_overlay_system,
            ui::update_background_system,
            update_decay_display_system,
            clear_decay_display_system,
            update_account_cash_display_system,
            update_story_history_display_system,
        ).chain())
        .add_systems(Update, (
            card_click_system,
            save_after_resolution_system,
        ).run_if(in_state(GameState::InRun)))
        .add_systems(Update, (
            deck_builder_card_click_system,
            start_run_button_system,
            roster_button_system,             // SOW-023: select/hire/bail
            populate_roster_panel_system,     // SOW-023: roster strip
            update_start_run_button_system,   // SOW-023: jailed = no run
            story_history_button_system,
            update_deck_builder_ui_system,
            populate_deck_builder_cards_system,
            // SOW-020: Shop systems
            shop_tab_system,
            shop_location_button_system,
            populate_shop_cards_system,
            shop_purchase_system,
            update_shop_tab_visuals,
            update_location_button_visuals,
            ui::ui_scroll_system, // Bevy 0.18: Manual scroll handling
        ).chain().run_if(in_state(GameState::DeckBuilding)))
        .run();
}

// SOW-013-B: Initialize DeckBuilder from loaded assets (OnEnter DeckBuilding state)
// SOW-020: Filter by unlocked cards from AccountState
// Only runs if DeckBuilder doesn't exist (first time entering DeckBuilding)
fn initialize_deck_builder_from_assets(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    existing_deck_builder: Option<Res<DeckBuilder>>,
    save_data: Option<Res<save::SaveData>>,
    deferred: Res<UpgradeChoiceDeferred>,
) {
    // RFC-019: Skip if pending upgrades (we're about to redirect to UpgradeChoice)
    // SOW-021: unless the player deferred them (DECIDE LATER) - then we stay here
    // RFC-023: pending upgrades belong to the active dealer
    if let Some(ref data) = save_data {
        if data.active_character().has_pending_upgrades() && !deferred.0 {
            return;
        }
    }

    // Only initialize if DeckBuilder doesn't exist yet
    if existing_deck_builder.is_none() {
        // SOW-020: Get unlocked cards from AccountState (or use starting collection for new saves)
        let unlocked_cards = save_data
            .as_ref()
            .map(|data| data.account.unlocked_cards.clone())
            .unwrap_or_else(|| save::AccountState::starting_collection());

        let deck_builder = DeckBuilder::from_assets_filtered(&game_assets, &unlocked_cards);
        let card_count = deck_builder.available_cards.len();
        commands.insert_resource(deck_builder);
        info!("DeckBuilder initialized from assets with {} unlocked cards", card_count);
    } else {
        info!("DeckBuilder already exists - preserving current deck selection");
    }
}

// SOW-013-B: Cleanup deck builder UI when leaving DeckBuilding state
fn cleanup_deck_builder_ui(
    mut commands: Commands,
    deck_builder_root_query: Query<Entity, With<ui::components::DeckBuilderRoot>>,
    story_overlay_query: Query<Entity, With<ui::components::StoryHistoryOverlay>>,
) {
    for entity in deck_builder_root_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in story_overlay_query.iter() {
        commands.entity(entity).despawn();
    }
}
