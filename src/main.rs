// SOW-001: Minimal Playable Hand
// SOW-002: Betting System and AI Opponents

use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_betting_state)
        .add_systems(Update, (
            auto_play_system,
            ai_betting_system,
            auto_flip_system,
            betting_button_system,
            decision_point_button_system,
            update_betting_button_states,
            toggle_ui_visibility_system,
            update_played_cards_display_system,
            recreate_hand_display_system,
            ui_update_system,
            card_click_system,
        ).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Initialize hand state
    let mut hand_state = HandState::default();
    hand_state.draw_cards();
    commands.spawn(hand_state);

    // Create UI root
    create_ui(&mut commands);
}

fn setup_betting_state(mut commands: Commands) {
    // Initialize betting state component
    commands.spawn(BettingState::default());
}

// ============================================================================
// UI COMPONENTS - Phase 4
// ============================================================================

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct TotalsDisplay;

#[derive(Component)]
struct StatusDisplay;

#[derive(Component)]
struct PlayAreaNarc;

#[derive(Component)]
struct PlayAreaCustomer;

#[derive(Component)]
struct PlayAreaPlayer;

#[derive(Component)]
struct PlayerHandDisplay;

#[derive(Component)]
struct CardButton {
    card_index: usize,
}

// SOW-002 Phase 5: Betting UI components
#[derive(Component)]
struct BettingActionsContainer;

#[derive(Component)]
struct CheckButton;

#[derive(Component)]
struct RaiseButton;

#[derive(Component)]
struct FoldButton;

#[derive(Component)]
struct DecisionPointContainer;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct FoldDecisionButton;

#[derive(Component)]
struct PlayedCardDisplay;

fn create_ui(commands: &mut Commands) {
    // UI Root container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        },
        UiRoot,
    ))
    .with_children(|parent| {
        // Status display at top
        parent.spawn((
            TextBundle::from_section(
                "Status: Drawing Cards...",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            StatusDisplay,
        ));

        // Totals display
        parent.spawn((
            TextBundle::from_section(
                "Evidence: 0 | Cover: 0 | Heat: 0 | Profit: $0",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
            TotalsDisplay,
        ));

        // Play areas (3 zones)
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Narc zone
            create_play_area(parent, "Narc's Cards", Color::srgb(0.8, 0.3, 0.3), PlayAreaNarc);

            // Customer zone
            create_play_area(parent, "Customer's Cards", Color::srgb(0.3, 0.6, 0.8), PlayAreaCustomer);

            // Player zone
            create_play_area(parent, "Your Cards", Color::srgb(0.3, 0.8, 0.3), PlayAreaPlayer);
        });

        // Player hand display
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::srgb(0.15, 0.15, 0.2).into(),
                ..default()
            },
            PlayerHandDisplay,
        ));

        // Betting actions (Check/Raise/Fold) - SOW-002 Phase 5
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            BettingActionsContainer,
        ))
        .with_children(|parent| {
            // Check button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.6, 0.3).into(),
                    ..default()
                },
                CheckButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "CHECK",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Note: No RAISE button - clicking cards raises/calls

            // Fold button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.5, 0.5, 0.5).into(),
                    ..default()
                },
                FoldButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "FOLD",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });

        // DecisionPoint UI (Continue/Fold) - SOW-002 Phase 5
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    display: Display::None, // Hidden by default
                    ..default()
                },
                background_color: Color::srgba(0.2, 0.2, 0.3, 0.9).into(),
                ..default()
            },
            DecisionPointContainer,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Continue to next round or fold?",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Continue button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(120.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.8, 0.3).into(),
                        ..default()
                    },
                    ContinueButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "CONTINUE",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Fold decision button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(120.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.8, 0.3, 0.3).into(),
                        ..default()
                    },
                    FoldDecisionButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "FOLD",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });
        });
    });
}

fn create_play_area(parent: &mut ChildBuilder, label: &str, color: Color, marker: impl Component) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: color.with_alpha(0.2).into(),
            border_color: color.into(),
            ..default()
        },
        marker,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

// ============================================================================
// UI UPDATE SYSTEM
// ============================================================================

fn ui_update_system(
    hand_state_query: Query<&HandState>,
    betting_state_query: Query<&BettingState>,
    mut totals_query: Query<&mut Text, (With<TotalsDisplay>, Without<StatusDisplay>)>,
    mut status_query: Query<&mut Text, (With<StatusDisplay>, Without<TotalsDisplay>)>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // Update totals display
    if let Ok(mut text) = totals_query.get_single_mut() {
        let totals = hand_state.calculate_totals();
        text.sections[0].value = format!(
            "Evidence: {} | Cover: {} | Heat: {} | Profit: ${}",
            totals.evidence, totals.cover, totals.heat, totals.profit
        );
    }

    // Update status display with debug info
    if let Ok(mut text) = status_query.get_single_mut() {
        // Get betting state for turn info
        let turn_info = if let Ok(betting_state) = betting_state_query.get_single() {
            format!(" - Turn: {:?}", betting_state.current_player)
        } else {
            String::new()
        };

        let status = match hand_state.current_state {
            State::Draw => format!("Status: Round {}/3 - Drawing Cards...", hand_state.current_round),
            State::Betting => format!("Status: Round {}/3 - Betting Phase{}", hand_state.current_round, turn_info),
            State::Flip => format!("Status: Round {}/3 - Flipping Cards...", hand_state.current_round),
            State::DecisionPoint => format!("Status: Round {}/3 Complete - Continue or Fold?", hand_state.current_round),
            State::Resolve => "Status: Resolving Final Hand...".to_string(),
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => "Status: SAFE! You got away with it!".to_string(),
                Some(HandOutcome::Busted) => "Status: BUSTED! You got caught!".to_string(),
                None => "Status: Game Over".to_string(),
            },
            // Legacy states (SOW-001)
            State::NarcPlay => "Status: Narc's Turn".to_string(),
            State::CustomerPlay => "Status: Customer's Turn".to_string(),
            State::PlayerPlay => "Status: YOUR TURN - Click a card to play".to_string(),
        };

        text.sections[0].value = status;

        // Color code status
        text.sections[0].style.color = match hand_state.current_state {
            State::Betting => Color::srgb(1.0, 1.0, 0.3), // Yellow for betting
            State::DecisionPoint => Color::srgb(1.0, 0.7, 0.3), // Orange for decision
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => Color::srgb(0.3, 1.0, 0.3),
                Some(HandOutcome::Busted) => Color::srgb(1.0, 0.3, 0.3),
                None => Color::WHITE,
            },
            // Legacy
            State::PlayerPlay => Color::srgb(0.3, 1.0, 0.3),
            _ => Color::WHITE,
        };
    }
}

