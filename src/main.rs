// SOW-001: Minimal Playable Hand
// SOW-002: Betting System and AI Opponents
// SOW-006: Deck Building

use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;

// ============================================================================
// SOW-006: GAME STATE AND DECK BUILDER
// ============================================================================

/// Game states for deck building vs gameplay
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    DeckBuilding,  // Pre-run deck selection
    InRun,         // Active gameplay
}

/// Deck builder resource for managing card selection
#[derive(Resource)]
struct DeckBuilder {
    available_cards: Vec<Card>,  // All 20 player cards
    selected_cards: Vec<Card>,   // Chosen cards (10-20)
}

impl DeckBuilder {
    fn new() -> Self {
        let available = create_player_deck();
        let selected = available.clone(); // Default: all 20 cards
        Self {
            available_cards: available,
            selected_cards: selected,
        }
    }

    fn is_valid(&self) -> bool {
        validate_deck(&self.selected_cards).is_ok()
    }

    fn load_preset(&mut self, preset: DeckPreset) {
        self.selected_cards = match preset {
            DeckPreset::Default => self.available_cards.clone(),
            DeckPreset::Aggro => create_aggro_deck(),
            DeckPreset::Control => create_control_deck(),
        };
    }
}

#[derive(Debug, Clone, Copy)]
enum DeckPreset {
    Default,
    Aggro,
    Control,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()  // SOW-006: Add state management
        .insert_resource(DeckBuilder::new())  // SOW-006: Initialize deck builder
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_betting_state)
        .add_systems(Startup, setup_deck_builder)  // SOW-006: Setup deck builder UI
        .add_systems(Update, toggle_game_state_ui_system)  // SOW-006: Show/hide UI based on state (separate to avoid type conflicts)
        .add_systems(Update, (
            auto_play_system,
            ai_betting_system,
            auto_flip_system,
            betting_button_system,
            decision_point_button_system,
            restart_button_system,
            go_home_button_system,
            update_betting_button_states,
            update_restart_button_states,
            toggle_ui_visibility_system,
            update_played_cards_display_system,
            recreate_hand_display_system,
            ui_update_system,
            card_click_system,
            deck_builder_card_click_system,  // SOW-006: Card selection
            preset_button_system,  // SOW-006: Preset buttons
            start_run_button_system,  // SOW-006: Start run button
            update_deck_builder_ui_system,  // SOW-006: Update deck stats
            populate_deck_builder_cards_system,  // SOW-006: Populate card displays
        ).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // SOW-006: Don't spawn HandState at startup - only when START RUN is pressed
    // HandState will be created when transitioning from DeckBuilding to InRun

    // Create gameplay UI root (initially hidden)
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
struct BustContainer;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct FoldDecisionButton;

// Restart buttons (appear at end of hand) - SOW-004
#[derive(Component)]
struct RestartButton; // "NEW DEAL" button

#[derive(Component)]
struct GoHomeButton; // "GO HOME" button

#[derive(Component)]
struct PlayedCardDisplay;

// SOW-006: Deck Builder UI components
#[derive(Component)]
struct DeckBuilderRoot;

#[derive(Component)]
struct CardPoolContainer;

#[derive(Component)]
struct SelectedDeckContainer;

#[derive(Component)]
struct DeckStatsDisplay;

#[derive(Component)]
struct DeckBuilderCardButton {
    card_id: u32,
    in_pool: bool,  // true = in available pool, false = in selected deck
}

#[derive(Component)]
struct PresetButton {
    preset: DeckPreset,
}

#[derive(Component)]
struct StartRunButton;

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
                "Round complete - Review cards played",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Continue button only (no fold - SOW-004)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
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
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });

        // Restart UI (appears at Bust state) - SOW-004: NEW DEAL / GO HOME
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    display: Display::None, // Hidden by default
                    ..default()
                },
                ..default()
            },
            BustContainer,
        ))
        .with_children(|parent| {
            // New Deal button (continue run)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.8, 0.3).into(),
                    ..default()
                },
                RestartButton, // Reuse RestartButton for "NEW DEAL"
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "NEW DEAL",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // End Run / Go Home button (context-dependent)
            // Safe/Folded: "GO HOME" (voluntary quit)
            // Busted: "END RUN" (forced end)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.8, 0.3, 0.3).into(),
                    ..default()
                },
                GoHomeButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "GO HOME", // Will be updated by update_restart_button_states
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
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
        // Only include current round cards after Flip (when cards are revealed)
        let include_current_round = matches!(
            hand_state.current_state,
            State::Flip | State::DecisionPoint | State::Resolve | State::Bust
        );
        let totals = hand_state.calculate_totals(include_current_round);
        text.sections[0].value = format!(
            "Evidence: {} | Cover: {} | Heat: {} | Profit: ${}\nCash: ${} | Total Heat: {} | Deck: {} cards",
            totals.evidence, totals.cover, totals.heat, totals.profit,
            hand_state.cash, hand_state.current_heat, hand_state.player_deck.len()
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

        let mut status = match hand_state.current_state {
            State::Draw => format!("Status: Round {}/3 - Drawing Cards...", hand_state.current_round),
            State::Betting => format!("Status: Round {}/3 - Betting Phase{}", hand_state.current_round, turn_info),
            State::Flip => format!("Status: Round {}/3 - Flipping Cards...", hand_state.current_round),
            State::DecisionPoint => format!("Status: Round {}/3 Complete - Continue or Fold?", hand_state.current_round),
            State::Resolve => "Status: Resolving Final Hand...".to_string(),
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => {
                    // Check if this was deck exhaustion
                    if hand_state.player_deck.len() < 3 {
                        format!("Status: Deck Exhausted ({} cards) - Run Ends", hand_state.player_deck.len())
                    } else {
                        "Status: SAFE! You got away with it!".to_string()
                    }
                }
                Some(HandOutcome::Busted) => {
                    // Check if this was deck exhaustion (not a real bust)
                    if hand_state.player_deck.len() < 3 {
                        format!("Status: Deck Exhausted ({} cards) - Run Ends", hand_state.player_deck.len())
                    } else {
                        "Status: BUSTED! You got caught!".to_string()
                    }
                }
                Some(HandOutcome::Folded) => {
                    // Show who folded
                    if let Ok(betting_state) = betting_state_query.get_single() {
                        if betting_state.last_action_narc == Some(BettingAction::Fold) {
                            "Status: Hand Ended - Narc Folded".to_string()
                        } else if betting_state.last_action_customer == Some(BettingAction::Fold) {
                            "Status: Hand Ended - Customer Folded".to_string()
                        } else {
                            "Status: Hand Ended - You Folded".to_string()
                        }
                    } else {
                        "Status: Hand Ended - Fold".to_string()
                    }
                }
                None => "Status: Game Over".to_string(),
            },
            // Legacy states (SOW-001)
            State::NarcPlay => "Status: Narc's Turn".to_string(),
            State::CustomerPlay => "Status: Customer's Turn".to_string(),
            State::PlayerPlay => "Status: YOUR TURN - Click a card to play".to_string(),
        };

        // SOW-003 Phase 5: Add Insurance/Conviction status info
        // SOW-005: Show after first reveal (persists for rest of hand)
        // Check if any cards have been finalized (moved to cards_played from previous rounds)
        let cards_ever_revealed = !hand_state.cards_played.is_empty()
            || matches!(hand_state.current_state, State::Flip | State::DecisionPoint | State::Resolve | State::Bust);

        if cards_ever_revealed {
            // Use include_current_round=true to check all cards (finalized + current)
            if let Some(insurance) = hand_state.active_insurance(true) {
                if let CardType::Insurance { cost, heat_penalty, .. } = insurance.card_type {
                    status.push_str(&format!("\nInsurance: {} (Cost: ${}, Heat: +{})", insurance.name, cost, heat_penalty));
                }
            }

            if let Some(conviction) = hand_state.active_conviction(true) {
                if let CardType::Conviction { heat_threshold } = conviction.card_type {
                    let at_risk = hand_state.current_heat >= heat_threshold;
                    if at_risk {
                        status.push_str(&format!("\n⚠️ CONVICTION ACTIVE: {} - Threshold: {} (Heat: {}) - INSURANCE WON'T WORK!",
                            conviction.name, heat_threshold, hand_state.current_heat));
                    } else {
                        status.push_str(&format!("\nConviction: {} - Threshold: {} (Heat: {})",
                            conviction.name, heat_threshold, hand_state.current_heat));
                    }
                }
            }
        }

        text.sections[0].value = status;

        // Color code status
        text.sections[0].style.color = match hand_state.current_state {
            State::Betting => Color::srgb(1.0, 1.0, 0.3), // Yellow for betting
            State::DecisionPoint => Color::srgb(1.0, 0.7, 0.3), // Orange for decision
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => Color::srgb(0.3, 1.0, 0.3),
                Some(HandOutcome::Busted) => Color::srgb(1.0, 0.3, 0.3),
                Some(HandOutcome::Folded) => Color::srgb(0.7, 0.7, 0.7), // Gray for fold
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
                    CardType::Insurance { .. } => Color::srgb(0.2, 0.8, 0.8), // Cyan for insurance
                    CardType::Conviction { .. } => Color::srgb(0.9, 0.2, 0.2), // Red for conviction
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
                    CardType::Insurance { cover, cost, heat_penalty } =>
                        format!("{}\nCover: {} | Cost: ${} | Heat: {}", card.name, cover, cost, heat_penalty),
                    CardType::Conviction { heat_threshold } =>
                        format!("{}\nThreshold: {}", card.name, heat_threshold),
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
            let result = betting_state.handle_action(Owner::Player, BettingAction::Check, &mut hand_state, None);

            if result.is_ok() {
                // Check if betting is complete
                if betting_state.is_complete() {
                    hand_state.transition_state(); // → Flip
                    betting_state.reset_for_round();
                }
            }
            // If error (facing raise), button click is ignored (button should be disabled)
        }
    }

    // Fold button
    for interaction in fold_query.iter() {
        if *interaction == Interaction::Pressed {
            // SOW-004: Fold penalty now handled in handle_action()
            let _result = betting_state.handle_action(Owner::Player, BettingAction::Fold, &mut hand_state, None);
            // handle_action sets outcome and state to Bust
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
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only during DecisionPoint phase
    if hand_state.current_state != State::DecisionPoint {
        return;
    }

    // Continue button - advance to next round
    for interaction in continue_query.iter() {
        if *interaction == Interaction::Pressed {
            hand_state.continue_to_next_round();
        }
    }
}

// ============================================================================
// RESTART BUTTON SYSTEM - Restart hand after bust/safe
// ============================================================================

fn restart_button_system(
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut hand_state_query: Query<&mut HandState>,
    mut betting_state_query: Query<&mut BettingState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != State::Bust {
        return;
    }

    // NEW DEAL button - only for Safe/Folded outcomes (not Busted)
    for interaction in restart_query.iter() {
        if *interaction == Interaction::Pressed {
            // SOW-005: Can't new deal if busted (game over)
            if matches!(hand_state.outcome, Some(HandOutcome::Busted)) {
                return; // Button should be hidden, but ignore click if somehow pressed
            }

            // Check if deck is exhausted
            if hand_state.player_deck.len() < 3 {
                // Button disabled, ignore click
                return;
            }

            // Start next hand (preserve cash/heat)
            let _can_continue = hand_state.start_next_hand();

            // Reset betting state
            if let Ok(mut betting_state) = betting_state_query.get_single_mut() {
                *betting_state = BettingState::default();
            }
        }
    }
}

// ============================================================================
// UPDATE RESTART BUTTON STATES - Disable NEW DEAL when deck exhausted
// ============================================================================

fn update_restart_button_states(
    hand_state_query: Query<&HandState>,
    mut restart_button_query: Query<(&mut BackgroundColor, &mut Visibility), With<RestartButton>>,
    go_home_button_query: Query<(Entity, &Children), With<GoHomeButton>>,
    mut text_query: Query<&mut Text>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != State::Bust {
        return;
    }

    let is_busted = matches!(hand_state.outcome, Some(HandOutcome::Busted));

    // NEW DEAL button: Hide if busted, disable if deck exhausted
    if let Ok((mut bg_color, mut visibility)) = restart_button_query.get_single_mut() {
        if is_busted {
            // Busted: Hide NEW DEAL button entirely
            *visibility = Visibility::Hidden;
        } else {
            // Safe/Folded: Show NEW DEAL, disable if deck exhausted
            *visibility = Visibility::Visible;
            let can_deal = hand_state.player_deck.len() >= 3;
            *bg_color = if can_deal {
                Color::srgb(0.3, 0.8, 0.3).into() // Green (enabled)
            } else {
                Color::srgb(0.2, 0.2, 0.2).into() // Dark gray (disabled)
            };
        }
    }

    // GO HOME button text: "GO HOME" if safe, "END RUN" if busted
    if let Ok((button_entity, children)) = go_home_button_query.get_single() {
        for &child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.sections[0].value = if is_busted {
                    "END RUN".to_string()
                } else {
                    "GO HOME".to_string()
                };
            }
        }
    }
}

