// SOW-001: Minimal Playable Hand
// SOW-002: Betting System and AI Opponents
// SOW-006: Deck Building

use bevy::prelude::*;
use bevy::asset::load_internal_binary_asset;
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

/// SOW-008 Phase 1: AI pacing timers
#[derive(Resource)]
struct AiActionTimer {
    ai_timer: Timer,
    dealer_timer: Timer,
    dealer_timer_started: bool, // Track if we've started the dealer timer this state
}

impl Default for AiActionTimer {
    fn default() -> Self {
        Self {
            ai_timer: Timer::from_seconds(1.0, TimerMode::Repeating), // 1s delay, repeating
            dealer_timer: Timer::from_seconds(1.0, TimerMode::Repeating), // 1s delay, repeating
            dealer_timer_started: false,
        }
    }
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
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_state::<GameState>()  // SOW-006: Add state management
        .insert_resource(DeckBuilder::new())  // SOW-006: Initialize deck builder
        .insert_resource(AiActionTimer::default())  // SOW-008: AI pacing timer
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_betting_state)
        .add_systems(Startup, setup_deck_builder);  // SOW-006: Setup deck builder UI

    // Embed custom font as the default font (supports emojis)
    load_internal_binary_asset!(
        app,
        TextStyle::default().font,
        "../assets/fonts/FiraCode.ttf",
        |bytes: &[u8], _path: String| {
            Font::try_from_bytes(bytes.to_vec()).unwrap()
        }
    );

    app
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
            render_buyer_visible_hand_system,  // SOW-009: Display Buyer's visible hand
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

// SOW-009: PlayAreaCustomer removed (Customer no longer exists)

#[derive(Component)]
struct PlayAreaDealer; // SOW-008: Repurposed from PlayAreaPlayer to show dealer cards
                       // SOW-009: Will show Buyer cards (played cards)

#[derive(Component)]
struct BuyerVisibleHand; // SOW-009: Displays Buyer's 3 visible cards (not yet played)

#[derive(Component)]
struct CardsContainer; // SOW-008: Container for cards within play area (horizontal layout)

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

            // SOW-009: Customer zone removed

            // Dealer zone (SOW-008: Repurposed from player zone)
            // SOW-009: Will show Buyer cards (played)
            create_play_area(parent, "Buyer Cards (Played)", Color::srgb(0.9, 0.9, 0.4), PlayAreaDealer);
        });

        // SOW-009: Buyer visible hand (3 cards face-up, not yet played)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(150.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.2, 0.25).into(),
                ..default()
            },
            BuyerVisibleHand,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Buyer's Visible Hand (anticipate what's coming!)",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.4),
                    ..default()
                },
            ));
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
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(30.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: color.with_alpha(0.2).into(),
        ..default()
    })
    .with_children(|parent| {
        // Label at top
        parent.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Cards container (horizontal layout) - THIS is the play area entity
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row, // SOW-008: Horizontal card layout
                    flex_wrap: FlexWrap::Wrap,
                    align_items: AlignItems::FlexStart,
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                ..default()
            },
            marker, // Apply the marker to the cards container
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
        // SOW-008: Cards reveal immediately, so always include current round
        let include_current_round = true;
        let totals = hand_state.calculate_totals(include_current_round);
        text.sections[0].value = format!(
            "Evidence: {} | Cover: {} | Heat: {} | Profit: ${}\nCash: ${} | Total Heat: {} | Deck: {} cards",
            totals.evidence, totals.cover, totals.heat, totals.profit,
            hand_state.cash, hand_state.current_heat, hand_state.player_deck.len()
        );
    }

    // Update status display with debug info
    if let Ok(mut text) = status_query.get_single_mut() {
        // SOW-008: Get current player info for turn display
        let turn_info = if hand_state.current_state == State::PlayerPhase {
            format!(" - Turn: {:?}", hand_state.current_player())
        } else {
            String::new()
        };

        let mut status = match hand_state.current_state {
            State::Draw => format!("Status: Round {}/3 - Drawing Cards...", hand_state.current_round),
            State::PlayerPhase => format!("Status: Round {}/3 - Playing Cards{}", hand_state.current_round, turn_info),
            State::DealerReveal => format!("Status: Round {}/3 - Dealer Reveal...", hand_state.current_round),
            State::FoldDecision => format!("Status: Round {}/3 - Fold or Continue?", hand_state.current_round),
            State::Resolve => "Status: Resolving Final Hand...".to_string(),
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => {
                    // Check if this was deck exhaustion
                    if hand_state.player_deck.len() < 3 {
                        format!("Status: Deck Exhausted ({} cards) - Run Ends", hand_state.player_deck.len())
                    } else {
                        let totals = hand_state.calculate_totals(true);
                        let demand_met = hand_state.is_demand_satisfied();
                        let multiplier = hand_state.get_profit_multiplier();

                        if demand_met {
                            format!("Status: SAFE! Deal Complete! (Evidence {} ≤ Cover {})\n   Profit: ${} (×{:.1} multiplier - demand satisfied!)",
                                totals.evidence, totals.cover, totals.profit, multiplier)
                        } else {
                            format!("Status: SAFE! Deal Complete! (Evidence {} ≤ Cover {})\n   Profit: ${} (×{:.1} multiplier - demand not met)",
                                totals.evidence, totals.cover, totals.profit, multiplier)
                        }
                    }
                }
                Some(HandOutcome::Busted) => {
                    // Check if this was deck exhaustion (not a real bust)
                    if hand_state.player_deck.len() < 3 {
                        format!("Status: Deck Exhausted ({} cards) - Run Ends", hand_state.player_deck.len())
                    } else {
                        let totals = hand_state.calculate_totals(true);
                        format!("Status: BUSTED! You got caught! (Evidence {} > Cover {})",
                            totals.evidence, totals.cover)
                    }
                }
                Some(HandOutcome::Folded) => "Status: Hand Ended - Folded".to_string(),
                Some(HandOutcome::InvalidDeal) => {
                    let has_product = hand_state.active_product(true).is_some();
                    let has_location = hand_state.active_location(true).is_some();

                    if !has_product && !has_location {
                        "Status: INVALID DEAL - Need both Product AND Location!".to_string()
                    } else if !has_product {
                        "Status: INVALID DEAL - Missing Product card!".to_string()
                    } else {
                        "Status: INVALID DEAL - Missing Location card!".to_string()
                    }
                }
                Some(HandOutcome::BuyerBailed) => {
                    if let Some(persona) = &hand_state.buyer_persona {
                        let totals = hand_state.calculate_totals(true);
                        if let Some(heat_threshold) = persona.heat_threshold {
                            if totals.heat as u32 > heat_threshold {
                                format!("Status: BUYER BAILED - {} got too nervous!\n   (Heat {} > Threshold {})",
                                    persona.display_name, totals.heat, heat_threshold)
                            } else {
                                format!("Status: BUYER BAILED - {} got too nervous!", persona.display_name)
                            }
                        } else {
                            format!("Status: BUYER BAILED - {} got too nervous!", persona.display_name)
                        }
                    } else {
                        "Status: BUYER BAILED - Deal fell through!".to_string()
                    }
                }
                None => "Status: Game Over".to_string(),
            },
        };

        // SOW-009: Add Buyer persona info
        if let Some(persona) = &hand_state.buyer_persona {
            status.push_str(&format!("\n\n👤 Buyer: {}", persona.display_name));

            // Show description
            status.push_str(&format!("\n   {}", persona.demand.description));

            // Show multipliers
            status.push_str(&format!("\n   Multiplier: ×{:.1} (demand met) | ×{:.1} (not met)",
                persona.base_multiplier, persona.reduced_multiplier));

            // Show thresholds
            if let Some(heat_threshold) = persona.heat_threshold {
                let heat_warning = if hand_state.current_heat >= heat_threshold - 5 {
                    " ⚠️ CLOSE!"
                } else {
                    ""
                };
                status.push_str(&format!("\n   Heat Limit: {} (Current: {}){}",
                    heat_threshold, hand_state.current_heat, heat_warning));
            } else {
                status.push_str("\n   Heat Limit: None (won't bail)");
            }

            if let Some(evidence_threshold) = persona.evidence_threshold {
                status.push_str(&format!("\n   Evidence Limit: {}", evidence_threshold));
            }
        }

        // SOW-003 Phase 5: Add Insurance/Conviction status info
        // SOW-008: Cards always revealed immediately, so show if any cards played
        let cards_ever_revealed = !hand_state.cards_played.is_empty();

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

        // Color code status (SOW-008)
        text.sections[0].style.color = match hand_state.current_state {
            State::PlayerPhase => Color::srgb(1.0, 1.0, 0.3), // Yellow for playing cards
            State::Bust => match hand_state.outcome {
                Some(HandOutcome::Safe) => Color::srgb(0.3, 1.0, 0.3),   // Green for safe
                Some(HandOutcome::Busted) => Color::srgb(1.0, 0.3, 0.3), // Red for busted
                Some(HandOutcome::Folded) => Color::srgb(0.7, 0.7, 0.7), // Gray for fold
                Some(HandOutcome::InvalidDeal) => Color::srgb(1.0, 0.6, 0.0), // Orange for invalid
                Some(HandOutcome::BuyerBailed) => Color::srgb(1.0, 0.8, 0.0), // Yellow-orange for bail
                None => Color::WHITE,
            },
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

    // Clear ALL existing children (card buttons and played card displays)
    if let Ok(children) = children_query.get(hand_entity) {
        for &child in children.iter() {
            commands.entity(child).despawn_recursive();
        }
    }

    // Add card buttons for current hand
    // SOW-008: Show cards during PlayerPhase and other states
    // SOW-009: Also show during DealerReveal (Buyer card reveal)
    let show_cards = hand_state.current_state == State::PlayerPhase ||
                     hand_state.current_state == State::DealerReveal ||
                     hand_state.current_state == State::FoldDecision ||
                     hand_state.current_state == State::Resolve ||
                     hand_state.current_state == State::Bust;

    if show_cards {
        // First show player's played cards and checks (grayed out, non-clickable)
        commands.entity(hand_entity).with_children(|parent| {
            let player_played: Vec<&Card> = hand_state.cards_played.iter()
                .filter(|c| c.owner == Owner::Player)
                .collect();

            let has_played_or_checked = !player_played.is_empty() ||
                hand_state.checks_this_hand.iter().any(|(o, _)| *o == Owner::Player);

            for card in player_played {
                // Dimmed color for played cards
                let base_color = match card.card_type {
                    CardType::Product { .. } => Color::srgb(0.5, 0.4, 0.1),
                    CardType::Location { .. } => Color::srgb(0.2, 0.3, 0.5),
                    CardType::Evidence { .. } => Color::srgb(0.4, 0.2, 0.2),
                    CardType::Cover { .. } => Color::srgb(0.2, 0.4, 0.2),
                    CardType::DealModifier { .. } => Color::srgb(0.4, 0.3, 0.5),
                    CardType::Insurance { .. } => Color::srgb(0.1, 0.4, 0.4),
                    CardType::Conviction { .. } => Color::srgb(0.5, 0.1, 0.1),
                    _ => Color::srgb(0.3, 0.3, 0.3),
                };

                // Format with stats for verification
                let card_info = match &card.card_type {
                    CardType::Product { price, heat } =>
                        format!("{}\n${} | Heat: {}", card.name, price, heat),
                    CardType::Location { evidence, cover, heat } =>
                        format!("{}\nE:{} C:{} H:{}", card.name, evidence, cover, heat),
                    CardType::Evidence { evidence, heat } =>
                        format!("{}\nEvidence: {}\nHeat: {}", card.name, evidence, heat),
                    CardType::Cover { cover, heat } =>
                        format!("{}\nCover: {}\nHeat: {}", card.name, cover, heat),
                    CardType::DealModifier { price_multiplier, evidence, cover, heat } =>
                        format!("{}\n×{:.1}\nE:{} C:{} H:{}", card.name, price_multiplier, evidence, cover, heat),
                    CardType::Insurance { cover, cost, heat_penalty } =>
                        format!("{}\nCover: {}\nCost: ${}", card.name, cover, cost),
                    CardType::Conviction { heat_threshold } =>
                        format!("{}\nThreshold: {}", card.name, heat_threshold),
                    _ => card.name.clone(),
                };

                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(120.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: base_color.into(),
                        border_color: Color::srgb(0.5, 0.5, 0.5).into(), // Dim border for played
                        ..default()
                    },
                    PlayedCardDisplay,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        card_info,
                        TextStyle {
                            font_size: 10.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            }

            // Show player checks (all rounds)
            for &(owner, round) in hand_state.checks_this_hand.iter() {
                if owner == Owner::Player {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(120.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(6.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.35, 0.35, 0.35).into(),
                            border_color: Color::srgb(0.5, 0.5, 0.5).into(),
                            ..default()
                        },
                        PlayedCardDisplay,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("CHECK\n(Round {})", round),
                            TextStyle {
                                font_size: 10.0,
                                color: Color::srgb(0.8, 0.8, 0.8),
                                ..default()
                            },
                        ));
                    });
                }
            }

            // Add separator if there are played cards
            if has_played_or_checked {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(4.0),
                        height: Val::Px(120.0),
                        margin: UiRect::horizontal(Val::Px(8.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.6, 0.6, 0.6).into(),
                    ..default()
                });
            }
        });

        // Then show unplayed cards (clickable during player's turn)
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

    // SOW-008: Legacy auto-play removed - sequential play handled by ai_betting_system
}