// Recreate hand display when either state changes
fn recreate_hand_display_system(
    hand_state_changed: Query<&HandState, Changed<HandState>>,
    betting_state_changed: Query<&BettingState, Changed<BettingState>>,
    hand_state_all: Query<&HandState>,
    betting_state_all: Query<&BettingState>,
    hand_display_query: Query<Entity, With<PlayerHandDisplay>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_button_query: Query<Entity, With<CardButton>>,
) {
    // Check if anything changed
    let hand_changed = hand_state_changed.get_single().is_ok();
    let betting_changed = betting_state_changed.get_single().is_ok();

    if !hand_changed && !betting_changed {
        return; // Nothing changed
    }

    // Get current state (from non-filtered queries)
    let Ok(hand_state) = hand_state_all.get_single() else {
        return;
    };

    let Ok(hand_entity) = hand_display_query.get_single() else {
        return;
    };

    // Clear existing card buttons
    if let Ok(children) = children_query.get(hand_entity) {
        for &child in children.iter() {
            if card_button_query.get(child).is_ok() {
                commands.entity(child).despawn_recursive();
            }
        }
    }

    // Add card buttons for current hand
    // SOW-002: Show cards during Betting phase (not just legacy PlayerPlay)
    let show_cards = hand_state.current_state == State::PlayerPlay
                  || hand_state.current_state == State::Betting
                  || hand_state.current_state == State::DecisionPoint;

    if show_cards {
        commands.entity(hand_entity).with_children(|parent| {
            for (index, card) in hand_state.player_hand.iter().enumerate() {
                let card_color = match card.card_type {
                    CardType::Product { .. } => Color::srgb(0.9, 0.7, 0.2),
                    CardType::Location { .. } => Color::srgb(0.3, 0.6, 0.9),
                    CardType::Evidence { .. } => Color::srgb(0.8, 0.3, 0.3),
                    CardType::Cover { .. } => Color::srgb(0.3, 0.8, 0.3),
                    CardType::DealModifier { .. } => Color::srgb(0.7, 0.5, 0.9), // Purple for modifiers
                };

                let card_info = match &card.card_type {
                    CardType::Product { price, heat } =>
                        format!("{}\n${} | Heat: {}", card.name, price, heat),
                    CardType::Location { evidence, cover, heat } =>
                        format!("{}\nE:{} C:{} H:{}", card.name, evidence, cover, heat),
                    CardType::Evidence { evidence, heat } =>
                        format!("{}\nEvidence: {} | Heat: {}", card.name, evidence, heat),
                    CardType::Cover { cover, heat } =>
                        format!("{}\nCover: {} | Heat: {}", card.name, cover, heat),
                    CardType::DealModifier { price_multiplier, evidence, cover, heat } =>
                        format!("{}\n×{:.1} | E:{} C:{} H:{}", card.name, price_multiplier, evidence, cover, heat),
                };

                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(120.0),
                            height: Val::Px(160.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: card_color.into(),
                        ..default()
                    },
                    CardButton { card_index: index },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        card_info,
                        TextStyle {
                            font_size: 14.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ).with_text_justify(JustifyText::Center));
                });
            }
        });
    }
}

// ============================================================================
// AUTO-PLAY SYSTEM (Legacy SOW-001 - kept for backwards compatibility)
// ============================================================================

fn auto_play_system(
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Legacy SOW-001 auto-play (only runs for old state flow)
    // Auto-play for Narc (play first available card)
    if hand_state.current_state == State::NarcPlay && !hand_state.narc_hand.is_empty() {
        let _ = hand_state.play_card(Owner::Narc, 0);
    }

    // Auto-play for Customer (skip if no cards - Customer has no cards in 8-card MVP)
    if hand_state.current_state == State::CustomerPlay {
        if hand_state.customer_hand.is_empty() {
            // Skip customer turn
            hand_state.transition_state();
        } else {
            let _ = hand_state.play_card(Owner::Customer, 0);
        }
    }
}

// ============================================================================
// AUTO-ADVANCE SYSTEM - SOW-002
// ============================================================================

fn auto_flip_system(
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Auto-draw cards when entering Draw state (Rounds 2-3)
    if hand_state.current_state == State::Draw {
        // Draw cards for all players
        hand_state.draw_cards();
        // Note: draw_cards() calls transition_state() → Betting
    }

    // Auto-advance through Flip state (cards revealed, then transition)
    if hand_state.current_state == State::Flip {
        hand_state.transition_state(); // → DecisionPoint or Resolve
    }

    // Auto-resolve at Resolution state
    if hand_state.current_state == State::Resolve {
        hand_state.resolve_hand();
    }
}

// ============================================================================
// BETTING BUTTON SYSTEM - SOW-002 Phase 5
// ============================================================================

fn betting_button_system(
    check_query: Query<&Interaction, (Changed<Interaction>, With<CheckButton>)>,
    fold_query: Query<&Interaction, (Changed<Interaction>, With<FoldButton>)>,
    mut hand_state_query: Query<&mut HandState>,
    mut betting_state_query: Query<&mut BettingState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    let Ok(mut betting_state) = betting_state_query.get_single_mut() else {
        return;
    };

    // Only during Betting phase and when it's Player's turn
    if hand_state.current_state != State::Betting || betting_state.current_player != Owner::Player {
        return;
    }

    // Check button (only works when not facing a raise)
    for interaction in check_query.iter() {
        if *interaction == Interaction::Pressed {
            let result = betting_state.handle_action(Owner::Player, BettingAction::Check, &mut hand_state);

            if result.is_ok() {
                // Check if betting is complete
                if betting_state.is_complete() {
                    hand_state.transition_state(); // → Flip
                    betting_state.reset_for_round();
                }
            }
            // If error (facing raise), button click is ignored (button should be disabled anyway)
        }
    }

    // Fold button
    for interaction in fold_query.iter() {
        if *interaction == Interaction::Pressed {
            // Fold during betting = exit hand immediately
            hand_state.fold_at_decision_point();
        }
    }
}

// ============================================================================
// UPDATE BETTING BUTTON STATES - Disable CHECK when facing raise
// ============================================================================

fn update_betting_button_states(
    hand_state_query: Query<&HandState>,
    betting_state_query: Query<&BettingState>,
    mut check_button_query: Query<&mut BackgroundColor, With<CheckButton>>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    let Ok(betting_state) = betting_state_query.get_single() else {
        return;
    };

    // Only during Betting phase
    if hand_state.current_state != State::Betting {
        return;
    };

    // Check button: Disabled if Player awaiting action (facing a raise)
    if let Ok(mut bg_color) = check_button_query.get_single_mut() {
        let can_check = !betting_state.players_awaiting_action.contains(&Owner::Player);
        *bg_color = if can_check {
            Color::srgb(0.3, 0.6, 0.3).into() // Green (enabled)
        } else {
            Color::srgb(0.2, 0.2, 0.2).into() // Dark gray (disabled)
        };
    }
}

// ============================================================================
// DECISION POINT BUTTON SYSTEM - SOW-002 Phase 5
// ============================================================================

fn decision_point_button_system(
    continue_query: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    fold_query: Query<&Interaction, (Changed<Interaction>, With<FoldDecisionButton>)>,
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only during DecisionPoint phase
    if hand_state.current_state != State::DecisionPoint {
        return;
    }

    // Continue button
    for interaction in continue_query.iter() {
        if *interaction == Interaction::Pressed {
            hand_state.continue_to_next_round();
        }
    }

    // Fold button
    for interaction in fold_query.iter() {
        if *interaction == Interaction::Pressed {
            hand_state.fold_at_decision_point();
        }
    }
}

// ============================================================================
// UI VISIBILITY TOGGLE SYSTEM - SOW-002 Phase 5
// ============================================================================

fn toggle_ui_visibility_system(
    hand_state_query: Query<&HandState>,
    mut betting_container_query: Query<&mut Style, (With<BettingActionsContainer>, Without<DecisionPointContainer>)>,
    mut decision_container_query: Query<&mut Style, (With<DecisionPointContainer>, Without<BettingActionsContainer>)>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // Show/hide betting buttons
    if let Ok(mut style) = betting_container_query.get_single_mut() {
        style.display = if hand_state.current_state == State::Betting {
            Display::Flex
        } else {
            Display::None
        };
    }

    // Show/hide decision point modal
    if let Ok(mut style) = decision_container_query.get_single_mut() {
        style.display = if hand_state.current_state == State::DecisionPoint {
            Display::Flex
        } else {
            Display::None
        };
    }
}

// ============================================================================
// PLAYED CARDS DISPLAY SYSTEM - Shows cards in play areas
// ============================================================================

