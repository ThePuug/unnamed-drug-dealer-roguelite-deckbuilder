// RFC-019: Upgrade Choice UI Systems
// Handles the UI for player stat upgrade selection

use bevy::prelude::*;
use crate::game_state::GameState;
use crate::save::{SaveData, SaveManager, UpgradeableStat};

// Color constants for upgrade UI
const HEADING_COLOR: Color = Color::srgb(1.0, 0.9, 0.4);
const BODY_TEXT_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

/// Marker for upgrade choice UI root
#[derive(Component)]
pub struct UpgradeChoiceUI;

/// Marker for upgrade option buttons
#[derive(Component)]
pub struct UpgradeOptionButton {
    pub stat: UpgradeableStat,
}

/// Setup the upgrade choice UI
pub fn setup_upgrade_choice_ui(
    mut commands: Commands,
    save_data: Res<SaveData>,
) {
    let Some(ref character) = save_data.character else {
        return;
    };

    let Some(pending) = character.next_pending_upgrade() else {
        return;
    };

    // Root container - high z-index to render on top of any other UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GlobalZIndex(100), // Render on top of other UI
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
        UpgradeChoiceUI,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new(format!("UPGRADE: {}", pending.card_name)),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(HEADING_COLOR),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));

        // Tier info
        parent.spawn((
            Text::new(format!("Reached {} - Choose your bonus:", pending.tier.name())),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(BODY_TEXT_COLOR),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ));

        // Options container
        let card_type = pending.card_type.clone();
        let options = pending.options;
        parent.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(40.0),
            ..default()
        }).with_children(|opt_parent| {
            // Spawn both option buttons
            for stat in options {
                let (stat_name, effect_desc) = get_stat_description(stat, &card_type);

                opt_parent.spawn((
                    Button,
                    Interaction::default(), // Explicitly add for Bevy 0.17
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(120.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(15.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.4, 0.5)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    UpgradeOptionButton { stat },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(stat_name),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        },
                    ));
                    btn.spawn((
                        Text::new(effect_desc),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
            }
        });
    });
}

fn get_stat_description(stat: UpgradeableStat, _card_type: &crate::models::card::CardType) -> (String, String) {
    match stat {
        UpgradeableStat::Price => ("PRICE".to_string(), "+10% profit".to_string()),
        UpgradeableStat::Cover => ("COVER".to_string(), "+10% cover".to_string()),
        UpgradeableStat::Evidence => ("EVIDENCE".to_string(), "-10% evidence".to_string()),
        UpgradeableStat::Heat => ("HEAT".to_string(), "-10% heat".to_string()),
        UpgradeableStat::HeatPenalty => ("HEAT PENALTY".to_string(), "-10% penalty".to_string()),
        UpgradeableStat::PriceMultiplier => ("PRICE BONUS".to_string(), "+10% multiplier".to_string()),
    }
}

/// Cleanup the upgrade choice UI
pub fn cleanup_upgrade_choice_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<UpgradeChoiceUI>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Handle upgrade option button clicks
pub fn upgrade_option_click_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &UpgradeOptionButton, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    all_buttons: Query<&UpgradeOptionButton>,
    ui_query: Query<Entity, With<UpgradeChoiceUI>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Helper to handle upgrade and refresh
    let mut handle_upgrade = |stat: UpgradeableStat| {
        let (applied, has_more) = {
            let Some(ref mut character) = save_data.character else {
                return;
            };
            let applied = character.apply_upgrade_choice(stat);
            let has_more = character.has_pending_upgrades();
            (applied, has_more)
        };

        if applied {
            if let Err(e) = save_manager.save(&save_data) {
                warn!("Failed to save after upgrade choice: {:?}", e);
            }

            // Clean up current UI
            for entity in ui_query.iter() {
                commands.entity(entity).despawn();
            }

            if has_more {
                // Rebuild UI for next upgrade (can't re-enter same state)
                rebuild_upgrade_ui(&mut commands, &save_data);
            } else {
                next_state.set(GameState::DeckBuilding);
            }
        }
    };

    // Keyboard shortcuts: Press 1 for first option, 2 for second option
    let buttons: Vec<_> = all_buttons.iter().collect();
    if keyboard.just_pressed(KeyCode::Digit1) && !buttons.is_empty() {
        handle_upgrade(buttons[0].stat);
        return;
    }
    if keyboard.just_pressed(KeyCode::Digit2) && buttons.len() > 1 {
        handle_upgrade(buttons[1].stat);
        return;
    }

    for (interaction, option, mut bg_color, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                handle_upgrade(option.stat);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.35));
                *border_color = BorderColor::all(Color::srgb(0.6, 0.6, 0.7));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.25));
                *border_color = BorderColor::all(Color::srgb(0.4, 0.4, 0.5));
            }
        }
    }
}

/// Rebuild the upgrade UI for the next pending upgrade
fn rebuild_upgrade_ui(commands: &mut Commands, save_data: &SaveData) {
    let Some(ref character) = save_data.character else {
        return;
    };

    let Some(pending) = character.next_pending_upgrade() else {
        return;
    };

    // Root container - high z-index to render on top of any other UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GlobalZIndex(100),
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
        UpgradeChoiceUI,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new(format!("UPGRADE: {}", pending.card_name)),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(HEADING_COLOR),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));

        // Tier info
        parent.spawn((
            Text::new(format!("Reached {} - Choose your bonus:", pending.tier.name())),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(BODY_TEXT_COLOR),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ));

        // Options container
        let card_type = pending.card_type.clone();
        let options = pending.options;
        parent.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(40.0),
            ..default()
        }).with_children(|opt_parent| {
            for stat in options {
                let (stat_name, effect_desc) = get_stat_description(stat, &card_type);

                opt_parent.spawn((
                    Button,
                    Interaction::default(),
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(120.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(15.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.4, 0.5)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    UpgradeOptionButton { stat },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(stat_name),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        },
                    ));
                    btn.spawn((
                        Text::new(effect_desc),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
            }
        });
    });
}

/// System to check for pending upgrades when entering DeckBuilding
/// If there are pending upgrades, redirect to UpgradeChoice state
pub fn check_pending_upgrades_system(
    save_data: Res<SaveData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(ref character) = save_data.character {
        if character.has_pending_upgrades() {
            info!("Found {} pending upgrades, entering UpgradeChoice state",
                  character.pending_upgrades.len());
            next_state.set(GameState::UpgradeChoice);
        }
    }
}