// ============================================================================
// AUTO-ADVANCE SYSTEM - SOW-002
// ============================================================================

fn auto_flip_system(
    mut hand_state_query: Query<&mut HandState>,
    mut ai_timer: ResMut<AiActionTimer>,
    time: Res<Time>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Auto-draw cards when entering Draw state
    if hand_state.current_state == State::Draw {
        // Draw cards for all players
        hand_state.draw_cards();
        // Note: draw_cards() calls transition_state() → PlayerPhase
    }

    // SOW-009 Phase 3: Buyer card play (replaces SOW-008 dealer reveal)
    if hand_state.current_state == State::DealerReveal {
        // On first frame in DealerReveal, reset the timer to start fresh 1s countdown
        if !ai_timer.dealer_timer_started {
            ai_timer.dealer_timer.reset();
            ai_timer.dealer_timer_started = true;

            // SOW-009: Buyer plays random card from visible hand
            if let Some(buyer_card) = hand_state.buyer_plays_card() {
                println!("Buyer plays: {} (starting 1s timer...)", buyer_card.name);
            } else {
                println!("Buyer has no cards to play");
            }
        }

        // Tick dealer timer
        ai_timer.dealer_timer.tick(time.delta());

        if ai_timer.dealer_timer.just_finished() {
            // Timer fired - advance to next round or resolve
            println!("Buyer card processed! Advancing...");

            // SOW-009: Buyer bail check will be added in Phase 4

            // Auto-advance to next round or resolve
            println!("Current round: {}, transitioning from DealerReveal...", hand_state.current_round);
            hand_state.transition_state();
            println!("New state: {:?}", hand_state.current_state);
            ai_timer.dealer_timer_started = false; // Reset for next buyer card
        }
    } else {
        // Not in DealerReveal - reset the flag
        ai_timer.dealer_timer_started = false;
    }

    // SOW-008: FoldDecision state no longer used
    // Fold happens during PlayerPhase (player's turn) via Fold button

    // Auto-resolve at Resolution state
    if hand_state.current_state == State::Resolve {
        println!("Auto-resolving hand...");
        let outcome = hand_state.resolve_hand();
        println!("Resolution outcome: {:?}, new state: {:?}", outcome, hand_state.current_state);
    }
}

// ============================================================================
// SOW-008: BETTING BUTTON SYSTEM - DISABLED (Phase 1 stub)
// ============================================================================
// TODO Phase 1: Add Check button for skipping card play
// Old betting system (Check/Raise/Fold with initiative) removed per ADR-006

// SOW-008: Check and Fold buttons during PlayerPhase
fn betting_button_system(
    check_query: Query<&Interaction, (Changed<Interaction>, With<CheckButton>)>,
    fold_query: Query<&Interaction, (Changed<Interaction>, With<FoldButton>)>,
    mut hand_state_query: Query<&mut HandState>,
    _betting_state_query: Query<&mut BettingState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only during PlayerPhase and when it's Player's turn
    if hand_state.current_state != State::PlayerPhase || hand_state.current_player() != Owner::Player {
        return;
    }

    // Check button - skip playing a card this turn
    for interaction in check_query.iter() {
        if *interaction == Interaction::Pressed {
            let current_round = hand_state.current_round;
            println!("Player checks (skips card) in Round {}", current_round);

            // Record that player checked this round
            hand_state.checks_this_hand.push((Owner::Player, current_round));

            // Advance to next player without playing a card
            hand_state.current_player_index += 1;

            // If all players have acted, transition to DealerReveal
            if hand_state.all_players_acted() {
                hand_state.transition_state();
            }
        }
    }

    // Fold button - player folds immediately (available during player's turn)
    for interaction in fold_query.iter() {
        if *interaction == Interaction::Pressed {
            println!("Player folds during turn!");

            // Fold immediately - discard played cards, keep unplayed, exit hand
            hand_state.cards_played.clear();
            hand_state.outcome = Some(HandOutcome::Folded);
            hand_state.current_state = State::Bust;
        }
    }
}

// ============================================================================
// SOW-008: UPDATE BETTING BUTTON STATES - DISABLED (Phase 1 stub)
// ============================================================================

fn update_betting_button_states(
    _hand_state_query: Query<&HandState>,
    _betting_state_query: Query<&BettingState>,
    _check_button_query: Query<&mut BackgroundColor, With<CheckButton>>,
) {
    // SOW-008 Phase 1: Stubbed out - betting system removed
    return;
}