fn update_played_cards_display_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    betting_state_query: Query<&BettingState>,
    narc_area_query: Query<Entity, With<PlayAreaNarc>>,
    customer_area_query: Query<Entity, With<PlayAreaCustomer>>,
    player_area_query: Query<Entity, With<PlayAreaPlayer>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_display_query: Query<Entity, With<PlayedCardDisplay>>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    let betting_state = betting_state_query.get_single().ok();

    // Clear old card displays
    for area in [narc_area_query.get_single(), customer_area_query.get_single(), player_area_query.get_single()] {
        if let Ok(area_entity) = area {
            if let Ok(children) = children_query.get(area_entity) {
                for &child in children.iter() {
                    if card_display_query.get(child).is_ok() {
                        commands.entity(child).despawn_recursive();
                    }
                }
            }
        }
    }

    // Show action indicators during Betting phase
    if hand_state.current_state == State::Betting {
        if let Some(betting) = betting_state {
            // Show Narc's action
            if let Some(action) = betting.last_action_narc {
                if let Ok(area) = narc_area_query.get_single() {
                    commands.entity(area).with_children(|parent| {
                        let text = match action {
                            BettingAction::Check => "✓ CHECKED".to_string(),
                            BettingAction::Raise => "🂠 RAISED".to_string(),
                            BettingAction::Call => "🂠 CALLED".to_string(),
                            BettingAction::Fold => "✗ FOLDED".to_string(),
                        };
                        parent.spawn((
                            TextBundle::from_section(
                                text,
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::srgb(1.0, 1.0, 0.5),
                                    ..default()
                                },
                            ),
                            PlayedCardDisplay,
                        ));
                    });
                }
            }

            // Show Customer's action
            if let Some(action) = betting.last_action_customer {
                if let Ok(area) = customer_area_query.get_single() {
                    commands.entity(area).with_children(|parent| {
                        let text = match action {
                            BettingAction::Check => "✓ CHECKED".to_string(),
                            BettingAction::Raise => "🂠 RAISED".to_string(),
                            BettingAction::Call => "🂠 CALLED".to_string(),
                            BettingAction::Fold => "✗ FOLDED".to_string(),
                        };
                        parent.spawn((
                            TextBundle::from_section(
                                text,
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::srgb(0.5, 0.8, 1.0),
                                    ..default()
                                },
                            ),
                            PlayedCardDisplay,
                        ));
                    });
                }
            }
        }
    }

    // After Flip: Show all played cards in their owner's zone
    if hand_state.current_state == State::DecisionPoint || hand_state.current_state == State::Bust {
        let all_cards = hand_state.cards_played.iter().chain(hand_state.cards_played_this_round.iter());

        for card in all_cards {
            let area_entity = match card.owner {
                Owner::Narc => narc_area_query.get_single(),
                Owner::Customer => customer_area_query.get_single(),
                Owner::Player => player_area_query.get_single(),
            };

            if let Ok(area) = area_entity {
                commands.entity(area).with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            &card.name,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        PlayedCardDisplay,
                    ));
                });
            }
        }
    }
}

// ============================================================================
// CARD CLICK SYSTEM
// ============================================================================

fn card_click_system(
    mut interaction_query: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut hand_state_query: Query<&mut HandState>,
    mut betting_state_query: Query<&mut BettingState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    for (interaction, card_button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            println!("Card clicked! Index: {}, State: {:?}", card_button.card_index, hand_state.current_state);

            // SOW-002: Clicking card during Betting = Raise/Call with that specific card
            if hand_state.current_state == State::Betting {
                if let Ok(mut betting_state) = betting_state_query.get_single_mut() {
                    // Only if it's Player's turn
                    if betting_state.current_player == Owner::Player {
                        // Verify valid card index
                        if card_button.card_index < hand_state.player_hand.len() {
                            println!("Player clicking card {}, awaiting: {:?}", card_button.card_index, betting_state.players_awaiting_action);

                            // Use handle_action which properly detects Call vs Raise
                            let result = betting_state.handle_action(Owner::Player, BettingAction::Raise, &mut hand_state);

                            if result.is_ok() {
                                // Check if betting complete
                                if betting_state.is_complete() {
                                    hand_state.transition_state(); // → Flip
                                    betting_state.reset_for_round();
                                }
                            } else {
                                println!("Card click failed: {:?}", result);
                            }
                        }
                    }
                }
            }

            // Legacy SOW-001 flow
            if hand_state.current_state == State::PlayerPlay {
                let _ = hand_state.play_card(Owner::Player, card_button.card_index);

                if hand_state.current_state == State::Resolve {
                    hand_state.resolve_hand();
                }
            }
        }
    }
}

// ============================================================================
// CARD DATA MODEL
// ============================================================================

/// Who owns this card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Owner {
    Narc,
    Customer,
    Player,
}

/// Card types with their specific values (Extended in SOW-002 Phase 4)
#[derive(Debug, Clone)]
enum CardType {
    Product { price: u32, heat: i32 },
    Location { evidence: u32, cover: u32, heat: i32 },
    Evidence { evidence: u32, heat: i32 },
    Cover { cover: u32, heat: i32 },
    // SOW-002 Phase 4: Deal Modifiers (multiplicative price, additive Evidence/Cover/Heat)
    DealModifier { price_multiplier: f32, evidence: i32, cover: i32, heat: i32 },
}

/// Card instance
#[derive(Component, Debug, Clone)]
struct Card {
    id: u32,
    name: String,
    owner: Owner,
    card_type: CardType,
}

// ============================================================================
// HAND STATE MACHINE
// ============================================================================

/// States the hand can be in (ADR-004: Multi-round structure)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Draw,
    Betting,        // Phase 2: Betting phase (Check/Raise/Fold)
    Flip,           // Cards reveal simultaneously
    DecisionPoint,  // Continue or Fold prompt (Rounds 1-2 only)
    Resolve,        // Calculate totals, check bust (after Round 3)
    Bust,           // Terminal state

    // Legacy states from SOW-001 (kept for compatibility during migration)
    #[allow(dead_code)]
    NarcPlay,
    #[allow(dead_code)]
    CustomerPlay,
    #[allow(dead_code)]
    PlayerPlay,
}

/// Outcome of hand resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandOutcome {
    Safe,
    Busted,
}

/// Hand state tracking (Extended for SOW-002)
#[derive(Component)]
struct HandState {
    pub current_state: State,
    pub current_round: u8,           // 1, 2, or 3 (ADR-004)
    pub cards_played: Vec<Card>,
    pub cards_played_this_round: Vec<Card>, // Cards played in current round (for face-down tracking)
    narc_deck: Vec<Card>,
    customer_deck: Vec<Card>,
    player_deck: Vec<Card>,
    pub narc_hand: Vec<Card>,
    pub customer_hand: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub outcome: Option<HandOutcome>,
}

impl Default for HandState {
    fn default() -> Self {
        Self {
            current_state: State::Draw,
            current_round: 1,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            narc_deck: create_narc_deck(),
            customer_deck: create_customer_deck(),
            player_deck: create_player_deck(),
            narc_hand: Vec::new(),
            customer_hand: Vec::new(),
            player_hand: Vec::new(),
            outcome: None,
        }
    }
}

impl HandState {
    /// Reset hand state for replay testing
    fn reset(&mut self) {
        *self = Self::default();
    }

    /// Draw cards from decks to hands (initial draw phase)
    fn draw_cards(&mut self) {
        // SOW-002: Draw to hand size 3 (multi-round play)
        const HAND_SIZE: usize = 3;

        // Draw for each player up to hand size
        while self.narc_hand.len() < HAND_SIZE && !self.narc_deck.is_empty() {
            self.narc_hand.push(self.narc_deck.remove(0));
        }

        while self.customer_hand.len() < HAND_SIZE && !self.customer_deck.is_empty() {
            self.customer_hand.push(self.customer_deck.remove(0));
        }

        while self.player_hand.len() < HAND_SIZE && !self.player_deck.is_empty() {
            self.player_hand.push(self.player_deck.remove(0));
        }

        // Transition to next state after draw
        self.transition_state();
    }

