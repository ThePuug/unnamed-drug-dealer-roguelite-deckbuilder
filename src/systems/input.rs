// SOW-AAA: Input and button systems
// Extracted from main.rs

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::{Owner, HandState, HandPhase, HandOutcome, DeckBuilder};
use crate::game_state::GameState;
use crate::ui::components::*;
use crate::ui::theme;
use crate::data::create_buyer_personas;

// ============================================================================
// SOW-008: BETTING BUTTON SYSTEM
// ============================================================================
// Check and Fold buttons during PlayerPhase
pub fn betting_button_system(
    check_query: Query<&Interaction, (Changed<Interaction>, With<CheckButton>)>,
    fold_query: Query<&Interaction, (Changed<Interaction>, With<FoldButton>)>,
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only during PlayerPhase and when it's Player's turn
    if hand_state.current_state != HandPhase::PlayerPhase || hand_state.current_player() != Owner::Player {
        return;
    }

    // Check button - skip playing a card this turn
    for interaction in check_query.iter() {
        if *interaction == Interaction::Pressed {
            let current_round = hand_state.current_round;
            println!("Player checks (skips card) in Round {current_round}");

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

            // SOW-012: Generate story before folding
            hand_state.generate_hand_story(HandOutcome::Folded);

            // Fold immediately - discard played cards, keep unplayed, exit hand
            hand_state.cards_played.clear();
            hand_state.outcome = Some(HandOutcome::Folded);
            hand_state.current_state = HandPhase::Bust;
        }
    }
}

// ============================================================================
// UPDATE BETTING BUTTON STATES
// ============================================================================
pub fn update_betting_button_states(
    hand_state_query: Query<&HandState>,
    mut check_button_query: Query<&mut BackgroundColor, (With<CheckButton>, Without<FoldButton>)>,
    mut fold_button_query: Query<&mut BackgroundColor, (With<FoldButton>, Without<CheckButton>)>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // SOW-011-B: Buttons enabled only during PlayerPhase when it's player's turn, disabled otherwise
    let is_player_turn = hand_state.current_state == HandPhase::PlayerPhase &&
                         hand_state.current_player() == Owner::Player;

    // Update Pass button (Check) - enabled when player's turn, disabled otherwise
    let mut bg = check_button_query.get_single_mut()
        .expect("Expected exactly one CheckButton");
    *bg = if is_player_turn {
        theme::BUTTON_ENABLED_BG.into()
    } else {
        theme::BUTTON_DISABLED_BG.into()
    };

    // Update Bail Out button (Fold) - enabled when player's turn, disabled otherwise
    let mut bg = fold_button_query.get_single_mut()
        .expect("Expected exactly one FoldButton");
    *bg = if is_player_turn {
        theme::BUTTON_NEUTRAL_BG.into()
    } else {
        theme::BUTTON_DISABLED_BG.into()
    };
}

// ============================================================================
// RESTART BUTTON SYSTEM
// ============================================================================
pub fn restart_button_system(
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != HandPhase::Bust {
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
            if hand_state.cards(Owner::Player).deck.len() < 3 {
                // Button disabled, ignore click
                return;
            }

            // Start next hand (preserve cash/heat)
            let _can_continue = hand_state.start_next_hand();
        }
    }
}