// ============================================================================
// SOW-008: DECISION POINT BUTTON SYSTEM - DISABLED (Phase 1 stub)
// ============================================================================
// TODO Phase 3: Replace with fold-after-dealer-reveal logic

// SOW-008: Decision point button system - NOT USED
// Fold happens during PlayerPhase (player's turn) now, not between rounds
fn decision_point_button_system(
    _continue_query: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    _fold_query: Query<&Interaction, (Changed<Interaction>, With<FoldButton>)>,
    _hand_state_query: Query<&mut HandState>,
) {
    // Not used - fold happens during PlayerPhase via betting_button_system
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

            // SOW-009 Phase 2: Select random Buyer persona
            let buyer_personas = create_buyer_personas();
            let random_buyer = buyer_personas.choose(&mut rand::thread_rng()).unwrap().clone();

            // Create new HandState with selected deck
            let mut hand_state = HandState::with_custom_deck(deck_builder.selected_cards.clone());
            hand_state.buyer_persona = Some(random_buyer);
            hand_state.draw_cards(); // This will also initialize buyer hand
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

    // SOW-008: Show Check button during PlayerPhase when it's player's turn
    if let Ok(mut style) = betting_container_query.get_single_mut() {
        style.display = if hand_state.current_state == State::PlayerPhase &&
                          hand_state.current_player() == Owner::Player {
            Display::Flex
        } else {
            Display::None
        };
    }

    // SOW-008: Hide decision point UI (fold happens during PlayerPhase now)
    if let Ok(mut style) = decision_container_query.get_single_mut() {
        style.display = Display::None;
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
    narc_area_query: Query<Entity, With<PlayAreaNarc>>,
    dealer_area_query: Query<Entity, With<PlayAreaDealer>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_display_query: Query<Entity, With<PlayedCardDisplay>>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // Clear old card displays (SOW-009: Only Narc and Dealer/Buyer areas)
    for area in [narc_area_query.get_single(), dealer_area_query.get_single()] {
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

    // SOW-009: Show played cards by owner (Narc, Buyer in play areas; Player in hand)
    for card in hand_state.cards_played.iter() {
        let area_entity = match card.owner {
            Owner::Narc => narc_area_query.get_single(),
            Owner::Player => {
                // Player cards shown in hand area now, not play area
                // Skip for now - will be shown with hand display
                continue;
            }
            Owner::Buyer => dealer_area_query.get_single(),  // Buyer cards show in dealer area
        };

        if let Ok(area) = area_entity {
            // Get card color
            let card_color = match card.card_type {
                CardType::Product { .. } => Color::srgb(0.9, 0.7, 0.2),
                CardType::Location { .. } => Color::srgb(0.3, 0.6, 0.9),
                CardType::Evidence { .. } => Color::srgb(0.8, 0.3, 0.3),
                CardType::Cover { .. } => Color::srgb(0.3, 0.8, 0.3),
                CardType::DealModifier { .. } => Color::srgb(0.7, 0.5, 0.9),
                CardType::Insurance { .. } => Color::srgb(0.2, 0.8, 0.8),
                CardType::Conviction { .. } => Color::srgb(0.9, 0.2, 0.2),
                _ => Color::srgb(0.5, 0.5, 0.5),
            };

            // Format card with stats
            let card_text = match &card.card_type {
                CardType::Product { price, heat } =>
                    format!("{}\n${} | Heat: {}", card.name, price, heat),
                CardType::Location { evidence, cover, heat } =>
                    format!("{}\nE:{} C:{} H:{}", card.name, evidence, cover, heat),
                CardType::Evidence { evidence, heat } =>
                    format!("{}\nEvidence: {}\nHeat: {}", card.name, evidence, heat),
                CardType::Cover { cover, heat } =>
                    format!("{}\nCover: {}\nHeat: {}", card.name, cover, heat),
                CardType::DealModifier { price_multiplier, evidence, cover, heat } =>
                    format!("{}\n×{:.1}\nE:{} C:{} H:{}", card.name, price_multiplier, evidence, cover, heat),
                CardType::Insurance { cover, cost, heat_penalty } =>
                    format!("{}\nCover: {}\nCost: ${}", card.name, cover, cost),
                CardType::Conviction { heat_threshold } =>
                    format!("{}\nThreshold: {}", card.name, heat_threshold),
                _ => card.name.clone(),
            };

            commands.entity(area).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(120.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                        background_color: card_color.into(),
                        border_color: Color::srgb(0.9, 0.9, 0.9).into(),
                        ..default()
                    },
                    PlayedCardDisplay,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        card_text,
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });
        }
    }

    // SOW-009: Show checks (players who skipped playing a card)
    for &(owner, round) in hand_state.checks_this_hand.iter() {
        let area_entity = match owner {
            Owner::Narc => narc_area_query.get_single(),
            Owner::Player => {
                // Player checks shown in hand area
                continue;
            }
            Owner::Buyer => {
                // Buyer never checks (always plays random card)
                continue;
            }
        };

        if let Ok(area) = area_entity {
            commands.entity(area).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(120.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.4, 0.4, 0.4).into(),
                        border_color: Color::srgb(0.6, 0.6, 0.6).into(),
                        ..default()
                    },
                    PlayedCardDisplay,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("CHECK\n(Round {})", round),
                        TextStyle {
                            font_size: 11.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ));
                });
            });
        }
    }

    // SOW-008 Phase 2: Show revealed dealer cards in dealer area
    let revealed_count = if hand_state.current_state == State::DealerReveal ||
                            hand_state.current_state == State::FoldDecision ||
                            hand_state.current_state == State::Resolve ||
                            hand_state.current_state == State::Bust {
        hand_state.current_round as usize
    } else {
        hand_state.current_round.saturating_sub(1) as usize
    };

    // SOW-009: Dealer card display removed, will be replaced by Buyer card display in Phase 3
    if let Ok(_dealer_area) = dealer_area_query.get_single() {
        // TODO: Display buyer_hand and buyer_played cards here instead
        for _i in 0..revealed_count {
            // Placeholder: Buyer card display will be implemented in Phase 3
            /*
            if let Some(dealer_card) = hand_state.dealer_hand.get(i) {
                // Dealer card color
                let card_color = match dealer_card.card_type {
                    CardType::Location { .. } => Color::srgb(0.5, 0.7, 1.0),
                    CardType::DealModifier { .. } => Color::srgb(0.9, 0.7, 1.0),
                    _ => Color::srgb(0.6, 0.6, 0.6),
                };

                // Format dealer card with stats
                let dealer_text = match &dealer_card.card_type {
                    CardType::Location { evidence, cover, heat } =>
                        format!("{}\nEvidence: {}\nCover: {}\nHeat: {}", dealer_card.name, evidence, cover, heat),
                    CardType::DealModifier { price_multiplier, evidence, cover, heat } =>
                        format!("{}\n×{:.1}\nE:{} C:{} H:{}", dealer_card.name, price_multiplier, evidence, cover, heat),
                    _ => dealer_card.name.clone(),
                };
                commands.entity(dealer_area).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(110.0),
                                height: Val::Px(130.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::all(Val::Px(3.0)),
                                margin: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: card_color.into(),
                            border_color: Color::srgb(1.0, 1.0, 0.8).into(), // Bright border for dealer
                            ..default()
                        },
                        PlayedCardDisplay,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            dealer_text,
                            TextStyle {
                                font_size: 12.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
                });
            }
            */
        }
    }
}

// ============================================================================
// SOW-009: BUYER VISIBLE HAND DISPLAY
// ============================================================================

/// Display the Buyer's 3 visible cards (not yet played) in the BuyerVisibleHand area
fn render_buyer_visible_hand_system(
    hand_state_query: Query<&HandState, Changed<HandState>>,
    buyer_area_query: Query<Entity, With<BuyerVisibleHand>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    card_display_query: Query<Entity, With<PlayedCardDisplay>>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    let Ok(buyer_area) = buyer_area_query.get_single() else {
        return;
    };

    // Clear old card displays
    if let Ok(children) = children_query.get(buyer_area) {
        for &child in children.iter() {
            if card_display_query.get(child).is_ok() {
                commands.entity(child).despawn_recursive();
            }
        }
    }

    // Display each card in buyer_hand
    for card in hand_state.buyer_hand.iter() {
        // Get card color
        let card_color = match card.card_type {
            CardType::Location { .. } => Color::srgb(0.5, 0.7, 1.0),  // Buyer Location cards
            CardType::DealModifier { .. } => Color::srgb(0.7, 0.5, 0.9),     // Buyer Price Modifier cards
            _ => Color::srgb(0.5, 0.5, 0.5),
        };

        // Format card with stats
        let card_text = match &card.card_type {
            CardType::Location { evidence, cover, heat } =>
                format!("{}\nE:{} C:{} H:{}", card.name, evidence, cover, heat),
            CardType::DealModifier { price_multiplier, evidence, cover, heat } =>
                format!("{}\n×{:.1}\nE:{} C:{} H:{}", card.name, price_multiplier, evidence, cover, heat),
            _ => card.name.clone(),
        };

        commands.entity(buyer_area).with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(140.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: card_color.into(),
                    border_color: Color::srgb(1.0, 1.0, 0.0).into(), // Bright yellow border for visibility
                    ..default()
                },
                PlayedCardDisplay,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    card_text,
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    }
}