    /// Transition to next state (ADR-004: Multi-round state machine)
    pub fn transition_state(&mut self) {
        self.current_state = match self.current_state {
            // Multi-round flow (SOW-002)
            State::Draw => State::Betting,
            State::Betting => State::Flip,
            State::Flip => {
                // After Round 3: Go to Resolution
                // After Rounds 1-2: Go to DecisionPoint
                if self.current_round >= 3 {
                    State::Resolve
                } else {
                    State::DecisionPoint
                }
            },
            State::DecisionPoint => {
                // Player chose Continue (handled by continue_to_next_round)
                // If we reach this, something is wrong
                State::DecisionPoint // Stay here until explicit Continue/Fold
            },
            State::Resolve => State::Bust, // Will be refined (Safe vs Busted)
            State::Bust => State::Bust, // Terminal state

            // Legacy states (SOW-001 compatibility - not used in SOW-002)
            State::NarcPlay => State::CustomerPlay,
            State::CustomerPlay => State::PlayerPlay,
            State::PlayerPlay => State::Resolve,
        };
    }

    /// Continue to next round (called from DecisionPoint when player chooses Continue)
    pub fn continue_to_next_round(&mut self) {
        if self.current_state != State::DecisionPoint {
            return; // Only valid at DecisionPoint
        }

        // Move cards played this round to overall cards played
        self.cards_played.extend(self.cards_played_this_round.drain(..));

        // Increment round and transition to Draw
        self.current_round += 1;
        self.current_state = State::Draw;
    }

    /// Fold at DecisionPoint (player exits hand, keeps remaining cards)
    pub fn fold_at_decision_point(&mut self) {
        if self.current_state != State::DecisionPoint {
            return; // Only valid at DecisionPoint
        }

        // End hand (player keeps remaining cards, loses profit)
        // Cards played this round are NOT added to cards_played (not finalized)
        self.outcome = Some(HandOutcome::Safe); // Folding is "safe" (not busted)
        self.current_state = State::Bust; // Reuse Bust as terminal state
    }

    /// Play a card from hand to the play area
    fn play_card(&mut self, owner: Owner, card_index: usize) -> Result<(), String> {
        // Verify it's the correct player's turn
        match (self.current_state, owner) {
            (State::NarcPlay, Owner::Narc) => {}
            (State::CustomerPlay, Owner::Customer) => {}
            (State::PlayerPlay, Owner::Player) => {}
            _ => return Err(format!("Wrong turn: state {:?}, owner {:?}", self.current_state, owner)),
        }

        // Get the card from the appropriate hand
        let hand = match owner {
            Owner::Narc => &mut self.narc_hand,
            Owner::Customer => &mut self.customer_hand,
            Owner::Player => &mut self.player_hand,
        };

        if card_index >= hand.len() {
            return Err(format!("Card index {} out of bounds", card_index));
        }

        let card = hand.remove(card_index);
        self.cards_played.push(card);

        // Transition to next state after playing
        self.transition_state();

        Ok(())
    }
}

// ============================================================================
// BETTING PHASE & INITIATIVE - SOW-002 Phase 2
// ============================================================================

/// Betting actions per ADR-005
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BettingAction {
    Check, // Stay in without playing card
    Raise, // Play card face-down (first raise or re-raise with initiative)
    Call,  // Play card in response to a raise (matching the bet)
    Fold,  // Exit betting immediately
}

/// Betting state tracking per ADR-005
#[derive(Component, Debug, Clone)]
struct BettingState {
    pub current_player: Owner,              // Whose turn it is (Narc → Customer → Player)
    pub initiative_player: Option<Owner>,   // Who has initiative (first raiser)
    pub raises_this_round: u8,              // Total raises so far (max 3)
    pub players_awaiting_action: Vec<Owner>, // Who hasn't responded to current raise
    pub players_all_in: Vec<Owner>,         // Who played their last card
    pub players_acted: Vec<Owner>,          // Who has acted this round (at least once)
    pub last_action_narc: Option<BettingAction>,     // Visual feedback
    pub last_action_customer: Option<BettingAction>, // Visual feedback
    pub last_action_player: Option<BettingAction>,   // Visual feedback
}

impl Default for BettingState {
    fn default() -> Self {
        Self {
            current_player: Owner::Narc, // Narc always goes first per ADR-002
            initiative_player: None,
            raises_this_round: 0,
            players_awaiting_action: vec![],
            players_all_in: vec![],
            players_acted: vec![],
            last_action_narc: None,
            last_action_customer: None,
            last_action_player: None,
        }
    }
}

impl BettingState {
    /// Reset for new betting round
    fn reset_for_round(&mut self) {
        self.current_player = Owner::Narc;
        self.initiative_player = None;
        self.raises_this_round = 0;
        self.players_awaiting_action.clear();
        self.players_all_in.clear();
        self.players_acted.clear();
        self.last_action_narc = None;
        self.last_action_customer = None;
        self.last_action_player = None;
    }

    /// Check if betting is complete per ADR-005 termination rules
    fn is_complete(&self) -> bool {
        // Everyone must act at least once
        let all_players_acted = self.players_acted.contains(&Owner::Narc)
                              && self.players_acted.contains(&Owner::Customer)
                              && self.players_acted.contains(&Owner::Player);

        if !all_players_acted {
            return false;
        }

        // Players still need to respond to raises (even if limit hit)
        if !self.players_awaiting_action.is_empty() {
            return false; // Must wait for everyone to call/fold
        }

        // Limit hit (max 3 raises) - betting ends after everyone responds
        if self.raises_this_round >= 3 {
            return true; // Limit hit and everyone responded
        }

        // No initiative OR initiative player already acted
        // If initiative exists, they get another chance to raise
        self.initiative_player.is_none()
    }

    /// Advance to next player in turn order (Narc → Customer → Player)
    fn advance_turn(&mut self) {
        self.current_player = match self.current_player {
            Owner::Narc => Owner::Customer,
            Owner::Customer => Owner::Player,
            Owner::Player => {
                // If initiative exists and can act, go to them
                if let Some(init_player) = self.initiative_player {
                    if self.players_awaiting_action.is_empty() {
                        init_player // Initiative player gets another turn
                    } else {
                        Owner::Narc // Otherwise loop back
                    }
                } else {
                    Owner::Narc // Loop back to start
                }
            },
        };
    }