// ============================================================================
// GO HOME BUTTON SYSTEM - Reset run completely
// ============================================================================

fn go_home_button_system(
    mut commands: Commands,
    go_home_query: Query<&Interaction, (Changed<Interaction>, With<GoHomeButton>)>,
    hand_state_query: Query<(Entity, &HandState)>,
    mut betting_state_query: Query<&mut BettingState>,
    mut deck_builder: ResMut<DeckBuilder>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((entity, hand_state)) = hand_state_query.get_single() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != State::Bust {
        return;
    }

    // Go Home button - return to deck builder
    for interaction in go_home_query.iter() {
        if *interaction == Interaction::Pressed {
            // Despawn HandState
            commands.entity(entity).despawn();

            // Reset betting state
            if let Ok(mut betting_state) = betting_state_query.get_single_mut() {
                *betting_state = BettingState::default();
            }

            // Reset deck builder to Default preset
            deck_builder.load_preset(DeckPreset::Default);

            // Transition back to DeckBuilding state
            next_state.set(GameState::DeckBuilding);
        }
    }
}

// ============================================================================
// SOW-006: DECK BUILDER SYSTEMS
// ============================================================================

/// Setup deck builder UI (called at startup)
fn setup_deck_builder(mut commands: Commands) {
    // Deck builder root container (initially visible, game starts in DeckBuilding state)
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
            ..default()
        },
        DeckBuilderRoot,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "DECK BUILDER",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Main content area (horizontal split: pool | selected)
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Left: Card pool
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    ..default()
                },
                CardPoolContainer,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Available Cards (click to add to deck)",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Right: Selected deck
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.25, 0.15).into(),
                    ..default()
                },
                SelectedDeckContainer,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Selected Deck",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });

        // Bottom: Stats and actions
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Deck stats
            parent.spawn((
                TextBundle::from_section(
                    "Deck: 20/20 cards",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                DeckStatsDisplay,
            ));

            // Preset buttons
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Default preset
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    },
                    PresetButton { preset: DeckPreset::Default },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "DEFAULT (20)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Aggro preset
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.5, 0.2, 0.2).into(),
                        ..default()
                    },
                    PresetButton { preset: DeckPreset::Aggro },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "AGGRO (10)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

                // Control preset
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.5).into(),
                        ..default()
                    },
                    PresetButton { preset: DeckPreset::Control },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "CONTROL (16)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });

            // START RUN button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                    ..default()
                },
                StartRunButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "START RUN",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    });
}

/// Handle deck builder card clicks (add/remove from deck)
fn deck_builder_card_click_system(
    interaction_query: Query<(&Interaction, &DeckBuilderCardButton), Changed<Interaction>>,
    mut deck_builder: ResMut<DeckBuilder>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Find the card in available cards
            let Some(card) = deck_builder.available_cards.iter()
                .find(|c| c.id == button.card_id)
                .cloned() else {
                continue;
            };

            // Check if card is already in selected deck
            let is_selected = deck_builder.selected_cards.iter()
                .any(|c| c.id == button.card_id);

            if is_selected {
                // Remove from selected deck
                deck_builder.selected_cards.retain(|c| c.id != button.card_id);
            } else {
                // Add to selected deck (if under max)
                if deck_builder.selected_cards.len() < 20 {
                    deck_builder.selected_cards.push(card);
                }
            }
        }
    }
}

/// Handle preset button clicks
fn preset_button_system(
    interaction_query: Query<(&Interaction, &PresetButton), Changed<Interaction>>,
    mut deck_builder: ResMut<DeckBuilder>,
) {
    for (interaction, preset_btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            deck_builder.load_preset(preset_btn.preset);
        }
    }
}

/// Handle START RUN button click
fn start_run_button_system(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartRunButton>)>,
    deck_builder: Res<DeckBuilder>,
    mut next_state: ResMut<NextState<GameState>>,
    hand_state_query: Query<Entity, With<HandState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed && deck_builder.is_valid() {
            // Despawn any existing HandState
            for entity in hand_state_query.iter() {
                commands.entity(entity).despawn();
            }

            // Create new HandState with selected deck
            let mut hand_state = HandState::with_custom_deck(deck_builder.selected_cards.clone());
            hand_state.draw_cards();
            commands.spawn(hand_state);

            // Transition to InRun state
            next_state.set(GameState::InRun);
        }
    }
}

/// Update deck builder UI (stats display)
fn update_deck_builder_ui_system(
    deck_builder: Res<DeckBuilder>,
    mut stats_query: Query<&mut Text, With<DeckStatsDisplay>>,
) {
    if !deck_builder.is_changed() {
        return;
    }

    for mut text in stats_query.iter_mut() {
        let count = deck_builder.selected_cards.len();
        let validation = validate_deck(&deck_builder.selected_cards);

        let is_valid = validation.is_ok();
        text.sections[0].value = match validation {
            Ok(_) => format!("Deck: {}/20 cards ✓ VALID", count),
            Err(msg) => format!("Deck: {}/20 cards ✗ {}", count, msg),
        };

        text.sections[0].style.color = if is_valid {
            Color::srgb(0.2, 0.8, 0.2)
        } else {
            Color::srgb(0.8, 0.2, 0.2)
        };
    }
}