// ============================================================================
// CARD CLICK SYSTEM
// ============================================================================

fn card_click_system(
    mut interaction_query: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut hand_state_query: Query<&mut HandState>,
    _betting_state_query: Query<&mut BettingState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    for (interaction, card_button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            println!("Card clicked! Index: {}, State: {:?}", card_button.card_index, hand_state.current_state);

            // SOW-008: Clicking card during PlayerPhase plays it immediately
            if hand_state.current_state == State::PlayerPhase {
                // Only if it's Player's turn
                if hand_state.current_player() == Owner::Player {
                    // Verify valid card index
                    if card_button.card_index < hand_state.player_hand.len() {
                        println!("Player playing card {}", card_button.card_index);

                        // Play the card face-up immediately
                        let _ = hand_state.play_card(Owner::Player, card_button.card_index);
                    }
                }
            }
        }
    }
}

// ============================================================================
// CARD DATA MODEL
// ============================================================================

/// Who owns this card (SOW-009: Narc, Player, and Buyer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Owner {
    Narc,
    Player,
    Buyer,  // SOW-009: Buyer reaction cards
}

/// Card types with their specific values (Extended in SOW-002/003/008)
#[derive(Debug, Clone)]
enum CardType {
    Product { price: u32, heat: i32 },
    Location { evidence: u32, cover: u32, heat: i32 },  // SOW-009: Used by both Player and Buyer (override rule)
    Evidence { evidence: u32, heat: i32 },
    Cover { cover: u32, heat: i32 },
    // SOW-002 Phase 4: Deal Modifiers (multiplicative price, additive Evidence/Cover/Heat)
    // SOW-009: Used by both Player and Buyer (price_multiplier defaults to 1.0 for non-price cards)
    DealModifier { price_multiplier: f32, evidence: i32, cover: i32, heat: i32 },
    // SOW-003 Phase 1: Insurance (Cover + bust activation)
    Insurance { cover: u32, cost: u32, heat_penalty: i32 },
    // SOW-003 Phase 2: Conviction (Heat threshold, overrides insurance)
    Conviction { heat_threshold: u32 },
    // SOW-009: DealerLocation removed (merged into Location)
    // SOW-009: DealerModifier removed (merged into DealModifier)
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
// SOW-009: BUYER SYSTEM
// ============================================================================

/// Buyer demand specification - what Products/Locations satisfy this Buyer
#[derive(Debug, Clone)]
struct BuyerDemand {
    products: Vec<String>,      // e.g., ["Pills", "Weed"] - product names that satisfy demand
    locations: Vec<String>,     // e.g., ["Private Residence", "Warehouse"] - location names that satisfy demand
    description: String,        // Human-readable description for UI
}

/// Special rules for Buyer persona (conditional effects)
#[derive(Debug, Clone)]
enum SpecialRule {
    // Future: Add special rules like "if public Location used, +10 Evidence"
    // For MVP: Empty, will be populated in Phase 2
}

/// Buyer persona - merges Dealer scenario deck + Customer modifiers into one entity
#[derive(Debug, Clone)]
struct BuyerPersona {
    id: String,                          // "college_party_host"
    display_name: String,                // "College Party Host"
    demand: BuyerDemand,                 // What Products/Locations satisfy demand
    base_multiplier: f32,                // ×1.0 to ×3.0 range (when demand met)
    reduced_multiplier: f32,             // When demand not met (typically ×1.0)
    heat_threshold: Option<u32>,         // Buyer bails if Heat exceeds (None = never bails)
    evidence_threshold: Option<u32>,     // Buyer bails if Evidence exceeds (None = never bails)
    special_rules: Vec<SpecialRule>,     // Conditional effects
    reaction_deck: Vec<Card>,            // 7 cards unique to this persona
}

// ============================================================================
// HAND STATE MACHINE
// ============================================================================

/// States the hand can be in (SOW-008: Sequential play with dealer reveals)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Draw,           // Draw cards to hand
    PlayerPhase,    // Sequential turn-based card play (SOW-008 Phase 1)
    DealerReveal,   // Dealer community card reveals (SOW-008 Phase 2)
    FoldDecision,   // Player/AI can fold after seeing dealer card (SOW-008 Phase 3, Rounds 1-2 only)
    Resolve,        // Calculate totals, check bust (after Round 3)
    Bust,           // Terminal state

    // REMOVED by SOW-008: Betting, Flip (obsolete betting system from ADR-002/005)
    // REMOVED by SOW-008: DecisionPoint (replaced by FoldDecision after dealer reveal)
    // REMOVED: Legacy NarcPlay/CustomerPlay/PlayerPlay states
}

/// Outcome of hand resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandOutcome {
    Safe,
    Busted,
    Folded,        // SOW-004: Player folded during hand (not bust, not completed)
    InvalidDeal,   // SOW-009: Missing Product or Location
    BuyerBailed,   // SOW-009: Buyer threshold exceeded
}

/// Hand state tracking (Extended for SOW-002/003, SOW-008)
#[derive(Component)]
struct HandState {
    pub current_state: State,
    pub current_round: u8,           // 1, 2, or 3 (ADR-004)
    pub cards_played: Vec<Card>,
    pub cards_played_this_round: Vec<Card>, // SOW-008: Repurposed for sequential play tracking
    narc_deck: Vec<Card>,
    player_deck: Vec<Card>,
    pub narc_hand: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub outcome: Option<HandOutcome>,
    // SOW-003: Cash and Heat tracking
    pub cash: u32,           // Cumulative profit for insurance affordability
    pub current_heat: u32,   // Cumulative Heat for conviction thresholds
    // SOW-008 Phase 1: Turn tracking for sequential play
    pub current_player_index: usize, // Index into turn_order (0 or 1 for 2 players)
    // SOW-009: dealer_deck, dealer_hand, customer_deck, customer_hand, customer_folded removed
    // SOW-008 Phase 1: Check tracking (for display)
    pub checks_this_hand: Vec<(Owner, u8)>, // Who checked and in which round (persists entire hand)
    // SOW-009: Buyer system (replaces dealer_deck + customer)
    buyer_persona: Option<BuyerPersona>,   // Selected Buyer for this hand (None during transition)
    buyer_deck: Vec<Card>,                 // 7 cards from persona (shuffled)
    pub buyer_hand: Vec<Card>,             // 3 visible cards drawn at hand start
    pub buyer_played: Vec<Card>,           // Cards played so far (for UI tracking)
}

impl Default for HandState {
    fn default() -> Self {
        // SOW-009: Dealer deck removed, replaced by Buyer system
        Self {
            current_state: State::Draw,
            current_round: 1,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            narc_deck: create_narc_deck(),
            player_deck: create_player_deck(),
            narc_hand: Vec::new(),
            player_hand: Vec::new(),
            outcome: None,
            cash: 0,          // SOW-003: Start with no cash
            current_heat: 0,  // SOW-003: Start with no Heat
            current_player_index: 0, // SOW-008: Start at first player in turn order
            checks_this_hand: Vec::new(), // SOW-008 Phase 1
            // SOW-009: Buyer system (initialized as None, will be set when Buyer selected)
            buyer_persona: None,
            buyer_deck: Vec::new(),
            buyer_hand: Vec::new(),
            buyer_played: Vec::new(),
        }
    }
}