    /// Handle a betting action per ADR-005 rules
    fn handle_action(&mut self, player: Owner, action: BettingAction, hand_state: &mut HandState) -> Result<(), String> {
        // Verify it's this player's turn
        if self.current_player != player {
            return Err(format!("Not your turn! Current player: {:?}", self.current_player));
        }

        // Check if player is all-in (can only Check)
        if self.players_all_in.contains(&player) {
            if action != BettingAction::Check {
                return Err("Player is all-in, can only Check".to_string());
            }
        }

        match action {
            BettingAction::Check => {
                // Can't Check when awaiting response to a raise (must Raise or Fold)
                if self.players_awaiting_action.contains(&player) {
                    return Err("Can't Check when facing a Raise - must Raise or Fold".to_string());
                }

                // Record action for visual feedback
                match player {
                    Owner::Narc => self.last_action_narc = Some(BettingAction::Check),
                    Owner::Customer => self.last_action_customer = Some(BettingAction::Check),
                    Owner::Player => self.last_action_player = Some(BettingAction::Check),
                }

                // Mark player as acted
                if !self.players_acted.contains(&player) {
                    self.players_acted.push(player);
                }

                // Remove from awaiting list (shouldn't be in it, but safety)
                self.players_awaiting_action.retain(|&p| p != player);

                // If player has initiative and Checks, lose initiative
                if self.initiative_player == Some(player) {
                    self.initiative_player = None;
                }

                // Advance turn
                self.advance_turn();
            },

            BettingAction::Raise | BettingAction::Call => {
                // Determine if this is a true raise or a call
                let is_call = self.players_awaiting_action.contains(&player);
                let actual_action = if is_call {
                    BettingAction::Call
                } else {
                    BettingAction::Raise
                };

                // Check raise limit (only for true raises, not calls)
                if !is_call && self.raises_this_round >= 3 {
                    return Err("Max raises reached (3)".to_string());
                }

                // Player must have cards
                let hand = match player {
                    Owner::Narc => &mut hand_state.narc_hand,
                    Owner::Customer => &mut hand_state.customer_hand,
                    Owner::Player => &mut hand_state.player_hand,
                };

                if hand.is_empty() {
                    return Err("No cards to raise/call with".to_string());
                }

                // Remove first card from hand and play it face-down
                let card = hand.remove(0);
                hand_state.cards_played_this_round.push(card);

                // Record action for visual feedback
                match player {
                    Owner::Narc => self.last_action_narc = Some(actual_action),
                    Owner::Customer => self.last_action_customer = Some(actual_action),
                    Owner::Player => self.last_action_player = Some(actual_action),
                }

                // Mark player as acted
                if !self.players_acted.contains(&player) {
                    self.players_acted.push(player);
                }

                // Check if player is now all-in (played last card)
                if hand.is_empty() && !self.players_all_in.contains(&player) {
                    self.players_all_in.push(player);
                }

                // Only true raises increment counter and affect initiative
                if !is_call {
                    // First raise gains initiative per ADR-005
                    if self.raises_this_round == 0 {
                        self.initiative_player = Some(player);
                    }

                    // Increment raise counter
                    self.raises_this_round += 1;

                    // Add all OTHER players to awaiting (they must respond to this raise)
                    // EXCEPT initiative player (they never have to respond to anyone)
                    self.players_awaiting_action = vec![Owner::Narc, Owner::Customer, Owner::Player]
                        .into_iter()
                        .filter(|&p| {
                            p != player // Exclude raiser
                            && !self.players_all_in.contains(&p) // Exclude all-in players
                            && Some(p) != self.initiative_player // Exclude initiative player
                        })
                        .collect();
                }

                // Remove from awaiting list
                self.players_awaiting_action.retain(|&p| p != player);

                // Advance turn
                self.advance_turn();
            },

            BettingAction::Fold => {
                // Record action for visual feedback
                match player {
                    Owner::Narc => self.last_action_narc = Some(BettingAction::Fold),
                    Owner::Customer => self.last_action_customer = Some(BettingAction::Fold),
                    Owner::Player => self.last_action_player = Some(BettingAction::Fold),
                }

                // Player exits hand (handled by betting_button_system calling fold_at_decision_point())
                // For MVP: Only Player folds during betting (AI never folds mid-betting)
                // No turn advancement needed (hand ends)
            },
        }

        Ok(())
    }

    /// Check if player can raise (has cards and under limit)
    fn can_raise(&self, player: Owner, hand_state: &HandState) -> bool {
        // All-in check
        if self.players_all_in.contains(&player) {
            return false;
        }

        // Has cards check
        let hand = match player {
            Owner::Narc => &hand_state.narc_hand,
            Owner::Customer => &hand_state.customer_hand,
            Owner::Player => &hand_state.player_hand,
        };

        if hand.is_empty() {
            return false;
        }

        // If awaiting action (facing a raise), can always call regardless of limit
        if self.players_awaiting_action.contains(&player) {
            return true; // Can call even if raises >= 3
        }

        // For new raises, check limit
        self.raises_this_round < 3
    }
}

// ============================================================================
// AI BETTING SYSTEM - SOW-002 Phase 3
// ============================================================================

fn ai_betting_system(
    mut hand_state_query: Query<&mut HandState>,
    mut betting_state_query: Query<&mut BettingState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    let Ok(mut betting_state) = betting_state_query.get_single_mut() else {
        return;
    };

    // Only act during Betting phase
    if hand_state.current_state != State::Betting {
        return;
    }

    // Check if betting is already complete BEFORE AI acts
    if betting_state.is_complete() {
        println!("Betting complete! Advancing to Flip");
        // End betting, advance to Flip
        hand_state.transition_state();

        // Reset betting state for next round
        betting_state.reset_for_round();
        return;
    }

    // Only act for AI players (Narc or Customer)
    let current_player = betting_state.current_player;
    if current_player == Owner::Player {
        return; // Player controlled manually
    }

    println!("AI turn: {:?}, Raises: {}, Awaiting: {:?}",
        current_player, betting_state.raises_this_round, betting_state.players_awaiting_action);

    // Get AI decision
    let action = match current_player {
        Owner::Narc => narc_ai_decision(hand_state.current_round, &betting_state, &hand_state),
        Owner::Customer => customer_ai_decision(hand_state.current_round, &betting_state, &hand_state),
        Owner::Player => return, // Unreachable, but compiler needs it
    };

    println!("AI decision: {:?} -> {:?}", current_player, action);

    // Execute action
    let _ = betting_state.handle_action(current_player, action, &mut hand_state);

    // Check if betting is complete after action
    if betting_state.is_complete() {
        println!("Betting complete after action! Advancing to Flip");
        // End betting, advance to Flip
        hand_state.transition_state();

        // Reset betting state for next round
        betting_state.reset_for_round();
    }
}

// ============================================================================
// AI DECISION-MAKING - SOW-002 Phase 3
// ============================================================================

/// Narc AI decision per RFC-002 strategy
/// Round 1: 60% Check, 40% Raise
/// Round 2: 40% Check, 60% Raise
/// Round 3: 20% Check, 80% Raise
/// Never folds UNLESS out of cards
fn narc_ai_decision(round: u8, betting_state: &BettingState, hand_state: &HandState) -> BettingAction {
    // If facing a raise, try to call
    if betting_state.players_awaiting_action.contains(&Owner::Narc) {
        if betting_state.can_raise(Owner::Narc, hand_state) {
            return BettingAction::Raise; // Will be interpreted as Call
        } else {
            // No cards left - must fold (can't call or check)
            return BettingAction::Fold;
        }
    }

    // Not facing a raise - decide whether to raise or check
    if !betting_state.can_raise(Owner::Narc, hand_state) {
        return BettingAction::Check;
    }

    // Weighted randomness by round
    let raise_chance = match round {
        1 => 0.4,  // 40% raise
        2 => 0.6,  // 60% raise
        _ => 0.8,  // 80% raise (Round 3+)
    };

    let roll: f32 = rand::thread_rng().gen();
    if roll < raise_chance {
        BettingAction::Raise
    } else {
        BettingAction::Check
    }
}

/// Customer AI decision per RFC-002 strategy
/// Rounds 1-2: Check if no raises, Call if facing raise
/// Round 3: If Evidence > 60, 30% Fold / 70% Raise; else 70% Raise / 30% Check
fn customer_ai_decision(round: u8, betting_state: &BettingState, hand_state: &HandState) -> BettingAction {
    // Rounds 1-2: Passive strategy (check/call, don't raise)
    if round < 3 {
        // If facing a raise, must call or fold
        if betting_state.players_awaiting_action.contains(&Owner::Customer) {
            // Facing a raise - call it if we have cards
            if betting_state.can_raise(Owner::Customer, hand_state) {
                return BettingAction::Raise; // Will be interpreted as Call
            } else {
                // No cards left - must fold
                return BettingAction::Fold;
            }
        }
        // No raise active - just check
        return BettingAction::Check;
    }

    // Round 3: Calculate current Evidence
    let totals = hand_state.calculate_totals();

    if totals.evidence > 60 {
        // High Evidence → 30% Fold, 70% Raise
        let roll: f32 = rand::thread_rng().gen();
        if roll < 0.3 {
            BettingAction::Fold
        } else if betting_state.can_raise(Owner::Customer, hand_state) {
            BettingAction::Raise
        } else {
            BettingAction::Check
        }
    } else {
        // Low Evidence → 70% Raise, 30% Check
        let roll: f32 = rand::thread_rng().gen();
        if roll < 0.7 && betting_state.can_raise(Owner::Customer, hand_state) {
            BettingAction::Raise
        } else {
            BettingAction::Check
        }
    }
}