/// Populate deck builder card displays (recreate when deck changes)
fn populate_deck_builder_cards_system(
    mut commands: Commands,
    deck_builder: Res<DeckBuilder>,
    pool_container_query: Query<Entity, With<CardPoolContainer>>,
    selected_container_query: Query<Entity, With<SelectedDeckContainer>>,
    card_button_query: Query<Entity, With<DeckBuilderCardButton>>,
) {
    if !deck_builder.is_changed() {
        return;
    }

    // Clear existing card buttons from both containers
    for entity in card_button_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Populate card pool (available cards)
    if let Ok(pool_entity) = pool_container_query.get_single() {
        commands.entity(pool_entity).with_children(|parent| {
            for card in &deck_builder.available_cards {
                let is_selected = deck_builder.selected_cards.iter().any(|c| c.id == card.id);
                let bg_color = if is_selected {
                    Color::srgb(0.3, 0.5, 0.3) // Green if selected
                } else {
                    Color::srgb(0.3, 0.3, 0.3) // Gray if not selected
                };

                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: bg_color.into(),
                        ..default()
                    },
                    DeckBuilderCardButton {
                        card_id: card.id,
                        in_pool: true,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("{}", card.name),
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            }
        });
    }

    // Populate selected deck
    if let Ok(selected_entity) = selected_container_query.get_single() {
        commands.entity(selected_entity).with_children(|parent| {
            for card in &deck_builder.selected_cards {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.4, 0.2).into(),
                        ..default()
                    },
                    DeckBuilderCardButton {
                        card_id: card.id,
                        in_pool: false,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("{}", card.name),
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            }
        });
    }
}

// ============================================================================
// UI VISIBILITY TOGGLE SYSTEM - SOW-002 Phase 5
// ============================================================================

fn toggle_ui_visibility_system(
    hand_state_query: Query<&HandState>,
    mut betting_container_query: Query<&mut Style, (With<BettingActionsContainer>, Without<DecisionPointContainer>, Without<BustContainer>)>,
    mut decision_container_query: Query<&mut Style, (With<DecisionPointContainer>, Without<BettingActionsContainer>, Without<BustContainer>)>,
    mut bust_container_query: Query<&mut Style, (With<BustContainer>, Without<BettingActionsContainer>, Without<DecisionPointContainer>)>,
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

    // Show/hide restart button (at Bust state)
    if let Ok(mut style) = bust_container_query.get_single_mut() {
        style.display = if hand_state.current_state == State::Bust {
            Display::Flex
        } else {
            Display::None
        };
    }
}

// ============================================================================
// SOW-006: GAME STATE UI VISIBILITY - Show deck builder or gameplay UI
// ============================================================================