impl HandState {
    /// Create HandState with a custom player deck (SOW-006)
    fn with_custom_deck(player_deck: Vec<Card>) -> Self {
        // SOW-009: Dealer deck removed, replaced by Buyer system
        Self {
            current_state: State::Draw,
            current_round: 1,
            cards_played: Vec::new(),
            cards_played_this_round: Vec::new(),
            narc_deck: create_narc_deck(),
            player_deck,  // Use custom deck
            narc_hand: Vec::new(),
            player_hand: Vec::new(),
            outcome: None,
            cash: 0,
            current_heat: 0,
            current_player_index: 0, // SOW-008
            checks_this_hand: Vec::new(), // SOW-008 Phase 1
            // SOW-009: Buyer system
            buyer_persona: None,
            buyer_deck: Vec::new(),
            buyer_hand: Vec::new(),
            buyer_played: Vec::new(),
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

        // SOW-009: Customer deck removed

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
        // SOW-009: Customer deck removed
        let preserved_player_deck = self.player_deck.clone();
        let preserved_narc_deck = self.narc_deck.clone();

        // SOW-004 Phase 3: Check deck exhaustion before starting new hand
        if preserved_player_deck.len() < 3 {
            // Deck exhausted - cannot start new hand
            self.outcome = Some(HandOutcome::Busted);
            self.current_state = State::Bust;
            // Don't reset - keep preserved decks to show deck size in UI
            self.player_deck = preserved_player_deck;
            self.narc_deck = preserved_narc_deck;
            return false;
        }

        // SOW-009: Preserve Buyer persona across hands
        let preserved_buyer_persona = self.buyer_persona.clone();

        // Reset state but preserve cash/heat/decks/buyer
        *self = Self::default();
        self.cash = preserved_cash;
        self.current_heat = preserved_heat;
        self.player_deck = preserved_player_deck;
        self.narc_deck = preserved_narc_deck;
        self.buyer_persona = preserved_buyer_persona;  // Keep same Buyer for entire session
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

        // SOW-009: Customer draw removed

        while self.player_hand.len() < HAND_SIZE && !self.player_deck.is_empty() {
            self.player_hand.push(self.player_deck.remove(0));
        }

        // SOW-009 Phase 3: Initialize Buyer hand (draw 3 visible cards)
        self.initialize_buyer_hand();

        // Transition to next state after draw
        self.transition_state();
    }

    /// Transition to next state (SOW-008: Sequential play state machine)
    pub fn transition_state(&mut self) {
        self.current_state = match self.current_state {
            // SOW-008: Sequential play flow
            State::Draw => State::PlayerPhase,
            State::PlayerPhase => {
                // After all players act, go to Dealer Reveal
                State::DealerReveal
            },
            State::DealerReveal => {
                // After Dealer reveals, check if customer folds, then advance
                // Player can fold during their turn in PlayerPhase (not here)
                if self.current_round >= 3 {
                    // Round 3: Go to Resolution
                    State::Resolve
                } else {
                    // Rounds 1-2: Advance to next round
                    self.current_round += 1;
                    self.reset_turn_tracking();
                    // Don't clear checks_this_hand - persist for entire hand
                    State::Draw
                }
            },
            State::FoldDecision => {
                // Legacy state - should not be used anymore
                // Fold happens during PlayerPhase now
                self.current_round += 1;
                self.reset_turn_tracking();
                State::Draw
            },
            State::Resolve => State::Bust, // Will be refined (Safe vs Busted)
            State::Bust => State::Bust, // Terminal state
        };
    }

    // SOW-008 Note: continue_to_next_round and fold_at_decision_point removed
    // These methods are obsolete - sequential play handles round advancement differently
    // Fold logic will be added in Phase 3

    /// Play a card from hand during PlayerPhase (SOW-008: Sequential play)
    fn play_card(&mut self, owner: Owner, card_index: usize) -> Result<(), String> {
        // Verify we're in PlayerPhase and it's the correct player's turn
        if self.current_state != State::PlayerPhase {
            return Err(format!("Not in PlayerPhase: {:?}", self.current_state));
        }

        let current_player = self.current_player();
        if owner != current_player {
            return Err(format!("Wrong turn: expected {:?}, got {:?}", current_player, owner));
        }

        // Get the card from the appropriate hand (SOW-009: Only Narc and Player can call this)
        let hand = match owner {
            Owner::Narc => &mut self.narc_hand,
            Owner::Player => &mut self.player_hand,
            Owner::Buyer => unreachable!("Buyer uses buyer_plays_card(), not play_card()"),
        };

        if card_index >= hand.len() {
            return Err(format!("Card index {} out of bounds", card_index));
        }

        // Remove card from hand and play it face-up immediately (SOW-008)
        let card = hand.remove(card_index);
        self.cards_played.push(card);

        // Advance to next player's turn (increments index)
        self.current_player_index += 1;

        // Check if all players have acted, then transition
        if self.all_players_acted() {
            self.transition_state();
        }

        Ok(())
    }

    // SOW-008 Phase 1: Sequential play helpers

    /// Get whose turn it is in the current round (SOW-008 rotating turn order)
    pub fn current_player(&self) -> Owner {
        let turn_order = get_turn_order(self.current_round);
        turn_order[self.current_player_index]
    }

    /// Advance to next player in turn order
    /// Note: Does NOT reset index - caller should check all_players_acted() and transition state
    pub fn advance_turn(&mut self) {
        self.current_player_index += 1;
        // Don't reset - let caller handle transition when all_players_acted()
    }

    /// Check if all players have acted this round
    pub fn all_players_acted(&self) -> bool {
        let turn_order = get_turn_order(self.current_round);
        self.current_player_index >= turn_order.len()
    }

    /// Reset turn tracking for new round
    pub fn reset_turn_tracking(&mut self) {
        self.current_player_index = 0;
    }

    // SOW-009: Dealer card system removed, replaced by Buyer reaction cards

    /// Get the dealer card for the current round (OBSOLETE - will be replaced by Buyer cards)
    pub fn current_dealer_card(&self) -> Option<&Card> {
        // TODO: Replace with buyer_hand logic in Phase 3
        None
    }

    // SOW-009 Phase 4: Resolution checks

    /// Check if deal is valid (must have at least 1 Product AND 1 Location)
    pub fn is_valid_deal(&self) -> bool {
        self.active_product(true).is_some() && self.active_location(true).is_some()
    }

    /// Check if Buyer should bail based on thresholds
    pub fn should_buyer_bail(&self) -> bool {
        if let Some(persona) = &self.buyer_persona {
            let totals = self.calculate_totals(true);

            // Check Heat threshold (only bail if heat is positive and exceeds threshold)
            if let Some(heat_threshold) = persona.heat_threshold {
                if totals.heat > 0 && (totals.heat as u32) > heat_threshold {
                    return true;
                }
            }

            // Check Evidence threshold
            if let Some(evidence_threshold) = persona.evidence_threshold {
                if totals.evidence > evidence_threshold {
                    return true;
                }
            }
        }
        false
    }

    /// Check if demand is satisfied (Product + Location match Buyer preferences)
    pub fn is_demand_satisfied(&self) -> bool {
        if let Some(persona) = &self.buyer_persona {
            // Check Product match
            let product_match = self.active_product(true)
                .map(|card| persona.demand.products.contains(&card.name))
                .unwrap_or(false);

            // Check Location match
            let location_match = self.active_location(true)
                .map(|card| persona.demand.locations.contains(&card.name))
                .unwrap_or(false);

            product_match && location_match
        } else {
            false // No Buyer persona set
        }
    }

    /// Get the appropriate profit multiplier based on demand satisfaction
    pub fn get_profit_multiplier(&self) -> f32 {
        if let Some(persona) = &self.buyer_persona {
            if self.is_demand_satisfied() {
                persona.base_multiplier
            } else {
                persona.reduced_multiplier
            }
        } else {
            1.0 // Default multiplier if no Buyer
        }
    }

    // SOW-008 Phase 3: Fold mechanics

    /// Player folds after seeing dealer reveal (Rounds 1-2 only)
    /// Consequences: Discard played cards, keep unplayed cards, keep Heat, lose profit
    pub fn player_fold(&mut self) {
        if self.current_state != State::FoldDecision {
            return; // Only valid at FoldDecision
        }

        // Discard all played cards (don't shuffle back)
        self.cards_played.clear();

        // Keep unplayed cards in hand (already there)
        // Keep accumulated Heat (already tracked in current_heat)
        // Lose profit (don't bank it)

        // Mark as folded and end hand
        self.outcome = Some(HandOutcome::Folded);
        self.current_state = State::Bust; // Terminal state
    }

    // SOW-009 Phase 3: Buyer card play system

    /// Buyer plays 1 random card from visible hand
    /// Returns the card that was played, or None if no cards available
    pub fn buyer_plays_card(&mut self) -> Option<Card> {
        if self.buyer_hand.is_empty() {
            return None;  // No cards left
        }

        // Select random card from visible hand
        let random_index = rand::thread_rng().gen_range(0..self.buyer_hand.len());
        let card = self.buyer_hand.remove(random_index);

        // Add to buyer_played for tracking
        self.buyer_played.push(card.clone());

        // Add to cards_played for totals calculation
        self.cards_played.push(card.clone());

        Some(card)
    }

    /// Initialize Buyer hand at start of hand (draw 3 visible cards)
    pub fn initialize_buyer_hand(&mut self) {
        if let Some(persona) = &self.buyer_persona {
            // Clone the reaction deck from persona
            self.buyer_deck = persona.reaction_deck.clone();
            self.buyer_deck.shuffle(&mut rand::thread_rng());

            // Draw 3 cards (all visible to player)
            let draw_count = std::cmp::min(3, self.buyer_deck.len());
            self.buyer_hand = self.buyer_deck.drain(0..draw_count).collect();
            self.buyer_played.clear();
        }
    }

    // SOW-009: should_customer_fold() and customer_fold() removed - Customer no longer exists
}

// ============================================================================
// SOW-009: TURN ORDER SYSTEM (Simplified from SOW-008)
// ============================================================================

/// Get turn order (SOW-009: No rotation, always Narc → Player)
/// Narc always plays first, Player always plays second
fn get_turn_order(_round: u8) -> Vec<Owner> {
    vec![Owner::Narc, Owner::Player]
}

// ============================================================================
// BETTING PHASE & INITIATIVE - SOW-002 Phase 2 (OBSOLETE - see ADR-006)
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
        // Everyone must act at least once (SOW-009: Only Narc and Player)
        let all_players_acted = self.players_acted.contains(&Owner::Narc)
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