// ============================================================================
// UPDATE RESTART BUTTON STATES
// ============================================================================
pub fn update_restart_button_states(
    hand_state_query: Query<&HandState>,
    mut restart_button_query: Query<(&mut BackgroundColor, &mut Visibility), With<RestartButton>>,
    go_home_button_query: Query<(Entity, &Children), With<GoHomeButton>>,
    mut text_query: Query<&mut Text>,
) {
    let Ok(hand_state) = hand_state_query.get_single() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != HandPhase::Bust {
        return;
    }

    let is_busted = matches!(hand_state.outcome, Some(HandOutcome::Busted));

    // NEW DEAL button: Hide if busted, disable if deck exhausted
    let (mut bg_color, mut visibility) = restart_button_query
        .get_single_mut()
        .expect("Expected exactly one RestartButton in resolution overlay");

    if is_busted {
        // Busted: Hide NEW DEAL button entirely
        *visibility = Visibility::Hidden;
    } else {
        // Safe/Folded: Show NEW DEAL, disable if deck exhausted
        *visibility = Visibility::Visible;
        let can_deal = hand_state.cards(Owner::Player).deck.len() >= 3;
        *bg_color = if can_deal {
            theme::BUTTON_ENABLED_BG.into()
        } else {
            theme::BUTTON_DISABLED_BG.into()
        };
    }

    // GO HOME button text: "GO HOME" if safe, "END RUN" if busted
    let (_button_entity, children) = go_home_button_query
        .get_single()
        .expect("Expected exactly one GoHomeButton in resolution overlay");

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

// ============================================================================
// GO HOME BUTTON SYSTEM
// ============================================================================
pub fn go_home_button_system(
    mut commands: Commands,
    go_home_query: Query<&Interaction, (Changed<Interaction>, With<GoHomeButton>)>,
    hand_state_query: Query<(Entity, &HandState)>,
    mut next_state: ResMut<NextState<GameState>>,
    game_assets: Res<crate::assets::GameAssets>, // SOW-013-B: Need for DeckBuilder::from_assets
) {
    let Ok((entity, hand_state)) = hand_state_query.get_single() else {
        return;
    };

    // Only at Bust state
    if hand_state.current_state != HandPhase::Bust {
        return;
    }

    // Go Home button - return to deck builder
    for interaction in go_home_query.iter() {
        if *interaction == Interaction::Pressed {
            // SOW-013-B: Collect all cards from HandState before despawning
            let mut player_cards = hand_state.owner_cards.get(&Owner::Player)
                .expect("Player cards not found")
                .clone();

            // Collect all cards (hand + deck + played) back into deck
            player_cards.collect_all();

            // Update DeckBuilder: available from assets, selected from your run
            let mut deck_builder = DeckBuilder::from_assets(&game_assets);
            deck_builder.selected_cards = player_cards.deck; // Cards you just played with
            commands.insert_resource(deck_builder);

            // Despawn HandState
            commands.entity(entity).despawn();

            // Transition back to DeckBuilding state
            next_state.set(GameState::DeckBuilding);
        }
    }
}

// ============================================================================
// DECK BUILDER SYSTEMS
// ============================================================================
pub fn deck_builder_card_click_system(
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

pub fn preset_button_system(
    interaction_query: Query<(&Interaction, &PresetButton), Changed<Interaction>>,
    mut deck_builder: ResMut<DeckBuilder>,
) {
    for (interaction, preset_btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            deck_builder.load_preset(preset_btn.preset);
        }
    }
}

pub fn start_run_button_system(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartRunButton>)>,
    deck_builder: Res<DeckBuilder>,
    mut next_state: ResMut<NextState<GameState>>,
    hand_state_query: Query<Entity, With<HandState>>,
    game_assets: Res<crate::assets::GameAssets>, // SOW-013-B: Need loaded assets for buyer/narc deck
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed && deck_builder.is_valid() {
            // Despawn any existing HandState
            for entity in hand_state_query.iter() {
                commands.entity(entity).despawn();
            }

            // SOW-013-B: Select random Buyer persona from loaded assets
            let buyer_personas = create_buyer_personas(&game_assets);
            let mut random_buyer = buyer_personas.choose(&mut rand::thread_rng()).unwrap().clone();

            // SOW-010: Randomly select one of the Buyer's 2 scenarios
            if !random_buyer.scenarios.is_empty() {
                let scenario_index = rand::thread_rng().gen_range(0..random_buyer.scenarios.len());
                random_buyer.active_scenario_index = Some(scenario_index);
            }

            // Create new HandState with selected deck
            let mut hand_state = HandState::with_custom_deck(deck_builder.selected_cards.clone(), &game_assets);
            hand_state.buyer_persona = Some(random_buyer);
            hand_state.draw_cards(); // This will also initialize buyer hand
            commands.spawn(hand_state);

            // Transition to InRun state
            next_state.set(GameState::InRun);
        }
    }
}

// ============================================================================
// CARD CLICK SYSTEM
// ============================================================================
pub fn card_click_system(
    mut interaction_query: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut hand_state_query: Query<&mut HandState>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    for (interaction, card_button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            println!("Card clicked! Index: {}, State: {:?}", card_button.card_index, hand_state.current_state);

            // SOW-008: Clicking card during PlayerPhase plays it immediately
            if hand_state.current_state == HandPhase::PlayerPhase {
                // Only if it's Player's turn
                if hand_state.current_player() == Owner::Player {
                    // Verify valid card index
                    let player_hand: Vec<_> = hand_state.cards(Owner::Player).into();
                    if card_button.card_index < player_hand.len() {
                        println!("Player playing card {}", card_button.card_index);

                        // Play the card face-up immediately
                        let _ = hand_state.play_card(Owner::Player, card_button.card_index);
                    }
                }
            }
        }
    }
}