fn toggle_game_state_ui_system(
    current_state: Res<bevy::state::state::State<GameState>>,
    mut deck_builder_query: Query<&mut Style, (With<DeckBuilderRoot>, Without<UiRoot>)>,
    mut gameplay_ui_query: Query<&mut Style, (With<UiRoot>, Without<DeckBuilderRoot>)>,
) {
    // Show deck builder in DeckBuilding state, hide in InRun
    if let Ok(mut style) = deck_builder_query.get_single_mut() {
        style.display = if current_state.get() == &GameState::DeckBuilding {
            Display::Flex
        } else {
            Display::None
        };
    }

    // Show gameplay UI in InRun state, hide in DeckBuilding
    if let Ok(mut style) = gameplay_ui_query.get_single_mut() {
        style.display = if current_state.get() == &GameState::InRun {
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

    // Show finalized cards from previous rounds (always visible once flipped)
    // During Betting: Show cards_played (from previous rounds)
    // After Flip/DecisionPoint/Bust: Show all cards
    let show_all_cards = matches!(hand_state.current_state, State::Flip | State::DecisionPoint | State::Resolve | State::Bust);

    if show_all_cards || hand_state.current_state == State::Betting {
        let cards_to_show: Vec<&Card> = if show_all_cards {
            // Show both finalized and current round
            hand_state.cards_played.iter().chain(hand_state.cards_played_this_round.iter()).collect()
        } else {
            // During Betting: Only show finalized (previous rounds)
            hand_state.cards_played.iter().collect()
        };

        for card in cards_to_show {

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

                            // Use handle_action which properly detects Call vs Raise, passing the clicked card index
                            let result = betting_state.handle_action(Owner::Player, BettingAction::Raise, &mut hand_state, Some(card_button.card_index));

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

/// Card types with their specific values (Extended in SOW-002/003)
#[derive(Debug, Clone)]
enum CardType {
    Product { price: u32, heat: i32 },
    Location { evidence: u32, cover: u32, heat: i32 },
    Evidence { evidence: u32, heat: i32 },
    Cover { cover: u32, heat: i32 },
    // SOW-002 Phase 4: Deal Modifiers (multiplicative price, additive Evidence/Cover/Heat)
    DealModifier { price_multiplier: f32, evidence: i32, cover: i32, heat: i32 },
    // SOW-003 Phase 1: Insurance (Cover + bust activation)
    Insurance { cover: u32, cost: u32, heat_penalty: i32 },
    // SOW-003 Phase 2: Conviction (Heat threshold, overrides insurance)
    Conviction { heat_threshold: u32 },
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
    Folded, // SOW-004: Hand ended by fold (not bust, not completed)
}

/// Hand state tracking (Extended for SOW-002/003)
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
    // SOW-003: Cash and Heat tracking
    pub cash: u32,           // Cumulative profit for insurance affordability
    pub current_heat: u32,   // Cumulative Heat for conviction thresholds
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
            cash: 0,          // SOW-003: Start with no cash
            current_heat: 0,  // SOW-003: Start with no Heat
        }
    }
}

impl HandState {
    /// Create HandState with a custom player deck (SOW-006)
    fn with_custom_deck(player_deck: Vec<Card>) -> Self {
        Self {
            current_state: State::Draw,
            current_round: 1,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            narc_deck: create_narc_deck(),
            customer_deck: create_customer_deck(),
            player_deck,  // Use custom deck
            narc_hand: Vec::new(),
            customer_hand: Vec::new(),
            player_hand: Vec::new(),
            outcome: None,
            cash: 0,
            current_heat: 0,
        }
    }

    /// Reset hand state for replay testing
    fn reset(&mut self) {
        *self = Self::default();
    }

    /// Shuffle unplayed hand cards back into deck (SOW-004: Card retention)
    /// Called at end of hand to return ONLY unplayed cards to deck
    /// Played cards are "spent" and discarded (not returned)
    fn shuffle_cards_back(&mut self) {
        // Player: Return only unplayed hand cards to deck
        self.player_deck.extend(self.player_hand.drain(..));
        self.player_deck.shuffle(&mut rand::thread_rng());

        // Narc: Return only unplayed hand cards to deck
        self.narc_deck.extend(self.narc_hand.drain(..));
        self.narc_deck.shuffle(&mut rand::thread_rng());

        // Customer: Return only unplayed hand cards to deck
        self.customer_deck.extend(self.customer_hand.drain(..));
        self.customer_deck.shuffle(&mut rand::thread_rng());

        // Played cards are discarded (not shuffled back)
        self.cards_played.clear();
        self.cards_played_this_round.clear();
    }

    /// Start next hand in the run (preserve cash/heat, shuffle cards back)
    /// Used after Safe outcome to continue the run
    /// Returns true if hand can start, false if deck exhausted
    fn start_next_hand(&mut self) -> bool {
        let preserved_cash = self.cash;
        let preserved_heat = self.current_heat;

        // SOW-004: Shuffle cards back into deck before resetting
        self.shuffle_cards_back();

        // SOW-004: Preserve decks (they've been modified by shuffle-back and fold penalties)
        let preserved_player_deck = self.player_deck.clone();
        let preserved_narc_deck = self.narc_deck.clone();
        let preserved_customer_deck = self.customer_deck.clone();

        // SOW-004 Phase 3: Check deck exhaustion before starting new hand
        if preserved_player_deck.len() < 3 {
            // Deck exhausted - cannot start new hand
            self.outcome = Some(HandOutcome::Busted);
            self.current_state = State::Bust;
            // Don't reset - keep preserved decks to show deck size in UI
            self.player_deck = preserved_player_deck;
            self.narc_deck = preserved_narc_deck;
            self.customer_deck = preserved_customer_deck;
            return false;
        }

        // Reset state but preserve cash/heat/decks
        *self = Self::default();
        self.cash = preserved_cash;
        self.current_heat = preserved_heat;
        self.player_deck = preserved_player_deck;
        self.narc_deck = preserved_narc_deck;
        self.customer_deck = preserved_customer_deck;
        true
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
            // Multi-round flow (SOW-002/004)
            State::Draw => State::Betting,
            State::Betting => State::Flip,
            State::Flip => {
                // SOW-004: Show round resolution at DecisionPoint
                if self.current_round >= 3 {
                    // After Round 3: Go to Resolution
                    State::Resolve
                } else {
                    // After Rounds 1-2: Go to DecisionPoint to show round results
                    State::DecisionPoint
                }
            },
            State::DecisionPoint => {
                // SOW-004: DecisionPoint removed, but keep for backwards compatibility
                State::DecisionPoint
            },
            State::Resolve => State::Bust, // Will be refined (Safe vs Busted)
            State::Bust => State::Bust, // Terminal state

            // Legacy states (SOW-001 compatibility - not used in SOW-002)
            State::NarcPlay => State::CustomerPlay,
            State::CustomerPlay => State::PlayerPlay,
            State::PlayerPlay => State::Resolve,
        };
    }

    /// Continue to next round (called from DecisionPoint after reviewing round results)
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
        // SOW-005: Don't clear all_in - persists for entire hand (can't get more cards)
        // self.players_all_in.clear();
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
    /// card_index: Optional card index for Raise/Call (None = use index 0)
    fn handle_action(&mut self, player: Owner, action: BettingAction, hand_state: &mut HandState, card_index: Option<usize>) -> Result<(), String> {
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

                // Remove clicked card from hand and play it face-down
                let index = card_index.unwrap_or(0); // Default to first card if no index specified
                if index >= hand.len() {
                    return Err(format!("Invalid card index: {}", index));
                }
                let card = hand.remove(index);
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

                // SOW-005: Check if player is now all-in (hand AND deck empty - can't draw more)
                let deck = match player {
                    Owner::Narc => &hand_state.narc_deck,
                    Owner::Customer => &hand_state.customer_deck,
                    Owner::Player => &hand_state.player_deck,
                };

                if hand.is_empty() && deck.is_empty() && !self.players_all_in.contains(&player) {
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

                // SOW-004: Apply fold penalty (remove 1 random card from folder's deck)
                let deck = match player {
                    Owner::Narc => &mut hand_state.narc_deck,
                    Owner::Customer => &mut hand_state.customer_deck,
                    Owner::Player => &mut hand_state.player_deck,
                };
                if !deck.is_empty() {
                    let idx = rand::thread_rng().gen_range(0..deck.len());
                    deck.remove(idx);
                }

                // SOW-004: Fold ends hand with Folded outcome
                hand_state.outcome = Some(HandOutcome::Folded);
                hand_state.current_state = State::Bust; // Terminal state
            },
        }

        Ok(())
    }

    /// Check if player can raise (has cards and under limit)
    fn can_raise(&self, player: Owner, hand_state: &HandState) -> bool {
        // Has cards check
        let hand = match player {
            Owner::Narc => &hand_state.narc_hand,
            Owner::Customer => &hand_state.customer_hand,
            Owner::Player => &hand_state.player_hand,
        };

        if hand.is_empty() {
            return false;
        }

        // If awaiting action (facing a raise), can always call regardless of all-in or limit
        if self.players_awaiting_action.contains(&player) {
            return true; // Can call even if raises >= 3 or someone is all-in
        }

        // SOW-005: If anyone is all-in (hand+deck empty), no NEW raises allowed
        if !self.players_all_in.is_empty() {
            return false;
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

    // Execute action (AI uses None for card_index, plays first card)
    let _ = betting_state.handle_action(current_player, action, &mut hand_state, None);

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
/// SOW-005: Never folds (narc has no bust risk)
fn narc_ai_decision(round: u8, betting_state: &BettingState, hand_state: &HandState) -> BettingAction {
    // If no cards, can only Check (all-in)
    if !betting_state.can_raise(Owner::Narc, hand_state) {
        return BettingAction::Check;
    }

    // If facing a raise, call it
    if betting_state.players_awaiting_action.contains(&Owner::Narc) {
        return BettingAction::Raise; // Will be interpreted as Call
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
/// Round 3: Aggressive (70% Raise, 30% Check)
/// SOW-005: Never folds (customer has no bust risk)
fn customer_ai_decision(round: u8, betting_state: &BettingState, hand_state: &HandState) -> BettingAction {
    // If no cards, can only Check (all-in)
    if !betting_state.can_raise(Owner::Customer, hand_state) {
        return BettingAction::Check;
    }

    // Rounds 1-2: Passive strategy (check/call, don't raise)
    if round < 3 {
        // If facing a raise, call it
        if betting_state.players_awaiting_action.contains(&Owner::Customer) {
            return BettingAction::Raise; // Will be interpreted as Call
        }
        // No raise active - just check
        return BettingAction::Check;
    }

    // Round 3: Aggressive (70% Raise, 30% Check)
    // SOW-005: Never folds - customer has no bust risk
    let roll: f32 = rand::thread_rng().gen();
    if roll < 0.7 && betting_state.can_raise(Owner::Customer, hand_state) {
        BettingAction::Raise
    } else {
        BettingAction::Check
    }
}

// ============================================================================
// BUST CHECK & RESOLUTION - Phase 3
// ============================================================================

impl HandState {
    /// Resolve hand outcome (bust check with insurance/conviction - SOW-003 Phase 3)
    ///
    /// Resolution Order (per ADR-003):
    /// 1. Evidence ≤ Cover → Safe (no insurance/conviction checks needed)
    /// 2. Evidence > Cover → Check Conviction:
    ///    - Conviction active AND current_heat >= threshold → Busted (override insurance)
    /// 3. Check Insurance:
    ///    - No insurance → Busted
    ///    - Has insurance, can't afford → Busted
    ///    - Has insurance, can afford → Pay cost, gain heat_penalty, burn insurance → Safe
    ///
    /// Post-resolution:
    /// - Safe outcome: Bank profit to cash (for future insurance affordability)
    /// - All outcomes: Accumulate totals.heat to current_heat (for conviction thresholds)
    fn resolve_hand(&mut self) -> HandOutcome {
        let totals = self.calculate_totals(true); // Always include all cards at resolution

        // Calculate projected heat (current heat + this hand's heat)
        // This is what heat will be AFTER this hand, used for conviction checks
        let projected_heat = self.current_heat.saturating_add(totals.heat as u32);

        // Step 1: Evidence ≤ Cover → Safe (tie goes to player)
        let outcome = if totals.evidence <= totals.cover {
            HandOutcome::Safe
        } else {
            // Evidence > Cover → Potential bust, check insurance/conviction

            // Step 2: Check Conviction override (using PROJECTED heat after this hand)
            if let Some(conviction) = self.active_conviction(true) {
                if let CardType::Conviction { heat_threshold } = conviction.card_type {
                    if projected_heat >= heat_threshold {
                        // Conviction overrides insurance - run ends
                        HandOutcome::Busted
                    } else {
                        // Heat below threshold, conviction doesn't activate
                        self.try_insurance_activation()
                    }
                } else {
                    self.try_insurance_activation()
                }
            } else {
                // No conviction active
                self.try_insurance_activation()
            }
        };

        // Post-resolution: Accumulate cash and heat
        match outcome {
            HandOutcome::Safe => {
                // Bank profit to cash (for future insurance purchases)
                self.cash += totals.profit;
            }
            HandOutcome::Busted => {
                // No cash gained on bust
            }
            HandOutcome::Folded => {
                // Unreachable - folding handled in handle_action(), not resolve_hand()
                // But added for exhaustive matching
            }
        }

        // Always accumulate heat (regardless of outcome)
        // SOW-005: Handle negative heat (can't go below 0)
        if totals.heat >= 0 {
            self.current_heat = self.current_heat.saturating_add(totals.heat as u32);
        } else {
            self.current_heat = self.current_heat.saturating_sub((-totals.heat) as u32);
        }

        self.outcome = Some(outcome);
        self.current_state = State::Bust; // Transition to terminal state
        outcome
    }

    /// Try to activate insurance (Step 3 of resolution order)
    ///
    /// Returns:
    /// - Safe: Insurance activated (cost paid, heat gained, card burned)
    /// - Busted: No insurance OR can't afford
    fn try_insurance_activation(&mut self) -> HandOutcome {
        // Extract insurance values first to avoid borrow issues
        let insurance_info = self.active_insurance(true).and_then(|insurance| {
            if let CardType::Insurance { cost, heat_penalty, .. } = insurance.card_type {
                Some((insurance.name.clone(), cost, heat_penalty))
            } else {
                None
            }
        });

        if let Some((insurance_name, cost, heat_penalty)) = insurance_info {
            // Check affordability
            if self.cash >= cost {
                // Activate insurance: pay cost, gain heat penalty
                self.cash -= cost;
                self.current_heat = self.current_heat.saturating_add(heat_penalty as u32);

                // Burn insurance card (remove from deck permanently)
                self.player_deck.retain(|card| card.name != insurance_name);

                HandOutcome::Safe
            } else {
                // Can't afford insurance
                HandOutcome::Busted
            }
        } else {
            // No insurance active
            HandOutcome::Busted
        }
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
    /// include_current_round: Whether to include face-down cards from this round
    fn active_product(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        cards.into_iter().rev().find(|card| matches!(card.card_type, CardType::Product { .. }))
    }

    /// Get active Location card (last Location played, required)
    /// Override rule: Only last Location matters
    /// include_current_round: Whether to include face-down cards from this round
    fn active_location(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        cards.into_iter().rev().find(|card| matches!(card.card_type, CardType::Location { .. }))
    }

    /// Get active Insurance card (last Insurance played, if any)
    /// Override rule: Only last Insurance matters (SOW-003 Phase 1)
    /// include_current_round: Whether to include face-down cards from this round
    fn active_insurance(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        cards.into_iter().rev().find(|card| matches!(card.card_type, CardType::Insurance { .. }))
    }

    /// Get active Conviction card (last Conviction played, if any)
    /// Override rule: Only last Conviction matters (SOW-003 Phase 2)
    /// include_current_round: Whether to include face-down cards from this round
    fn active_conviction(&self, include_current_round: bool) -> Option<&Card> {
        let cards: Vec<&Card> = if include_current_round {
            self.cards_played.iter().chain(self.cards_played_this_round.iter()).collect()
        } else {
            self.cards_played.iter().collect()
        };

        cards.into_iter().rev().find(|card| matches!(card.card_type, CardType::Conviction { .. }))
    }

    /// Calculate current totals from all played cards
    ///
    /// Override rules:
    /// - Last Product played becomes active (previous discarded)
    /// - Last Location played becomes active (Evidence/Cover base changes)
    /// - Last Insurance played becomes active (SOW-003)
    /// - Last Conviction played becomes active (SOW-003)
    ///
    /// Additive rules:
    /// - Evidence = Location base + sum(all Evidence cards + DealModifier evidence)
    /// - Cover = Location base + sum(all Cover cards + Insurance cover + DealModifier cover)
    /// - Heat = sum(all heat modifiers from all cards)
    /// - Profit = Active Product price × product(all DealModifier price_multiplier)
    ///
    /// Special rules (SOW-003):
    /// - Insurance acts as Cover card during totals calculation
    /// - Conviction has no effect on totals (only affects bust resolution)
    ///
    /// Multi-round (SOW-002):
    /// - include_current_round: Whether to include cards_played_this_round (after Flip) or not (during Betting)
    fn calculate_totals(&self, include_current_round: bool) -> Totals {
        let mut totals = Totals::default();
        let mut price_multiplier: f32 = 1.0; // SOW-002 Phase 4: multiplicative modifiers

        // Get base Evidence/Cover from active Location
        if let Some(location) = self.active_location(include_current_round) {
            if let CardType::Location { evidence, cover, heat } = location.card_type {
                totals.evidence = evidence;
                totals.cover = cover;
                totals.heat += heat;
            }
        }

        // Process finalized cards from previous rounds (always included)
        let cards_to_process: Box<dyn Iterator<Item = &Card>> = if include_current_round {
            // After Flip: Include both finalized and current round cards
            Box::new(self.cards_played.iter().chain(self.cards_played_this_round.iter()))
        } else {
            // During Betting: Only finalized cards (current round is face-down)
            Box::new(self.cards_played.iter())
        };

        for card in cards_to_process {
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
                // SOW-003 Phase 1: Insurance acts as Cover (dual function)
                CardType::Insurance { cover, .. } => {
                    totals.cover += cover; // Adds to Cover total (like Cover cards)
                    // Note: heat_penalty only applies when insurance activates on bust
                }
                // SOW-003 Phase 2: Conviction has no effect on totals
                CardType::Conviction { .. } => {
                    // No effect on totals - only affects bust resolution
                }
                _ => {}
            }
        }

        // Get profit from active Product (apply multipliers)
        if let Some(product) = self.active_product(include_current_round) {
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
    // SOW-005: Narc deck (25 cards - Law Enforcement Theme)
    let mut deck = vec![];
    let mut id = 1;

    // 17 Evidence cards (varied threat levels)
    // 8× Low threat
    for _ in 0..8 {
        deck.push(Card {
            id,
            name: "Donut Break".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 0, heat: 0 },
        });
        id += 1;
    }

    // 6× Medium threat
    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Patrol".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 5, heat: 2 },
        });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Surveillance".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 20, heat: 5 },
        });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card {
            id,
            name: "Stakeout".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Evidence { evidence: 10, heat: 3 },
        });
        id += 1;
    }

    // 3× High threat
    deck.push(Card {
        id,
        name: "Undercover Op".to_string(),
        owner: Owner::Narc,
        card_type: CardType::Evidence { evidence: 30, heat: 10 },
    });
    id += 1;

    deck.push(Card {
        id,
        name: "Raid".to_string(),
        owner: Owner::Narc,
        card_type: CardType::Evidence { evidence: 40, heat: 20 },
    });
    id += 1;

    deck.push(Card {
        id,
        name: "Wiretap".to_string(),
        owner: Owner::Narc,
        card_type: CardType::Evidence { evidence: 35, heat: 15 },
    });
    id += 1;

    // 8 Conviction cards (SOW-005: Moved from player deck)
    for _ in 0..4 {
        deck.push(Card {
            id,
            name: "Warrant".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });
        id += 1;
    }

    for _ in 0..3 {
        deck.push(Card {
            id,
            name: "DA Approval".to_string(),
            owner: Owner::Narc,
            card_type: CardType::Conviction { heat_threshold: 60 },
        });
        id += 1;
    }

    deck.push(Card {
        id,
        name: "RICO Case".to_string(),
        owner: Owner::Narc,
        card_type: CardType::Conviction { heat_threshold: 80 },
    });

    // Shuffle deck for variety
    deck.shuffle(&mut rand::thread_rng());
    deck
}

