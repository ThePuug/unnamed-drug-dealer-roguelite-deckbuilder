// RFC-019: Upgrade Choice UI Systems
// SOW-021: All pending upgrades are presented on ONE batched screen (rows per
// card) instead of one full-screen modal per card, and choices are deferrable
// via DECIDE LATER (skipped upgrades stay pending for the next return home).

use bevy::prelude::*;
use crate::game_state::GameState;
use crate::save::{SaveData, SaveManager, UpgradeableStat};

// Color constants for upgrade UI
const HEADING_COLOR: Color = Color::srgb(1.0, 0.9, 0.4);
const BODY_TEXT_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const BUTTON_BG: Color = Color::srgb(0.2, 0.2, 0.25);
const BUTTON_BG_HOVER: Color = Color::srgb(0.3, 0.3, 0.35);
const BUTTON_BORDER: Color = Color::srgb(0.4, 0.4, 0.5);
const BUTTON_BORDER_HOVER: Color = Color::srgb(0.6, 0.6, 0.7);

/// Marker for upgrade choice UI root
#[derive(Component)]
pub struct UpgradeChoiceUI;

/// Marker for upgrade option buttons
/// SOW-021: Carries the card name so any row can be resolved in any order
#[derive(Component)]
pub struct UpgradeOptionButton {
    pub card_name: String,
    pub stat: UpgradeableStat,
}

/// SOW-021: Marker for the DECIDE LATER button
#[derive(Component)]
pub struct UpgradeDeferButton;

/// SOW-021: When true, check_pending_upgrades_system does not redirect to
/// UpgradeChoice (player chose DECIDE LATER). Cleared when a run starts, so
/// the player is re-prompted on their next return to the deck builder.
#[derive(Resource, Default)]
pub struct UpgradeChoiceDeferred(pub bool);

/// Setup the upgrade choice UI (OnEnter UpgradeChoice)
pub fn setup_upgrade_choice_ui(
    mut commands: Commands,
    save_data: Res<SaveData>,
) {
    spawn_upgrade_choice_ui(&mut commands, &save_data);
}

