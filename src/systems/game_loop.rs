// SOW-AAA: Game loop systems
// Extracted from main.rs

use bevy::prelude::*;
use crate::{Owner, HandState, HandPhase};
use crate::game_state::AiActionTimer;
use crate::ui::setup::create_ui;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // SOW-006: Don't spawn HandState at startup - only when START RUN is pressed
    // HandState will be created when transitioning from DeckBuilding to InRun

    // Create gameplay UI root (initially hidden)
    create_ui(&mut commands);
}

pub fn auto_flip_system(
    mut hand_state_query: Query<&mut HandState>,
    mut ai_timer: ResMut<AiActionTimer>,
    time: Res<Time>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Auto-draw cards when entering Draw state
    if hand_state.current_state == HandPhase::Draw {
        // Draw cards for all players
        hand_state.draw_cards();
        // Note: draw_cards() calls transition_state() → PlayerPhase
    }

    // SOW-009 Phase 3: Buyer card play (replaces SOW-008 dealer reveal)
    if hand_state.current_state == HandPhase::DealerReveal {
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
    if hand_state.current_state == HandPhase::Resolve {
        println!("Auto-resolving hand...");
        let outcome = hand_state.resolve_hand();
        println!("Resolution outcome: {:?}, new state: {:?}", outcome, hand_state.current_state);
    }
}

pub fn ai_betting_system(
    mut hand_state_query: Query<&mut HandState>,
    mut ai_timer: ResMut<AiActionTimer>,
    time: Res<Time>,
) {
    let Ok(mut hand_state) = hand_state_query.get_single_mut() else {
        return;
    };

    // Only act during PlayerPhase
    if hand_state.current_state != HandPhase::PlayerPhase {
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