fn create_customer_deck() -> Vec<Card> {
    // SOW-005: Customer deck (25 cards - Deal Dynamics Theme)
    let mut deck = vec![];
    let mut id = 100;

    // 5 Products (customer requests)
    deck.push(Card { id: id, name: "Weed Request".to_string(), owner: Owner::Customer,
        card_type: CardType::Product { price: 30, heat: 5 } });
    id += 1;

    deck.push(Card { id: id, name: "Meth Request".to_string(), owner: Owner::Customer,
        card_type: CardType::Product { price: 100, heat: 30 } });
    id += 1;

    deck.push(Card { id: id, name: "Heroin Request".to_string(), owner: Owner::Customer,
        card_type: CardType::Product { price: 150, heat: 45 } });
    id += 1;

    deck.push(Card { id: id, name: "Cocaine Request".to_string(), owner: Owner::Customer,
        card_type: CardType::Product { price: 120, heat: 35 } });
    id += 1;

    deck.push(Card { id: id, name: "Pills Request".to_string(), owner: Owner::Customer,
        card_type: CardType::Product { price: 80, heat: 20 } });
    id += 1;

    // 5 Locations (customer venue preferences)
    deck.push(Card { id: id, name: "Park".to_string(), owner: Owner::Customer,
        card_type: CardType::Location { evidence: 20, cover: 25, heat: -5 } });
    id += 1;

    deck.push(Card { id: id, name: "Nightclub".to_string(), owner: Owner::Customer,
        card_type: CardType::Location { evidence: 35, cover: 15, heat: 15 } });
    id += 1;

    deck.push(Card { id: id, name: "Apartment".to_string(), owner: Owner::Customer,
        card_type: CardType::Location { evidence: 10, cover: 30, heat: -10 } });
    id += 1;

    deck.push(Card { id: id, name: "Office".to_string(), owner: Owner::Customer,
        card_type: CardType::Location { evidence: 15, cover: 30, heat: 0 } });
    id += 1;

    deck.push(Card { id: id, name: "Parking Garage".to_string(), owner: Owner::Customer,
        card_type: CardType::Location { evidence: 25, cover: 20, heat: 5 } });
    id += 1;

    // 15 Deal Modifiers (negotiation, terms, conditions)
    // Price modifiers
    for _ in 0..2 {
        deck.push(Card { id: id, name: "Bulk Order".to_string(), owner: Owner::Customer,
            card_type: CardType::DealModifier { price_multiplier: 1.5, evidence: 20, cover: 0, heat: 15 } });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card { id: id, name: "Quick Sale".to_string(), owner: Owner::Customer,
            card_type: CardType::DealModifier { price_multiplier: 0.8, evidence: -10, cover: 0, heat: -5 } });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card { id: id, name: "Haggling".to_string(), owner: Owner::Customer,
            card_type: CardType::DealModifier { price_multiplier: 0.7, evidence: -5, cover: 0, heat: 0 } });
        id += 1;
    }

    // Evidence/Cover modifiers
    for _ in 0..2 {
        deck.push(Card { id: id, name: "Cash Upfront".to_string(), owner: Owner::Customer,
            card_type: CardType::DealModifier { price_multiplier: 1.2, evidence: 0, cover: 10, heat: 5 } });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card { id: id, name: "On Credit".to_string(), owner: Owner::Customer,
            card_type: CardType::DealModifier { price_multiplier: 0.9, evidence: 15, cover: 0, heat: -5 } });
        id += 1;
    }

    for _ in 0..2 {
        deck.push(Card { id: id, name: "Trusted Regular".to_string(), owner: Owner::Customer,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: -10, cover: 15, heat: -10 } });
        id += 1;
    }

    for _ in 0..3 {
        deck.push(Card { id: id, name: "Fair Deal".to_string(), owner: Owner::Customer,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 0, heat: 0 } });
        id += 1;
    }

    // Shuffle deck for variety
    deck.shuffle(&mut rand::thread_rng());
    deck
}