/// SOW-021: Spawn the batched upgrade screen - one row per pending upgrade
fn spawn_upgrade_choice_ui(commands: &mut Commands, save_data: &SaveData) {
    let Some(ref character) = save_data.character else {
        return;
    };

    if character.pending_upgrades.is_empty() {
        return;
    }

    let pending_count = character.pending_upgrades.len();

    // Root container - high z-index to render on top of any other UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        },
        GlobalZIndex(100), // Render on top of other UI
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
        UpgradeChoiceUI,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new(if pending_count == 1 {
                "UPGRADE EARNED".to_string()
            } else {
                format!("{} UPGRADES EARNED", pending_count)
            }),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(HEADING_COLOR),
        ));

        parent.spawn((
            Text::new("Choose a bonus for each card, or decide later"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(BODY_TEXT_COLOR),
            Node {
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
        ));

        // One row per pending upgrade: [card name + tier] [option 1] [option 2]
        for pending in &character.pending_upgrades {
            let card_type = pending.card_type.clone();
            let options = pending.options;
            let card_name = pending.card_name.clone();
            let tier_label = format!("{} → ★ Tier upgrade", pending.card_name);

            parent.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                ..default()
            }).with_children(|row| {
                // Card label
                row.spawn(Node {
                    width: Val::Px(300.0),
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                }).with_children(|label| {
                    label.spawn((
                        Text::new(tier_label),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

                // Option buttons
                for stat in options {
                    let (stat_name, effect_desc) = get_stat_description(stat, &card_type);

                    row.spawn((
                        Button,
                        Interaction::default(), // Explicitly add for Bevy 0.18
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(70.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(3.0)),
                            ..default()
                        },
                        BorderColor::all(BUTTON_BORDER),
                        BackgroundColor(BUTTON_BG),
                        UpgradeOptionButton { card_name: card_name.clone(), stat },
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new(stat_name),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                        btn.spawn((
                            Text::new(effect_desc),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        ));
                    });
                }
            });
        }

        // SOW-021: DECIDE LATER - keeps remaining upgrades pending
        parent.spawn((
            Button,
            Interaction::default(),
            Node {
                margin: UiRect::top(Val::Px(25.0)),
                padding: UiRect::axes(Val::Px(25.0), Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(BUTTON_BORDER),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.18)),
            UpgradeDeferButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("DECIDE LATER"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(BODY_TEXT_COLOR),
            ));
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

/// Handle upgrade option button clicks (and DECIDE LATER)
pub fn upgrade_option_click_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &UpgradeOptionButton, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    defer_query: Query<&Interaction, (Changed<Interaction>, With<UpgradeDeferButton>)>,
    ui_query: Query<Entity, With<UpgradeChoiceUI>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut save_data: ResMut<SaveData>,
    save_manager: Res<SaveManager>,
    mut deferred: ResMut<UpgradeChoiceDeferred>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // SOW-021: DECIDE LATER - exit to DeckBuilding, keep pending upgrades
    for interaction in defer_query.iter() {
        if *interaction == Interaction::Pressed {
            deferred.0 = true;
            next_state.set(GameState::DeckBuilding);
            return;
        }
    }

    // Snapshot before the closure takes a mutable capture of save_data
    let first_pending = save_data.character.as_ref()
        .and_then(|c| c.next_pending_upgrade())
        .map(|p| (p.card_name.clone(), p.options));

    // Helper to apply an upgrade for a specific card and refresh the screen
    let mut handle_upgrade = |card_name: &str, stat: UpgradeableStat| {
        let (applied, has_more) = {
            let Some(ref mut character) = save_data.character else {
                return;
            };
            let applied = character.apply_upgrade_choice_for(card_name, stat);
            let has_more = character.has_pending_upgrades();
            (applied, has_more)
        };

        if applied {
            if let Err(e) = save_manager.save(&save_data) {
                warn!("Failed to save after upgrade choice: {:?}", e);
            }

            // Rebuild the batched screen without the resolved row
            for entity in ui_query.iter() {
                commands.entity(entity).despawn();
            }

            if has_more {
                spawn_upgrade_choice_ui(&mut commands, &save_data);
            } else {
                next_state.set(GameState::DeckBuilding);
            }
        }
    };

    // Keyboard shortcuts: 1 / 2 resolve the FIRST pending upgrade's options
    if let Some((card_name, options)) = first_pending {
        if keyboard.just_pressed(KeyCode::Digit1) {
            handle_upgrade(&card_name, options[0]);
            return;
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            handle_upgrade(&card_name, options[1]);
            return;
        }
    }

    for (interaction, option, mut bg_color, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let card_name = option.card_name.clone();
                let stat = option.stat;
                handle_upgrade(&card_name, stat);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(BUTTON_BG_HOVER);
                *border_color = BorderColor::all(BUTTON_BORDER_HOVER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(BUTTON_BG);
                *border_color = BorderColor::all(BUTTON_BORDER);
            }
        }
    }
}

/// SOW-021: Clear the deferral flag when a run starts, so pending upgrades
/// prompt again on the next return to the deck builder
pub fn clear_upgrade_deferral(mut deferred: ResMut<UpgradeChoiceDeferred>) {
    deferred.0 = false;
}

/// System to check for pending upgrades when entering DeckBuilding
/// If there are pending upgrades, redirect to UpgradeChoice state
/// SOW-021: unless the player chose DECIDE LATER this visit
pub fn check_pending_upgrades_system(
    save_data: Res<SaveData>,
    deferred: Res<UpgradeChoiceDeferred>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if deferred.0 {
        return;
    }

    if let Some(ref character) = save_data.character {
        if character.has_pending_upgrades() {
            info!("Found {} pending upgrades, entering UpgradeChoice state",
                  character.pending_upgrades.len());
            next_state.set(GameState::UpgradeChoice);
        }
    }
}