    /// Advance to next player in turn order (SOW-009: Narc → Player)
    fn advance_turn(&mut self) {
        self.current_player = match self.current_player {
            Owner::Narc => Owner::Player,
            Owner::Buyer => unreachable!("Buyer does not use betting system"),
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

                // Record action for visual feedback (SOW-009: Only Narc and Player)
                match player {
                    Owner::Narc => self.last_action_narc = Some(BettingAction::Check),
                    Owner::Player => self.last_action_player = Some(BettingAction::Check),
                    Owner::Buyer => unreachable!("Buyer does not use betting system"),
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

                // Player must have cards (SOW-009: Only Narc and Player)
                let hand = match player {
                    Owner::Narc => &mut hand_state.narc_hand,
                    Owner::Player => &mut hand_state.player_hand,
            Owner::Buyer => unreachable!("Buyer does not use betting system"),
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

                // Record action for visual feedback (SOW-009: Only Narc and Player)
                match player {
                    Owner::Narc => self.last_action_narc = Some(actual_action),
                    Owner::Player => self.last_action_player = Some(actual_action),
                    Owner::Buyer => unreachable!("Buyer does not use betting system"),
                }

                // Mark player as acted
                if !self.players_acted.contains(&player) {
                    self.players_acted.push(player);
                }

                // SOW-005: Check if player is now all-in (hand AND deck empty - can't draw more)
                let deck = match player {
                    Owner::Narc => &hand_state.narc_deck,
                    Owner::Player => &hand_state.player_deck,
                    Owner::Buyer => unreachable!("Buyer does not use betting system"),
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
                    // SOW-009: Only Narc and Player
                    self.players_awaiting_action = vec![Owner::Narc, Owner::Player]
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
                // Record action for visual feedback (SOW-009: Only Narc and Player)
                match player {
                    Owner::Narc => self.last_action_narc = Some(BettingAction::Fold),
                    Owner::Player => self.last_action_player = Some(BettingAction::Fold),
                    Owner::Buyer => unreachable!("Buyer does not use betting system"),
                }

                // SOW-004: Apply fold penalty (remove 1 random card from folder's deck)
                let deck = match player {
                    Owner::Narc => &mut hand_state.narc_deck,
                    Owner::Player => &mut hand_state.player_deck,
                    Owner::Buyer => unreachable!("Buyer does not use betting system"),
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
        // Has cards check (SOW-009: Only Narc and Player)
        let hand = match player {
            Owner::Narc => &hand_state.narc_hand,
            Owner::Player => &hand_state.player_hand,
            Owner::Buyer => unreachable!("Buyer does not use betting system"),
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

// SOW-008: AI card play system with pacing delay
fn ai_betting_system(
    mut hand_state_query: Query<&mut HandState>,
    _betting_state_query: Query<&mut BettingState>,
    mut ai_timer: ResMut<AiActionTimer>,
    time: Res<Time>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only act during PlayerPhase
    if hand_state.current_state != State::PlayerPhase {
        return;
    }

    // Only act for AI players (Narc or Customer)
    let current_player = hand_state.current_player();
    if current_player == Owner::Player {
        return; // Player controlled manually
    }

    // SOW-008 Phase 1: Tick AI timer and act when it fires
    ai_timer.ai_timer.tick(time.delta());

    if ai_timer.ai_timer.just_finished() {
        // Timer fired - AI acts now (SOW-009: Only Narc is AI)
        let hand = match current_player {
            Owner::Narc => &hand_state.narc_hand,
            Owner::Player => return, // Player is human, not AI
            Owner::Buyer => return, // Buyer uses different system (DealerReveal state)
        };

        if !hand.is_empty() {
            println!("AI plays card after 1s delay");
            // Play first card (index 0) - play_card handles turn advance and transition
            let _ = hand_state.play_card(current_player, 0);
        } else {
            // No cards to play - skip turn
            hand_state.current_player_index += 1;
            if hand_state.all_players_acted() {
                hand_state.transition_state();
            }
        }
    }
}

// SOW-009: AI decision functions removed (obsolete betting system)
// Narc now plays cards directly via auto_play_system (no betting/raising)

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
        // SOW-009 Phase 4: Add validity and bail checks before bust check

        // Check 1: Validity (must have Product AND Location)
        if !self.is_valid_deal() {
            println!("Invalid deal: Must play at least 1 Product AND 1 Location");
            self.outcome = Some(HandOutcome::InvalidDeal);  // Can retry, not game over
            self.current_state = State::Bust;
            return HandOutcome::InvalidDeal;
        }

        // Check 2: Buyer bail (threshold exceeded)
        if self.should_buyer_bail() {
            if let Some(persona) = &self.buyer_persona {
                println!("Buyer ({}) bailed! Threshold exceeded", persona.display_name);
            }
            self.outcome = Some(HandOutcome::BuyerBailed);  // Can retry with same Buyer
            self.current_state = State::Bust;
            return HandOutcome::BuyerBailed;
        }

        let totals = self.calculate_totals(true); // Always include all cards at resolution

        // Calculate projected heat (current heat + this hand's heat)
        // This is what heat will be AFTER this hand, used for conviction checks
        let projected_heat = self.current_heat.saturating_add(totals.heat as u32);

        // Step 3: Evidence ≤ Cover → Safe (tie goes to player)
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
            HandOutcome::InvalidDeal | HandOutcome::BuyerBailed => {
                // No cash gained on invalid deal or buyer bail (deal didn't complete)
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

        // Find last Location (player or Buyer)
        // SOW-009: Buyer Location cards can override player Location cards (both use CardType::Location)
        cards.into_iter().rev().find(|card| {
            matches!(card.card_type, CardType::Location { .. })
        })
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

        // Get base Evidence/Cover from active Location (player or dealer)
        if let Some(location) = self.active_location(include_current_round) {
            match location.card_type {
                CardType::Location { evidence, cover, heat } => {
                    totals.evidence = evidence;
                    totals.cover = cover;
                    totals.heat += heat;
                }
                _ => {} // Shouldn't happen
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

        // SOW-008 Phase 2: Process revealed dealer modifier cards
        // Dealer cards are revealed AFTER PlayerPhase, so count previous rounds only
        // Round 1: No dealer cards revealed yet (0 cards)
        // After Round 1 DealerReveal: 1 dealer card revealed
        // After Round 2 DealerReveal: 2 dealer cards revealed
        // After Round 3 DealerReveal: 3 dealer cards revealed
        let revealed_dealer_cards = if self.current_state == State::DealerReveal ||
                                        self.current_state == State::Resolve ||
                                        self.current_state == State::Bust {
            // After DealerReveal or later: Include dealer card for current round
            self.current_round as usize
        } else {
            // Before DealerReveal: Only include previous rounds
            self.current_round.saturating_sub(1) as usize
        };

        // SOW-009: Dealer card processing removed, will be replaced by Buyer cards in Phase 3
        // TODO: Process buyer_played cards here instead
        for _i in 0..revealed_dealer_cards {
            // Placeholder: Buyer card effects will be added in Phase 3
        }

        // Get profit from active Product (apply multipliers)
        if let Some(product) = self.active_product(include_current_round) {
            if let CardType::Product { price, heat } = product.card_type {
                // SOW-009 Phase 4: Apply Buyer multiplier in addition to DealModifier multipliers
                let buyer_multiplier = self.get_profit_multiplier();
                totals.profit = (price as f32 * price_multiplier * buyer_multiplier) as u32;
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

// SOW-009: create_customer_deck() removed - replaced by Buyer reaction decks in Phase 2
// Customer deck is obsolete (merged into Buyer persona system)

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
// SOW-009 PHASE 2: BUYER PERSONAS AND REACTION DECKS
// ============================================================================

/// Create all available Buyer personas (MVP: 3 personas)
fn create_buyer_personas() -> Vec<BuyerPersona> {
    vec![
        create_college_party_host(),
        create_stay_at_home_mom(),
        create_executive(),
    ]
}

/// Buyer Persona 1: College Party Host
/// High profit (×2.5), no threshold (won't bail), high Evidence risk
fn create_college_party_host() -> BuyerPersona {
    let mut id = 2000; // Start Buyer cards at 2000

    BuyerPersona {
        id: "college_party_host".to_string(),
        display_name: "College Party Host".to_string(),
        demand: BuyerDemand {
            products: vec!["Weed".to_string(), "Pills".to_string()],
            locations: vec!["Dorm".to_string(), "Party".to_string(), "Park".to_string()],
            description: "Wants Weed or Pills, high volume, public locations".to_string(),
        },
        base_multiplier: 2.5,
        reduced_multiplier: 1.0,
        heat_threshold: None,  // Not paranoid, won't bail
        evidence_threshold: None,
        special_rules: vec![],  // TODO: Add "+10 Evidence if public Location" in future phase
        reaction_deck: vec![
            // 1. Increase Evidence
            Card {
                id: id,
                name: "Cops Called".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 20, cover: 0, heat: 5 },
            },
            // 2. Increase Cover
            { id += 1; Card {
                id: id,
                name: "VIP Room".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 15, heat: -5 },
            }},
            // 3. Change Location
            { id += 1; Card {
                id: id,
                name: "Move to Dorm".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::Location { evidence: 10, cover: 5, heat: 10 },
            }},
            // 4. Volume/Price Up
            { id += 1; Card {
                id: id,
                name: "Invite More People".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.5, evidence: 15, cover: 0, heat: 10 },
            }},
            // 5. Volume/Price Down
            { id += 1; Card {
                id: id,
                name: "Party Shutdown".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 0.7, evidence: 5, cover: 0, heat: -5 },
            }},
            // 6. Thematic card (Heat focus)
            { id += 1; Card {
                id: id,
                name: "Word of Mouth".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 5, heat: 15 },
            }},
            // 7. Thematic card (Mixed)
            { id += 1; Card {
                id: id,
                name: "Noise Complaint".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 10, cover: 0, heat: 10 },
            }},
        ],
    }
}

/// Buyer Persona 2: Stay-at-Home Mom
/// Low profit (×1.2), paranoid (Heat < 30), private only
fn create_stay_at_home_mom() -> BuyerPersona {
    let mut id = 2100; // Mom cards start at 2100

    BuyerPersona {
        id: "stay_at_home_mom".to_string(),
        display_name: "Stay-at-Home Mom".to_string(),
        demand: BuyerDemand {
            products: vec!["Pills".to_string()],
            locations: vec!["Private Residence".to_string(), "Warehouse".to_string()],
            description: "Wants Pills only, private locations only".to_string(),
        },
        base_multiplier: 1.2,
        reduced_multiplier: 1.0,
        heat_threshold: Some(30),  // Paranoid, bails if Heat > 30
        evidence_threshold: None,
        special_rules: vec![],
        reaction_deck: vec![
            // 1. Increase Evidence
            Card {
                id: id,
                name: "Kids Are Watching".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 15, cover: 0, heat: 5 },
            },
            // 2. Increase Cover
            { id += 1; Card {
                id: id,
                name: "Safe Exchange".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 20, heat: -5 },
            }},
            // 3. Change Location
            { id += 1; Card {
                id: id,
                name: "Move Inside".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::Location { evidence: 5, cover: 15, heat: -5 },
            }},
            // 4. Volume/Price Up
            { id += 1; Card {
                id: id,
                name: "Needs Extra".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.3, evidence: 10, cover: 5, heat: 5 },
            }},
            // 5. Volume/Price Down
            { id += 1; Card {
                id: id,
                name: "Can't Afford It".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 0.8, evidence: 5, cover: 5, heat: 0 },
            }},
            // 6. Thematic card (Paranoia/Heat)
            { id += 1; Card {
                id: id,
                name: "Paranoid Check".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 5, cover: 0, heat: 15 },
            }},
            // 7. Thematic card (Panic)
            { id += 1; Card {
                id: id,
                name: "Panic Attack".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 10, cover: 0, heat: 20 },
            }},
        ],
    }
}

/// Buyer Persona 3: Executive
/// Highest profit (×2.8), very paranoid (Heat < 25), private only
fn create_executive() -> BuyerPersona {
    let mut id = 2200; // Executive cards start at 2200

    BuyerPersona {
        id: "executive".to_string(),
        display_name: "Executive".to_string(),
        demand: BuyerDemand {
            products: vec!["Pills".to_string()],
            locations: vec!["Private Residence".to_string(), "Office".to_string()],
            description: "Wants premium Pills, private only, very paranoid".to_string(),
        },
        base_multiplier: 2.8,  // Highest profit in game
        reduced_multiplier: 1.0,
        heat_threshold: Some(25),  // Very paranoid, bails easily
        evidence_threshold: None,
        special_rules: vec![],
        reaction_deck: vec![
            // 1. Increase Evidence
            Card {
                id: id,
                name: "Security Check".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 15, cover: 0, heat: 10 },
            },
            // 2. Increase Cover
            { id += 1; Card {
                id: id,
                name: "Discrete Meeting".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 0, cover: 25, heat: -10 },
            }},
            // 3. Change Location
            { id += 1; Card {
                id: id,
                name: "Move to Office".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::Location { evidence: 5, cover: 20, heat: -5 },
            }},
            // 4. Volume/Price Up
            { id += 1; Card {
                id: id,
                name: "Expensive Taste".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.8, evidence: 5, cover: 10, heat: 5 },
            }},
            // 5. Volume/Price Down
            { id += 1; Card {
                id: id,
                name: "Budget Conscious".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 0.6, evidence: 0, cover: 15, heat: -5 },
            }},
            // 6. Thematic card (Background check)
            { id += 1; Card {
                id: id,
                name: "Background Check".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 10, cover: 5, heat: 20 },
            }},
            // 7. Thematic card (Disruption)
            { id += 1; Card {
                id: id,
                name: "Assistant Interrupt".to_string(),
                owner: Owner::Buyer,
                card_type: CardType::DealModifier { price_multiplier: 1.0, evidence: 5, cover: 0, heat: 15 },
            }},
        ],
    }
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