fn create_player_deck() -> Vec<Card> {
    let mut deck = vec![
        // SOW-005: Player Deck (20 cards - Dealer Theme)
        // 5 Products
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
        Card {
            id: 13,
            name: "Cocaine".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 120, heat: 35 },
        },
        Card {
            id: 14,
            name: "Fentanyl".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 200, heat: 50 },
        },
        // 4 Locations
        Card {
            id: 15,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        },
        Card {
            id: 16,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        },
        Card {
            id: 17,
            name: "Warehouse".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 15, cover: 25, heat: -10 },
        },
        Card {
            id: 18,
            name: "Back Alley".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 25, cover: 20, heat: 0 },
        },
        // 4 Cover cards
        Card {
            id: 19,
            name: "Alibi".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 },
        },
        Card {
            id: 20,
            name: "Bribe".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 25, heat: 10 },
        },
        Card {
            id: 21,
            name: "Fake Receipts".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 20, heat: 5 },
        },
        Card {
            id: 22,
            name: "Bribed Witness".to_string(),
            owner: Owner::Player,
            card_type: CardType::Cover { cover: 15, heat: -10 },
        },
        // 2 Insurance cards
        Card {
            id: 23,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 },
        },
        Card {
            id: 24,
            name: "Fake ID".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 },
        },
        // 5 Deal Modifiers (defensive focus)
        Card {
            id: 25,
            name: "Disguise".to_string(),
            owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 },
        },
        Card {
            id: 26,
            name: "Burner Phone".to_string(),
            owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 },
        },
        Card {
            id: 27,
            name: "Lookout".to_string(),
            owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 },
        },
        Card {
            id: 28,
            name: "Clean Money".to_string(),
            owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 0.9, evidence: 0, cover: 10, heat: -15 },
        },
        Card {
            id: 29,
            name: "False Trail".to_string(),
            owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: -10, cover: 15, heat: -5 },
        },
    ];

    // Shuffle deck for variety
    deck.shuffle(&mut rand::thread_rng());
    deck
}

// ============================================================================
// SOW-006: DECK VALIDATION AND PRESETS
// ============================================================================

/// Validate deck meets all constraints
fn validate_deck(deck: &[Card]) -> Result<(), String> {
    // Check size constraints
    if deck.len() < 10 {
        return Err(format!("Need {} more cards (minimum 10)", 10 - deck.len()));
    }
    if deck.len() > 20 {
        return Err("Maximum 20 cards".to_string());
    }

    // Check required card types
    let has_product = deck.iter().any(|c| matches!(c.card_type, CardType::Product { .. }));
    let has_location = deck.iter().any(|c| matches!(c.card_type, CardType::Location { .. }));

    if !has_product {
        return Err("Need at least 1 Product card".to_string());
    }
    if !has_location {
        return Err("Need at least 1 Location card".to_string());
    }

    Ok(())
}

/// Create aggro preset deck (12-14 cards: high-profit, risky, minimal defense)
fn create_aggro_deck() -> Vec<Card> {
    vec![
        // High-profit products (5)
        Card { id: 10, name: "Weed".to_string(), owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 } },
        Card { id: 11, name: "Meth".to_string(), owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 } },
        Card { id: 12, name: "Heroin".to_string(), owner: Owner::Player,
            card_type: CardType::Product { price: 150, heat: 45 } },
        Card { id: 13, name: "Cocaine".to_string(), owner: Owner::Player,
            card_type: CardType::Product { price: 120, heat: 35 } },
        Card { id: 14, name: "Fentanyl".to_string(), owner: Owner::Player,
            card_type: CardType::Product { price: 200, heat: 50 } },
        // Risky locations (2)
        Card { id: 16, name: "School Zone".to_string(), owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 } },
        Card { id: 18, name: "Back Alley".to_string(), owner: Owner::Player,
            card_type: CardType::Location { evidence: 25, cover: 20, heat: 0 } },
        // Minimal defense - just 1 Cover
        Card { id: 19, name: "Alibi".to_string(), owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 } },
        // 2 modifiers (to reach minimum 10 cards)
        Card { id: 25, name: "Disguise".to_string(), owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 } },
        Card { id: 27, name: "Lookout".to_string(), owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 } },
    ]
}