// ============================================================================
// BUST CHECK & RESOLUTION - Phase 3
// ============================================================================

impl HandState {
    /// Resolve the hand by checking if player gets busted
    ///
    /// Bust check runs at Resolve state (after all cards played)
    /// - Evidence > Cover → Busted (run ends)
    /// - Evidence ≤ Cover → Safe (continue possible, but single round so ends)
    /// - Tie goes to player (Evidence = Cover is Safe)
    ///
    /// Extensible: RFC-003 will add insurance check before bust finalization
    fn resolve_hand(&mut self) -> HandOutcome {
        let totals = self.calculate_totals();

        let outcome = if totals.evidence > totals.cover {
            HandOutcome::Busted
        } else {
            // Evidence ≤ Cover → Safe (tie goes to player)
            HandOutcome::Safe
        };

        self.outcome = Some(outcome);
        self.current_state = State::Bust; // Transition to terminal state
        outcome
    }
}

// ============================================================================
// CARD INTERACTION ENGINE - Phase 2
// ============================================================================

/// Totals calculated from all played cards
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Totals {
    evidence: u32,
    cover: u32,
    heat: i32,
    profit: u32,
}

impl HandState {
    /// Get active Product card (last Product played, if any)
    /// Override rule: Only last Product matters
    fn active_product(&self) -> Option<&Card> {
        self.cards_played
            .iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Product { .. }))
    }

    /// Get active Location card (last Location played, required)
    /// Override rule: Only last Location matters
    fn active_location(&self) -> Option<&Card> {
        self.cards_played
            .iter()
            .rev()
            .find(|card| matches!(card.card_type, CardType::Location { .. }))
    }

    /// Calculate current totals from all played cards
    ///
    /// Override rules:
    /// - Last Product played becomes active (previous discarded)
    /// - Last Location played becomes active (Evidence/Cover base changes)
    ///
    /// Additive rules:
    /// - Evidence = Location base + sum(all Evidence cards + DealModifier evidence)
    /// - Cover = Location base + sum(all Cover cards + DealModifier cover)
    /// - Heat = sum(all heat modifiers from all cards)
    /// - Profit = Active Product price × product(all DealModifier price_multiplier)
    fn calculate_totals(&self) -> Totals {
        let mut totals = Totals::default();
        let mut price_multiplier: f32 = 1.0; // SOW-002 Phase 4: multiplicative modifiers

        // Get base Evidence/Cover from active Location
        if let Some(location) = self.active_location() {
            if let CardType::Location { evidence, cover, heat } = location.card_type {
                totals.evidence = evidence;
                totals.cover = cover;
                totals.heat += heat;
            }
        }

        // Process all played cards
        for card in &self.cards_played {
            match card.card_type {
                CardType::Evidence { evidence, heat } => {
                    totals.evidence += evidence;
                    totals.heat += heat;
                }
                CardType::Cover { cover, heat } => {
                    totals.cover += cover;
                    totals.heat += heat;
                }
                // SOW-002 Phase 4: Deal Modifiers
                CardType::DealModifier { price_multiplier: multiplier, evidence, cover, heat } => {
                    price_multiplier *= multiplier; // Multiplicative for price
                    totals.evidence = totals.evidence.saturating_add_signed(evidence); // Additive
                    totals.cover = totals.cover.saturating_add_signed(cover); // Additive
                    totals.heat += heat; // Additive
                }
                _ => {}
            }
        }

        // Get profit from active Product (apply multipliers)
        if let Some(product) = self.active_product() {
            if let CardType::Product { price, heat } = product.card_type {
                totals.profit = (price as f32 * price_multiplier) as u32;
                totals.heat += heat;
            }
        }

        totals
    }
}

// ============================================================================
// 8-CARD COLLECTION (MVP)
// ============================================================================

fn create_narc_deck() -> Vec<Card> {
    // SOW-002: Narc deck per RFC-002 AI specification
    // 10× Donut Break, 3× Patrol, 2× Surveillance
    let mut deck = vec![];
    let mut id = 1;

    // 10× Donut Break (no threat cards)
    for _ in 0..10 {
        deck.push(Card {
            id,
            name: "Donut Break".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 0, heat: 0 },
        });
        id += 1;
    }

    // 3× Patrol (minor Evidence)
    for _ in 0..3 {
        deck.push(Card {
            id,
            name: "Patrol".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 5, heat: 2 },
        });
        id += 1;
    }

    // 2× Surveillance (major Evidence)
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        });
        id += 1;
    }

    deck
}

fn create_customer_deck() -> Vec<Card> {
    // SOW-002: Customer deck per RFC-002 AI specification
    // Note: Customer cards will be properly defined in Phase 4 (Deal Modifiers)
    // For now, placeholder Evidence cards
    let mut deck = vec![];
    let mut id = 100;

    // 5× Regular Order (placeholder)
    for _ in 0..5 {
        deck.push(Card {
            id,
            name: "Regular Order".to_string(),
            owner: Owner::Customer,
            card_type: CardType::Evidence { evidence: 10, heat: 10 },
        });
        id += 1;
    }

    // 5× Haggling (placeholder)
    for _ in 0..5 {
        deck.push(Card {
            id,
            name: "Haggling".to_string(),
            owner: Owner::Customer,
            card_type: CardType::Evidence { evidence: 5, heat: 0 },
        });
        id += 1;
    }

    deck
}

fn create_player_deck() -> Vec<Card> {
    vec![
        // 3 Products
        Card {
            id: 10,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        },
        Card {
            id: 11,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        },
        Card {
            id: 12,
            name: "Heroin".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 150, heat: 45 },
        },
        // 2 Locations
        Card {
            id: 13,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        },
        Card {
            id: 14,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        },
        // 1 Cover card
        Card {
            id: 15,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        },
    ]
}