// SOW-009: create_dealer_deck() removed - replaced by Buyer reaction decks

// ============================================================================
// SOW-008 PHASE 2: DEALER DECK CREATION (REMOVED IN SOW-009)
// ============================================================================

// Dealer deck function removed - ~140 lines of obsolete code deleted

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

        // SOW-009: Test Buyer personas instead of customer deck
        let buyer_personas = create_buyer_personas();
        assert_eq!(buyer_personas.len(), 3); // MVP: 3 personas

        // Verify each persona has 7 reaction cards
        for persona in buyer_personas.iter() {
            assert_eq!(persona.reaction_deck.len(), 7);
        }

        // Verify personas have distinct identities
        assert_eq!(buyer_personas[0].id, "college_party_host");
        assert_eq!(buyer_personas[1].id, "stay_at_home_mom");
        assert_eq!(buyer_personas[2].id, "executive");

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

        // SOW-008: Draw → PlayerPhase → DealerReveal → Draw (next round)
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::PlayerPhase);

        // After PlayerPhase completes
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::DealerReveal);

        // After DealerReveal (Round 1), auto-advance to next round
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.current_round, 2);
    }

    #[test]
    fn test_reset() {
        let mut hand_state = HandState::default();

        // Modify state (SOW-008: Draw → PlayerPhase)
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::PlayerPhase);

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

        // SOW-002/009: Draw to hand size 3 (multi-round play, 2 players now)
        assert_eq!(hand_state.narc_hand.len(), 3);
        assert_eq!(hand_state.player_hand.len(), 3);
        // SOW-009: Customer removed

        // State should advance to PlayerPhase (SOW-008)
        assert_eq!(hand_state.current_state, State::PlayerPhase);
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

        // State is PlayerPhase after draw (SOW-008)
        assert_eq!(hand_state.current_state, State::PlayerPhase);
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

        // SOW-009: Need Product for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location with high Evidence, low Cover
        let location = Card {
            id: 2,
            name: "School Zone".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 40, cover: 5, heat: 20 },
        };
        hand_state.cards_played.push(location);

        // Add more Evidence
        let evidence = Card {
            id: 3,
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

        // SOW-009: Need Product for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location with low Evidence, high Cover
        let location = Card {
            id: 2,
            name: "Safe House".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 10, cover: 30, heat: -5 },
        };
        hand_state.cards_played.push(location);

        // Add Cover
        let cover = Card {
            id: 3,
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

        // SOW-009: Need Product + Location for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location with equal Evidence and Cover
        let location = Card {
            id: 2,
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

        // SOW-009: Need Product for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location
        let location = Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Add 1 Evidence (31 > 30 → Busted)
        let evidence = Card {
            id: 3,
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

        // SOW-009: Need Product for valid deal
        let product = Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        };
        hand_state.cards_played.push(product);

        // Location
        let location = Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 30, heat: 0 },
        };
        hand_state.cards_played.push(location);

        // Add 1 Cover (30 ≤ 31 → Safe)
        let cover = Card {
            id: 3,
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

        // SOW-008: Draw → PlayerPhase → DealerReveal → Draw (next round)
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::PlayerPhase);
        assert_eq!(hand_state.current_round, 1);

        // After all players act, go to DealerReveal
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::DealerReveal);

        // After DealerReveal (Round 1), auto-advance to next round
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.current_round, 2); // Round advances
    }

    // SOW-008: test_continue_to_next_round and test_fold_at_decision_point removed
    // These methods don't exist in sequential play - rounds advance automatically

    // SOW-008 Phase 3: Fold mechanic tests

    #[test]
    fn test_player_fold() {
        let mut hand_state = HandState::default();
        hand_state.current_state = State::PlayerPhase;
        hand_state.current_round = 1;

        // Play some cards
        hand_state.cards_played.push(Card {
            id: 1, name: "Test".to_string(), owner: Owner::Player,
            card_type: CardType::Evidence { evidence: 10, heat: 5 },
        });
        hand_state.player_hand.push(Card {
            id: 2, name: "Unplayed".to_string(), owner: Owner::Player,
            card_type: CardType::Cover { cover: 20, heat: 0 },
        });

        let initial_played_count = hand_state.cards_played.len();
        let initial_hand_count = hand_state.player_hand.len();

        // Simulate fold during PlayerPhase (like clicking Fold button)
        hand_state.cards_played.clear();
        hand_state.outcome = Some(HandOutcome::Folded);
        hand_state.current_state = State::Bust;

        // Verify fold consequences
        assert_eq!(hand_state.outcome, Some(HandOutcome::Folded));
        assert_eq!(hand_state.current_state, State::Bust); // Terminal
        assert_eq!(hand_state.cards_played.len(), 0); // Played cards discarded
        assert_eq!(hand_state.player_hand.len(), initial_hand_count); // Unplayed cards kept
    }

    // SOW-009: test_customer_fold_removes_cards OBSOLETE (Customer removed)
    // TODO: Add test for Buyer bail mechanics
    /*
    #[test]
    fn test_customer_fold_removes_cards() {
        // OBSOLETE: Customer fold no longer exists
    */

    // SOW-009: test_dealer_deck_creation OBSOLETE (dealer deck removed)
    // TODO: Add test for Buyer persona creation

    // SOW-009: test_dealer_hand_initialization OBSOLETE (dealer deck removed)
    // TODO: Add test for buyer_hand initialization

    #[test]
    fn test_turn_order_simplified() {
        // SOW-009: Turn order simplified to always Narc → Player
        let order1 = get_turn_order(1);
        assert_eq!(order1, vec![Owner::Narc, Owner::Player]);

        let order2 = get_turn_order(2);
        assert_eq!(order2, vec![Owner::Narc, Owner::Player]);

        // All rounds have same order (no rotation)
        let order3 = get_turn_order(3);
        assert_eq!(order3, vec![Owner::Narc, Owner::Player]);
    }

    #[test]
    fn test_current_player_tracking() {
        let mut hand_state = HandState::default();
        hand_state.current_state = State::PlayerPhase;
        hand_state.current_round = 1;

        // SOW-009: Turn order simplified to 2 players
        // Round 1 starts with Narc
        assert_eq!(hand_state.current_player(), Owner::Narc);
        assert!(!hand_state.all_players_acted()); // 0/2 acted

        // Advance to Player
        hand_state.advance_turn();
        assert_eq!(hand_state.current_player(), Owner::Player);
        assert!(!hand_state.all_players_acted()); // 1/2 acted

        // Advance past Player
        hand_state.advance_turn();
        // current_player_index is now 2 (>= 2)
        assert!(hand_state.all_players_acted()); // 2/2 acted
    }

    #[test]
    fn test_round_3_goes_to_resolve() {
        let mut hand_state = HandState::default();

        // Set to Round 3 DealerReveal
        hand_state.current_state = State::DealerReveal;
        hand_state.current_round = 3;

        // Transition should go to Resolve (no next round after Round 3)
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Resolve);
    }

    #[test]
    fn test_round_1_advances_to_round_2() {
        let mut hand_state = HandState::default();

        // Set to Round 1 DealerReveal
        hand_state.current_state = State::DealerReveal;
        hand_state.current_round = 1;

        // SOW-008: DealerReveal auto-advances to next round
        hand_state.transition_state();
        assert_eq!(hand_state.current_state, State::Draw);
        assert_eq!(hand_state.current_round, 2);
    }

    #[test]
    fn test_full_three_round_flow() {
        let mut hand_state = HandState::default();

        // SOW-008: Round 1: Draw → PlayerPhase → DealerReveal → Draw (R2)
        assert_eq!(hand_state.current_round, 1);
        hand_state.transition_state(); // → PlayerPhase
        assert_eq!(hand_state.current_state, State::PlayerPhase);

        hand_state.transition_state(); // → DealerReveal
        assert_eq!(hand_state.current_state, State::DealerReveal);

        hand_state.transition_state(); // → Draw (Round 2)
        assert_eq!(hand_state.current_round, 2);
        assert_eq!(hand_state.current_state, State::Draw);

        // Round 2: Draw → PlayerPhase → DealerReveal → Draw (R3)
        hand_state.transition_state(); // → PlayerPhase
        hand_state.transition_state(); // → DealerReveal
        hand_state.transition_state(); // → Draw (Round 3)
        assert_eq!(hand_state.current_round, 3);
        assert_eq!(hand_state.current_state, State::Draw);

        // Round 3: Draw → PlayerPhase → DealerReveal → Resolve
        hand_state.transition_state(); // → PlayerPhase
        hand_state.transition_state(); // → DealerReveal
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

        // Set raises to 3 and mark everyone as acted (SOW-009: Only Narc and Player)
        betting_state.raises_this_round = 3;
        betting_state.players_acted = vec![Owner::Narc, Owner::Player];
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

        // Mark all players as acted (everyone Checked) - SOW-009: Only Narc and Player
        betting_state.players_acted = vec![Owner::Narc, Owner::Player];

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

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Cost $1000, Heat +20
        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Evidence > Cover, but insurance should save us
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        // SOW-009: Cash = initial (1500) - insurance cost (1000) + profit (30) = 530
        assert_eq!(hand_state.cash, 530);
        // SOW-009: Heat = Product heat (5) + penalty (20) = 25 (not 20)
        assert_eq!(hand_state.current_heat, 25);
    }

    #[test]
    fn test_insurance_activation_unaffordable() {
        let mut hand_state = HandState::default();
        hand_state.cash = 500; // Not enough cash

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Cost $1000 (too expensive)
        hand_state.cards_played.push(Card {
            id: 3,
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

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20, no insurance card
        hand_state.cards_played.push(Card {
            id: 2,
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

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Available and affordable
        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Conviction: Threshold 40 (we're at 50, so it activates)
        hand_state.cards_played.push(Card {
            id: 4,
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

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        // Insurance: Available and affordable
        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        // Conviction: Threshold 40 (we're at 30, so it doesn't activate)
        hand_state.cards_played.push(Card {
            id: 4,
            name: "Warrant".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Evidence > Cover, heat < threshold, insurance works → Safe
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Safe);
        // SOW-009: Cash = initial (2000) - insurance (1000) + profit (30) = 1030
        assert_eq!(hand_state.cash, 1030);
        // SOW-009: Heat = Product heat (5) + penalty (20) = 55 (not 50)
        assert_eq!(hand_state.current_heat, 55);
    }

    #[test]
    fn test_conviction_at_threshold_activates() {
        let mut hand_state = HandState::default();
        hand_state.cash = 2000;
        hand_state.current_heat = 40; // Exactly at threshold

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 0 },
        });

        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        hand_state.cards_played.push(Card {
            id: 4,
            name: "Warrant".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 40 },
        });

        // Heat >= threshold, conviction activates (boundary test)
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted);
        // SOW-009: Heat includes Product heat (5)
        assert_eq!(hand_state.current_heat, 45); // 40 + 5 from product
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

        // SOW-009: Need Product for valid deal
        hand_state.cards_played.push(Card {
            id: 1,
            name: "Weed".to_string(),
            owner: Owner::Player,
            card_type: CardType::Product { price: 30, heat: 5 },
        });

        // Bust scenario: E:30 C:20, this hand adds +30 heat
        hand_state.cards_played.push(Card {
            id: 2,
            name: "Location".to_string(),
            owner: Owner::Player,
            card_type: CardType::Location { evidence: 30, cover: 20, heat: 30 }, // +30 heat
        });

        hand_state.cards_played.push(Card {
            id: 3,
            name: "Plea Bargain".to_string(),
            owner: Owner::Player,
            card_type: CardType::Insurance { cover: 5, cost: 1000, heat_penalty: 20 },
        });

        hand_state.cards_played.push(Card {
            id: 4,
            name: "DA Approval".to_string(),
            owner: Owner::Player,
            card_type: CardType::Conviction { heat_threshold: 60 },
        });

        // Projected heat: 40 + 30 (location) + 5 (product) = 75, which is >= 60, so conviction should activate
        let outcome = hand_state.resolve_hand();
        assert_eq!(outcome, HandOutcome::Busted); // Conviction blocks insurance
        assert_eq!(hand_state.cash, 2000); // Cash unchanged (conviction blocked insurance)
        assert_eq!(hand_state.current_heat, 75); // Heat accumulated correctly (40 + 30 + 5)
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
        let initial_player = hand_state.player_deck.len();
        // SOW-009: Customer deck removed

        // Draw cards for all players
        hand_state.draw_cards();

        // Shuffle back
        hand_state.shuffle_cards_back();

        // All decks should be restored (SOW-009: Only Narc and Player)
        assert_eq!(hand_state.narc_deck.len(), initial_narc);
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