/// Create control preset deck (15-18 cards: heavy defense, safe locations)
fn create_control_deck() -> Vec<Card> {
    vec![
        // Conservative products (2)
        Card { id: 10, name: "Weed".to_string(), owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 } },
        Card { id: 11, name: "Meth".to_string(), owner: Owner::Player,
            card_type: CardType::Product { price: 100, heat: 30 } },
        // Safe locations (3)
        Card { id: 15, name: "Safe House".to_string(), owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 } },
        Card { id: 17, name: "Warehouse".to_string(), owner: Owner::Player,
            card_type: CardType::Location { evidence: 15, cover: 25, heat: -10 } },
        Card { id: 18, name: "Back Alley".to_string(), owner: Owner::Player,
            card_type: CardType::Location { evidence: 25, cover: 20, heat: 0 } },
        // All Cover cards (4)
        Card { id: 19, name: "Alibi".to_string(), owner: Owner::Player,
            card_type: CardType::Cover { cover: 30, heat: -5 } },
        Card { id: 20, name: "Bribe".to_string(), owner: Owner::Player,
            card_type: CardType::Cover { cover: 25, heat: 10 } },
        Card { id: 21, name: "Fake Receipts".to_string(), owner: Owner::Player,
            card_type: CardType::Cover { cover: 20, heat: 5 } },
        Card { id: 22, name: "Bribed Witness".to_string(), owner: Owner::Player,
            card_type: CardType::Cover { cover: 15, heat: -10 } },
        // All Insurance (2)
        Card { id: 23, name: "Plea Bargain".to_string(), owner: Owner::Player,
            card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 } },
        Card { id: 24, name: "Fake ID".to_string(), owner: Owner::Player,
            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 } },
        // All defensive modifiers (5)
        Card { id: 25, name: "Disguise".to_string(), owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 } },
        Card { id: 26, name: "Burner Phone".to_string(), owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 } },
        Card { id: 27, name: "Lookout".to_string(), owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 } },
        Card { id: 28, name: "Clean Money".to_string(), owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 0.9, evidence: 0, cover: 10, heat: -15 } },
        Card { id: 29, name: "False Trail".to_string(), owner: Owner::Player,
            card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: -10, cover: 15, heat: -5 } },
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
        // SOW-005: Verify rebalanced deck sizes
        let narc_deck = create_narc_deck();
        assert_eq!(narc_deck.len(), 25); // 17 Evidence + 8 Conviction

        // Verify deck composition (shuffled, so can't check specific positions)
        let evidence_count = narc_deck.iter().filter(|c| matches!(c.card_type, CardType::Evidence { .. })).count();
        let conviction_count = narc_deck.iter().filter(|c| matches!(c.card_type, CardType::Conviction { .. })).count();
        assert_eq!(evidence_count, 17); // Variety of threat levels
        assert_eq!(conviction_count, 8); // Moved from player + new

        let customer_deck = create_customer_deck();
        assert_eq!(customer_deck.len(), 25); // SOW-005: Expanded thematic deck

        // Verify customer deck composition
        let customer_product_count = customer_deck.iter().filter(|c| matches!(c.card_type, CardType::Product { .. })).count();
        let customer_location_count = customer_deck.iter().filter(|c| matches!(c.card_type, CardType::Location { .. })).count();
        let customer_modifier_count = customer_deck.iter().filter(|c| matches!(c.card_type, CardType::DealModifier { .. })).count();
        assert_eq!(customer_product_count, 5); // Customer requests
        assert_eq!(customer_location_count, 5); // Customer venues
        assert_eq!(customer_modifier_count, 15); // Deal terms

        let player_deck = create_player_deck();
        assert_eq!(player_deck.len(), 20); // SOW-005: Rebalanced dealer deck

        // Verify deck has expected card types (can't check positions due to shuffling)
        let product_count = player_deck.iter().filter(|c| matches!(c.card_type, CardType::Product { .. })).count();
        let location_count = player_deck.iter().filter(|c| matches!(c.card_type, CardType::Location { .. })).count();
        let cover_count = player_deck.iter().filter(|c| matches!(c.card_type, CardType::Cover { .. })).count();
        let insurance_count = player_deck.iter().filter(|c| matches!(c.card_type, CardType::Insurance { .. })).count();
        let conviction_count = player_deck.iter().filter(|c| matches!(c.card_type, CardType::Conviction { .. })).count();
        let modifier_count = player_deck.iter().filter(|c| matches!(c.card_type, CardType::DealModifier { .. })).count();
        let evidence_count = player_deck.iter().filter(|c| matches!(c.card_type, CardType::Evidence { .. })).count();

        // SOW-005: Thematic dealer deck
        assert_eq!(product_count, 5); // Weed, Meth, Heroin, Cocaine, Fentanyl
        assert_eq!(location_count, 4); // Safe House, School Zone, Warehouse, Back Alley
        assert_eq!(cover_count, 4); // Alibi, Bribe, Fake Receipts, Bribed Witness
        assert_eq!(insurance_count, 2); // Plea Bargain, Fake ID
        assert_eq!(conviction_count, 0); // Moved to Narc deck
        assert_eq!(modifier_count, 5); // Disguise, Burner Phone, Lookout, Clean Money, False Trail
        assert_eq!(evidence_count, 0); // Removed (self-harming)
    }

    #[test]
    fn test_state_transitions() {
        let mut hand_state = HandState::default();

        // Initial state should be Draw
        assert_eq!(hand_state.current_state, State::Draw);

        // New state flow (SOW-004): Draw → Betting → Flip → DecisionPoint
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Betting);

        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Flip);

        hand_state.transition_state();
        // SOW-004: DecisionPoint shows round results (with Continue button only)
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
        let active = hand_state.active_product(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Meth");

        // Totals should reflect Meth price (100), not Weed price (30)
        let totals = hand_state.calculate_totals(true);
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
        let active = hand_state.active_location(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Safe House");

        // Totals should reflect Safe House base (Evidence 10, Cover 30)
        let totals = hand_state.calculate_totals(true);
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
        let totals = hand_state.calculate_totals(true);
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
        let totals = hand_state.calculate_totals(true);
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
        let totals = hand_state.calculate_totals(true);
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
        let totals = hand_state.calculate_totals(true);
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

        let totals = hand_state.calculate_totals(true);

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
        // SOW-004: DecisionPoint shows round results
        assert_eq!(hand_state.current_state, State::DecisionPoint);
        assert_eq!(hand_state.current_round, 1); // Still Round 1 until Continue
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

        // SOW-004: Transition goes to DecisionPoint to show round results
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

        // Round 3: Draw → Betting → Flip → Resolve
        hand_state.transition_state(); // → Betting
        hand_state.transition_state(); // → Flip
        hand_state.transition_state(); // → Resolve

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
        let result = betting_state.handle_action(Owner::Narc, BettingAction::Raise, &mut hand_state, None);
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
        betting_state.handle_action(Owner::Narc, BettingAction::Raise, &mut hand_state, None).unwrap();
        assert_eq!(betting_state.initiative_player, Some(Owner::Narc));
        assert_eq!(hand_state.narc_hand.len(), 2); // Drew 3, played 1, now 2
        assert_eq!(betting_state.raises_this_round, 1);

        // Player calls in response (not a raise - is awaiting action)
        betting_state.current_player = Owner::Player;
        assert!(betting_state.players_awaiting_action.contains(&Owner::Player));
        betting_state.handle_action(Owner::Player, BettingAction::Raise, &mut hand_state, None).unwrap();

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
        betting_state.handle_action(Owner::Narc, BettingAction::Raise, &mut hand_state, None).unwrap();
        assert_eq!(betting_state.initiative_player, Some(Owner::Narc));

        // Narc Checks later (loses initiative)
        betting_state.current_player = Owner::Narc;
        betting_state.handle_action(Owner::Narc, BettingAction::Check, &mut hand_state, None).unwrap();

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

    // ========================================================================
    // TESTS - SOW-003 (Insurance and Conviction Mechanics)
    // ========================================================================

    #[test]
    fn test_insurance_acts_as_cover() {
        let mut hand_state = HandState::default();

        // Location: E:20 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 20, cover: 20, heat: 0 },
        });

        // Insurance: +15 Cover
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Fake ID".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 },
        });

        // Totals: E:20 C:35 (20 + 15 from insurance)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 20);
        assert_eq!(totals.cover, 35); // Insurance adds to cover
        assert_eq!(totals.heat, 0); // Insurance heat penalty only applies on activation, not in totals
    }

    #[test]
    fn test_conviction_no_effect_on_totals() {
        let mut hand_state = HandState::default();

        // Location: E:20 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 20, cover: 20, heat: 0 },
        });

        // Conviction: No effect on totals
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Warrant".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Totals: E:20 C:20 (conviction doesn't change anything)
        let totals = hand_state.calculate_totals(true);
        assert_eq!(totals.evidence, 20);
        assert_eq!(totals.cover, 20);
        assert_eq!(totals.heat, 0);
    }

    #[test]
    fn test_insurance_activation_affordable() {
        let mut hand_state = HandState::default();
        hand_state.cash = 1500; // Have enough cash

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Cost $1000, Heat +20
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Evidence > Cover, but insurance should save us
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.cash, 500); // 1500 - 1000
        assert_eq!(hand_state.current_heat, 20); // Heat penalty applied
    }

    #[test]
    fn test_insurance_activation_unaffordable() {
        let mut hand_state = HandState::default();
        hand_state.cash = 500; // Not enough cash

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Cost $1000 (too expensive)
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Evidence > Cover, can't afford insurance → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.cash, 500); // Cash unchanged (didn't pay)
    }

    #[test]
    fn test_insurance_no_insurance_busts() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000; // Have cash, but no insurance

        // Bust scenario: E:30 C:20, no insurance card
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Evidence > Cover, no insurance → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
    }

    #[test]
    fn test_conviction_overrides_insurance() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000; // Can afford insurance
        hand_state.current_heat = 50; // Heat above threshold

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Available and affordable
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Conviction: Threshold 40 (we're at 50, so it activates)
        hand_state.cards_played.push(Card {
            id: 3,
            name: "Warrant".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Evidence > Cover, conviction overrides insurance → Busted
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.cash, 2000); // Cash unchanged (conviction blocked insurance)
    }

    #[test]
    fn test_conviction_below_threshold_insurance_works() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000; // Can afford insurance
        hand_state.current_heat = 30; // Heat below threshold

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Available and affordable
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Conviction: Threshold 40 (we're at 30, so it doesn't activate)
        hand_state.cards_played.push(Card {
            id: 3,
            name: "Warrant".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Evidence > Cover, heat < threshold, insurance works → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.cash, 1000); // Paid insurance
        assert_eq!(hand_state.current_heat, 50); // 30 + 20 penalty
    }

    #[test]
    fn test_conviction_at_threshold_activates() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000;
        hand_state.current_heat = 40; // Exactly at threshold

        // Bust scenario
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        hand_state.cards_played.push(Card {
            id: 2,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        hand_state.cards_played.push(Card {
            id: 3,
            name: "Warrant".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Heat >= threshold, conviction activates (boundary test)
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
    }

    #[test]
    fn test_cash_accumulation_safe_hands() {
        let mut hand_state = HandState::default();
        hand_state.cash = 100; // Starting cash

        // Safe scenario with profit
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 20, cover: 30, heat: 0 },
        });

        hand_state.cards_played.push(Card {
            id: 2,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 50, heat: 5 },
        });

        // Safe outcome, profit should be banked
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        assert_eq!(hand_state.cash, 150); // 100 + 50 profit
    }

    #[test]
    fn test_cash_not_gained_on_bust() {
        let mut hand_state = HandState::default();
        hand_state.cash = 100;

        // Bust scenario with profit (but won't get it)
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        hand_state.cards_played.push(Card {
            id: 2,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 50, heat: 5 },
        });

        // Bust outcome, no profit gained
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        assert_eq!(hand_state.cash, 100); // Cash unchanged
    }

    #[test]
    fn test_heat_accumulation_across_hands() {
        let mut hand_state = HandState::default();
        hand_state.current_heat = 10; // Starting heat

        // Safe hand with heat
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 20, cover: 30, heat: 15 },
        });

        hand_state.cards_played.push(Card {
            id: 2,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 50, heat: 5 },
        });

        hand_state.resolve_hand();

        // Heat should accumulate: 10 + 15 + 5 = 30
        assert_eq!(hand_state.current_heat, 30);
    }

    #[test]
    fn test_active_insurance_override() {
        let mut hand_state = HandState::default();

        // First insurance
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Fake ID".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 },
        });

        // Second insurance (should override first)
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 },
        });

        // Active insurance should be Plea Bargain (last one)
        let active = hand_state.active_insurance(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Plea Bargain");
    }

    #[test]
    fn test_active_conviction_override() {
        let mut hand_state = HandState::default();

        // First conviction
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Warrant".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Second conviction (should override first)
        hand_state.cards_played.push(Card {
            id: 2,
            name: "DA Approval".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 60 },
        });

        // Active conviction should be DA Approval (last one)
        let active = hand_state.active_conviction(true);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "DA Approval");
    }

    #[test]
    fn test_conviction_uses_projected_heat() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000;
        hand_state.current_heat = 40; // Heat BEFORE this hand

        // Bust scenario: E:30 C:20, this hand adds +30 heat
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 30 }, // +30 heat
        });

        hand_state.cards_played.push(Card {
            id: 2,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        hand_state.cards_played.push(Card {
            id: 3,
            name: "DA Approval".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 60 },
        });

        // Projected heat: 40 + 30 = 70, which is >= 60, so conviction should activate
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted); // Conviction blocks insurance
        assert_eq!(hand_state.cash, 2000); // Cash unchanged (conviction blocked insurance)
        assert_eq!(hand_state.current_heat, 70); // Heat accumulated correctly
    }

    #[test]
    fn test_start_next_hand_preserves_cash_and_heat() {
        let mut hand_state = HandState::default();

        // Simulate some cash and heat
        hand_state.cash = 1500;
        hand_state.current_heat = 45;

        // Start next hand
        hand_state.start_next_hand();

        // Cash and heat should be preserved
        assert_eq!(hand_state.cash, 1500);
        assert_eq!(hand_state.current_heat, 45);

        // Everything else should be reset
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.current_round, 1);
        assert_eq!(hand_state.cards_played.len(), 0);
        assert!(hand_state.outcome.is_none());
    }

    // ========================================================================
    // TESTS - SOW-004 (Card Retention Between Hands)
    // ========================================================================

    #[test]
    fn test_shuffle_cards_back_returns_unplayed_only() {
        let mut hand_state = HandState::default();
        let initial_deck_size = hand_state.player_deck.len();

        // Draw 3 cards
        hand_state.draw_cards();

        // Play 2 cards (1 remains in hand)
        hand_state.cards_played.push(hand_state.player_hand.remove(0));
        hand_state.cards_played.push(hand_state.player_hand.remove(0));
        let unplayed_cards = hand_state.player_hand.len(); // Should be 1

        // Shuffle back
        hand_state.shuffle_cards_back();

        // Only unplayed cards (1) returned to deck
        // Deck: initial - 3 (drawn) + 1 (unplayed) = initial - 2
        assert_eq!(hand_state.player_deck.len(), initial_deck_size - 2);
        assert_eq!(hand_state.player_hand.len(), 0);
        assert_eq!(hand_state.cards_played.len(), 0); // Cleared
    }

    #[test]
    fn test_shuffle_cards_back_handles_all_players() {
        let mut hand_state = HandState::default();
        let initial_narc = hand_state.narc_deck.len();
        let initial_customer = hand_state.customer_deck.len();
        let initial_player = hand_state.player_deck.len();

        // Draw cards for all players
        hand_state.draw_cards();

        // Shuffle back
        hand_state.shuffle_cards_back();

        // All decks should be restored
        assert_eq!(hand_state.narc_deck.len(), initial_narc);
        assert_eq!(hand_state.customer_deck.len(), initial_customer);
        assert_eq!(hand_state.player_deck.len(), initial_player);
    }

    #[test]
    fn test_start_next_hand_only_returns_unplayed() {
        let mut hand_state = HandState::default();
        hand_state.cash = 500;
        hand_state.current_heat = 30;

        let initial_deck_size = hand_state.player_deck.len();

        // Draw 3 cards, play 1 (2 remain in hand)
        hand_state.draw_cards();
        hand_state.cards_played.push(hand_state.player_hand.remove(0));

        // Start next hand (should shuffle back only the 2 unplayed)
        hand_state.start_next_hand();

        // Deck should be reduced: initial - 3 (drawn) + 2 (unplayed returned) = initial - 1
        assert_eq!(hand_state.player_deck.len(), initial_deck_size - 1);
        assert_eq!(hand_state.cash, 500); // Cash preserved
        assert_eq!(hand_state.current_heat, 30); // Heat preserved
    }

    #[test]
    fn test_fold_penalty_removes_card() {
        let mut hand_state = HandState::default();
        let initial_deck_size = hand_state.player_deck.len();

        // Draw cards
        hand_state.draw_cards();

        // Simulate fold (remove 1 random card)
        if !hand_state.player_deck.is_empty() {
            let idx = rand::thread_rng().gen_range(0..hand_state.player_deck.len());
            hand_state.player_deck.remove(idx);
        }

        // Deck should be 1 smaller
        assert_eq!(hand_state.player_deck.len(), initial_deck_size - 3 - 1); // Drew 3, removed 1
    }

    #[test]
    fn test_deck_exhaustion_ends_run() {
        let mut hand_state = HandState::default();
        hand_state.cash = 1000;
        hand_state.current_heat = 50;

        // Deplete deck to 2 cards
        while hand_state.player_deck.len() > 2 {
            hand_state.player_deck.pop();
        }

        // Try to start next hand with < 3 cards
        let can_continue = hand_state.start_next_hand();

        assert_eq!(can_continue, false); // Cannot continue
        assert_eq!(hand_state.outcome, Some(HandOutcome::Busted)); // Run ends
        assert_eq!(hand_state.current_state, State::Bust); // Terminal state
    }

    #[test]
    fn test_fold_penalty_persists_across_hands() {
        let mut hand_state = HandState::default();
        let initial_deck_size = hand_state.player_deck.len();

        // Draw cards
        hand_state.draw_cards();

        // Simulate fold penalty (remove 1 card)
        if !hand_state.player_deck.is_empty() {
            hand_state.player_deck.remove(0);
        }

        // Deck is now initial_deck_size - 3 (drawn) - 1 (fold penalty) = initial_deck_size - 4
        let deck_after_fold = hand_state.player_deck.len();
        assert_eq!(deck_after_fold, initial_deck_size - 4);

        // Start next hand (should preserve the smaller deck)
        hand_state.start_next_hand();

        // Deck should still be reduced (fold penalty persists)
        assert_eq!(hand_state.player_deck.len(), initial_deck_size - 1); // Only 1 card lost (shuffled back 3 drawn)
    }

    // ========================================================================
    // SOW-006: DECK BUILDING TESTS
    // ========================================================================

    #[test]
    fn test_validate_deck_valid() {
        let deck = create_player_deck(); // Default 20-card deck
        assert!(validate_deck(&deck).is_ok());
    }

    #[test]
    fn test_validate_deck_too_small() {
        let deck = vec![
            Card { id: 10, name: "Weed".to_string(), owner: Owner::Player,
                card_type: CardType::Product { price: 30, heat: 5 } },
            Card { id: 15, name: "Safe House".to_string(), owner: Owner::Player,
                card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 } },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("more cards"));
    }

    #[test]
    fn test_validate_deck_too_large() {
        let mut deck = create_player_deck(); // 20 cards
        deck.push(Card { id: 99, name: "Extra".to_string(), owner: Owner::Player,
            card_type: CardType::Cover { cover: 10, heat: 0 } });
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Maximum 20 cards");
    }

    #[test]
    fn test_validate_deck_missing_product() {
        let deck = vec![
            Card { id: 15, name: "Safe House".to_string(), owner: Owner::Player,
                card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 } },
            Card { id: 19, name: "Alibi".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 30, heat: -5 } },
            Card { id: 20, name: "Bribe".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 25, heat: 10 } },
            Card { id: 21, name: "Fake Receipts".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 20, heat: 5 } },
            Card { id: 22, name: "Bribed Witness".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 15, heat: -10 } },
            Card { id: 23, name: "Plea Bargain".to_string(), owner: Owner::Player,
                card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 } },
            Card { id: 24, name: "Fake ID".to_string(), owner: Owner::Player,
                card_type: CardType::Insurance { cover: 15, cost: 0, heat_penalty: 40 } },
            Card { id: 25, name: "Disguise".to_string(), owner: Owner::Player,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 } },
            Card { id: 26, name: "Burner Phone".to_string(), owner: Owner::Player,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -10 } },
            Card { id: 27, name: "Lookout".to_string(), owner: Owner::Player,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: 0 } },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Product card");
    }

    #[test]
    fn test_validate_deck_missing_location() {
        let deck = vec![
            Card { id: 10, name: "Weed".to_string(), owner: Owner::Player,
                card_type: CardType::Product { price: 30, heat: 5 } },
            Card { id: 11, name: "Meth".to_string(), owner: Owner::Player,
                card_type: CardType::Product { price: 100, heat: 30 } },
            Card { id: 12, name: "Heroin".to_string(), owner: Owner::Player,
                card_type: CardType::Product { price: 150, heat: 45 } },
            Card { id: 13, name: "Cocaine".to_string(), owner: Owner::Player,
                card_type: CardType::Product { price: 120, heat: 35 } },
            Card { id: 14, name: "Fentanyl".to_string(), owner: Owner::Player,
                card_type: CardType::Product { price: 200, heat: 50 } },
            Card { id: 19, name: "Alibi".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 30, heat: -5 } },
            Card { id: 20, name: "Bribe".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 25, heat: 10 } },
            Card { id: 21, name: "Fake Receipts".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 20, heat: 5 } },
            Card { id: 22, name: "Bribed Witness".to_string(), owner: Owner::Player,
                card_type: CardType::Cover { cover: 15, heat: -10 } },
            Card { id: 23, name: "Plea Bargain".to_string(), owner: Owner::Player,
                card_type: CardType::Insurance { cover: 20, cost: 1000, heat_penalty: 20 } },
        ];
        let result = validate_deck(&deck);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 1 Location card");
    }

    #[test]
    fn test_preset_aggro_valid() {
        let deck = create_aggro_deck();
        assert!(validate_deck(&deck).is_ok());
        assert_eq!(deck.len(), 10); // Aggro is minimal deck (meets 10 card minimum)
        // Verify has products and locations
        let has_product = deck.iter().any(|c| matches!(c.card_type, CardType::Product { .. }));
        let has_location = deck.iter().any(|c| matches!(c.card_type, CardType::Location { .. }));
        assert!(has_product);
        assert!(has_location);
    }

    #[test]
    fn test_preset_control_valid() {
        let deck = create_control_deck();
        assert!(validate_deck(&deck).is_ok());
        assert_eq!(deck.len(), 16); // Control is larger deck
        // Verify has products and locations
        let has_product = deck.iter().any(|c| matches!(c.card_type, CardType::Product { .. }));
        let has_location = deck.iter().any(|c| matches!(c.card_type, CardType::Location { .. }));
        assert!(has_product);
        assert!(has_location);
    }

    #[test]
    fn test_deck_builder_default() {
        let builder = DeckBuilder::new();
        assert_eq!(builder.available_cards.len(), 20);
        assert_eq!(builder.selected_cards.len(), 20); // Default: all cards selected
        assert!(builder.is_valid());
    }

    #[test]
    fn test_deck_builder_load_presets() {
        let mut builder = DeckBuilder::new();

        // Load aggro
        builder.load_preset(DeckPreset::Aggro);
        assert_eq!(builder.selected_cards.len(), 10);
        assert!(builder.is_valid());

        // Load control
        builder.load_preset(DeckPreset::Control);
        assert_eq!(builder.selected_cards.len(), 16);
        assert!(builder.is_valid());

        // Load default
        builder.load_preset(DeckPreset::Default);
        assert_eq!(builder.selected_cards.len(), 20);
        assert!(builder.is_valid());
    }
}