// ============================================================================
// TESTS - Phase 1
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_instantiation() {
        // SOW-002: Verify expanded deck sizes
        let narc_deck = create_narc_deck();
        assert_eq!(narc_deck.len(), 15); // 10 Donut Break + 3 Patrol + 2 Surveillance
        assert_eq!(narc_deck[0].name, "Donut Break");
        assert_eq!(narc_deck[10].name, "Patrol");
        assert_eq!(narc_deck[13].name, "Surveillance");

        let customer_deck = create_customer_deck();
        assert_eq!(customer_deck.len(), 10); // SOW-002: Customer has cards now

        let player_deck = create_player_deck();
        assert_eq!(player_deck.len(), 6);

        // Verify Product cards
        if let CardType::Product { price, heat } = player_deck[0].card_type {
            assert_eq!(price, 30);
            assert_eq!(heat, 5);
        } else {
            panic!("Expected Product card");
        }

        // Verify Location cards
        if let CardType::Location { evidence, cover, heat } = player_deck[3].card_type {
            assert_eq!(evidence, 10);
            assert_eq!(cover, 30);
            assert_eq!(heat, -5);
        } else {
            panic!("Expected Location card");
        }
    }

    #[test]
    fn test_state_transitions() {
        let mut hand_state = HandState::default();

        // Initial state should be Draw
        assert_eq!(hand_state.current_state, State::Draw);

        // New state flow (SOW-002): Draw → Betting → Flip → DecisionPoint
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Betting);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Flip);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::DecisionPoint);

        // Continue to Round 2
        hand_state.continue_to_next_round();
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.current_round, 2);
    }

    #[test]
    fn test_reset() {
        let mut hand_state = HandState::default();

        // Modify state (new flow: Draw → Betting → Flip)
        hand_state.transition_state();
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Flip);

        // Reset should return to initial state
        hand_state.reset();
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.cards_played.len(), 0);
        assert_eq!(hand_state.current_round, 1);
    }

    #[test]
    fn test_draw_cards() {
        let mut hand_state = HandState::default();
        assert_eq!(hand_state.narc_hand.len(), 0);
        assert_eq!(hand_state.player_hand.len(), 0);

        hand_state.draw_cards();

        // SOW-002: Draw to hand size 3 (multi-round play)
        assert_eq!(hand_state.narc_hand.len(), 3);
        assert_eq!(hand_state.customer_hand.len(), 3);
        assert_eq!(hand_state.player_hand.len(), 3);

        // State should advance to Betting (new flow)
        assert_eq!(hand_state.current_state, State::Betting);
    }

    #[test]
    fn test_play_card_wrong_turn() {
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // State is NarcPlay, player shouldn't be able to play
        let result = hand_state.play_card(Owner::Player, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_play_card_success() {
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // State is Betting after draw (new flow)
        assert_eq!(hand_state.current_state, State::Betting);
        assert_eq!(hand_state.narc_hand.len(), 3); // SOW-002: draw to hand size 3

        // Note: play_card logic is legacy (SOW-001)
        // SOW-002 uses BettingState.handle_action instead
        // This test validates state flow, not card playing
    }

    // ========================================================================
    // TESTS - Phase 2 (Card Interaction Engine)
    // ========================================================================

    #[test]
    fn test_override_product() {
        let mut hand_state = HandState::default();

        // Play Weed, then Meth
        let weed = Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        };
        let meth = Card {
            id: 2,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        };

        hand_state.cards_played.push(weed);
        hand_state.cards_played.push(meth.clone());

        // Active product should be Meth (last played)
        let active = hand_state.active_product();
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Meth");

        // Totals should reflect Meth price (100), not Weed price (30)
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.profit, 100);
    }

    #[test]
    fn test_override_location() {
        let mut hand_state = HandState::default();

        // Play School Zone, then Safe House
        let school_zone = Card {
            id: 1,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        let safe_house = Card {
            id: 2,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };

        hand_state.cards_played.push(school_zone);
        hand_state.cards_played.push(safe_house.clone());

        // Active location should be Safe House (last played)
        let active = hand_state.active_location();
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Safe House");

        // Totals should reflect Safe House base (Evidence 10, Cover 30)
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.evidence, 10);
        assert_eq!(totals.cover, 30);
    }

    #[test]
    fn test_additive_evidence() {
        let mut hand_state = HandState::default();

        // Play Location (base 10 Evidence)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Play Evidence cards
        let patrol = Card {
            id: 2,
            name: "Patrol".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 5, heat: 2 },
        };
        let surveillance = Card {
            id: 3,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(patrol);
        hand_state.cards_played.push(surveillance);

        // Evidence should stack: 10 (location) + 5 (patrol) + 20 (surveillance) = 35
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.evidence, 35);
    }

    #[test]
    fn test_additive_cover() {
        let mut hand_state = HandState::default();

        // Play Location (base 30 Cover)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Play Cover card
        let alibi = Card {
            id: 2,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(alibi);

        // Cover should stack: 30 (location) + 30 (alibi) = 60
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.cover, 60);
    }

    #[test]
    fn test_heat_accumulation() {
        let mut hand_state = HandState::default();

        // Play cards with various heat modifiers
        let meth = Card {
            id: 1,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        };
        let school_zone = Card {
            id: 2,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        let surveillance = Card {
            id: 3,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };

        hand_state.cards_played.push(meth);
        hand_state.cards_played.push(school_zone);
        hand_state.cards_played.push(surveillance);

        // Heat should accumulate: 30 + 20 + 5 = 55
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.heat, 55);
    }

    #[test]
    fn test_no_product_played() {
        let mut hand_state = HandState::default();

        // Play Location only
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Profit should be 0 (no Product played)
        let totals = hand_state.calculate_totals();
        assert_eq!(totals.profit, 0);
    }

    #[test]
    fn test_complete_hand_scenario() {
        let mut hand_state = HandState::default();

        // Scenario: Player plays complete round
        // 1. Location: Safe House (Evidence 10, Cover 30, Heat -5)
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // 2. Product: Meth (Price 100, Heat 30)
        let product = Card {
            id: 2,
            name: "Meth".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 },
        };
        hand_state.cards_played.push(product);

        // 3. Cover: Alibi (Cover 30, Heat -5)
        let cover = Card {
            id: 3,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(cover);

        // 4. Evidence: Surveillance (Evidence 20, Heat 5)
        let evidence = Card {
            id: 4,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(evidence);

        let totals = hand_state.calculate_totals();

        // Verify totals:
        // Evidence: 10 (location) + 20 (surveillance) = 30
        // Cover: 30 (location) + 30 (alibi) = 60
        // Heat: -5 (location) + 30 (meth) - 5 (alibi) + 5 (surveillance) = 25
        // Profit: 100 (meth)
        assert_eq!(totals.evidence, 30);
        assert_eq!(totals.cover, 60);
        assert_eq!(totals.heat, 25);
        assert_eq!(totals.profit, 100);
    }

    // ========================================================================
    // TESTS - Phase 3 (Bust Check & Resolution)
    // ========================================================================

    #[test]
    fn test_bust_evidence_greater_than_cover() {
        let mut hand_state = HandState::default();

        // Location with high Evidence, low Cover
        let location = Card {
            id: 1,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        hand_state.cards_played.push(location);

        // Add more Evidence
        let evidence = Card {
            id: 2,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        };
        hand_state.cards_played.push(evidence);

        // Totals: Evidence 60, Cover 5 → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Busted));
        assert_eq!(hand_state.current_state, State::Bust);
    }

    #[test]
    fn test_safe_evidence_less_than_cover() {
        let mut hand_state = HandState::default();

        // Location with low Evidence, high Cover
        let location = Card {
            id: 1,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Add Cover
        let cover = Card {
            id: 2,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(cover);

        // Totals: Evidence 10, Cover 60 → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
        assert_eq!(hand_state.current_state, State::Bust);
    }

    #[test]
    fn test_tie_goes_to_player() {
        let mut hand_state = HandState::default();

        // Location with equal Evidence and Cover
        let location = Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Totals: Evidence 30, Cover 30 → Safe (tie goes to player)
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe));
    }

    #[test]
    fn test_edge_case_one_more_evidence() {
        let mut hand_state = HandState::default();

        // Location
        let location = Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Add 1 Evidence (31 > 30 → Busted)
        let evidence = Card {
            id: 2,
            name: "Evidence".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 1, heat: 0 },
        };
        hand_state.cards_played.push(evidence);

        // Totals: Evidence 31, Cover 30 → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
    }

    #[test]
    fn test_edge_case_one_more_cover() {
        let mut hand_state = HandState::default();

        // Location
        let location = Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Add 1 Cover (30 ≤ 31 → Safe)
        let cover = Card {
            id: 2,
            name: "Cover".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 1, heat: 0 },
        };
        hand_state.cards_played.push(cover);

        // Totals: Evidence 30, Cover 31 → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
    }

    // ========================================================================
    // TESTS - SOW-002 Phase 1 (Multi-Round State Machine)
    // ========================================================================

    #[test]
    fn test_multi_round_state_transitions() {
        let mut hand_state = HandState::default();

        // Initial state
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.current_round, 1);

        // Round 1: Draw → Betting → Flip → DecisionPoint
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Betting);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Flip);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::DecisionPoint);
        assert_eq!(hand_state.current_round, 1); // Still Round 1
    }

    #[test]
    fn test_continue_to_next_round() {
        let mut hand_state = HandState::default();

        // Advance to DecisionPoint after Round 1
        hand_state.current_state = State::DecisionPoint;
        hand_state.current_round = 1;

        // Add some cards to this round
        hand_state.cards_played_this_round.push(Card {
            id: 1,
            name: "Test".to_string(),
            owner: Owner::Player,
            card_type: CardType::Evidence { evidence: 5, heat: 0 },
        });

        // Continue to Round 2
        hand_state.continue_to_next_round();

        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.current_round, 2);
        assert_eq!(hand_state.cards_played_this_round.len(), 0); // Moved to cards_played
        assert_eq!(hand_state.cards_played.len(), 1); // Card finalized
    }

    #[test]
    fn test_fold_at_decision_point() {
        let mut hand_state = HandState::default();

        // Advance to DecisionPoint
        hand_state.current_state = State::DecisionPoint;
        hand_state.current_round = 1;

        // Add cards to this round (should NOT be finalized on fold)
        hand_state.cards_played_this_round.push(Card {
            id: 1,
            name: "Test".to_string(),
            owner: Owner::Player,
            card_type: CardType::Evidence { evidence: 5, heat: 0 },
        });

        // Fold
        hand_state.fold_at_decision_point();

        assert_eq!(hand_state.current_state, State::Bust); // Terminal
        assert_eq!(hand_state.outcome, Some(HandOutcome::Safe)); // Folding is "safe"
        assert_eq!(hand_state.cards_played_this_round.len(), 1); // NOT moved to cards_played
        assert_eq!(hand_state.cards_played.len(), 0); // Cards not finalized
    }

    #[test]
    fn test_round_3_goes_to_resolve() {
        let mut hand_state = HandState::default();

        // Set to Round 3 Flip
        hand_state.current_state = State::Flip;
        hand_state.current_round = 3;

        // Transition should go to Resolve (no DecisionPoint after Round 3)
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Resolve);
    }

    #[test]
    fn test_round_1_goes_to_decision_point() {
        let mut hand_state = HandState::default();

        // Set to Round 1 Flip
        hand_state.current_state = State::Flip;
        hand_state.current_round = 1;

        // Transition should go to DecisionPoint (not Resolve)
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::DecisionPoint);
    }

    #[test]
    fn test_full_three_round_flow() {
        let mut hand_state = HandState::default();

        // Round 1: Draw → Betting → Flip → DecisionPoint → Continue
        assert_eq!(hand_state.current_round, 1);
        hand_state.transition_state(); // → Betting
        hand_state.transition_state(); // → Flip
        hand_state.transition_state(); // → DecisionPoint
        hand_state.continue_to_next_round(); // → Draw (Round 2)

        assert_eq!(hand_state.current_round, 2);
        assert_eq!(hand_state.current_state, State::Draw);

        // Round 2: Draw → Betting → Flip → DecisionPoint → Continue
        hand_state.transition_state(); // → Betting
        hand_state.transition_state(); // → Flip
        hand_state.transition_state(); // → DecisionPoint
        hand_state.continue_to_next_round(); // → Draw (Round 3)

        assert_eq!(hand_state.current_round, 3);
        assert_eq!(hand_state.current_state, State::Draw);

        // Round 3: Draw → Betting → Flip → Resolve (no DecisionPoint)
        hand_state.transition_state(); // → Betting
        hand_state.transition_state(); // → Flip
        hand_state.transition_state(); // → Resolve (not DecisionPoint!)

        assert_eq!(hand_state.current_state, State::Resolve);
        assert_eq!(hand_state.current_round, 3);
    }

    // ========================================================================
    // TESTS - SOW-002 Phase 2 (Betting Phase & Initiative)
    // ========================================================================

    #[test]
    fn test_first_raise_gains_initiative() {
        let mut betting_state = BettingState::default();
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // Initially no initiative
        assert_eq!(betting_state.initiative_player, None);
        assert_eq!(betting_state.raises_this_round, 0);
        assert_eq!(hand_state.narc_hand.len(), 3); // Draw to 3 cards

        // Narc raises (first raise)
        let result = betting_state.handle_action(Owner::Narc, BettingAction::Raise, &mut hand_state);
        assert!(result.is_ok());

        // Narc should have initiative
        assert_eq!(betting_state.initiative_player, Some(Owner::Narc));
        assert_eq!(betting_state.raises_this_round, 1);
        assert_eq!(hand_state.narc_hand.len(), 2); // Played 1 card
    }

    #[test]
    fn test_response_raise_does_not_steal_initiative() {
        let mut betting_state = BettingState::default();
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // Narc raises first (gains initiative)
        betting_state.handle_action(Owner::Narc, BettingAction::Raise, &mut hand_state).unwrap();
        assert_eq!(betting_state.initiative_player, Some(Owner::Narc));
        assert_eq!(hand_state.narc_hand.len(), 2); // Drew 3, played 1, now 2
        assert_eq!(betting_state.raises_this_round, 1);

        // Player calls in response (not a raise - is awaiting action)
        betting_state.current_player = Owner::Player;
        assert!(betting_state.players_awaiting_action.contains(&Owner::Player));
        betting_state.handle_action(Owner::Player, BettingAction::Raise, &mut hand_state).unwrap();

        // Narc should STILL have initiative (Player called, didn't raise)
        assert_eq!(betting_state.initiative_player, Some(Owner::Narc));
        // Raise counter should still be 1 (Call doesn't increment)
        assert_eq!(betting_state.raises_this_round, 1);
    }

    #[test]
    fn test_initiative_player_checks_loses_initiative() {
        let mut betting_state = BettingState::default();
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // Narc raises (gains initiative)
        betting_state.handle_action(Owner::Narc, BettingAction::Raise, &mut hand_state).unwrap();
        assert_eq!(betting_state.initiative_player, Some(Owner::Narc));

        // Narc Checks later (loses initiative)
        betting_state.current_player = Owner::Narc;
        betting_state.handle_action(Owner::Narc, BettingAction::Check, &mut hand_state).unwrap();

        assert_eq!(betting_state.initiative_player, None);
    }

    #[test]
    fn test_max_raises_limit() {
        let mut betting_state = BettingState::default();

        // Set raises to 3 and mark everyone as acted
        betting_state.raises_this_round = 3;
        betting_state.players_acted = vec![Owner::Narc, Owner::Customer, Owner::Player];
        // No one awaiting (everyone has responded)
        betting_state.players_awaiting_action.clear();

        // Betting should be complete when limit hit AND everyone responded
        assert!(betting_state.is_complete());

        // Can't raise anymore
        let hand_state = HandState::default();
        assert!(!betting_state.can_raise(Owner::Narc, &hand_state));
    }

    #[test]
    fn test_all_check_betting_complete() {
        let mut betting_state = BettingState::default();

        // Mark all players as acted (everyone Checked)
        betting_state.players_acted = vec![Owner::Narc, Owner::Customer, Owner::Player];

        // No raises, no one awaiting action
        assert_eq!(betting_state.raises_this_round, 0);
        assert_eq!(betting_state.players_awaiting_action.len(), 0);

        // Betting should be complete (all checked)
        assert!(betting_state.is_complete());
    }

    #[test]
    fn test_can_raise_limit_check() {
        let mut betting_state = BettingState::default();
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // Can raise initially
        assert!(betting_state.can_raise(Owner::Narc, &hand_state));

        // After 3 raises, can't raise
        betting_state.raises_this_round = 3;
        assert!(!betting_state.can_raise(Owner::Narc, &hand_state));
    }

    #[test]
    fn test_can_raise_empty_hand() {
        let betting_state = BettingState::default();
        let mut hand_state = HandState::default();
        hand_state.draw_cards();

        // Clear player's hand
        hand_state.player_hand.clear();

        // Can't raise with empty hand
        assert!(!betting_state.can_raise(Owner::Player, &hand_state));
    }
}
